use diesel::{
  deserialize::Queryable, prelude::Insertable, ExpressionMethods, OptionalExtension, QueryDsl,
  RunQueryDsl, Selectable, SelectableHelper,
};
use diesel_derive_enum::DbEnum;

use crate::{error::DatabaseError, TransactionHandler};

use super::user::User;

#[derive(Debug, DbEnum, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::Repotype"]
pub enum Repotype {
  Default,
  Ci,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::repositories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Repository {
  pub id: i32,
  pub name: String,
  pub repo_type: Repotype,
  pub owner_id: i32,
  pub assignment_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::repositories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRepository<'a> {
  pub name: &'a str,
  pub repo_type: &'a Repotype,
  pub owner_id: i32,
  pub assignment_id: Option<i32>,
}

pub trait RepositoryTransactionHandler {
  fn create_repository(
    &mut self,
    name: &str,
    repo_type: &Repotype,
    owner_id: i32,
    assignment_id: Option<i32>,
  ) -> Result<Repository, DatabaseError>;

  fn get_repository_by_name(&mut self, name: &str) -> Result<Option<Repository>, DatabaseError>;

  fn list_user_repositories(&mut self, user_id: i32) -> Result<Vec<Repository>, DatabaseError>;

  fn get_repository_owner(&mut self, repository_id: i32) -> Result<User, DatabaseError>;

  fn delete_repository(&mut self, repository_id: i32) -> bool;
}

impl<'a> RepositoryTransactionHandler for TransactionHandler<'a> {
  fn create_repository(
    &mut self,
    name: &str,
    repo_type: &Repotype,
    owner_id: i32,
    assignment_id: Option<i32>,
  ) -> Result<Repository, DatabaseError> {
    use crate::schema::repositories;

    let new_repository = NewRepository {
      name,
      repo_type,
      owner_id,
      assignment_id,
    };

    diesel::insert_into(repositories::table)
      .values(&new_repository)
      .returning(Repository::as_returning())
      .get_result(self.conn)
      .map_err(DatabaseError::from)
  }

  fn get_repository_by_name(&mut self, name: &str) -> Result<Option<Repository>, DatabaseError> {
    use crate::schema::repositories::dsl;

    let repository = dsl::repositories
      .filter(dsl::name.eq(name))
      .select(Repository::as_select())
      .first(self.conn)
      .optional();

    match repository {
      Ok(repository) => Ok(repository),
      Err(e) => match e {
        diesel::result::Error::NotFound => Ok(None),
        _ => Err(DatabaseError::from(e)),
      },
    }
  }

  fn list_user_repositories(&mut self, user_id: i32) -> Result<Vec<Repository>, DatabaseError> {
    use crate::schema::repositories::dsl;

    dsl::repositories
      .filter(dsl::owner_id.eq(user_id))
      .select(Repository::as_select())
      .load(self.conn)
      .map_err(DatabaseError::from)
  }

  fn get_repository_owner(&mut self, repository_id: i32) -> Result<User, DatabaseError> {
    use crate::schema::repositories::dsl;
    use crate::schema::users;

    dsl::repositories
      .inner_join(users::table)
      .filter(dsl::id.eq(repository_id))
      .select(User::as_select())
      .first(self.conn)
      .map_err(DatabaseError::from)
  }

  fn delete_repository(&mut self, repository_id: i32) -> bool {
    use crate::schema::repositories::dsl::repositories;

    diesel::delete(repositories.find(repository_id))
      .execute(self.conn)
      .is_ok()
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    db_handle::{
      repository::{RepositoryTransactionHandler, Repotype},
      user::UserTransactionHandler,
    },
    error::DatabaseError,
    transaction_tests,
  };

  transaction_tests! {
    fn create_repository_unknown_user_fails(tx: &mut TransactionHandler) {
      let name = "test-repo";
      let repo_type = Repotype::Default;
      let repository = tx.create_repository(name, &repo_type, 1, None);

      let err = repository.expect_err("Expected error");
      assert!(matches!(err, DatabaseError::DieselError(_)), "Expected diesel error, got: {:?}", err);
    }

    fn create_repository(tx: &mut TransactionHandler) {
      let name = "test-repo";
      let repo_type = Repotype::Default;
      let user = tx.create_user("test_create_repository", "abc", "abc", None)?;

      let repository = tx.create_repository(name, &repo_type, user.id, None)?;
      assert_eq!(repository.name, name);
      assert_eq!(repository.repo_type, repo_type);
      assert_eq!(repository.owner_id, user.id);
    }

    fn get_unknown_repository_by_name(tx: &mut TransactionHandler) {
      let name = "test-repo";
      let repository = tx.get_repository_by_name(name)?;

      assert!(repository.is_none());
    }

    fn get_repository_by_name(tx: &mut TransactionHandler) {
      let name = "test-repo";
      let repo_type = Repotype::Default;

      let user = tx.create_user("test_get_repository_by_name", "abc", "abc", None)?;
      tx.create_repository(name, &repo_type, user.id, None)?;

      let repository = tx.get_repository_by_name(name)?.expect("Repository not found");
      assert_eq!(repository.name, name);
      assert_eq!(repository.repo_type, repo_type);
      assert_eq!(repository.owner_id, user.id);
    }

    fn list_user_repositories(tx: &mut TransactionHandler) {
      let repos = vec![
        ("test-repo-1", Repotype::Default),
        ("test-repo-2", Repotype::Default),
        ("test-repo-3", Repotype::Default),
      ];

      let user = tx.create_user("test_list_user_repositories", "abc", "abc", None)?;
      repos.iter().for_each(|(name, repo_type)| {
        tx.create_repository(name, repo_type, user.id, None).expect("Error creating repository");
      });

      let repositories = tx.list_user_repositories(user.id)?;
      assert_eq!(repositories.len(), repos.len());
    }

    fn list_no_user_repositories(tx: &mut TransactionHandler) {
      let repositories = tx.list_user_repositories(1)?;

      assert!(repositories.is_empty());
    }

    fn get_repository_owner(tx: &mut TransactionHandler) {
      let user = tx.create_user("test_get_repository_owner", "abc", "abc", None)?;
      let repository = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;

      let owner = tx.get_repository_owner(repository.id)?;
      assert_eq!(owner.id, user.id);
    }

    fn delete_repository(tx: &mut TransactionHandler) {
      let user = tx.create_user("test_delete_repository", "abc", "abc", None)?;
      let repository = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;

      let result = tx.delete_repository(repository.id);
      assert!(result);
    }
  }
}
