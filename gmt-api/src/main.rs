use tokio::net::TcpListener;
use tracing_subscriber::fmt;

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
  fmt::init();

  let app = routes::create_app();

  let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
