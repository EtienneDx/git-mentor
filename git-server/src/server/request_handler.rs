use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use log::debug;
use russh::{
  server::{Auth, Handle, Handler, Msg, Session},
  Channel, ChannelId, CryptoVec,
};
use russh_keys::key::PublicKey;
use tokio::{io::AsyncWriteExt, process::ChildStdin};

use crate::{
  authenticator::Authenticator,
  error::{GitError, GitProcessError},
  git_server_config::GitServerConfig,
  repository::RepositoryProvider,
  server::git_process::start_process,
};

pub struct RequestHandler<A, R, U>
where
  A: Authenticator<User = U>,
  R: RepositoryProvider<User = U>,
  U: Sync + Send + 'static,
{
  authenticator: Arc<A>,
  repository_provider: Arc<R>,
  config: GitServerConfig,
  user: Option<A::User>,
  git_process: HashMap<ChannelId, ChildStdin>,
}
impl<A, R, U> RequestHandler<A, R, U>
where
  A: Authenticator<User = U>,
  R: RepositoryProvider<User = U>,
  U: Sync + Send + 'static,
{
  pub fn new(
    authenticator: Arc<A>,
    repository_provider: Arc<R>,
    config: GitServerConfig,
    _: Option<std::net::SocketAddr>,
  ) -> Self {
    RequestHandler {
      authenticator,
      repository_provider,
      config,
      user: None,
      git_process: HashMap::new(),
    }
  }
}

#[async_trait]
impl<A, R, U> Handler for RequestHandler<A, R, U>
where
  A: Authenticator<User = U>,
  R: RepositoryProvider<User = U>,
  U: Sync + Send + 'static,
{
  type Error = GitError;

  /// Authenticates the user based on their public key.
  async fn auth_publickey(
    mut self,
    user: &str,
    key: &PublicKey,
  ) -> Result<(Self, Auth), Self::Error> {
    if self.user.is_some() {
      // We shouldn't be able to authenticate twice
      return Err(GitError::AlreadyAuthenticated);
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
      return Err(GitError::NotAuthenticated);
    }
    debug!("Executing function, trying to start git process");
    let handle = session.handle();

    if let Ok(data_str) = std::str::from_utf8(data) {
      match start_process(
        data_str,
        handle.clone(),
        channel_id,
        self.user.as_ref().unwrap(),
        self.repository_provider.as_ref(),
        &self.config,
      )
      .await
      {
        Err(e) => {
          debug!("Error starting git process: {:?}", e);
          send_error(&handle, channel_id, e.message()).await;
          handle.close(channel_id).await.unwrap();
        }
        Ok(process) => {
          self.git_process.insert(channel_id, process);
        }
      }
    } else {
      debug!("Invalid UTF-8 exec received: {:?}", data);
      send_error(
        &handle,
        channel_id,
        GitProcessError::InvalidCommandError.message(),
      )
      .await;
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
        GitError::ProcessNotStartedError
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
