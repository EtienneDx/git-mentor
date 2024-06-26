use std::error::Error;

use diesel::ConnectionError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
  #[error("Connection error: {0}")]
  ConnectionError(#[from] ConnectionError),
  #[error("Missing env error: {0}")]
  VarError(#[from] std::env::VarError),
  #[error("Diesel error: {0}")]
  DieselError(#[from] diesel::result::Error),
  #[error("Connection pool error: {0}")]
  ConnectionPoolError(String),
  #[error("Migration error: {0}")]
  MigrationError(#[from] Box<dyn Error + Send + Sync>),
  #[error("Not found")]
  NotFound,
}
