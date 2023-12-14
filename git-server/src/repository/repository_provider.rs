use ssh_server::user::User;

use crate::repository::Repository;
/// A trait for providing repositories.
pub trait RepositoryProvider: Sync + Send + 'static {
  type User: User;
  type Repository: Repository<User = Self::User>;

  /// Finds a repository given its path.
  ///
  /// # Arguments
  ///
  /// * `user` - The user requesting the repository
  /// * `path` - The path requested by the user
  ///
  /// # Returns
  ///
  /// A repository if it exists, otherwise `None`.
  fn find_repository(&self, user: &Self::User, path: &str) -> Option<Self::Repository>;
}
