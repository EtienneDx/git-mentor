use std::{error::Error, ops::DerefMut};

use diesel::{
  migration::MigrationVersion,
  r2d2::{ConnectionManager, PooledConnection},
  PgConnection,
};
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

#[cfg_attr(feature = "mock", faux::create)]
pub struct BaseDbHandle<T>
where
  T: DerefMut<Target = PgConnection>,
{
  pub(crate) conn: T,
}

pub type DbHandle = BaseDbHandle<PooledConnection<ConnectionManager<PgConnection>>>;

#[cfg_attr(feature = "mock", faux::methods)]
impl<T> BaseDbHandle<T>
where
  T: DerefMut<Target = PgConnection>,
{
  pub fn run_migrations(&mut self) -> Result<Vec<MigrationVersion>, DatabaseError> {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    self
      .conn
      .run_pending_migrations(MIGRATIONS)
      .map_err(|e: Box<dyn Error + Send + Sync>| DatabaseError::MigrationError(e))
  }

  #[cfg(test)]
  pub fn begin_test_transaction(&mut self) -> diesel::QueryResult<()> {
    use diesel::Connection;
    self.conn.begin_test_transaction()
  }
}

#[cfg_attr(feature = "mock", faux::methods)]
impl From<PooledConnection<ConnectionManager<PgConnection>>>
  for BaseDbHandle<PooledConnection<ConnectionManager<PgConnection>>>
{
  fn from(conn: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
    BaseDbHandle { conn }
  }
}
