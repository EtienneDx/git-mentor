use jwt::VerifyWithKey;
use log::error;
use poem_openapi::{auth::ApiKey, ApiResponse, SecurityScheme};
use sha2::digest::InvalidLength;

use crate::{
  error_from,
  services::auth_service::{get_secret_key, user_token::UserToken},
};

#[derive(SecurityScheme)]
#[oai(ty = "api_key", key_name = "Authorization", key_in = "header")]
pub struct GmtToken(ApiKey);

#[derive(ApiResponse, thiserror::Error, Debug)]
pub enum TokenError {
  #[oai(status = 403)]
  #[error("The username or password is incorrect")]
  Unauthorized,
  #[oai(status = 500)]
  #[error("Internal Server Error")]
  InternalServerError,
}

error_from!(jwt::Error, TokenError, Unauthorized);
error_from!(InvalidLength, TokenError, InternalServerError);

impl GmtToken {
  pub fn get_user(self) -> Result<UserToken, TokenError> {
    let key = get_secret_key()?;

    let token: UserToken = self.0.key.verify_with_key(&key)?;

    Ok(token)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use jwt::{Header, SignWithKey, Token};
  use poem_openapi::auth::ApiKey;
  use rstest::{fixture, rstest};

  #[fixture]
  fn token() -> String {
    let key = get_secret_key().expect("Unable to get secret key");

    let user = UserToken {
      user_id: 1,
      username: "john_doe".to_string(),
      email: "john.doe@example.com".to_string(),
      pubkeys: vec![],
    };

    Token::new(Header::default(), UserToken::from(user))
      .sign_with_key(&key)
      .expect("Unable to sign token")
      .into()
  }

  #[rstest]
  fn test_get_user(token: String) {
    let gmt_token = GmtToken(ApiKey { key: token });

    let user = gmt_token.get_user().expect("Unable to get user");

    assert_eq!(user.user_id, 1);
    assert_eq!(user.username, "john_doe");
    assert_eq!(user.email, "john.doe@example.com");
    assert_eq!(user.pubkeys.len(), 0);
  }
}
