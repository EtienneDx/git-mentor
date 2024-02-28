use std::{collections::HashMap, pin::Pin, sync::Arc};

use async_trait::async_trait;
use log::debug;
use russh::{
  server::{Auth, Handler, Msg, Session},
  Channel, ChannelId, CryptoVec,
};
use russh_keys::key::PublicKey;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::{
  authenticator::Authenticator,
  error::SshError,
  handler::HandlerResult,
  user::User,
  wrapper::{HandleWrapper, WrappedHandle},
};

pub struct RequestHandler<A, U, CId, HW>
where
  A: Authenticator<User = U>,
  U: User,
  CId: 'static,
  HW: HandleWrapper<ChannelId = CId> + 'static,
{
  authenticator: Arc<A>,
  handlers:
    Vec<Arc<Box<dyn crate::handler::Handler<ChannelId = CId, HandleWrapper = HW, User = U>>>>,
  user: Option<A::User>,
  processes: HashMap<ChannelId, Pin<Box<dyn AsyncWrite + Sync + Send + 'static>>>,
}

impl<A, U, CId, HW> RequestHandler<A, U, CId, HW>
where
  A: Authenticator<User = U>,
  U: User,
  CId: Copy + 'static,
  HW: HandleWrapper<ChannelId = CId> + 'static,
{
  pub fn new(
    authenticator: Arc<A>,
    handlers: Vec<
      Arc<Box<dyn crate::handler::Handler<ChannelId = CId, HandleWrapper = HW, User = U>>>,
    >,
    _: Option<std::net::SocketAddr>,
  ) -> Self {
    RequestHandler {
      authenticator,
      handlers,
      user: None,
      processes: HashMap::new(),
    }
  }

  fn handle(&self, handle: HW, channel_id: CId, command: &str) -> HandlerResult {
    if let Some(user) = &self.user {
      for handler in &self.handlers {
        let result = handler.handle(user, handle.clone(), channel_id, command);
        match result {
          HandlerResult::Accepted(stdin) => {
            return HandlerResult::Accepted(stdin);
          }
          HandlerResult::Rejected(message) => {
            return HandlerResult::Rejected(message);
          }
          HandlerResult::Skipped => {}
        }
      }
    }
    HandlerResult::Skipped
  }
}

#[async_trait]
impl<A, U> Handler for RequestHandler<A, U, ChannelId, WrappedHandle>
where
  A: Authenticator<User = U>,
  U: User,
{
  type Error = SshError;

  /// Authenticates the user based on their public key.
  async fn auth_publickey(
    mut self,
    user: &str,
    key: &PublicKey,
  ) -> Result<(Self, Auth), Self::Error> {
    if self.user.is_some() {
      // We shouldn't be able to authenticate twice
      return Err(SshError::AlreadyAuthenticated);
    }
    if let Some(user) = self.authenticator.validate_public_key(user, key)? {
      self.user = Some(user);
      Ok((self, Auth::Accept))
    } else {
      Ok((
        self,
        Auth::Reject {
          proceed_with_methods: None,
        },
      ))
    }
  }

  /// Opens a new session. Required for russh to accept the ssh connection.
  async fn channel_open_session(
    mut self,
    _channel: Channel<Msg>,
    session: Session,
  ) -> Result<(Self, bool, Session), Self::Error> {
    Ok((self, true, session))
  }

  /// Executes a ssh command. This is where the git command is received.
  ///
  /// Should any new commands become supported, they should be added here.
  async fn exec_request(
    mut self,
    channel_id: ChannelId,
    data: &[u8],
    session: Session,
  ) -> Result<(Self, Session), Self::Error> {
    if self.user.is_none() {
      // We shouldn't be able to authenticate twice
      return Err(SshError::NotAuthenticated);
    }
    debug!("Executing function, trying to start git process");
    let handle = WrappedHandle(session.handle());

    if let Ok(data_str) = std::str::from_utf8(data) {
      let response = self.handle(handle.clone(), channel_id, data_str);

      match response {
        HandlerResult::Accepted(stdin) => {
          self.processes.insert(channel_id, stdin);
        }
        HandlerResult::Rejected(message) => {
          send_error(&handle, channel_id, &message).await;
          handle.close(channel_id).await.unwrap();
        }
        HandlerResult::Skipped => {
          send_error(&handle, channel_id, "Command not supported").await;
          handle.close(channel_id).await.unwrap();
        }
      }
    } else {
      debug!("Invalid UTF-8 exec received: {:?}", data);
      send_error(&handle, channel_id, "Invalid command received").await;
      handle.close(channel_id).await.unwrap();
    }

    Ok((self, session))
  }

  /// Receives data from the client and forwards it to the git process.
  async fn data(
    mut self,
    channel_id: ChannelId,
    data: &[u8],
    session: Session,
  ) -> Result<(Self, Session), Self::Error> {
    if let Some(process) = self.processes.get_mut(&channel_id) {
      process.write_all(data).await.map_err(|e| {
        debug!("Error writing to git process: {:?}", e);
        SshError::ProcessNotStartedError
      })?;
    }
    Ok((self, session))
  }

  /// Receives an EOF from the client and stops the git process.
  async fn channel_eof(
    mut self,
    channel_id: ChannelId,
    session: Session,
  ) -> Result<(Self, Session), Self::Error> {
    let process = self.processes.remove(&channel_id);
    if let Some(mut process) = process {
      process.shutdown().await?;
    }

    Ok((self, session))
  }
}

/// Util function to send an error message to the client.
async fn send_error<CId, HW>(handle: &HW, channel_id: CId, message: &str)
where
  HW: HandleWrapper<ChannelId = CId>,
{
  let message = CryptoVec::from_slice(message.as_bytes());
  if handle.extended_data(channel_id, 1, message).await.is_err() {
    debug!("Failed to send error message");
  }
}

#[cfg(test)]
mod test {
  use crate::test_utils::{
    MockHandle, SimpleAuthenticator, SimpleHandler, SimpleHandlerResult, User,
  };

  use super::*;

  #[test]
  fn test_skipped_if_no_user() {
    let handler = RequestHandler::new(
      Arc::new(SimpleAuthenticator),
      vec![Arc::new(Box::new(SimpleHandler::<u32, MockHandle>(
        SimpleHandlerResult::Accepted,
        std::marker::PhantomData,
      )))],
      None,
    );
    let handle = MockHandle;
    let result = handler.handle(handle, 0, "test");
    assert!(
      matches!(result, HandlerResult::Skipped),
      "Expected HandlerResult::Skipped, got {:?}",
      result
    );
  }

  #[test]
  fn test_accept_handle() {
    let mut handler = RequestHandler::new(
      Arc::new(SimpleAuthenticator),
      vec![Arc::new(Box::new(SimpleHandler::<u32, MockHandle>(
        SimpleHandlerResult::Accepted,
        std::marker::PhantomData,
      )))],
      None,
    );
    let handle = MockHandle;
    handler.user = Some(User);
    let result = handler.handle(handle, 0, "test");
    assert!(
      matches!(result, HandlerResult::Accepted(_)),
      "Expected HandlerResult::Accepted, got {:?}",
      result
    );
  }

  #[test]
  fn test_reject_handle() {
    let mut handler = RequestHandler::new(
      Arc::new(SimpleAuthenticator),
      vec![Arc::new(Box::new(SimpleHandler::<u32, MockHandle>(
        SimpleHandlerResult::Rejected,
        std::marker::PhantomData,
      )))],
      None,
    );
    let handle = MockHandle;
    handler.user = Some(User);
    let result = handler.handle(handle, 0, "test");
    assert!(
      matches!(result, HandlerResult::Rejected(_)),
      "Expected HandlerResult::Rejected, got {:?}",
      result
    );
  }

  #[test]
  fn test_skip_handle() {
    let mut handler = RequestHandler::new(
      Arc::new(SimpleAuthenticator),
      vec![Arc::new(Box::new(SimpleHandler::<u32, MockHandle>(
        SimpleHandlerResult::Skipped,
        std::marker::PhantomData,
      )))],
      None,
    );
    let handle = MockHandle;
    handler.user = Some(User);
    let result = handler.handle(handle, 0, "test");
    assert!(matches!(result, HandlerResult::Skipped));
  }
}
