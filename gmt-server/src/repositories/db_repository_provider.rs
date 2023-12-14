use git_server::repository::RepositoryProvider;

use crate::ssh_user::SshUser;

use super::db_repository::DbRepository;

pub struct DbRepositoryProvider {
  // db_handle: DbConnection,
}

impl DbRepositoryProvider {
  pub fn new() -> Self {
    DbRepositoryProvider {}
  }
}

impl RepositoryProvider for DbRepositoryProvider {
  type User = SshUser;
  type Repository = DbRepository;

  fn find_repository(&self, _user: &Self::User, _path: &str) -> Option<Self::Repository> {
    todo!("Get repository from database")
  }
}
