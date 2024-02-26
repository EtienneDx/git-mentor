use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route};
use services::make_service;
use swagger::add_swagger_ui;

pub mod services;
pub mod swagger;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let mut api_service = make_service();
  let mut app = Route::new();

  (api_service, app) = add_swagger_ui(api_service, app);

  app = app.nest("/", api_service);

  poem::Server::new(TcpListener::bind("0.0.0.0:3001"))
    .run(app.with(Cors::new().allow_origin("http://localhost:3000")))
    .await
}