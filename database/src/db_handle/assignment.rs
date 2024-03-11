use diesel::{
  deserialize::Queryable, prelude::Insertable, ExpressionMethods, JoinOnDsl,
  NullableExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, Selectable,
  SelectableHelper,
};

use crate::{
  error::DatabaseError,
  schema::{assignments, repositories},
  TransactionHandler,
};

use super::{group::Group, repository::Repository};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::assignments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Assignment {
  pub id: i32,
  pub group_id: i32,
  pub base_repo_id: i32,
  pub test_repo_id: Option<i32>,
  pub correction_repo_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::assignments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAssignment {
  pub group_id: i32,
  pub base_repo_id: i32,
  pub test_repo_id: Option<i32>,
  pub correction_repo_id: Option<i32>,
}

pub trait AssignmentTransactionHandler {
  fn create_assignment(
    &mut self,
    group_id: i32,
    base_repo_id: i32,
  ) -> Result<Assignment, DatabaseError>;

  fn create_assignment_with_ci(
    &mut self,
    group_id: i32,
    base_repo_id: i32,
    test_repo_id: i32,
  ) -> Result<Assignment, DatabaseError>;

  fn create_assignment_with_correction(
    &mut self,
    group_id: i32,
    base_repo_id: i32,
    correction_repo_id: i32,
  ) -> Result<Assignment, DatabaseError>;

  fn create_assignment_with_ci_and_correction(
    &mut self,
    group_id: i32,
    base_repo_id: i32,
    test_repo_id: i32,
    correction_repo_id: i32,
  ) -> Result<Assignment, DatabaseError>;

  fn get_assignment_by_id(
    &mut self,
    assignment_id: i32,
  ) -> Result<Option<Assignment>, DatabaseError>;

  fn get_assignment_group(&mut self, assignment_id: i32) -> Result<Option<Group>, DatabaseError>;

  fn get_assignment_base_repo(
    &mut self,
    assignment_id: i32,
  ) -> Result<Option<Repository>, DatabaseError>;

  fn get_assignment_test_repo(
    &mut self,
    assignment_id: i32,
  ) -> Result<Option<Repository>, DatabaseError>;

  fn get_assignment_correction_repo(
    &mut self,
    assignment_id: i32,
  ) -> Result<Option<Repository>, DatabaseError>;

  fn get_assignment_submission_repos(
    &mut self,
    assignment_id: i32,
  ) -> Result<Vec<Repository>, DatabaseError>;

  fn delete_assignment(&mut self, assignment_id: i32) -> Result<bool, DatabaseError>;
}

impl<'a> TransactionHandler<'a> {
  fn create_assignment_inner(
    &mut self,
    group_id: i32,
    base_repo_id: i32,
    test_repo_id: Option<i32>,
    correction_repo_id: Option<i32>,
  ) -> Result<Assignment, DatabaseError> {
    let new_assignment = NewAssignment {
      group_id,
      base_repo_id,
      test_repo_id,
      correction_repo_id,
    };

    diesel::insert_into(assignments::table)
      .values(&new_assignment)
      .returning(Assignment::as_select())
      .get_result(self.conn)
      .map_err(DatabaseError::from)
  }
}

impl<'a> AssignmentTransactionHandler for TransactionHandler<'a> {
  fn create_assignment(
    &mut self,
    group_id: i32,
    base_repo_id: i32,
  ) -> Result<Assignment, DatabaseError> {
    self.create_assignment_inner(group_id, base_repo_id, None, None)
  }

  fn create_assignment_with_ci(
    &mut self,
    group_id: i32,
    base_repo_id: i32,
    test_repo_id: i32,
  ) -> Result<Assignment, DatabaseError> {
    self.create_assignment_inner(group_id, base_repo_id, Some(test_repo_id), None)
  }

  fn create_assignment_with_correction(
    &mut self,
    group_id: i32,
    base_repo_id: i32,
    correction_repo_id: i32,
  ) -> Result<Assignment, DatabaseError> {
    self.create_assignment_inner(group_id, base_repo_id, None, Some(correction_repo_id))
  }

  fn create_assignment_with_ci_and_correction(
    &mut self,
    group_id: i32,
    base_repo_id: i32,
    test_repo_id: i32,
    correction_repo_id: i32,
  ) -> Result<Assignment, DatabaseError> {
    self.create_assignment_inner(
      group_id,
      base_repo_id,
      Some(test_repo_id),
      Some(correction_repo_id),
    )
  }

  fn get_assignment_by_id(
    &mut self,
    assignment_id: i32,
  ) -> Result<Option<Assignment>, DatabaseError> {
    use crate::schema::assignments::dsl;

    assignments::table
      .filter(dsl::id.eq(assignment_id))
      .select(Assignment::as_select())
      .first(self.conn)
      .optional()
      .map_err(DatabaseError::from)
  }

  fn get_assignment_group(&mut self, assignment_id: i32) -> Result<Option<Group>, DatabaseError> {
    use crate::schema::assignments::dsl;

    let group = assignments::table
      .filter(dsl::id.eq(assignment_id))
      .inner_join(crate::schema::groups::table)
      .select(Group::as_select())
      .first(self.conn)
      .optional();

    match group {
      Ok(group) => Ok(group),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(DatabaseError::from(e)),
    }
  }

  fn get_assignment_base_repo(
    &mut self,
    assignment_id: i32,
  ) -> Result<Option<Repository>, DatabaseError> {
    use crate::schema::assignments::dsl;

    let repo = assignments::table
      .filter(dsl::id.eq(assignment_id))
      .inner_join(
        crate::schema::repositories::table.on(dsl::base_repo_id.eq(repositories::dsl::id)),
      )
      .select(Repository::as_select())
      .first(self.conn)
      .optional();

    match repo {
      Ok(repo) => Ok(repo),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(DatabaseError::from(e)),
    }
  }

  fn get_assignment_test_repo(
    &mut self,
    assignment_id: i32,
  ) -> Result<Option<Repository>, DatabaseError> {
    use crate::schema::assignments::dsl;

    let repo = assignments::table
      .filter(dsl::id.eq(assignment_id))
      .inner_join(
        crate::schema::repositories::table
          .on(dsl::test_repo_id.eq(repositories::dsl::id.nullable())),
      )
      .select(Repository::as_select())
      .first(self.conn)
      .optional();

    match repo {
      Ok(repo) => Ok(repo),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(DatabaseError::from(e)),
    }
  }

  fn get_assignment_correction_repo(
    &mut self,
    assignment_id: i32,
  ) -> Result<Option<Repository>, DatabaseError> {
    use crate::schema::assignments::dsl;

    let repo = assignments::table
      .filter(dsl::id.eq(assignment_id))
      .inner_join(
        crate::schema::repositories::table
          .on(dsl::correction_repo_id.eq(repositories::dsl::id.nullable())),
      )
      .select(Repository::as_select())
      .first(self.conn)
      .optional();

    match repo {
      Ok(repo) => Ok(repo),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(DatabaseError::from(e)),
    }
  }

  fn get_assignment_submission_repos(
    &mut self,
    assignment_id: i32,
  ) -> Result<Vec<Repository>, DatabaseError> {
    use crate::schema::repositories::dsl;

    repositories::table
      .filter(dsl::assignment_id.eq(assignment_id))
      .select(Repository::as_select())
      .load(self.conn)
      .map_err(DatabaseError::from)
  }

  fn delete_assignment(&mut self, assignment_id: i32) -> Result<bool, DatabaseError> {
    use crate::schema::assignments::dsl;

    diesel::delete(assignments::table.filter(dsl::id.eq(assignment_id)))
      .execute(self.conn)
      .map(|n| n > 0)
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
    transaction_tests,
  };

  use super::AssignmentTransactionHandler;

  transaction_tests! {
    fn create_assignment_with_invalid_group(tx: &mut TransactionHandler) {
      let user = tx.create_user("username", "email", "password", None)?;
      let repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let group_id = 1;
      let res = tx.create_assignment(group_id, repo.id);
      assert!(res.is_err());
    }

    fn create_assignment_with_invalid_base_repo(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let res = tx.create_assignment(group.id, 1);
      assert!(res.is_err());
    }

    fn create_assignment_with_invalid_test_repo(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let res = tx.create_assignment_with_ci(group.id, repo.id, 1);
      assert!(res.is_err());
    }

    fn create_assignment_with_invalid_correction_repo(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let res = tx.create_assignment_with_correction(group.id, repo.id, 1);
      assert!(res.is_err());
    }

    fn create_assignment_with_invalid_ci_and_correction_repos(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let res = tx.create_assignment_with_ci_and_correction(group.id, repo.id, 1, 1);
      assert!(res.is_err());
    }

    fn create_assignment_success(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let assignment = tx.create_assignment(group.id, repo.id)?;
      assert_eq!(assignment.group_id, group.id);
      assert_eq!(assignment.base_repo_id, repo.id);
      assert_eq!(assignment.test_repo_id, None);
      assert_eq!(assignment.correction_repo_id, None);
    }

    fn create_assignment_with_ci_success(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let base_repo = tx.create_repository("base-repo", &Repotype::Default, user.id, None)?;
      let test_repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let assignment = tx.create_assignment_with_ci(group.id, base_repo.id, test_repo.id)?;
      assert_eq!(assignment.group_id, group.id);
      assert_eq!(assignment.base_repo_id, base_repo.id);
      assert_eq!(assignment.test_repo_id, Some(test_repo.id));
      assert_eq!(assignment.correction_repo_id, None);
    }

    fn create_assignment_with_correction_success(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let base_repo = tx.create_repository("base-repo", &Repotype::Default, user.id, None)?;
      let correction_repo = tx.create_repository("correction-repo", &Repotype::Default, user.id, None)?;
      let assignment = tx.create_assignment_with_correction(group.id, base_repo.id, correction_repo.id)?;
      assert_eq!(assignment.group_id, group.id);
      assert_eq!(assignment.base_repo_id, base_repo.id);
      assert_eq!(assignment.test_repo_id, None);
      assert_eq!(assignment.correction_repo_id, Some(correction_repo.id));
    }

    fn create_assignment_with_ci_and_correction_success(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let base_repo = tx.create_repository("base-repo", &Repotype::Default, user.id, None)?;
      let test_repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let correction_repo = tx.create_repository("correction-repo", &Repotype::Default, user.id, None)?;
      let assignment = tx.create_assignment_with_ci_and_correction(group.id, base_repo.id, test_repo.id, correction_repo.id)?;
      assert_eq!(assignment.group_id, group.id);
      assert_eq!(assignment.base_repo_id, base_repo.id);
      assert_eq!(assignment.test_repo_id, Some(test_repo.id));
      assert_eq!(assignment.correction_repo_id, Some(correction_repo.id));
    }

    fn get_nonexistent_assignment_by_id(tx: &mut TransactionHandler) {
      let assignment = tx.get_assignment_by_id(1)?;
      assert!(assignment.is_none());
    }

    fn get_assignment_by_id(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let assignment = tx.create_assignment(group.id, repo.id)?;

      let assignment = tx.get_assignment_by_id(assignment.id)?.expect("Assignment not found");
      assert_eq!(assignment.group_id, group.id);
      assert_eq!(assignment.base_repo_id, repo.id);
      assert_eq!(assignment.test_repo_id, None);
      assert_eq!(assignment.correction_repo_id, None);
    }

    fn get_assignment_group(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let assignment = tx.create_assignment(group.id, repo.id)?;

      let group = tx.get_assignment_group(assignment.id)?.expect("Group not found");
      assert_eq!(group.id, group.id);
      assert_eq!(group.name, "test-group");
    }

    fn get_assignment_base_repo(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let assignment = tx.create_assignment(group.id, repo.id)?;

      let repo = tx.get_assignment_base_repo(assignment.id)?.expect("Repository not found");
      assert_eq!(repo.id, repo.id);
    }

    fn get_assignment_test_repo(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let base_repo = tx.create_repository("base-repo", &Repotype::Default, user.id, None)?;
      let test_repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let assignment = tx.create_assignment_with_ci(group.id, base_repo.id, test_repo.id)?;

      let repo = tx.get_assignment_test_repo(assignment.id)?.expect("Repository not found");
      assert_eq!(repo.id, repo.id);
    }

    fn get_assignment_correction_repo(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let base_repo = tx.create_repository("base-repo", &Repotype::Default, user.id, None)?;
      let correction_repo = tx.create_repository("correction-repo", &Repotype::Default, user.id, None)?;
      let assignment = tx.create_assignment_with_correction(group.id, base_repo.id, correction_repo.id)?;

      let repo = tx.get_assignment_correction_repo(assignment.id)?.expect("Repository not found");
      assert_eq!(repo.id, repo.id);
    }

    fn get_assignment_submission_repos(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let base_repo = tx.create_repository("base-repo", &Repotype::Default, user.id, None)?;
      let assignment = tx.create_assignment(group.id, base_repo.id)?;

      let repos = vec![
        "repo1",
        "repo2",
        "repo3",
      ];
      repos.iter().for_each(|repo_name| {
        tx.create_repository(repo_name, &Repotype::Default, user.id, Some(assignment.id)).expect("Error creating repository");
      });

      let submission_repos = tx.get_assignment_submission_repos(assignment.id)?;
      assert_eq!(submission_repos.len(), repos.len());
    }

    fn delete_assignment(tx: &mut TransactionHandler) {
      let group = tx.create_group("test-group", None)?;
      let user = tx.create_user("username", "email", "password", None)?;
      let repo = tx.create_repository("test-repo", &Repotype::Default, user.id, None)?;
      let assignment = tx.create_assignment(group.id, repo.id)?;

      let res = tx.delete_assignment(assignment.id)?;
      assert!(res, "Error deleting assignment");

      let assignment = tx.get_assignment_by_id(assignment.id)?;
      assert!(assignment.is_none());
    }
  }
}
