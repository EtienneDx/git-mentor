use poem::Route;
use poem_openapi::{OpenApi, OpenApiService, Webhook};

#[cfg(debug_assertions)]
pub fn add_swagger_ui<T: OpenApi + 'static, W: Webhook + 'static>(mut api_service: OpenApiService<T, W>, mut app: Route) -> (OpenApiService<T, W>, Route) {

  api_service = api_service.server("http://localhost:3001");
  let ui = api_service.swagger_ui();
  app = app.nest("/docs", ui);
  (api_service, app)
}

#[cfg(not(debug_assertions))]
pub fn add_swagger_ui<T: OpenApi + 'static, W: Webhook + 'static>(api_service: OpenApiService<T, W>, app: Route) -> (OpenApiService<T, W>, Route) {
  (api_service, app)
}
