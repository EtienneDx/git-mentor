use git_server::{GitHandler, GitHandlerConfig};
use log::debug;
use simple_auth::SimpleAuth;
use ssh_server::SshServer;

use crate::simple_repository_provider::SimpleRepositoryProvider;

mod simple_auth;
mod simple_repository;
mod simple_repository_provider;
mod simple_user;

#[tokio::main]
async fn main() {
  simple_logger::SimpleLogger::new().env().init().unwrap();
  debug!("Logger initialized");

  let auth = SimpleAuth;
  let repository_provider = SimpleRepositoryProvider::new("repositories".to_string());
  let config = GitHandlerConfig {
    use_git_command: true,
  };
  let mut server = SshServer::new(auth);
  server.add_handler(GitHandler::new(config, repository_provider));

  server
    .listen(2222)
    .await
    .expect("Unexpected error while running server");
}
