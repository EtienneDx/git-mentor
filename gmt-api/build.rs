mod src {
  pub mod services;
}

use src::services::make_service;

fn main() {
  std::fs::create_dir_all("openapi").unwrap();

  let api_service = (make_service()).server("http://localhost:3001");
  let specs = api_service.spec();
  std::fs::write("openapi/main_service.json", specs).unwrap();
}
