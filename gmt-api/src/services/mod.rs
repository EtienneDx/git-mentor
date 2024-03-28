use std::sync::{Arc, Mutex};

use database::db_handle::{
  assignment::AssignmentDbHandle, cirun::CirunDbHandle, comment::CommentDbHandle,
  group::GroupDbHandle, repository::RepositoryDbHandle, user::UserDbHandle,
};
use gmt_common::password::PasswordAuthImpl;
use poem_openapi::{OpenApi, OpenApiService};

use self::{auth_service::AuthService, hello_service::HelloService};

pub mod auth_service;
pub mod hello_service;

pub mod structs;

pub fn make_service<T>(db: Arc<Mutex<T>>) -> OpenApiService<impl OpenApi, ()>
where
  Arc<Mutex<T>>: 'static + Send + Sync,
  T: 'static
    + AssignmentDbHandle
    + CirunDbHandle
    + CommentDbHandle
    + GroupDbHandle
    + RepositoryDbHandle
    + UserDbHandle,
{
  OpenApiService::new(
    (HelloService, AuthService::<T, PasswordAuthImpl>::new(db)),
    "Git Mentor APIs",
    "1.0",
  )
}
