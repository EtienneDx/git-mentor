use std::env;

use git_server::repository::RepositoryProvider;
use log::debug;

use crate::{simple_repository::SimpleRepository, simple_user::User};

pub struct SimpleRepositoryProvider {
  repositories_path: String,
}

impl SimpleRepositoryProvider {
  pub fn new(repositories_path: String) -> Self {
    Self { repositories_path }
  }
}

impl RepositoryProvider for SimpleRepositoryProvider {
  type User = User;

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
  use std::process::Command;

  use super::*;
  use git_server::repository::RepositoryProvider;
  use tempfile::tempdir;

  #[test]
  fn test_simple_repository_provider() {
    // Setup temp directory and temp repository
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let path = temp_dir.path();

    let repo_path = path.join("test");

    let res = Command::new("git")
      .arg("init")
      .arg(repo_path)
      .output()
      .expect("Failed to execute git init command");

    assert!(res.status.success());

    let provider = SimpleRepositoryProvider::new(path.to_str().unwrap().to_string());
    let repository = provider.find_repository(&User, "test");
    assert!(repository.is_some());

    let repository = provider.find_repository(&User, "another");
    assert!(repository.is_none());
  }
}
