use database::db_handle::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserToken {
  pub user_id: i32,
  pub username: String,
  pub email: String,
}

impl From<User> for UserToken {
  fn from(user: User) -> Self {
    Self {
      user_id: user.id,
      username: user.username,
      email: user.email,
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
      pubkey: vec![],
    };

    let user_token: UserToken = user.into();

    assert_eq!(user_token.user_id, 1);
    assert_eq!(user_token.username, "john_doe");
    assert_eq!(user_token.email, "john.doe@example.com");
  }
}
