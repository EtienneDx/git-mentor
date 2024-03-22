use diesel::{migration::MigrationVersion, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::error::DatabaseError;

#[cfg(test)]
pub mod tests;

pub mod assignment;
pub mod cirun;
pub mod comment;
pub mod group;
pub mod repository;
pub mod user;

pub struct DbHandle {
  conn: PgConnection,
}

impl DbHandle {
  pub fn new(database_url: &str) -> Result<DbHandle, DatabaseError> {
    let conn = PgConnection::establish(database_url)?;
    Ok(DbHandle { conn })
  }

  pub fn new_from_env() -> Result<DbHandle, DatabaseError> {
    let database_url = std::env::var("DATABASE_URL")?;
    DbHandle::new(&database_url)
  }

  pub fn run_migrations(&mut self) -> Result<Vec<MigrationVersion>, DatabaseError> {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    self
      .conn
      .run_pending_migrations(MIGRATIONS)
      .map_err(|_| DatabaseError::MigrationError)
  }
}
