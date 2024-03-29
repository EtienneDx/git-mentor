use std::sync::{Arc, Mutex};

use gmt_common::password::PasswordAuth;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token};
use poem_openapi::{payload::Json, OpenApi};
use sha2::Sha256;

use self::user_token::UserToken;

pub mod structs;
pub mod user_token;

pub use structs::*;

#[OpenApi]
impl<Db, Pass> AuthService<Db, Pass>
where
  Db: DbType,
  Arc<Mutex<Db>>: Send + Sync,
  Pass: PasswordAuth + Send + Sync + 'static,
{
  #[oai(path = "/login", method = "post")]
  async fn login(
    &self,
    req: Json<LoginRequest>,
  ) -> Result<Json<LoginResponse>, AuthenticationError> {
    let mut db = self.db.lock()?;
    let (user, password) = match req.0 {
      LoginRequest::UsernameLogin(req) => (
        db.get_user_by_username(&req.username)?,
        req.password.clone(),
      ),
      LoginRequest::EmailLogin(req) => (db.get_user_by_email(&req.email)?, req.password.clone()),
    };
    let user = user.ok_or(AuthenticationError::Unauthorized)?;

    if !Pass::verify_password(password, &user.password) {
      return Err(AuthenticationError::Unauthorized);
    }

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

    let token = Token::new(Header::default(), UserToken::from(user)).sign_with_key(&key)?;

    Ok(Json(LoginResponse {
      token: token.as_str().to_string(),
    }))
  }

  #[oai(path = "/signup", method = "post")]
  async fn signup(
    &self,
    req: Json<SignUpRequest>,
  ) -> Result<Json<LoginResponse>, AuthenticationError> {
    let mut db = self.db.lock()?;

    if db.get_user_by_username(&req.username)?.is_some() {
      return Err(AuthenticationError::Conflict(
        "Username already exists".into(),
      ));
    }
    if db.get_user_by_email(&req.email)?.is_some() {
      return Err(AuthenticationError::Conflict("Email already exists".into()));
    }

    let hash = Pass::generate_hash(&req.password);
    let user = db.create_user(&req.username, &req.email, &hash, None)?;

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

    let token = Token::new(Header::default(), UserToken::from(user)).sign_with_key(&key)?;

    Ok(Json(LoginResponse {
      token: token.as_str().to_string(),
    }))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use database::{db_handle::user::User, DbHandle};
  use gmt_common::password::PasswordAuthImpl;
  use rstest::rstest;

  fn get_user() -> User {
    User {
      id: 1,
      username: "test".to_string(),
      password: PasswordAuthImpl::generate_hash("password"),
      email: "email".to_string(),
      pubkey: vec![],
    }
  }

  #[rstest]
  #[case(LoginRequest::UsernameLogin(UsernameLoginRequest {
    username: "test".to_string(),
    password: "password".to_string(),
  }),
  Some(get_user()),
  Ok(()))]
  #[case(LoginRequest::EmailLogin(EmailLoginRequest {
    email: "email".to_string(),
    password: "password".to_string(),
  }),
  Some(get_user()),
  Ok(()))]
  #[case(LoginRequest::UsernameLogin(UsernameLoginRequest {
    username: "wrong".to_string(),
    password: "password".to_string(),
  }),
  None,
  Err(AuthenticationError::Unauthorized))]
  #[case(LoginRequest::EmailLogin(EmailLoginRequest {
    email: "wrong".to_string(),
    password: "password".to_string(),
  }),
  None,
  Err(AuthenticationError::Unauthorized))]
  #[case(LoginRequest::UsernameLogin(UsernameLoginRequest {
    username: "test".to_string(),
    password: "wrong".to_string(),
  }),
  Some(get_user()),
  Err(AuthenticationError::Unauthorized))]
  #[tokio::test]
  async fn test_login(
    #[case] req: LoginRequest,
    #[case] user: Option<User>,
    #[case] expected: Result<(), AuthenticationError>,
  ) {
    let mut user_handle = DbHandle::faux();
    let u = user.clone();
    faux::when!(user_handle.get_user_by_username).then(move |_| Ok(user.clone()));
    faux::when!(user_handle.get_user_by_email).then(move |_| Ok(u.clone()));

    let user_handle = Arc::new(Mutex::new(user_handle));
    let auth_service: AuthService<DbHandle, _> =
      AuthService::<DbHandle, PasswordAuthImpl>::new(user_handle);
    let req = Json(req);

    let res = auth_service.login(req).await;
    match expected {
      Ok(_) => assert!(res.is_ok()),
      Err(e) => {
        let err = res.expect_err("Expected error");
        assert_eq!(err, e);
      }
    }
  }

  #[rstest]
  #[case(SignUpRequest {
    username: "new".to_string(),
    email: "new".to_string(),
    password: "password".to_string(),
  },
  None,
  None,
  Ok(()))]
  #[case(SignUpRequest {
    username: "test".to_string(),
    email: "email".to_string(),
    password: "password".to_string(),
  },
  Some(get_user()),
  None,
  Err(AuthenticationError::Conflict("Username already exists".into())))]
  #[case(SignUpRequest {
    username: "new".to_string(),
    email: "email".to_string(),
    password: "password".to_string(),
  },
  None,
  Some(get_user()),
  Err(AuthenticationError::Conflict("Email already exists".into())))]
  #[tokio::test]
  async fn test_signup(
    #[case] req: SignUpRequest,
    #[case] username_user: Option<User>,
    #[case] email_user: Option<User>,
    #[case] expected: Result<(), AuthenticationError>,
  ) {
    let mut user_handle = DbHandle::faux();
    faux::when!(user_handle.get_user_by_username).then(move |_| Ok(username_user.clone()));
    faux::when!(user_handle.get_user_by_email).then(move |_| Ok(email_user.clone()));
    faux::when!(user_handle.create_user).then(move |(_, _, _, _)| Ok(get_user()));

    let user_handle = Arc::new(Mutex::new(user_handle));
    let auth_service: AuthService<DbHandle, _> =
      AuthService::<DbHandle, PasswordAuthImpl>::new(user_handle);
    let req = Json(req);

    let res = auth_service.signup(req).await;
    match expected {
      Ok(_) => assert!(res.is_ok()),
      Err(e) => {
        let err = res.expect_err("Expected error");
        assert_eq!(err, e);
      }
    }
  }
}
