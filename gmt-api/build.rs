mod src {
  pub mod error;
  pub mod services;
}

use std::sync::{Arc, Mutex};

use database::db_handle::DbHandle;
use src::services::make_service;

fn main() {
  dotenv::dotenv().ok();

  let db = DbHandle::faux();
  let db = Arc::new(Mutex::new(db));
  std::fs::create_dir_all("openapi").unwrap();

  let api_service = (make_service(db)).server("http://localhost:3001");
  let specs = api_service.spec();
  std::fs::write("openapi/main_service.json", specs).unwrap();
}
