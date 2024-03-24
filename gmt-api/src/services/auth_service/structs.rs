use std::sync::{Arc, Mutex};

use database::db_handle::user::UserDbHandle;
use gmt_common::password::PasswordAuth;
use poem_openapi::{Object, Union};
use serde::{Deserialize, Serialize};

pub trait DbType: UserDbHandle + 'static {}
impl<T: UserDbHandle + 'static> DbType for T {}

pub struct AuthService<Db, Pass>
where
  Db: DbType,
  Arc<Mutex<Db>>: Send + Sync,
  Pass: PasswordAuth + 'static,
{
  pub db: Arc<Mutex<Db>>,
  _phantom: std::marker::PhantomData<Pass>,
}
impl<Db, Pass> AuthService<Db, Pass>
where
  Db: DbType,
  Arc<Mutex<Db>>: Send + Sync,
  Pass: PasswordAuth + 'static,
{
  pub fn new(db: Arc<Mutex<Db>>) -> Self {
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
