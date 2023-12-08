use git_server::repository::{Repository, RepositoryPermission};

pub struct SimpleRepository {
  path: String,
}

impl SimpleRepository {
  pub fn new(path: String) -> Self {
    Self { path }
  }
}

impl Repository for SimpleRepository {
  type User = ();

  fn has_permission(&self, _user: &Self::User, _permission: RepositoryPermission) -> bool {
    true
  }

  fn get_path(&self) -> &str {
    &self.path
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use git_server::repository::RepositoryPermission;

  #[test]
  fn test_simple_repository() {
    let repository = SimpleRepository::new("test".to_string());
    assert_eq!(repository.get_path(), "test");
    assert!(repository.has_permission(&(), RepositoryPermission::Read));
    assert!(repository.has_permission(&(), RepositoryPermission::Write));
  }
}
