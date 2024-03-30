use database::connection_pool::ConnectionPool;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route};
use services::make_service;
use swagger::add_swagger_ui;

pub mod error;
pub mod services;
pub mod swagger;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  simple_logger::SimpleLogger::new().env().init().unwrap();
  dotenv::dotenv().ok();

  let connection_pool = ConnectionPool::new_from_env().expect("Failed to create connection pool");
  connection_pool
    .run_migrations()
    .expect("Failed to run migrations");

  let mut api_service = make_service(connection_pool);
  let mut app = Route::new();

  (api_service, app) = add_swagger_ui(api_service, app);

  app = app.nest("/", api_service);

  log::info!("Listening on port 3001");

  poem::Server::new(TcpListener::bind("0.0.0.0:3001"))
    .run(app.with(Cors::new().allow_origin("http://localhost:3000")))
    .await
}
