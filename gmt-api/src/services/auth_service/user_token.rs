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

#[cfg(test)]
mod tests {
  use database::db_handle::user::User;

  use super::*;

  #[test]
  fn test_from_user_to_user_token() {
    let user = User {
      id: 1,
      username: "john_doe".to_string(),
      password: "password".to_string(),
      email: "john.doe@example.com".to_string(),
      pubkey: vec![Some("key1".to_string()), None, Some("key2".to_string())],
    };

    let user_token: UserToken = user.into();

    assert_eq!(user_token.user_id, 1);
    assert_eq!(user_token.username, "john_doe");
    assert_eq!(user_token.email, "john.doe@example.com");
    assert_eq!(user_token.pubkeys, vec!["key1", "key2"]);
  }
}
