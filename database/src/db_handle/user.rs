use diesel::{
  deserialize::Queryable, prelude::Insertable, ExpressionMethods, OptionalExtension, QueryDsl,
  RunQueryDsl, Selectable, SelectableHelper,
};

use crate::{error::DatabaseError, TransactionHandler};

use super::group::Group;

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

pub trait UserTransactionHandler {
  fn create_user(
    &mut self,
    username: &str,
    email: &str,
    password: &str,
    pubkey: Option<Vec<&str>>,
  ) -> Result<User, DatabaseError>;

  fn get_user_by_id(&mut self, user_id: i32) -> Result<Option<User>, DatabaseError>;

  fn get_user_by_username(&mut self, username: &str) -> Result<Option<User>, DatabaseError>;

  fn get_user_by_email(&mut self, email: &str) -> Result<Option<User>, DatabaseError>;

  fn insert_user_public_key(&mut self, user_id: i32, pubkey: &str) -> Result<(), DatabaseError>;

  fn delete_user(&mut self, user_id: i32) -> Result<(), DatabaseError>;

  fn list_teaching_groups(&mut self, user_id: i32) -> Result<Vec<Group>, DatabaseError>;

  fn list_belongs_groups(&mut self, user_id: i32) -> Result<Vec<Group>, DatabaseError>;
}

impl<'a> UserTransactionHandler for TransactionHandler<'a> {
  fn create_user(
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

  fn get_user_by_id(&mut self, user_id: i32) -> Result<Option<User>, DatabaseError> {
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

  fn get_user_by_username(&mut self, username: &str) -> Result<Option<User>, DatabaseError> {
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

  fn get_user_by_email(&mut self, email: &str) -> Result<Option<User>, DatabaseError> {
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

  fn insert_user_public_key(&mut self, user_id: i32, pubkey: &str) -> Result<(), DatabaseError> {
    diesel::sql_query(format!(
      "UPDATE users SET pubkey = array_append(pubkey, '{}') WHERE id = {}",
      pubkey, user_id
    ))
    .execute(self.conn)
    .map(|_| ())
    .map_err(DatabaseError::from)
  }

  fn delete_user(&mut self, user_id: i32) -> Result<(), DatabaseError> {
    use crate::schema::users::dsl::users;

    diesel::delete(users.find(user_id))
      .execute(self.conn)
      .map(|_| ())
      .map_err(DatabaseError::from)
  }

  fn list_teaching_groups(&mut self, user_id: i32) -> Result<Vec<Group>, DatabaseError> {
    use crate::schema::groups::dsl;

    dsl::groups
      .filter(dsl::teacher_id.eq(user_id))
      .select(Group::as_select())
      .load(self.conn)
      .map_err(DatabaseError::from)
  }

  fn list_belongs_groups(&mut self, user_id: i32) -> Result<Vec<Group>, DatabaseError> {
    use crate::schema::group_students::dsl;

    dsl::group_students
      .filter(dsl::student_id.eq(user_id))
      .inner_join(crate::schema::groups::table)
      .select(Group::as_select())
      .load(self.conn)
      .map_err(DatabaseError::from)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    db_handle::{
      group::GroupTransactionHandler,
      repository::{RepositoryTransactionHandler, Repotype},
      user::UserTransactionHandler,
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
      tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;

      let err = tx.delete_user(user.id).expect_err("Expected error");
      assert!(matches!(err, DatabaseError::DieselError(_)), "Expected diesel error, got: {:?}", err);
    }

    fn list_teaching_groups_none(tx: &mut TransactionHandler) {
      let username = "list_teaching_groups";
      let email = "abc";
      let password = "abc";

      let user = tx.create_user(username, email, password, None)?;
      let groups = tx.list_teaching_groups(user.id)?;

      assert!(groups.is_empty());
    }

    fn list_teaching_groups_one(tx: &mut TransactionHandler) {
      let username = "list_teaching_groups";
      let email = "abc";
      let password = "abc";

      let user = tx.create_user(username, email, password, None)?;
      tx.create_group("test_group", Some(user.id))?;
      let groups = tx.list_teaching_groups(user.id)?;

      assert_eq!(groups.len(), 1);
    }

    fn list_belongs_groups_none(tx: &mut TransactionHandler) {
      let username = "list_belongs_groups";
      let email = "abc";
      let password = "abc";

      let user = tx.create_user(username, email, password, None)?;
      let groups = tx.list_belongs_groups(user.id)?;

      assert!(groups.is_empty());
    }

    fn list_belongs_groups(tx: &mut TransactionHandler) {
      let username = "list_belongs_groups";
      let email = "abc";
      let password = "abc";

      let user = tx.create_user(username, email, password, None)?;
      let groups = vec![
        ("test_group_1", None),
        ("test_group_2", None),
        ("test_group_3", None),
      ];
      groups.iter().for_each(|(name, teacher_id)| {
        let group = tx.create_group(name, teacher_id.clone()).expect("Error creating group");
        tx.add_student(group.id, user.id).expect("Error adding student");
      });
      let groups = tx.list_belongs_groups(user.id)?;

      assert_eq!(groups.len(), 3);
    }
  }
}
