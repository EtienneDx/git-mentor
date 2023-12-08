/// Configuration for the git server.
#[derive(Default, Debug, Clone)]
pub struct GitServerConfig {
  /// Should the server use `git receive-pack` and `git upload-pack` commands to handle requests
  /// or should it use `git-receive-pack` and `git-upload-pack` binaries.
  pub use_git_command: bool,
}
