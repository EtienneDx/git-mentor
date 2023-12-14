use git_server::repository::{Repository, RepositoryPermission};

use crate::ssh_user::SshUser;

pub struct DbRepository {
  // TODO: Add inner fields backed by DB
}

impl Repository for DbRepository {
  type User = SshUser;

  fn has_permission(&self, user: &Self::User, _permission: RepositoryPermission) -> bool {
    match user {
      SshUser::Admin => true,
      SshUser::Connected() => true,
      SshUser::Public => false,
    }
  }

  fn get_path(&self) -> &str {
    todo!()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_permissions() {
    let repo = DbRepository {};

    assert_eq!(
      repo.has_permission(&SshUser::Admin, RepositoryPermission::Read),
      true
    );
    assert_eq!(
      repo.has_permission(&SshUser::Connected(), RepositoryPermission::Read),
      true
    );
    assert_eq!(
      repo.has_permission(&SshUser::Public, RepositoryPermission::Read),
      false
    );

    assert_eq!(
      repo.has_permission(&SshUser::Admin, RepositoryPermission::Write),
      true
    );
    assert_eq!(
      repo.has_permission(&SshUser::Connected(), RepositoryPermission::Write),
      true
    );
    assert_eq!(
      repo.has_permission(&SshUser::Public, RepositoryPermission::Write),
      false
    );
  }
}
