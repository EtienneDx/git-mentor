/// Configuration for the git handler.
#[derive(Default, Debug, Clone)]
pub struct GitHandlerConfig {
  /// Should the handler use `git receive-pack` and `git upload-pack` commands to handle requests
  /// or should it use `git-receive-pack` and `git-upload-pack` binaries.
  pub use_git_command: bool,
}
