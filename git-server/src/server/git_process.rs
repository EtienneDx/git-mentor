use std::process::Stdio;

use log::{debug, error};
use russh::server::Handle;
use russh::{ChannelId, CryptoVec};
use tokio::io::AsyncReadExt;
use tokio::process::{Child, ChildStdin, Command};

use crate::error::GitProcessError;
use crate::repository::{Repository, RepositoryPermission, RepositoryProvider};

use crate::git_server_config::GitServerConfig;

const GIT_UPLOAD_PACK: &str = "git-upload-pack";
const GIT_RECEIVE_PACK: &str = "git-receive-pack";
const ALLOWED_COMMANDS: [&str; 2] = [GIT_UPLOAD_PACK, GIT_RECEIVE_PACK];

/// Starts a new git process.
pub(crate) async fn start_process<U, R: RepositoryProvider<User = U>>(
  command: &str,
  handle: Handle,
  channel_id: ChannelId,
  user: &U,
  repo_provider: &R,
  config: &GitServerConfig,
) -> Result<ChildStdin, GitProcessError> {
  let (command, repo_path) = parse_command(command)?;
  if !is_command_allowed(&command) {
    return Err(GitProcessError::InvalidCommandError);
  }

  let repository = repo_provider
    .find_repository(user, &repo_path)
    .ok_or(GitProcessError::RepositoryNotFoundError)?;
  let permission = get_permission(&command)?;

  if !repository.has_permission(user, permission) {
    return Err(GitProcessError::PermissionDeniedError);
  }

  let mut process = if config.use_git_command {
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
  let git_process = GitProcess {
    process,
    handle,
    channel_id,
  };

  git_process.forward_output();

  Ok(stdin)
}

/// A struct representing a git process.
struct GitProcess {
  process: Child,
  handle: Handle,
  channel_id: ChannelId,
}

impl GitProcess {
  /// Forwards the output of the git process to the client.
  fn forward_output(mut self) {
    let mut git_stdout = self.process.stdout.take().unwrap();

    tokio::spawn(async move {
      const BUF_SIZE: usize = 1024 * 32;
      let mut buf = [0u8; BUF_SIZE];
      loop {
        let read = git_stdout.read(&mut buf).await.map_err(|e| {
          error!("Error reading from git process: {}", e);
        })?;
        if read == 0 {
          break;
        }
        self.data(&buf[..read]).await?;
      }

      let status = self
        .process
        .wait()
        .await
        .map_err(|e| {
          error!("Error waiting for git process: {}", e);
        })?
        .code()
        .unwrap_or(128) as u32;
      self.exit_status(status).await?;

      self.eof().await?;
      self.close().await?;
      Ok::<(), ()>(())
    });
  }

  /// Closes the channel.
  async fn close(&self) -> Result<(), ()> {
    self.handle.close(self.channel_id).await.map_err(|_| {
      error!("Failed to close handle");
    })?;
    Ok(())
  }
  /// Sends data to the client.
  async fn data(&self, data: &[u8]) -> Result<(), ()> {
    let buf: russh::CryptoVec = CryptoVec::from_slice(data);
    self.handle.data(self.channel_id, buf).await.map_err(|_| {
      error!("Failed to write data to channel");
    })?;
    Ok(())
  }
  /// Sets the exit status of the process.
  async fn exit_status(&self, status: u32) -> Result<(), ()> {
    self
      .handle
      .exit_status_request(self.channel_id, status)
      .await
      .map_err(|_| {
        error!("Failed to set exit status");
      })?;
    Ok(())
  }
  /// Sends an EOF to the client.
  async fn eof(&self) -> Result<(), ()> {
    self.handle.eof(self.channel_id).await.map_err(|_| {
      error!("Failed to send EOF");
    })?;
    Ok(())
  }
}

/// What kind of permission is required for the given command.
fn get_permission(command: &str) -> Result<RepositoryPermission, GitProcessError> {
  match command {
    GIT_UPLOAD_PACK => Ok(RepositoryPermission::Read),
    GIT_RECEIVE_PACK => Ok(RepositoryPermission::Write),
    _ => Err(GitProcessError::InvalidCommandError),
  }
}

/// Tries to parse the command and the repository path from the given string.
fn parse_command(command: &str) -> Result<(String, String), GitProcessError> {
  let parts = shell_words::split(command)?;

  if parts.len() != 2 {
    return Err(GitProcessError::InvalidCommandError);
  }

  let command = parts
    .get(0)
    .ok_or(GitProcessError::InvalidCommandError)?
    .to_owned();
  let repo_path = parts
    .get(1)
    .ok_or(GitProcessError::InvalidCommandError)?
    .to_owned();

  let repo_path = repo_path.trim_matches(|c| c == '\'' || c == '"').to_owned();
  Ok((command, repo_path))
}

fn is_command_allowed(command: &str) -> bool {
  ALLOWED_COMMANDS.contains(&command)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_permission() {
    assert_eq!(
      get_permission("git-upload-pack").expect("ok"),
      RepositoryPermission::Read
    );
    assert_eq!(
      get_permission("git-receive-pack").expect("ok"),
      RepositoryPermission::Write
    );
    get_permission("invalid-command").expect_err("Expected error");
  }

  #[test]
  fn test_parse_command() {
    assert_eq!(
      parse_command("git-upload-pack '/path/to/repo'").expect("ok"),
      ("git-upload-pack".to_string(), "/path/to/repo".to_string())
    );
    assert_eq!(
      parse_command("git-receive-pack \"/path/to/repo\"").expect("ok"),
      ("git-receive-pack".to_string(), "/path/to/repo".to_string())
    );
    parse_command("invalid-command").expect_err("Expected error");
    parse_command("git-upload-pack '/path/to/repo' extra-arg").expect_err("Expected error");
  }

  #[test]
  fn test_is_command_allowed() {
    assert_eq!(is_command_allowed("git-upload-pack"), true);
    assert_eq!(is_command_allowed("git-receive-pack"), true);
    assert_eq!(is_command_allowed("invalid-command"), false);
  }
}
