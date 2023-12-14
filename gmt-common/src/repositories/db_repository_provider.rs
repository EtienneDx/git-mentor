use git_server::repository::RepositoryProvider;

use crate::gmt_user::GmtUser;

use super::db_repository::DbRepository;

pub struct DbRepositoryProvider {
  // db_handle: DbConnection,
}

impl DbRepositoryProvider {
  #[allow(clippy::new_without_default)] // TODO: remove this when adding the database
  pub fn new() -> Self {
    DbRepositoryProvider {}
  }
}

impl RepositoryProvider for DbRepositoryProvider {
  type User = GmtUser;
  type Repository = DbRepository;

  fn find_repository(&self, _user: &Self::User, _path: &str) -> Option<Self::Repository> {
    todo!("Get repository from database")
  }
}
