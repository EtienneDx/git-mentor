use std::sync::Arc;

use russh::server::{Config, Server};

use crate::{
  authenticator::Authenticator, error::GitError, git_server_config::GitServerConfig,
  repository::RepositoryProvider,
};

use crate::server::request_handler::RequestHandler;

pub struct GitServer<A, R, U>
where
  A: Authenticator<User = U>,
  R: RepositoryProvider<User = U>,
  U: Sync + Send + 'static,
{
  authenticator: Arc<A>,
  repository_provider: Arc<R>,
  config: GitServerConfig,
}

/// A Git server implementation that uses an authenticator and repository provider.
///
/// `A` is the authenticator type that provides user authentication.
/// `R` is the repository provider type that provides access to Git repositories.
/// `U` is the user type that is returned by the authenticator and used by the repositories to check permissions.
impl<A, R, U> GitServer<A, R, U>
where
  A: Authenticator<User = U>,
  R: RepositoryProvider<User = U>,
  U: Sync + Send + 'static,
{
  /// Creates a new `GitServer`, using the given `Authenticator`, `RepositoryProvider` and configuration.
  ///
  /// - The `Authenticator` is used to authenticate users based on their public key.
  /// - The `RepositoryProvider` is used to provide repositories based on the user and the path requested.
  /// - The `GitServerConfig` is used to configure the server.
  ///
  /// The generic arguments are deduced from the arguments passed to this function.
  pub fn new(authenticator: A, repository_provider: R, config: GitServerConfig) -> Self {
    GitServer {
      authenticator: Arc::new(authenticator),
      repository_provider: Arc::new(repository_provider),
      config,
    }
  }

  /// Starts listening for connections on the given port.
  pub async fn listen(self, port: u16) -> Result<(), GitError> {
    let config = Config {
      inactivity_timeout: Some(std::time::Duration::from_secs(30)),
      auth_rejection_time: std::time::Duration::from_secs(3),
      keys: vec![russh_keys::key::KeyPair::generate_ed25519().unwrap()],
      ..Default::default()
    };
    let config = Arc::new(config);
    let res = russh::server::run(config, ("0.0.0.0", port), self);
    println!("Listening on port {}", port);
    res.await.map_err(|e| e.into())
  }
}

impl<A, R, U> Server for GitServer<A, R, U>
where
  A: Authenticator<User = U>,
  R: RepositoryProvider<User = U>,
  U: Sync + Send + 'static,
{
  type Handler = RequestHandler<A, R, U>;

  fn new_client(&mut self, peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
    RequestHandler::new(
      self.authenticator.clone(),
      self.repository_provider.clone(),
      self.config.clone(),
      peer_addr,
    )
  }
}
