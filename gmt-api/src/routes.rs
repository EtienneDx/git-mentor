use crate::handlers;
use axum::{routing::post, Router};

pub fn create_app() -> Router {
  Router::new().route("/login", post(handlers::login))
}
