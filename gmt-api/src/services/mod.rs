use std::sync::{Arc, Mutex};

use database::DbHandle;
use gmt_common::password::PasswordAuthImpl;
use poem_openapi::{OpenApi, OpenApiService};

use self::{auth_service::AuthService, hello_service::HelloService};

pub mod auth_service;
pub mod hello_service;

pub fn make_service(db: Arc<Mutex<DbHandle>>) -> OpenApiService<impl OpenApi, ()> {
  OpenApiService::new(
    (
      HelloService,
      AuthService::<DbHandle, PasswordAuthImpl>::new(db),
    ),
    "Git Mentor APIs",
    "1.0",
  )
}
