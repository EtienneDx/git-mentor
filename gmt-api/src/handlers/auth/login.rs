use axum::http::StatusCode;
use gmt_common::messages::{Error, ErrorCode, LoginRequest, LoginResponse};
use log::info;

use crate::json::{Json, JsonError, JsonResult};

pub async fn login(Json(_req): Json<LoginRequest>) -> JsonResult<LoginResponse> {
  info!("Login request received");
  Err(JsonError(
    StatusCode::NOT_IMPLEMENTED,
    Error {
      msg: "Unimplemented".to_string(),
      code: ErrorCode::UNIMPLEMENTED.into(),
      ..Default::default()
    },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use gmt_common::messages::{ErrorCode, LoginRequest};

  #[tokio::test]
  async fn test_login() {
    let res = login(Json(LoginRequest::default()))
      .await
      .unwrap_err();

    assert_eq!(res.0, StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
      res.1,
      Error {
        msg: "Unimplemented".to_string(),
        code: ErrorCode::UNIMPLEMENTED.into(),
        ..Default::default()
      }
    );
  }
}