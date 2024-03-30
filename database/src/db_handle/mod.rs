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
pub struct DbHandle {
  conn: PooledConnection<ConnectionManager<PgConnection>>,
}

#[cfg_attr(feature = "mock", faux::methods)]
impl DbHandle {
  pub fn run_migrations(&mut self) -> Result<Vec<MigrationVersion>, DatabaseError> {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    self
      .conn
      .run_pending_migrations(MIGRATIONS)
      .map_err(|_| DatabaseError::MigrationError)
  }

  #[cfg(test)]
  pub fn begin_test_transaction(&mut self) -> diesel::QueryResult<()> {
    use diesel::Connection;
    self.conn.begin_test_transaction()
  }

  /// Method which aims at rendering the connection unoperable. This is useful for testing to ensure
  /// that the connection is not sent back to the pool after a test, since there is no "end_test_transaction" method
  /// in diesel.
  #[cfg(test)]
  pub fn end_test_transaction(&mut self) -> () {
    use diesel::RunQueryDsl;
    use std::ops::DerefMut;
    let _ = diesel::dsl::sql_query("ERROR").execute(self.conn.deref_mut());
  }
}

#[cfg_attr(feature = "mock", faux::methods)]
impl From<PooledConnection<ConnectionManager<PgConnection>>> for DbHandle {
  fn from(conn: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
    DbHandle { conn }
  }
}
