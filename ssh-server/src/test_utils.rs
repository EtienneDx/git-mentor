use async_trait::async_trait;

use crate::{
  authenticator::Authenticator,
  error::SshError,
  handler::{Handler, HandlerResult},
  wrapper::HandleWrapper,
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

#[derive(Clone)]
pub struct MockHandle;

#[async_trait]
impl HandleWrapper for MockHandle {
  type ChannelId = u32;

  async fn extended_data(
    &self,
    _id: Self::ChannelId,
    _ext: u32,
    _data: russh::CryptoVec,
  ) -> Result<(), russh::CryptoVec> {
    Ok(())
  }
  async fn close(&self, _id: russh::ChannelId) -> Result<(), ()> {
    Ok(())
  }

  async fn data(
    &self,
    _id: russh::ChannelId,
    _data: russh::CryptoVec,
  ) -> Result<(), russh::CryptoVec> {
    Ok(())
  }

  async fn eof(&self, _id: russh::ChannelId) -> Result<(), ()> {
    Ok(())
  }

  async fn exit_status_request(&self, _id: russh::ChannelId, _exit_status: u32) -> Result<(), ()> {
    Ok(())
  }
}

#[allow(dead_code)]
pub enum SimpleHandlerResult {
  Accepted,
  Rejected,
  Skipped,
}
pub struct SimpleHandler<CId, HW>(
  pub SimpleHandlerResult,
  pub std::marker::PhantomData<(CId, HW)>,
)
where
  CId: 'static,
  HW: HandleWrapper<ChannelId = CId> + 'static;

impl<CId, HW> Handler for SimpleHandler<CId, HW>
where
  CId: Send + Sync + 'static,
  HW: Send + Sync + HandleWrapper<ChannelId = CId> + 'static,
{
  type User = User;
  type ChannelId = CId;
  type HandleWrapper = HW;

  fn handle(
    &self,
    _user: &Self::User,
    _handle: Self::HandleWrapper,
    _channel_id: Self::ChannelId,
    _command: &str,
  ) -> HandlerResult {
    match self.0 {
      SimpleHandlerResult::Accepted => HandlerResult::Accepted(Box::pin(tokio::io::sink())),
      SimpleHandlerResult::Rejected => HandlerResult::Rejected("Rejected".to_string()),
      SimpleHandlerResult::Skipped => HandlerResult::Skipped,
    }
  }
}
