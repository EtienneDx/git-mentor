use crate::error::SshError;

use russh_keys::key::PublicKey;
/// A trait for authenticating users based on their public key.
pub trait Authenticator: Sync + Send + 'static {
  type User: Sync + Send;

  /// Validates a public key for a given user.
  ///
  /// # Arguments
  ///
  /// * `user` - The linux username used to authenticate. Usually, git servers enforce all users use the same `git`linux user.
  /// * `key` - The public key to validate.
  ///
  /// # Returns
  ///
  /// A `Result` containing either `Some(User)` if the key is valid, or `None` if the key is invalid.
  fn validate_public_key(
    &self,
    user: &str,
    key: &PublicKey,
  ) -> Result<Option<Self::User>, SshError>;
}
