use poem_openapi::OpenApiService;

use self::hello_service::HelloService;

pub mod hello_service;

pub fn make_service() -> OpenApiService<HelloService, ()> {
  OpenApiService::new(HelloService, "Calculator", "1.0")
}
