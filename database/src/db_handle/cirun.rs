use diesel::{
  deserialize::Queryable, prelude::Insertable, ExpressionMethods, OptionalExtension, QueryDsl,
  RunQueryDsl, Selectable, SelectableHelper,
};
use diesel_derive_enum::DbEnum;
use std::ops::DerefMut;

use crate::{error::DatabaseError, DbHandle};

#[derive(Debug, DbEnum, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::Status"]
pub enum Status {
  Success,
  Pending,
  Cancelled,
  Failed,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::cirun)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cirun {
  pub id: i32,
  pub repository_id: i32,
  pub commit: String,
  pub status: Status,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::cirun)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCirun<'a> {
  pub repository_id: i32,
  pub commit: &'a str,
  pub status: &'a Status,
}

pub trait CirunDbHandle {
  fn create_cirun(&mut self, repository_id: i32, commit: &str) -> Result<Cirun, DatabaseError>;

  fn create_cirun_with_status(
    &mut self,
    repository_id: i32,
    commit: &str,
    status: &Status,
  ) -> Result<Cirun, DatabaseError>;

  fn get_cirun_by_id(&mut self, cirun_id: i32) -> Result<Option<Cirun>, DatabaseError>;

  fn get_cirun_by_commit(
    &mut self,
    repository_id: i32,
    commit: &str,
  ) -> Result<Option<Cirun>, DatabaseError>;

  fn list_repository_ciruns(&mut self, repository_id: i32) -> Result<Vec<Cirun>, DatabaseError>;

  fn update_cirun_status(&mut self, cirun_id: i32, status: &Status)
    -> Result<Cirun, DatabaseError>;
}

#[cfg_attr(feature = "mock", faux::methods(path = "super"))]
impl CirunDbHandle for DbHandle {
  fn create_cirun(&mut self, repository_id: i32, commit: &str) -> Result<Cirun, DatabaseError> {
    self.create_cirun_with_status(repository_id, commit, &Status::Pending)
  }

  fn create_cirun_with_status(
    &mut self,
    repository_id: i32,
    commit: &str,
    status: &Status,
  ) -> Result<Cirun, DatabaseError> {
    use crate::schema::cirun;

    let new_cirun = NewCirun {
      repository_id,
      commit,
      status,
    };

    diesel::insert_into(cirun::table)
      .values(&new_cirun)
      .returning(Cirun::as_returning())
      .get_result(self.conn.deref_mut())
      .map_err(DatabaseError::from)
  }

  fn get_cirun_by_id(&mut self, cirun_id: i32) -> Result<Option<Cirun>, DatabaseError> {
    use crate::schema::cirun::dsl;

    let cirun = dsl::cirun
      .filter(dsl::id.eq(cirun_id))
      .select(Cirun::as_select())
      .first(self.conn.deref_mut())
      .optional();
    match cirun {
      Ok(cirun) => Ok(cirun),
      Err(_) => Err(DatabaseError::NotFound),
    }
  }

  fn get_cirun_by_commit(
    &mut self,
    repository_id: i32,
    commit: &str,
  ) -> Result<Option<Cirun>, DatabaseError> {
    use crate::schema::cirun::dsl;

    let cirun = dsl::cirun
      .filter(dsl::repository_id.eq(repository_id))
      .filter(dsl::commit.eq(commit))
      .select(Cirun::as_select())
      .first(self.conn.deref_mut())
      .optional();
    match cirun {
      Ok(cirun) => Ok(cirun),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(DatabaseError::from(e)),
    }
  }

  fn list_repository_ciruns(&mut self, repository_id: i32) -> Result<Vec<Cirun>, DatabaseError> {
    use crate::schema::cirun::dsl;

    dsl::cirun
      .filter(dsl::repository_id.eq(repository_id))
      .select(Cirun::as_select())
      .load(self.conn.deref_mut())
      .map_err(DatabaseError::from)
  }

  fn update_cirun_status(
    &mut self,
    cirun_id: i32,
    status: &Status,
  ) -> Result<Cirun, DatabaseError> {
    use crate::schema::cirun::dsl;

    diesel::update(dsl::cirun.filter(dsl::id.eq(cirun_id)))
      .set(dsl::status.eq(status))
      .returning(Cirun::as_returning())
      .get_result(self.conn.deref_mut())
      .map_err(DatabaseError::from)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    db_handle::{
      repository::{RepositoryDbHandle, Repotype},
      user::UserDbHandle,
    },
    transaction_tests,
  };

  use super::CirunDbHandle;

  transaction_tests! {
    fn create_cirun(tx: &mut DbHandle) {
      let user = tx.create_user("user", "email", "password", None)?;
      let repo = tx.create_repository("name", &Repotype::Default, user.id, None)?;
      let cirun = tx.create_cirun(repo.id, "commit")?;
      assert_eq!(cirun.repository_id, repo.id);
      assert_eq!(cirun.commit, "commit");
      assert_eq!(cirun.status, crate::db_handle::cirun::Status::Pending);
    }

    fn create_cirun_invalid_repo_fails(tx: &mut DbHandle) {
      let cirun = tx.create_cirun(1, "commit");
      assert!(cirun.is_err());
    }

    fn create_cirun_with_status(tx: &mut DbHandle) {
      let user = tx.create_user("user", "email", "password", None)?;
      let repo = tx.create_repository("name", &Repotype::Default, user.id, None)?;
      let cirun = tx.create_cirun_with_status(repo.id, "commit", &crate::db_handle::cirun::Status::Success)?;
      assert_eq!(cirun.repository_id, repo.id);
      assert_eq!(cirun.commit, "commit");
      assert_eq!(cirun.status, crate::db_handle::cirun::Status::Success);
    }

    fn get_cirun_by_id(tx: &mut DbHandle) {
      let user = tx.create_user("user", "email", "password", None)?;
      let repo = tx.create_repository("name", &Repotype::Default, user.id, None)?;
      let cirun = tx.create_cirun(repo.id, "commit")?;
      let found_cirun = tx.get_cirun_by_id(cirun.id)?.expect("Cirun not found");
      assert_eq!(found_cirun.repository_id, repo.id);
      assert_eq!(found_cirun.commit, "commit");
      assert_eq!(found_cirun.status, crate::db_handle::cirun::Status::Pending);
    }

    fn get_nonexistent_cirun_by_id(tx: &mut DbHandle) {
      let cirun = tx.get_cirun_by_id(1)?;
      assert!(cirun.is_none());
    }

    fn get_cirun_by_commit(tx: &mut DbHandle) {
      let user = tx.create_user("user", "email", "password", None)?;
      let repo = tx.create_repository("name", &Repotype::Default, user.id, None)?;
      tx.create_cirun(repo.id, "commit")?;
      let found_cirun = tx.get_cirun_by_commit(repo.id, "commit")?.expect("Cirun not found");
      assert_eq!(found_cirun.repository_id, repo.id);
      assert_eq!(found_cirun.commit, "commit");
      assert_eq!(found_cirun.status, crate::db_handle::cirun::Status::Pending);
    }

    fn get_nonexistent_repo_cirun_by_commit(tx: &mut DbHandle) {
      let cirun = tx.get_cirun_by_commit(1, "commit")?;
      assert!(cirun.is_none());
    }

    fn get_cirun_by_nonexistent_commit(tx: &mut DbHandle) {
      let user = tx.create_user("user", "email", "password", None)?;
      let repo = tx.create_repository("name", &Repotype::Default, user.id, None)?;
      let cirun = tx.get_cirun_by_commit(repo.id, "commit")?;
      assert!(cirun.is_none());
    }

    fn list_repository_ciruns(tx: &mut DbHandle) {
      let user = tx.create_user("user", "email", "password", None)?;
      let repo = tx.create_repository("name", &Repotype::Default, user.id, None)?;
      let commits = vec![
        "commit1",
        "commit2",
        "commit3",
      ];
      commits.iter().for_each(|commit| {
        tx.create_cirun(repo.id, commit).expect("Error creating cirun");
      });
      let ciruns = tx.list_repository_ciruns(repo.id)?;
      assert_eq!(ciruns.len(), commits.len());
    }

    fn update_cirun_status(tx: &mut DbHandle) {
      let user = tx.create_user("user", "email", "password", None)?;
      let repo = tx.create_repository("name", &Repotype::Default, user.id, None)?;
      let cirun = tx.create_cirun(repo.id, "commit")?;
      tx.update_cirun_status(cirun.id, &crate::db_handle::cirun::Status::Success)?;
      let updated_cirun = tx.get_cirun_by_id(cirun.id)?.expect("Cirun not found");
      assert_eq!(updated_cirun.status, crate::db_handle::cirun::Status::Success);
    }

    fn update_nonexistent_cirun_status_fails(tx: &mut DbHandle) {
      let status = tx.update_cirun_status(1, &crate::db_handle::cirun::Status::Success);
      assert!(status.is_err());
    }
  }
}
