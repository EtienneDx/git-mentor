use std::sync::PoisonError;

use database::error::DatabaseError;
use hmac::digest::InvalidLength;
use poem_openapi::{payload::PlainText, ApiResponse};

#[derive(ApiResponse, thiserror::Error, Debug, PartialEq, Eq)]
pub enum ApiError {
  #[oai(status = 400)]
  #[error("Bad Request")]
  BadRequest,
  #[oai(status = 401)]
  #[error("Unauthorized")]
  Unauthorized,
  #[oai(status = 401)]
  #[error("Conflict")]
  Conflict(PlainText<String>),
  #[oai(status = 403)]
  #[error("Forbidden")]
  Forbidden,
  #[oai(status = 404)]
  #[error("Not Found")]
  NotFound,
  #[oai(status = 500)]
  #[error("Internal Server Error")]
  InternalServerError,
}

impl From<DatabaseError> for ApiError {
  fn from(_: DatabaseError) -> Self {
    ApiError::InternalServerError
  }
}

impl<T> From<PoisonError<T>> for ApiError {
  fn from(_: PoisonError<T>) -> Self {
    ApiError::InternalServerError
  }
}

impl From<jwt::Error> for ApiError {
  fn from(_: jwt::Error) -> Self {
    ApiError::InternalServerError
  }
}

impl From<InvalidLength> for ApiError {
  fn from(_: InvalidLength) -> Self {
    ApiError::InternalServerError
  }
}

pub type ApiResult<T> = Result<T, ApiError>;
