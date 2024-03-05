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
  #[error("Migration error")]
  MigrationError,
}
