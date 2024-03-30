use crate::{
  connection_pool::{ConnectionPool, ConnectionProvider},
  DbHandle,
};

#[rstest::fixture]
#[once]
fn connection_string() -> String {
  dotenv::dotenv().ok();
  let database_url = std::env::var("DATABASE_URL").unwrap();

  database_url
}

#[rstest::fixture]
#[once]
fn connection_pool(connection_string: &str) -> ConnectionPool {
  let pool = ConnectionPool::new(&connection_string).expect("Error creating connection pool");
  pool.run_migrations().expect("Error running migrations");
  pool
}

#[rstest::fixture]
pub fn db_handle(connection_pool: &ConnectionPool) -> DbHandle {
  connection_pool
    .get_connection()
    .expect("Error getting connection")
}

#[macro_export]
macro_rules! transaction_tests {
  {$(fn $name:ident($tx:ident : &mut DbHandle) { $($body:tt)* })*} => {
    use crate::db_handle::{DbHandle, tests::{db_handle}};
    $(
      #[rstest::rstest]
      fn $name(db_handle: DbHandle) {
        let mut $tx = db_handle;
        $tx.begin_test_transaction().expect("Error beginning test transaction");

        let mut f = move || {
          $($body)*
          $tx.end_test_transaction();
          Result::<(), crate::error::DatabaseError>::Ok(())
        };

        let err = f();
        err.expect("Unexpected error in test transaction");
      }
    )*
  };
}
