use russh::{server::Handle, ChannelId};

use crate::user::User;

use super::HandlerResult;

pub trait Handler: Send + Sync + 'static {
  type User: User;

  fn handle(
    &self,
    user: &Self::User,
    handle: Handle,
    channel_id: ChannelId,
    command: &str,
  ) -> HandlerResult;
}
