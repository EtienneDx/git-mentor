use std::{marker::PhantomData, process::Stdio};

use log::debug;
use russh::ChannelId;
use ssh_server::{
  handler::{Handler, HandlerResult},
  user::User,
  wrapper::WrappedHandle,
};
use tokio::{io::AsyncWrite, process::Command};

use crate::{
  error::GitProcessError,
  get_permission,
  git_process::GitProcess,
  is_command_allowed, parse_command,
  repository::{Repository, RepositoryProvider},
  GitHandlerConfig,
};

#[derive(Clone)]
pub struct GitHandler<R: RepositoryProvider<User = U>, U: User> {
  config: GitHandlerConfig,
  repo_provider: R,
  _u: PhantomData<U>,
}

impl<R: RepositoryProvider<User = U>, U: User> GitHandler<R, U> {
  pub fn new(config: GitHandlerConfig, repository_provider: R) -> Self {
    Self {
      config,
      repo_provider: repository_provider,
      _u: PhantomData,
    }
  }

  /// Handles the given command, assumin the command is a valid one.
  fn handle_command(
    &self,
    command: String,
    repo_path: String,
    user: &U,
    handle: WrappedHandle,
    channel_id: ChannelId,
  ) -> Result<std::pin::Pin<Box<dyn AsyncWrite + Send + Sync>>, GitProcessError> {
    let repository = self
      .repo_provider
      .find_repository(user, &repo_path)
      .ok_or(GitProcessError::RepositoryNotFoundError)?;

    let permission = get_permission(&command)?;
    if !repository.has_permission(user, permission) {
      return Err(GitProcessError::PermissionDeniedError);
    }

    // based on config, use git command or use binaries
    let mut process = if self.config.use_git_command {
      let mut cmd = Command::new("git");
      cmd.arg(&command[4..]);
      cmd
    } else {
      Command::new(&command)
    };

    debug!("Starting process: {}", &command);

    let mut process = process
      .arg(repository.get_path())
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()?;

    let stdin = process.stdin.take().unwrap();

    GitProcess::forward_output(process, handle, channel_id);

    Ok(Box::pin(stdin))
  }
}

impl<R: RepositoryProvider<User = U>, U: User> Handler for GitHandler<R, U> {
  type User = U;
  type ChannelId = ChannelId;
  type HandleWrapper = WrappedHandle;

  /// Validates the command is one of the valid git commands, then calls the inner `handle_command` method.
  fn handle(
    &self,
    user: &Self::User,
    handle: Self::HandleWrapper,
    channel_id: Self::ChannelId,
    command: &str,
  ) -> HandlerResult {
    let (command, repo_path) = match parse_command(command) {
      Ok(e) => e,
      Err(err) => {
        return HandlerResult::Rejected(err.message().to_string());
      }
    };
    if !is_command_allowed(&command) {
      return HandlerResult::Skipped;
    }

    match self.handle_command(command, repo_path, user, handle, channel_id) {
      Ok(stdin) => HandlerResult::Accepted(stdin),
      Err(err) => HandlerResult::Rejected(err.message().to_string()),
    }
  }
}
