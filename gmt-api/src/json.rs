use axum::{
  extract::rejection::JsonRejection, extract::FromRequest, http::StatusCode, response::IntoResponse,
};
use gmt_common::messages::{Error, ErrorCode};
use log::warn;
use serde::Serialize;

/// Custom Json implementation making the generated error a Json response
#[derive(Debug, FromRequest)]
#[from_request(via(axum::Json), rejection(JsonError))]
pub struct Json<T>(pub T);

/// A custom error type for Json responses
#[derive(Debug)]
pub struct JsonError(pub StatusCode, pub Error);

pub type JsonResult<T> = Result<Json<T>, JsonError>;

impl<T: Serialize> IntoResponse for Json<T> {
  fn into_response(self) -> axum::response::Response {
    let Self(value) = self;
    axum::Json(value).into_response()
  }
}

impl From<JsonRejection> for JsonError {
  fn from(rejection: JsonRejection) -> Self {
    Self(
      rejection.status(),
      Error {
        msg: rejection.body_text(),
        code: ErrorCode::INVALID_PAYLOAD.into(),
        ..Default::default()
      },
    )
  }
}

impl IntoResponse for JsonError {
  fn into_response(self) -> axum::response::Response {
    warn!("ApiError: {:?}", self);
    let payload = self.1;

    (self.0, axum::Json(payload)).into_response()
  }
}
