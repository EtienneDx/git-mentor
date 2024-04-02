use std::ops::DerefMut;

use diesel::{Connection, PgConnection};

use crate::db_handle::BaseDbHandle;

pub struct HandleWrapper(PgConnection);

#[cfg_attr(feature = "mock", faux::methods(path = "super"))]
impl BaseDbHandle<HandleWrapper> {
  fn new(connection_string: &str) -> Result<Self, crate::error::DatabaseError> {
    let conn = PgConnection::establish(connection_string)?;
    Ok(BaseDbHandle::<HandleWrapper> {
      conn: HandleWrapper(conn),
    })
  }
}

impl std::ops::Deref for HandleWrapper {
  type Target = PgConnection;

  fn deref(&self) -> &PgConnection {
    &self.0
  }
}

impl DerefMut for HandleWrapper {
  fn deref_mut(&mut self) -> &mut PgConnection {
    &mut self.0
  }
}

#[rstest::fixture]
#[once]
fn connection_string() -> String {
  dotenv::dotenv().ok();
  let database_url = std::env::var("DATABASE_URL").unwrap();

  // Run migrations once
  let mut handle = BaseDbHandle::new(&database_url).unwrap();

  handle.run_migrations().expect("Error running migrations");
  drop(handle);

  database_url
}

#[rstest::fixture]
pub fn db_handle<'a>(connection_string: &str) -> BaseDbHandle<HandleWrapper> {
  BaseDbHandle::<HandleWrapper>::new(&connection_string).expect("Error creating DbHandle")
}

#[macro_export]
macro_rules! transaction_tests {
  {$(fn $name:ident($tx:ident : &mut DbHandle) { $($body:tt)* })*} => {
    use crate::db_handle::{BaseDbHandle, tests::{db_handle, HandleWrapper}};
    $(
      #[rstest::rstest]
      fn $name(db_handle: BaseDbHandle<HandleWrapper>) {
        let mut $tx = db_handle;
        $tx.begin_test_transaction().expect("Error beginning test transaction");

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
