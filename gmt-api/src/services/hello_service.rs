use poem_openapi::{
  payload::{Json, PlainText},
  Object, OpenApi,
};

use crate::security::gmt_token::{GmtToken, TokenError};

pub struct HelloService;

#[derive(Object, serde::Serialize, serde::Deserialize)]
struct HelloRequest {
  name: String,
}

#[derive(Object, serde::Serialize, serde::Deserialize)]
struct HelloResponse {
  message: String,
}

#[OpenApi]
impl HelloService {
  #[oai(path = "/hello", method = "get")]
  async fn hello(&self) -> PlainText<String> {
    PlainText("Hello world!".to_string())
  }

  #[oai(path = "/hello", method = "post")]
  async fn hello_name(&self, req: Json<HelloRequest>) -> Json<HelloResponse> {
    Json(HelloResponse {
      message: format!("Hello, {}!", req.name),
    })
  }

  #[oai(path = "/hello/me", method = "get")]
  async fn hello_user(&self, token: GmtToken) -> Result<Json<HelloResponse>, TokenError> {
    Ok(Json(HelloResponse {
      message: format!("Hello, {}!", token.get_user()?.username),
    }))
  }
}

#[cfg(test)]
mod tests {
  use crate::services::test_utils::valid_token;

  use super::*;
  use poem::{http::StatusCode, test::TestClient, Route};
  use poem_openapi::OpenApiService;
  use rstest::{fixture, rstest};

  #[fixture]
  fn client() -> TestClient<Route> {
    let service = OpenApiService::new(HelloService, "", "");

    let app = Route::new().nest("/", service);

    TestClient::new(app)
  }

  #[rstest]
  #[tokio::test]
  async fn test_hello(client: TestClient<Route>) {
    let resp = client.get("/hello").send().await;
    resp.assert_status_is_ok();
    resp.assert_text("Hello world!").await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_hello_name(client: TestClient<Route>) {
    let resp = client
      .post("/hello")
      .body_json(&HelloRequest {
        name: "world".to_string(),
      })
      .send()
      .await;
    resp.assert_status_is_ok();
    resp
      .assert_json(HelloResponse {
        message: "Hello, world!".to_string(),
      })
      .await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_hello_user_missing_auth_header(client: TestClient<Route>) {
    let resp = client.get("/hello/me").send().await;

    resp.assert_status(StatusCode::UNAUTHORIZED);
  }

  #[rstest]
  #[tokio::test]
  async fn test_hello_user_invalid_auth_header(client: TestClient<Route>) {
    let resp = client
      .get("/hello/me")
      .header("Authentication", "someinvalidtoken")
      .send()
      .await;

    resp.assert_status(StatusCode::UNAUTHORIZED);
  }

  #[rstest]
  #[tokio::test]
  async fn test_hello_user_valid_auth_header(client: TestClient<Route>, valid_token: String) {
    let resp = client
      .get("/hello/me")
      .header("Authentication", valid_token)
      .send()
      .await;

    resp.assert_status(StatusCode::UNAUTHORIZED);
  }
}
