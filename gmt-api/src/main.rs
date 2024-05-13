use database::connection_pool::ConnectionPool;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route};
use services::make_service;
use swagger::add_swagger_ui;

pub mod error;
pub mod security;
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

  let port = std::env::var("API_PORT").unwrap_or_else(|_| "3001".to_string());
  let cors = std::env::var("API_CORS").unwrap_or_else(|_| "http://localhost:3000".to_string());

  log::info!("Listening on port {}", port);

  poem::Server::new(TcpListener::bind(format!("0.0.0.0:{}", port)))
    .run(app.with(Cors::new().allow_origin(cors)))
    .await
}
