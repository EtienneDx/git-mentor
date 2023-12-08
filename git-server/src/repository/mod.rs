mod repository_provider;

pub use repository_provider::*;

/// Enum representing the different permissions that can be granted to a user for a repository.
#[derive(Debug, PartialEq)]
pub enum RepositoryPermission {
  Read,
  Write,
}

/// Trait representing a repository.
pub trait Repository: Sync + Send + 'static {
  type User;

  /// Checks if the given user has the specified permission for this repository.
  fn has_permission(&self, user: &Self::User, permission: RepositoryPermission) -> bool;

  /// Returns the path of this repository on disk. This is the path used by the git command to access the repository, not necessarily the path given by the user.
  fn get_path(&self) -> &str;
}
