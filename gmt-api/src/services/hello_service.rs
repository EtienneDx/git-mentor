use poem_openapi::{
  payload::{Json, PlainText},
  Object, OpenApi,
};

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
}

#[cfg(test)]
mod tests {
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
  async fn test_hello() {
    let service = OpenApiService::new(HelloService, "", "");

    let app = Route::new().nest("/", service);

    let client = TestClient::new(app);
    let resp = client.get("/hello").send().await;
    resp.assert_status(StatusCode::OK);
    resp.assert_text("Hello world!").await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_hello_name() {
    let service = OpenApiService::new(HelloService, "", "");

    let app = Route::new().nest("/", service);

    let client = TestClient::new(app);
    let resp = client
      .post("/hello")
      .body_json(&HelloRequest {
        name: "world".to_string(),
      })
      .send()
      .await;
    resp.assert_status(StatusCode::OK);
    resp
      .assert_json(HelloResponse {
        message: "Hello, world!".to_string(),
      })
      .await;
  }
}
