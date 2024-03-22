use crate::DbHandle;

#[rstest::fixture]
#[once]
fn connection_string() -> String {
  dotenv::dotenv().ok();
  let database_url = std::env::var("DATABASE_URL").unwrap();

  // Run migrations once
  let mut handle = DbHandle::new(&database_url).unwrap();

  handle.run_migrations().expect("Error running migrations");
  drop(handle);

  database_url
}

#[rstest::fixture]
pub fn db_handle(connection_string: &str) -> DbHandle {
  DbHandle::new(&connection_string).expect("Error creating DbHandle")
}

#[macro_export]
macro_rules! transaction_tests {
  {$(fn $name:ident($tx:ident : &mut DbHandle) { $($body:tt)* })*} => {
    use crate::db_handle::{DbHandle, tests::{db_handle}};
    use diesel::Connection;
    $(
      #[rstest::rstest]
      fn $name(db_handle: DbHandle) {
        let mut $tx = db_handle;
        $tx.conn.begin_test_transaction().expect("Error beginning test transaction");

        let mut f = move || {
          $($body)*
          Result::<(), crate::error::DatabaseError>::Ok(())
        };

        let err = f();
        err.expect("Unexpected error in test transaction");
      }
    )*
  };
}
