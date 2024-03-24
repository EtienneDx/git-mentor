use std::sync::{Arc, Mutex};

use gmt_common::password::PasswordAuth;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token};
use poem_openapi::{
  payload::{Json, PlainText},
  OpenApi,
};
use sha2::Sha256;

use self::user_token::UserToken;

use super::super::error::{ApiError, ApiResult};

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
  async fn login(&self, req: Json<LoginRequest>) -> ApiResult<Json<LoginResponse>> {
    let mut db = self.db.lock()?;
    let (user, password) = match req.0 {
      LoginRequest::UsernameLogin(req) => (
        db.get_user_by_username(&req.username)?,
        req.password.clone(),
      ),
      LoginRequest::EmailLogin(req) => (db.get_user_by_email(&req.email)?, req.password.clone()),
    };
    let user = user.ok_or(ApiError::Unauthorized)?;

    if !Pass::verify_password(password, &user.password) {
      return Err(ApiError::Unauthorized);
    }

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

    let token = Token::new(Header::default(), UserToken::from(user)).sign_with_key(&key)?;

    Ok(Json(LoginResponse {
      token: token.as_str().to_string(),
    }))
  }

  #[oai(path = "/signup", method = "post")]
  async fn signup(&self, req: Json<SignUpRequest>) -> ApiResult<Json<LoginResponse>> {
    let mut db = self.db.lock()?;

    if db.get_user_by_username(&req.username)?.is_some() {
      return Err(ApiError::Conflict(PlainText(
        "Username already exists".to_string(),
      )));
    }
    if db.get_user_by_email(&req.email)?.is_some() {
      return Err(ApiError::Conflict(PlainText(
        "Email already exists".to_string(),
      )));
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
  use database::{
    db_handle::user::{User, UserDbHandle},
    error::DatabaseError,
  };
  use rstest::{fixture, rstest};

  struct MockDbHandle {
    user: Option<User>,
  }
  impl UserDbHandle for MockDbHandle {
    fn get_user_by_username(&mut self, username: &str) -> Result<Option<User>, DatabaseError> {
      Ok(self.user.clone().filter(|x| x.username == username))
    }

    fn create_user(
      &mut self,
      username: &str,
      email: &str,
      password: &str,
      pubkey: Option<Vec<&str>>,
    ) -> Result<User, database::error::DatabaseError> {
      Ok(User {
        id: 1,
        username: username.to_string(),
        email: email.to_string(),
        password: password.to_string(),
        pubkey: pubkey
          .unwrap_or_default()
          .iter()
          .map(|x| Some(x.to_string()))
          .collect(),
      })
    }

    fn get_user_by_id(
      &mut self,
      _user_id: i32,
    ) -> Result<Option<User>, database::error::DatabaseError> {
      Ok(self.user.clone().filter(|x| x.id == _user_id))
    }

    fn get_user_by_email(
      &mut self,
      _email: &str,
    ) -> Result<Option<User>, database::error::DatabaseError> {
      Ok(self.user.clone().filter(|x| x.email == _email))
    }

    fn insert_user_public_key(
      &mut self,
      _user_id: i32,
      _pubkey: &str,
    ) -> Result<(), database::error::DatabaseError> {
      Ok(())
    }

    fn delete_user(&mut self, _user_id: i32) -> Result<(), database::error::DatabaseError> {
      Ok(())
    }

    fn list_teaching_groups(
      &mut self,
      _user_id: i32,
    ) -> Result<Vec<database::db_handle::group::Group>, database::error::DatabaseError> {
      Ok(vec![])
    }

    fn list_belongs_groups(
      &mut self,
      _user_id: i32,
    ) -> Result<Vec<database::db_handle::group::Group>, database::error::DatabaseError> {
      Ok(vec![])
    }
  }

  struct MockPasswordAuth;
  impl PasswordAuth for MockPasswordAuth {
    fn generate_hash(_password: impl AsRef<[u8]>) -> String {
      "password".to_string()
    }

    fn verify_password(password: impl AsRef<[u8]>, hash: &str) -> bool {
      password.as_ref() == hash.as_bytes()
    }
  }

  #[fixture]
  fn user_handle() -> Arc<Mutex<MockDbHandle>> {
    Arc::new(Mutex::new(MockDbHandle {
      user: Some(User {
        id: 1,
        username: "test".to_string(),
        email: "email".to_string(),
        password: "password".to_string(),
        pubkey: vec![],
      }),
    }))
  }

  #[rstest]
  #[case(LoginRequest::UsernameLogin(UsernameLoginRequest {
    username: "test".to_string(),
    password: "password".to_string(),
  }),
  Ok(()))]
  #[case(LoginRequest::EmailLogin(EmailLoginRequest {
    email: "email".to_string(),
    password: "password".to_string(),
  }),
  Ok(()))]
  #[case(LoginRequest::UsernameLogin(UsernameLoginRequest {
    username: "wrong".to_string(),
    password: "password".to_string(),
  }),
  Err(ApiError::Unauthorized))]
  #[case(LoginRequest::EmailLogin(EmailLoginRequest {
    email: "wrong".to_string(),
    password: "password".to_string(),
  }),
  Err(ApiError::Unauthorized))]
  #[case(LoginRequest::UsernameLogin(UsernameLoginRequest {
    username: "test".to_string(),
    password: "wrong".to_string(),
  }),
  Err(ApiError::Unauthorized))]
  #[tokio::test]
  async fn test_login(
    user_handle: Arc<Mutex<MockDbHandle>>,
    #[case] req: LoginRequest,
    #[case] expected: ApiResult<()>,
  ) {
    let auth_service: AuthService<MockDbHandle, _> =
      AuthService::<MockDbHandle, MockPasswordAuth>::new(user_handle);
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
  Ok(()))]
  #[case(SignUpRequest {
    username: "test".to_string(),
    email: "email".to_string(),
    password: "password".to_string(),
  },
  Err(ApiError::Conflict(PlainText("Username already exists".to_string()))))]
  #[case(SignUpRequest {
    username: "new".to_string(),
    email: "email".to_string(),
    password: "password".to_string(),
  },
  Err(ApiError::Conflict(PlainText("Email already exists".to_string()))))]
  #[tokio::test]
  async fn test_signup(
    user_handle: Arc<Mutex<MockDbHandle>>,
    #[case] req: SignUpRequest,
    #[case] expected: ApiResult<()>,
  ) {
    let auth_service: AuthService<MockDbHandle, _> =
      AuthService::<MockDbHandle, MockPasswordAuth>::new(user_handle);
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
