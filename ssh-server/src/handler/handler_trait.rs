use crate::{user::User, wrapper::HandleWrapper};

use super::HandlerResult;

pub trait Handler: Send + Sync + 'static {
  type User: User;
  type ChannelId;
  type HandleWrapper: HandleWrapper<ChannelId = Self::ChannelId>;

  fn handle(
    &self,
    user: &Self::User,
    handle: Self::HandleWrapper,
    channel_id: Self::ChannelId,
    command: &str,
  ) -> HandlerResult;
}
