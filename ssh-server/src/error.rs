use thiserror::Error;

/// Ssh related errors.
#[derive(Error, Debug)]
pub enum SshError {
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
