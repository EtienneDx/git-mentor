use crate::{
  authenticator::Authenticator,
  error::SshError,
  handler::{Handler, HandlerResult},
};

pub struct User;
impl crate::user::User for User {}

pub struct SimpleAuthenticator;
impl Authenticator for SimpleAuthenticator {
  type User = User;
  fn validate_public_key(
    &self,
    _user: &str,
    _key: &russh_keys::key::PublicKey,
  ) -> Result<Option<Self::User>, SshError> {
    Ok(Some(User))
  }
}

#[allow(dead_code)]
pub enum SimpleHandlerResult {
  Accepted,
  Rejected,
  Skipped,
}
pub struct SimpleHandler(pub SimpleHandlerResult);
impl Handler for SimpleHandler {
  type User = User;

  fn handle(
    &self,
    _user: &Self::User,
    _handle: russh::server::Handle,
    _channel_id: russh::ChannelId,
    _command: &str,
  ) -> HandlerResult {
    match self.0 {
      SimpleHandlerResult::Accepted => HandlerResult::Accepted(Box::pin(tokio::io::sink())),
      SimpleHandlerResult::Rejected => HandlerResult::Rejected("Rejected".to_string()),
      SimpleHandlerResult::Skipped => HandlerResult::Skipped,
    }
  }
}
