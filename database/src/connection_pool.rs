use diesel::{
  r2d2::{ConnectionManager, Pool, PooledConnection},
  PgConnection,
};

use crate::{db_handle::BaseDbHandle, error::DatabaseError};

pub trait ConnectionProvider: Send + Sync {
  type Connection;
  fn get_connection(&self) -> Result<Self::Connection, DatabaseError>;
}

#[cfg_attr(feature = "mock", faux::create)]
pub struct ConnectionPool {
  pool: Pool<ConnectionManager<PgConnection>>,
}

#[cfg_attr(feature = "mock", faux::methods)]
impl ConnectionPool {
  pub fn new(database_url: &str) -> Result<Self, DatabaseError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
      .build(manager)
      .map_err(|e| DatabaseError::ConnectionPoolError(e.to_string()))?;
    Ok(ConnectionPool { pool })
  }

  pub fn new_from_env() -> Result<Self, DatabaseError> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    ConnectionPool::new(&database_url)
  }

  pub fn run_migrations(&self) -> Result<(), DatabaseError> {
    let mut handle = self.get_connection()?;
    handle.run_migrations()?;
    Ok(())
  }
}

#[cfg_attr(feature = "mock", faux::methods)]
impl ConnectionProvider for ConnectionPool {
  type Connection = BaseDbHandle<PooledConnection<ConnectionManager<PgConnection>>>;

  fn get_connection(
    &self,
  ) -> Result<BaseDbHandle<PooledConnection<ConnectionManager<PgConnection>>>, DatabaseError> {
    let conn = self
      .pool
      .get()
      .map_err(|e| DatabaseError::ConnectionPoolError(e.to_string()))?;
    Ok(conn.into())
  }
}
