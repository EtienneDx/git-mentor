use std::{collections::HashMap, pin::Pin, sync::Arc};

use async_trait::async_trait;
use log::debug;
use russh::{
  server::{Auth, Handle, Handler, Msg, Session},
  Channel, ChannelId, CryptoVec,
};
use russh_keys::key::PublicKey;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::{authenticator::Authenticator, error::SshError, handler::HandlerResult, user::User};

pub struct RequestHandler<A, U>
where
  A: Authenticator<User = U>,
  U: User,
{
  authenticator: Arc<A>,
  handlers: Vec<Arc<Box<dyn crate::handler::Handler<User = U>>>>,
  user: Option<A::User>,
  git_process: HashMap<ChannelId, Pin<Box<dyn AsyncWrite + Sync + Send + 'static>>>,
}

impl<A, U> RequestHandler<A, U>
where
  A: Authenticator<User = U>,
  U: User,
{
  pub fn new(
    authenticator: Arc<A>,
    handlers: Vec<Arc<Box<dyn crate::handler::Handler<User = U>>>>,
    _: Option<std::net::SocketAddr>,
  ) -> Self {
    RequestHandler {
      authenticator,
      handlers,
      user: None,
      git_process: HashMap::new(),
    }
  }

  fn handle(&self, handle: Handle, channel_id: ChannelId, command: &str) -> HandlerResult {
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
impl<A, U> Handler for RequestHandler<A, U>
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
    let handle = session.handle();

    if let Ok(data_str) = std::str::from_utf8(data) {
      let response = self.handle(handle.clone(), channel_id, data_str);

      match response {
        HandlerResult::Accepted(stdin) => {
          self.git_process.insert(channel_id, stdin);
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
    if let Some(process) = self.git_process.get_mut(&channel_id) {
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
    let process = self.git_process.remove(&channel_id);
    if let Some(mut process) = process {
      process.shutdown().await?;
    }

    Ok((self, session))
  }
}

/// Util function to send an error message to the client.
async fn send_error(handle: &Handle, channel_id: ChannelId, message: &str) {
  let message = CryptoVec::from_slice(message.as_bytes());
  if handle.extended_data(channel_id, 1, message).await.is_err() {
    debug!("Failed to send error message");
  }
}
