use poem_openapi::{payload::{Json, PlainText}, Object, OpenApi};

pub struct HelloService;

#[derive(Object, serde::Deserialize)]
struct HelloRequest {
  name: String,
}

#[derive(Object, serde::Serialize)]
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