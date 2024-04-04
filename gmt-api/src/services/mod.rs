use std::sync::{Arc, Mutex};

use database::{
  connection_pool::ConnectionProvider,
  db_handle::{
    assignment::AssignmentDbHandle, cirun::CirunDbHandle, comment::CommentDbHandle,
    group::GroupDbHandle, repository::RepositoryDbHandle, user::UserDbHandle,
  },
};
use gmt_common::password::PasswordAuthImpl;
use poem_openapi::{OpenApi, OpenApiService};

use self::{auth_service::AuthService, hello_service::HelloService};

pub mod auth_service;
pub mod hello_service;

pub mod structs;

pub fn make_service<DbPool, Db>(db: DbPool) -> OpenApiService<impl OpenApi, ()>
where
  Arc<Mutex<Db>>: 'static + Send + Sync,
  DbPool: ConnectionProvider<Connection = Db> + 'static,
  Db: 'static
    + AssignmentDbHandle
    + CirunDbHandle
    + CommentDbHandle
    + GroupDbHandle
    + RepositoryDbHandle
    + UserDbHandle,
{
  OpenApiService::new(
    (
      HelloService,
      AuthService::<DbPool, Db, PasswordAuthImpl>::new(db),
    ),
    "Git Mentor APIs",
    "1.0",
  )
}
