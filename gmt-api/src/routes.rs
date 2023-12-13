use crate::handlers;
use axum::{
  routing::{get, post},
  Router,
};

pub fn create_app() -> Router {
  Router::new()
    .route("/", get(handlers::root))
    .route("/users", post(handlers::create_user))
}
