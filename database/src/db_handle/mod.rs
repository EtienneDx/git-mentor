use diesel::{migration::MigrationVersion, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::error::DatabaseError;

#[cfg(test)]
pub mod tests;

pub mod repository;
pub mod user;

pub struct DbHandle {
  conn: PgConnection,
}

impl DbHandle {
  pub fn new(database_url: String) -> Result<DbHandle, DatabaseError> {
    let conn = PgConnection::establish(&database_url)?;
    Ok(DbHandle { conn })
  }

  pub fn new_from_env() -> Result<DbHandle, DatabaseError> {
    let database_url = std::env::var("DATABASE_URL")?;
    DbHandle::new(database_url)
  }

  pub fn run_migrations(&mut self) -> Result<Vec<MigrationVersion>, DatabaseError> {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    self
      .conn
      .run_pending_migrations(MIGRATIONS)
      .map_err(|_| DatabaseError::MigrationError)
  }

  pub fn transaction<T, E, F>(&mut self, f: F) -> Result<T, E>
  where
    F: FnOnce(&mut TransactionHandler) -> Result<T, E>,
    E: From<diesel::result::Error>,
  {
    self
      .conn
      .transaction(|conn| f(&mut TransactionHandler { conn }))
  }
}

pub struct TransactionHandler<'a> {
  conn: &'a mut PgConnection,
}
