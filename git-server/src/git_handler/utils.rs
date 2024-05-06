use crate::{error::GitProcessError, repository::RepositoryPermission};

const GIT_UPLOAD_PACK: &str = "git-upload-pack";
const GIT_RECEIVE_PACK: &str = "git-receive-pack";
const ALLOWED_COMMANDS: [&str; 2] = [GIT_UPLOAD_PACK, GIT_RECEIVE_PACK];

/// What kind of permission is required for the given command.
pub(crate) fn get_permission(command: &str) -> Result<RepositoryPermission, GitProcessError> {
  match command {
    GIT_UPLOAD_PACK => Ok(RepositoryPermission::Read),
    GIT_RECEIVE_PACK => Ok(RepositoryPermission::Write),
    _ => Err(GitProcessError::InvalidCommandError),
  }
}

/// Tries to parse the command and the repository path from the given string.
pub(crate) fn parse_command(command: &str) -> Result<(String, String), GitProcessError> {
  let parts = shell_words::split(command)?;

  if parts.len() != 2 {
    return Err(GitProcessError::InvalidCommandError);
  }

  let command = parts
    .first()
    .ok_or(GitProcessError::InvalidCommandError)?
    .to_owned();
  let repo_path = parts
    .get(1)
    .ok_or(GitProcessError::InvalidCommandError)?
    .to_owned();

  Ok((command, repo_path))
}

pub(crate) fn is_command_allowed(command: &str) -> bool {
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
