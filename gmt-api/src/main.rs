use log::info;
use tokio::net::TcpListener;

mod handlers;
mod json;
mod routes;

#[tokio::main]
async fn main() {
  simple_logger::SimpleLogger::new()
    .with_level(log::LevelFilter::Info)
    .env()
    .init()
    .unwrap();
  info!("Logger initialized");

  let app = routes::create_app();

  let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  let server = axum::serve(listener, app);
  info!("Server listening on port 3000");
  server.await.unwrap();
}
