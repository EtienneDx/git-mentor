use std::sync::{Arc, Mutex};

use database::connection_pool::ConnectionProvider;
use gmt_common::password::PasswordAuth;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token};
use poem_openapi::{
  param::Path,
  payload::{Json, PlainText},
  OpenApi,
};
use sha2::{digest::InvalidLength, Sha256};

use self::user_token::UserToken;

pub mod structs;
pub mod user_token;

pub use structs::*;

#[OpenApi]
impl<DbPool, Db, Pass> AuthService<DbPool, Db, Pass>
where
  DbPool: ConnectionProvider<Connection = Db> + 'static,
  Db: DbType,
  Arc<Mutex<Db>>: Send + Sync,
  Pass: PasswordAuth + Send + Sync + 'static,
{
  #[oai(path = "/login", method = "post")]
  async fn login(
    &self,
    req: Json<LoginRequest>,
  ) -> Result<Json<LoginResponse>, AuthenticationError> {
    let mut db = self.db.get_connection()?;
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

    let key = get_secret_key()?;

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
    let mut db = self.db.get_connection()?;

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

    let key = get_secret_key()?;

    let token = Token::new(Header::default(), UserToken::from(user)).sign_with_key(&key)?;

    Ok(Json(LoginResponse {
      token: token.as_str().to_string(),
    }))
  }

  /// Returns a \n separated list of keys
  #[oai(path = "/keys/:username", method = "get")]
  async fn keys(&self, username: Path<String>) -> Result<PlainText<String>, PubKeysError> {
    let mut db = self.db.get_connection()?;

    if let Some(user) = db.get_user_by_username(&username.0)? {
      Ok(PlainText(
        user
          .pubkey
          .iter()
          .filter_map(|k| k.to_owned())
          .collect::<Vec<String>>()
          .join("\n"),
      ))
    } else {
      Err(PubKeysError::UsernameDoesNotExist)
    }
  }
}

pub fn get_secret_key() -> Result<Hmac<Sha256>, InvalidLength> {
  Hmac::new_from_slice(
    std::env::var("JWT_SECRET")
      .unwrap_or_else(|_| "secret".to_string())
      .as_bytes(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use database::{connection_pool::ConnectionPool, db_handle::user::User, DbHandle};
  use gmt_common::password::PasswordAuthImpl;
  use rstest::rstest;

  fn get_user() -> User {
    User {
      id: 1,
      username: "test".to_string(),
      password: PasswordAuthImpl::generate_hash("password"),
      email: "email".to_string(),
      pubkey: vec![Some("key1".to_string()), None, Some("key2".to_string())],
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
    let mut pool = ConnectionPool::faux();
    faux::when!(pool.get_connection).then(move |_| {
      let mut user_handle = DbHandle::faux();
      let u = user.clone();
      let u2 = user.clone();
      faux::when!(user_handle.get_user_by_username).then(move |_| Ok(u2.clone()));
      faux::when!(user_handle.get_user_by_email).then(move |_| Ok(u.clone()));
      Ok(user_handle)
    });
    let auth_service: AuthService<ConnectionPool, DbHandle, _> =
      AuthService::<ConnectionPool, DbHandle, PasswordAuthImpl>::new(pool);
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
    let mut pool = ConnectionPool::faux();
    faux::when!(pool.get_connection).then(move |_| {
      let mut user_handle = DbHandle::faux();
      let username_user = username_user.clone();
      let email_user = email_user.clone();
      faux::when!(user_handle.get_user_by_username).then(move |_| Ok(username_user.clone()));
      faux::when!(user_handle.get_user_by_email).then(move |_| Ok(email_user.clone()));
      faux::when!(user_handle.create_user).then(move |(_, _, _, _)| Ok(get_user()));
      Ok(user_handle)
    });
    let auth_service: AuthService<ConnectionPool, DbHandle, _> =
      AuthService::<ConnectionPool, DbHandle, PasswordAuthImpl>::new(pool);

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

  #[rstest]
  fn test_get_secret_key() {
    let key = get_secret_key();
    assert!(key.is_ok(), "Unable to get encryption key");
  }

  #[rstest]
  #[tokio::test]
  async fn test_get_keys(
    #[values("valid-username".to_string(), "not-valid-username".to_string())] username: String,
  ) {
    let mut pool = ConnectionPool::faux();
    faux::when!(pool.get_connection).then(move |_| {
      let mut user_handle = DbHandle::faux();
      faux::when!(user_handle.get_user_by_username(_)).then(|_| Ok(None));
      faux::when!(user_handle.get_user_by_username("valid-username"))
        .then(|_| Ok(Some(get_user())));
      Ok(user_handle)
    });
    let auth_service: AuthService<ConnectionPool, DbHandle, _> =
      AuthService::<ConnectionPool, DbHandle, PasswordAuthImpl>::new(pool);

    let res = auth_service.keys(Path(username.clone())).await;

    if username == "valid-username" {
      let res = res.expect("Expected success");
      assert_eq!(res.0, "key1\nkey2")
    } else {
      let err = res.expect_err("Expected error");
      assert_eq!(err, PubKeysError::UsernameDoesNotExist)
    }
  }
}
