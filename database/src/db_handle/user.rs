use diesel::{
  deserialize::Queryable, prelude::Insertable, ExpressionMethods, OptionalExtension, QueryDsl,
  RunQueryDsl, Selectable, SelectableHelper,
};

use crate::{error::DatabaseError, TransactionHandler};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub password: String,
  pub pubkey: Vec<Option<String>>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub email: &'a str,
  pub password: &'a str,
  pub pubkey: &'a Vec<Option<String>>,
}

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

  pub fn delete_user(&mut self, user_id: i32) -> Result<(), DatabaseError> {
    use crate::schema::users::dsl::users;

    diesel::delete(users.find(user_id))
      .execute(self.conn)
      .map(|_| ())
      .map_err(DatabaseError::from)
  }
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use crate::{
    db_handle::{
      repository::Repotype,
      tests::{db_handle, DbHandleGuard, TestError},
    },
    error::DatabaseError,
    transaction_tests,
  };

  transaction_tests! {
    fn create_user(tx: &mut TransactionHandler) {
      let username = "create_user";
      let email = "abc";
      let password = "abc";
      let user = tx.create_user(username, email, password, None)?;

      assert_eq!(user.username, username);
      assert_eq!(user.email, email);
      assert_eq!(user.password, password);
      assert_eq!(user.pubkey, vec![]);
    }

    fn get_user_by_id(tx: &mut TransactionHandler) {
      let username = "get_user_by_id";
      let email = "abc";
      let password = "abc";
      let user = tx.create_user(username, email, password, None)?;
      let user = tx.get_user_by_id(user.id)?.expect("User not found");

      assert_eq!(user.username, username);
      assert_eq!(user.email, email);
      assert_eq!(user.password, password);
      assert_eq!(user.pubkey, vec![]);
    }

    fn get_user_by_username(tx: &mut TransactionHandler) {
      let username = "get_user_by_username";
      let email = "abc";
      let password = "abc";

      tx.create_user(username, email, password, None)?;
      let user = tx.get_user_by_username(username)?.expect("User not found");

      assert_eq!(user.username, username);
      assert_eq!(user.email, email);
      assert_eq!(user.password, password);
      assert_eq!(user.pubkey, vec![]);
    }

    fn get_user_by_email(tx: &mut TransactionHandler) {
      let username = "get_user_by_email";
      let email = "abc";
      let password = "abc";

      tx.create_user(username, email, password, None)?;
      let user = tx.get_user_by_email(email)?.expect("User not found");

      assert_eq!(user.username, username);
      assert_eq!(user.email, email);
      assert_eq!(user.password, password);
      assert_eq!(user.pubkey, vec![]);
    }

    fn insert_user_public_key_without_existing(tx: &mut TransactionHandler) {
      let username = "insert_user_public_key";
      let email = "abc";
      let password = "abc";
      let pubkey = "pubkey";

      let user = tx.create_user(username, email, password, None)?;
      tx.insert_user_public_key(user.id, pubkey)?;

      let user = tx.get_user_by_id(user.id)?.expect("User not found");
      assert!(
        user.pubkey.contains(&Some(pubkey.to_string())),
        "Expected {:?} to contain '{}'",
        user.pubkey,
        pubkey
      );
    }

    fn insert_user_public_key_with_existing(tx: &mut TransactionHandler) {
      let username = "insert_user_public_key";
      let email = "abc";
      let password = "abc";
      let basepubkey = "pubkey";
      let extrapubkey = "extrapubkey";

      let user = tx
        .create_user(username, email, password, Some(vec![basepubkey]))?;
      tx.insert_user_public_key(user.id, extrapubkey)?;

      let user = tx.get_user_by_id(user.id)?.expect("User not found");
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
    }

    fn delete_user(tx: &mut TransactionHandler) {
      let username = "delete_user";
      let email = "abc";
      let password = "abc";

      let user = tx.create_user(username, email, password, None)?;
      tx.delete_user(user.id).expect("Error deleting user");

      let user = tx.get_user_by_id(user.id)?;
      assert!(user.is_none());
    }

    fn delete_user_with_repo_fails(tx: &mut TransactionHandler) {
      let username = "delete_user_with_repo_fails";
      let email = "abc";
      let password = "abc";

      let user = tx.create_user(username, email, password, None)?;
      tx.create_repository("test-repo", &Repotype::Default, user.id)?;

      let err = tx.delete_user(user.id).expect_err("Expected error");
      assert!(matches!(err, DatabaseError::DieselError(_)), "Expected diesel error, got: {:?}", err);
    }
  }
}
