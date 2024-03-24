use database::db_handle::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserToken {
  user_id: i32,
  username: String,
  email: String,
  pubkeys: Vec<String>,
}

impl From<User> for UserToken {
  fn from(user: User) -> Self {
    Self {
      user_id: user.id,
      username: user.username,
      email: user.email,
      pubkeys: user
        .pubkey
        .iter()
        .filter(|x| x.is_some())
        .cloned()
        .map(|x| x.unwrap())
        .collect(),
    }
  }
}
