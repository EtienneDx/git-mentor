use std::{marker::PhantomData, process::Stdio};

use log::debug;
use ssh_server::{
  handler::{Handler, HandlerResult},
  user::User,
  wrapper::HandleWrapper,
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
pub struct GitHandler<R, U, CId, HW>
where
  R: RepositoryProvider<User = U>,
  U: User,
  CId: Copy + Send + Sync + 'static,
  HW: HandleWrapper<ChannelId = CId> + Send + Sync + 'static,
{
  config: GitHandlerConfig,
  repo_provider: R,
  _u: PhantomData<(U, CId, HW)>,
}

impl<R, U, CId, HW> GitHandler<R, U, CId, HW>
where
  R: RepositoryProvider<User = U>,
  U: User,
  CId: Copy + Send + Sync + 'static,
  HW: HandleWrapper<ChannelId = CId> + Send + Sync + 'static,
{
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
    handle: HW,
    channel_id: CId,
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

impl<R, U, CId, HW> Handler for GitHandler<R, U, CId, HW>
where
  R: RepositoryProvider<User = U>,
  U: User,
  CId: Copy + Send + Sync + 'static,
  HW: HandleWrapper<ChannelId = CId> + Send + Sync + 'static,
{
  type User = U;
  type ChannelId = CId;
  type HandleWrapper = HW;

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
        if matches!(err, GitProcessError::InvalidCommandError) {
          // Invalid command means we're not supposed to process it
          return HandlerResult::Skipped;
        }
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

#[cfg(test)]
mod tests {
  use crate::test_utils::{MockHandle, SimpleRepositoryProvider, SimpleUser};

  use super::*;

  #[test]
  fn when_wrong_command_then_skip() {
    let config = GitHandlerConfig {
      use_git_command: false,
    };
    let repo_provider = SimpleRepositoryProvider {
      find_repository: false,
      has_permission: false,
    };
    let handler = GitHandler::new(config, repo_provider);

    let user = SimpleUser;
    let handle = MockHandle;
    let channel_id = 0;

    let result = handler.handle(&user, handle, channel_id, "long invalid command");
    assert!(
      matches!(result, HandlerResult::Skipped),
      "Expected HandlerResult::Skipped, got {:?}",
      result
    );
  }

  #[test]
  fn when_valid_command_but_no_repository_then_reject() {
    let config = GitHandlerConfig {
      use_git_command: false,
    };
    let repo_provider = SimpleRepositoryProvider {
      find_repository: false,
      has_permission: false,
    };
    let handler = GitHandler::new(config, repo_provider);

    let user = SimpleUser;
    let handle = MockHandle;
    let channel_id = 0;

    let result = handler.handle(&user, handle, channel_id, "git-upload-pack '/path/to/repo'");
    assert!(
      matches!(result, HandlerResult::Rejected(_)),
      "Expected HandlerResult::Rejected, got {:?}",
      result
    );
  }

  #[test]
  fn when_valid_command_but_no_permission_then_reject() {
    let config = GitHandlerConfig {
      use_git_command: false,
    };
    let repo_provider = SimpleRepositoryProvider {
      find_repository: true,
      has_permission: false,
    };
    let handler = GitHandler::new(config, repo_provider);

    let user = SimpleUser;
    let handle = MockHandle;
    let channel_id = 0;

    let result = handler.handle(&user, handle, channel_id, "git-upload-pack '/path/to/repo'");
    assert!(
      matches!(result, HandlerResult::Rejected(_)),
      "Expected HandlerResult::Rejected, got {:?}",
      result
    );
  }
}
