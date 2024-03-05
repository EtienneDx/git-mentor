use std::sync::{Mutex, MutexGuard};

use crate::{error::DatabaseError, DbHandle};

#[rstest::fixture]
#[once]
fn db_handle_mux() -> Mutex<DbHandle> {
  dotenv::dotenv().ok();

  let database_url = std::env::var("DATABASE_URL").unwrap();
  let mut handle = DbHandle::new(database_url).unwrap();

  handle.run_migrations().expect("Error running migrations");

  Mutex::new(handle)
}

pub type DbHandleGuard<'a> = MutexGuard<'a, DbHandle>;

#[rstest::fixture]
pub fn db_handle<'a>(db_handle_mux: &'a Mutex<DbHandle>) -> DbHandleGuard<'a> {
  db_handle_mux.lock().unwrap()
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TestError {
  #[error("Expected")]
  Expected,
  #[error("Unexpected diesel error: {0}")]
  UnexpectedDiesel(#[from] diesel::result::Error),
  #[error("Unexpected wrapper error: {0}")]
  UnexpectedWrapper(#[from] DatabaseError),
}

#[macro_export]
macro_rules! transaction_tests {
  {$(fn $name:ident($tx:ident : &mut TransactionHandler) { $($body:tt)* })*} => {
    use crate::db_handle::tests::{DbHandleGuard, TestError, db_handle};
    $(
      #[rstest::rstest]
      fn $name(mut db_handle: DbHandleGuard) {
        let err: Result<(), TestError> = db_handle.transaction(|$tx| {
          $($body)*
          Err(TestError::Expected)
        });

        assert!(err.is_err());
        let err = err.unwrap_err();
        assert!(matches!(err, TestError::Expected), "{:?}", err);
      }
    )*
  };
}
