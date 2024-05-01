mod src {
  pub mod error;
  pub mod security;
  pub mod services;
}

pub use src::error;
pub use src::security;
pub use src::services;

use database::connection_pool::ConnectionPool;
use src::services::make_service;

fn main() {
  dotenv::dotenv().ok();

  let db = ConnectionPool::faux();

  std::fs::create_dir_all("openapi").unwrap();

  let api_service = (make_service(db)).server("http://localhost:3001");
  let specs = api_service.spec();
  std::fs::write("openapi/main_service.json", specs).unwrap();
}
