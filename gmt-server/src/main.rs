use git_server::{GitHandler, GitHandlerConfig};
use gmt_common::repositories::db_repository_provider::DbRepositoryProvider;
use log::info;
use ssh_server::SshServer;

use crate::authentication::DbAuthenticator;

mod authentication;

#[tokio::main]
async fn main() {
  simple_logger::SimpleLogger::new()
    .with_level(log::LevelFilter::Info)
    .env()
    .init()
    .unwrap();
  info!("Logger initialized");

  let auth = DbAuthenticator::new();
  let repository_provider = DbRepositoryProvider::new();
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
