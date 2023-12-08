use git_server::{GitServer, GitServerConfig};
use log::debug;
use simple_auth::SimpleAuth;

use crate::simple_repository_provider::SimpleRepositoryProvider;

mod simple_auth;
mod simple_repository;
mod simple_repository_provider;

#[tokio::main]
async fn main() {
  simple_logger::SimpleLogger::new().env().init().unwrap();
  debug!("Logger initialized");

  let auth = SimpleAuth;
  let repository_provider = SimpleRepositoryProvider::new("repositories".to_string());
  let config = GitServerConfig {
    use_git_command: true,
  };
  let server = GitServer::new(auth, repository_provider, config);

  server
    .listen(2222)
    .await
    .expect("Unexpected error while running server");
}
