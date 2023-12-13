use std::pin::Pin;

use tokio::io::AsyncWrite;

pub enum HandlerResult {
  Accepted(Pin<Box<dyn AsyncWrite + Sync + Send + 'static>>),
  Skipped,
  Rejected(String),
}
