use git_server::repository::{Repository, RepositoryPermission};

use crate::gmt_user::GmtUser;

pub struct DbRepository {
  // TODO: Add inner fields backed by DB
}

impl Repository for DbRepository {
  type User = GmtUser;

  fn has_permission(&self, user: &Self::User, _permission: RepositoryPermission) -> bool {
    match user {
      GmtUser::Admin => true,
      GmtUser::Connected() => true,
      GmtUser::Public => false,
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
      repo.has_permission(&GmtUser::Admin, RepositoryPermission::Read),
      true
    );
    assert_eq!(
      repo.has_permission(&GmtUser::Connected(), RepositoryPermission::Read),
      true
    );
    assert_eq!(
      repo.has_permission(&GmtUser::Public, RepositoryPermission::Read),
      false
    );

    assert_eq!(
      repo.has_permission(&GmtUser::Admin, RepositoryPermission::Write),
      true
    );
    assert_eq!(
      repo.has_permission(&GmtUser::Connected(), RepositoryPermission::Write),
      true
    );
    assert_eq!(
      repo.has_permission(&GmtUser::Public, RepositoryPermission::Write),
      false
    );
  }
}
