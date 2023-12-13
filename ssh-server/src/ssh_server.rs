use std::sync::Arc;

use russh::server::{Config, Server};

use crate::{
  authenticator::Authenticator, error::SshError, handler::Handler,
  server::request_handler::RequestHandler, user::User,
};

pub struct SshServer<A, U>
where
  A: Authenticator<User = U>,
  U: User,
{
  authenticator: Arc<A>,
  handlers: Vec<Arc<Box<dyn Handler<User = U>>>>,
}

/// A Git server implementation that uses an authenticator and repository provider.
///
/// `A` is the authenticator type that provides user authentication.
/// `R` is the repository provider type that provides access to Git repositories.
/// `U` is the user type that is returned by the authenticator and used by the repositories to check permissions.
impl<A, U> SshServer<A, U>
where
  A: Authenticator<User = U>,
  U: User,
{
  /// Creates a new `SshServer`, using the given `Authenticator`, `RepositoryProvider` and configuration.
  ///
  /// - The `Authenticator` is used to authenticate users based on their public key.
  /// - The `RepositoryProvider` is used to provide repositories based on the user and the path requested.
  /// - The `SshServerConfig` is used to configure the server.
  ///
  /// The generic arguments are deduced from the arguments passed to this function.
  pub fn new(authenticator: A) -> Self {
    SshServer {
      authenticator: Arc::new(authenticator),
      handlers: Vec::new(),
    }
  }

  /// Starts listening for connections on the given port.
  pub async fn listen(self, port: u16) -> Result<(), SshError> {
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

  /// Adds a handler to the server. Handlers are called in the order they are added, stopping when one of them returns `Accepted` or `Rejected`.
  pub fn add_handler<H: Handler<User = U> + 'static>(&mut self, handler: H) {
    self.handlers.push(Arc::new(Box::new(handler)));
  }
}

impl<A, U> Server for SshServer<A, U>
where
  A: Authenticator<User = U>,
  U: User,
{
  type Handler = RequestHandler<A, U>;

  fn new_client(&mut self, peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
    RequestHandler::new(self.authenticator.clone(), self.handlers.clone(), peer_addr)
  }
}
