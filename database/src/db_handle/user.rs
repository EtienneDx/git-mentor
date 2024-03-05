use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{error::DatabaseError, NewUser, TransactionHandler, User};

impl<'a> TransactionHandler<'a> {
  pub fn create_user(
    &mut self,
    username: &str,
    email: &str,
    password: &str,
    pubkey: Option<Vec<&str>>,
  ) -> Result<User, DatabaseError> {
    use crate::schema::users;

    let new_user = NewUser {
      username,
      email,
      password,
      pubkey: &pubkey
        .unwrap_or(vec![])
        .into_iter()
        .map(|key| Some(key.to_string()))
        .collect(),
    };

    diesel::insert_into(users::table)
      .values(&new_user)
      .returning(User::as_returning())
      .get_result(self.conn)
      .map_err(DatabaseError::from)
  }

  pub fn get_user_by_id(&mut self, user_id: i32) -> Result<Option<User>, DatabaseError> {
    use crate::schema::users::dsl;

    let user = dsl::users
      .filter(dsl::id.eq(user_id))
      .select(User::as_select())
      .first(self.conn)
      .optional();

    match user {
      Ok(user) => Ok(user),
      Err(e) => match e {
        diesel::result::Error::NotFound => Ok(None),
        _ => Err(DatabaseError::from(e)),
      },
    }
  }

  pub fn get_user_by_username(&mut self, username: &str) -> Result<Option<User>, DatabaseError> {
    use crate::schema::users::dsl;

    let user = dsl::users
      .filter(dsl::username.eq(username))
      .select(User::as_select())
      .first(self.conn)
      .optional();

    match user {
      Ok(user) => Ok(user),
      Err(e) => match e {
        diesel::result::Error::NotFound => Ok(None),
        _ => Err(DatabaseError::from(e)),
      },
    }
  }

  pub fn get_user_by_email(&mut self, email: &str) -> Result<Option<User>, DatabaseError> {
    use crate::schema::users::dsl;

    let user = dsl::users
      .filter(dsl::email.eq(email))
      .select(User::as_select())
      .first(self.conn)
      .optional();

    match user {
      Ok(user) => Ok(user),
      Err(e) => match e {
        diesel::result::Error::NotFound => Ok(None),
        _ => Err(DatabaseError::from(e)),
      },
    }
  }

  pub fn insert_user_public_key(
    &mut self,
    user_id: i32,
    pubkey: &str,
  ) -> Result<(), DatabaseError> {
    diesel::sql_query(format!(
      "UPDATE users SET pubkey = array_append(pubkey, '{}') WHERE id = {}",
      pubkey, user_id
    ))
    .execute(self.conn)
    .map(|_| ())
    .map_err(DatabaseError::from)
  }

  pub fn delete_user(&mut self, user_id: i32) -> bool {
    use crate::schema::users::dsl::users;

    diesel::delete(users.find(user_id))
      .execute(self.conn)
      .is_ok()
  }
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use crate::db_handle::tests::{db_handle, DbHandleGuard};

  #[derive(thiserror::Error, Debug, PartialEq)]
  enum TestError {
    #[error("Expected")]
    Expected,
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] diesel::result::Error),
  }

  #[rstest]
  fn test_create_user(mut db_handle: DbHandleGuard) {
    let username = "test_create_user";
    let email = "abc";
    let password = "abc";

    let err: Result<(), TestError> = db_handle.transaction(|tx| {
      let user = tx.create_user(username, email, password, None).unwrap();

      assert_eq!(user.username, username);
      assert_eq!(user.email, email);
      assert_eq!(user.password, password);
      assert_eq!(user.pubkey, vec![]);
      Err(TestError::Expected)
    });

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert!(matches!(err, TestError::Expected), "{:?}", err);
  }

  #[rstest]
  fn test_get_user_by_id(mut db_handle: DbHandleGuard) {
    let username = "test_get_user_by_id";
    let email = "abc";
    let password = "abc";

    let err: Result<(), TestError> = db_handle.transaction(|tx| {
      let user = tx.create_user(username, email, password, None).unwrap();
      let user = tx.get_user_by_id(user.id).unwrap().unwrap();

      assert_eq!(user.username, username);
      assert_eq!(user.email, email);
      assert_eq!(user.password, password);
      assert_eq!(user.pubkey, vec![]);
      Err(TestError::Expected)
    });

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert!(matches!(err, TestError::Expected), "{:?}", err);
  }

  #[rstest]
  fn test_get_user_by_username(mut db_handle: DbHandleGuard) {
    let username = "test_get_user_by_username";
    let email = "abc";
    let password = "abc";

    let err: Result<(), TestError> = db_handle.transaction(|tx| {
      tx.create_user(username, email, password, None).unwrap();
      let user = tx.get_user_by_username(username).unwrap().unwrap();

      assert_eq!(user.username, username);
      assert_eq!(user.email, email);
      assert_eq!(user.password, password);
      assert_eq!(user.pubkey, vec![]);
      Err(TestError::Expected)
    });

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert!(matches!(err, TestError::Expected), "{:?}", err);
  }

  #[rstest]
  fn test_get_user_by_email(mut db_handle: DbHandleGuard) {
    let username = "test_get_user_by_email";
    let email = "abc";
    let password = "abc";

    let err: Result<(), TestError> = db_handle.transaction(|tx| {
      tx.create_user(username, email, password, None).unwrap();
      let user = tx.get_user_by_email(email).unwrap().unwrap();

      assert_eq!(user.username, username);
      assert_eq!(user.email, email);
      assert_eq!(user.password, password);
      assert_eq!(user.pubkey, vec![]);
      Err(TestError::Expected)
    });

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert!(matches!(err, TestError::Expected), "{:?}", err);
  }

  #[rstest]
  fn test_insert_user_public_key_without_existing(mut db_handle: DbHandleGuard) {
    let username = "test_insert_user_public_key";
    let email = "abc";
    let password = "abc";
    let pubkey = "pubkey";

    let err: Result<(), TestError> = db_handle.transaction(|tx| {
      let user = tx.create_user(username, email, password, None).unwrap();
      tx.insert_user_public_key(user.id, pubkey).unwrap();

      let user = tx.get_user_by_id(user.id).unwrap().unwrap();
      assert!(
        user.pubkey.contains(&Some(pubkey.to_string())),
        "Expected {:?} to contain '{}'",
        user.pubkey,
        pubkey
      );
      Err(TestError::Expected)
    });

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert!(matches!(err, TestError::Expected), "{:?}", err);
  }

  #[rstest]
  fn test_insert_user_public_key_with_existing(mut db_handle: DbHandleGuard) {
    let username = "test_insert_user_public_key";
    let email = "abc";
    let password = "abc";
    let basepubkey = "pubkey";
    let extrapubkey = "extrapubkey";

    let err: Result<(), TestError> = db_handle.transaction(|tx| {
      let user = tx
        .create_user(username, email, password, Some(vec![basepubkey]))
        .unwrap();
      tx.insert_user_public_key(user.id, extrapubkey).unwrap();

      let user = tx.get_user_by_id(user.id).unwrap().unwrap();
      assert!(
        user.pubkey.contains(&Some(basepubkey.to_string())),
        "Expected {:?} to contain '{}'",
        user.pubkey,
        basepubkey
      );
      assert!(
        user.pubkey.contains(&Some(extrapubkey.to_string())),
        "Expected {:?} to contain '{}'",
        user.pubkey,
        extrapubkey
      );
      Err(TestError::Expected)
    });

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert!(matches!(err, TestError::Expected), "{:?}", err);
  }

  #[rstest]
  fn test_delete_user(mut db_handle: DbHandleGuard) {
    let username = "test_delete_user";
    let email = "abc";
    let password = "abc";

    let err: Result<(), TestError> = db_handle.transaction(|tx| {
      let user = tx.create_user(username, email, password, None).unwrap();
      assert!(tx.delete_user(user.id));

      let user = tx.get_user_by_id(user.id).unwrap();
      assert!(user.is_none());
      Err(TestError::Expected)
    });

    assert!(err.is_err());
    let err = err.unwrap_err();
    assert!(matches!(err, TestError::Expected), "{:?}", err);
  }
}
