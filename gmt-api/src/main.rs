use std::sync::{Arc, Mutex};

use database::DbHandle;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route};
use services::make_service;
use swagger::add_swagger_ui;

pub mod error;
pub mod services;
pub mod swagger;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  dotenv::dotenv().ok();

  let mut db = DbHandle::new_from_env().expect("Failed to connect to database");
  db.run_migrations().expect("Failed to run migrations");

  let db = Arc::new(Mutex::new(db));

  let mut api_service = make_service(db);
  let mut app = Route::new();

  (api_service, app) = add_swagger_ui(api_service, app);

  app = app.nest("/", api_service);

  poem::Server::new(TcpListener::bind("0.0.0.0:3001"))
    .run(app.with(Cors::new().allow_origin("http://localhost:3000")))
    .await
}
