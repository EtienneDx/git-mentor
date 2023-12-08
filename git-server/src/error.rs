use shell_words::ParseError;
use thiserror::Error;

/// Errors that can occur in the git server.
#[derive(Error, Debug)]
pub enum GitError {
  #[error("Unknown error occurred")]
  Unknown,
  #[error("Invalid key")]
  InvalidKey,
  #[error("Already authenticated")]
  AlreadyAuthenticated,
  #[error("Not authenticated")]
  NotAuthenticated,
  #[error("Process not started")]
  ProcessNotStartedError,
  #[error("Process already started forwarding data")]
  ForwardingAlreadyStartedError,
  #[error("Channel not found")]
  ChannelNotFoundError,
  #[error("Ssh error: {0}")]
  SshError(#[from] russh::Error),
  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),
}

/// Errors that can occur during the git process.
#[derive(Error, Debug)]
pub enum GitProcessError {
  #[error("Invalid command error")]
  InvalidCommandError,
  #[error("Unable to parse command error")]
  ParseFailureError(#[from] ParseError),
  #[error("Repository not found error")]
  RepositoryNotFoundError,
  #[error("Permission denied error")]
  PermissionDeniedError,
  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),
}

impl GitProcessError {
  /// User friendly message for the error. These are sent to the client.
  pub fn message(&self) -> &str {
    match self {
      GitProcessError::InvalidCommandError => "Invalid command",
      GitProcessError::ParseFailureError(_) => "Invalid command",
      GitProcessError::RepositoryNotFoundError => "Repository not found",
      GitProcessError::PermissionDeniedError => "Permission denied",
      GitProcessError::IoError(_) => "IO error",
    }
  }
}
