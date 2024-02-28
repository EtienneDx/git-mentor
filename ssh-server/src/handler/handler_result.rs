use std::{
  fmt::{Debug, Formatter},
  pin::Pin,
};

use tokio::io::AsyncWrite;

pub enum HandlerResult {
  Accepted(Pin<Box<dyn AsyncWrite + Sync + Send + 'static>>),
  Skipped,
  Rejected(String),
}

impl Debug for HandlerResult {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      HandlerResult::Accepted(_) => write!(f, "Accepted"),
      HandlerResult::Skipped => write!(f, "Skipped"),
      HandlerResult::Rejected(s) => write!(f, "Rejected: {}", s),
    }
  }
}
