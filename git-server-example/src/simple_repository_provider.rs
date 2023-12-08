use std::env;

use git_server::repository::RepositoryProvider;
use log::debug;

use crate::simple_repository::SimpleRepository;

pub struct SimpleRepositoryProvider {
  repositories_path: String,
}

impl SimpleRepositoryProvider {
  pub fn new(repositories_path: String) -> Self {
    Self { repositories_path }
  }
}

impl RepositoryProvider for SimpleRepositoryProvider {
  type User = ();

  type Repository = SimpleRepository;

  fn find_repository(&self, _user: &Self::User, path: &str) -> Option<Self::Repository> {
    let path = path.trim_start_matches('/');
    let mut dir = env::current_dir().unwrap();
    dir.push(&self.repositories_path);
    dir.push(path);
    debug!("Checking if path {} exists", dir.display());

    if dir.exists() && dir.is_dir() {
      Some(SimpleRepository::new(dir.to_str().unwrap().to_string()))
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use git_server::repository::RepositoryProvider;

  #[test]
  fn test_simple_repository_provider() {
    let provider = SimpleRepositoryProvider::new("repositories".to_string());
    let repository = provider.find_repository(&(), "test");
    assert!(repository.is_some());
    // not checking path, it is installation dependant
  }
}