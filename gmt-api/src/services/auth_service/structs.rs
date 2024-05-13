use std::sync::{Arc, Mutex, PoisonError};

use database::{
  connection_pool::ConnectionProvider, db_handle::user::UserDbHandle, error::DatabaseError,
};
use gmt_common::password::PasswordAuth;
use poem_openapi::{ApiResponse, Object, Union};
use serde::{Deserialize, Serialize};
use sha2::digest::InvalidLength;

use crate::error_from;

use super::super::structs::StringResponse;

pub trait DbType: UserDbHandle + 'static {}
impl<T: UserDbHandle + 'static> DbType for T {}

pub struct AuthService<DbPool, Db, Pass>
where
  DbPool: ConnectionProvider<Connection = Db> + 'static,
  Db: DbType,
  Arc<Mutex<Db>>: Send + Sync,
  Pass: PasswordAuth + 'static,
{
  pub db: DbPool,
  _phantom: std::marker::PhantomData<Pass>,
}
impl<DbPool, Db, Pass> AuthService<DbPool, Db, Pass>
where
  DbPool: ConnectionProvider<Connection = Db> + 'static,
  Db: DbType,
  Arc<Mutex<Db>>: Send + Sync,
  Pass: PasswordAuth + 'static,
{
  pub fn new(db: DbPool) -> Self {
    Self {
      db,
      _phantom: std::marker::PhantomData,
    }
  }
}

#[derive(Object, Deserialize, Serialize)]
pub struct UsernameLoginRequest {
  pub username: String,
  pub password: String,
}

#[derive(Object, Deserialize, Serialize)]
pub struct EmailLoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Union, Deserialize, Serialize)]
pub enum LoginRequest {
  UsernameLogin(UsernameLoginRequest),
  EmailLogin(EmailLoginRequest),
}

#[derive(Object, Deserialize, Serialize, Debug)]
pub struct LoginResponse {
  pub token: String,
}

#[derive(Object, Deserialize, Serialize)]
pub struct SignUpRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(ApiResponse, thiserror::Error, Debug, PartialEq, Eq)]
pub enum AuthenticationError {
  #[oai(status = 403)]
  #[error("The username or password is incorrect")]
  Unauthorized,
  #[oai(status = 409)]
  #[error("The {0} is already in use")]
  Conflict(StringResponse),
  #[oai(status = 500)]
  #[error("Internal Server Error")]
  InternalServerError,
}

error_from!(DatabaseError, AuthenticationError, InternalServerError);
error_from!(jwt::Error, AuthenticationError, InternalServerError);
error_from!(InvalidLength, AuthenticationError, InternalServerError);

impl<T> From<PoisonError<T>> for AuthenticationError {
  fn from(_: PoisonError<T>) -> Self {
    AuthenticationError::InternalServerError
  }
}

#[derive(ApiResponse, thiserror::Error, Debug, PartialEq, Eq)]
pub enum PubKeysError {
  #[oai(status = 404)]
  #[error("Username does not exist")]
  UsernameDoesNotExist,
  #[oai(status = 500)]
  #[error("Internal Server Error")]
  InternalServerError,
}

error_from!(DatabaseError, PubKeysError, InternalServerError);
