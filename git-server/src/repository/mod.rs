mod repository_provider;
mod repository_trait;

pub use repository_provider::*;
pub use repository_trait::*;

/// Enum representing the different permissions that can be granted to a user for a repository.
#[derive(Debug, PartialEq)]
pub enum RepositoryPermission {
  Read,
  Write,
}
