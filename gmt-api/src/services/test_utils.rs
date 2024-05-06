use jwt::{Header, SignWithKey, Token};
use rstest::fixture;

use super::auth_service::{get_secret_key, user_token::UserToken};

#[fixture]
pub fn valid_token() -> String {
  let user_token = UserToken {
    user_id: 1,
    username: "bob".to_string(),
    email: "test@test.com".to_string(),
  };

  let key = get_secret_key().expect("Unable to get secret key");

  let token = Token::new(Header::default(), UserToken::from(user_token))
    .sign_with_key(&key)
    .expect("Unable to generate token");

  token.into()
}
