use git_server::authenticator::Authenticator;

use russh_keys::key::PublicKey;

pub struct SimpleAuth;

impl Authenticator for SimpleAuth {
  type User = ();

  fn validate_public_key(
    &self,
    _user: &str,
    _key: &PublicKey,
  ) -> Result<Option<Self::User>, git_server::error::GitError> {
    Ok(Some(()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use russh_keys::key::KeyPair;

  #[test]
  fn test_simple_auth() {
    let auth = SimpleAuth;
    let key = KeyPair::generate_ed25519().unwrap();
    let user = auth
      .validate_public_key(
        "test",
        &key
          .clone_public_key()
          .expect("A public key should have been generated"),
      )
      .unwrap()
      .unwrap();
    assert_eq!(user, ());
  }
}
