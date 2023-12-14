use ssh_server::{authenticator::Authenticator, error::SshError};

use crate::ssh_user::SshUser;

const GIT_USER: &str = "git";

/// The authenticator object, allowing to connect users based on their public keys
pub struct DbAuthenticator {
  // db_handle: DatabaseConnection <- TODO: add interface to make the whole stuff easily mockable or replaceable in future iterations
}

impl DbAuthenticator {
  pub fn new() -> Self {
    DbAuthenticator {}
  }
}

impl Authenticator for DbAuthenticator {
  type User = SshUser;

  fn validate_public_key(
    &self,
    user: &str,
    _key: &russh_keys::key::PublicKey,
  ) -> Result<Option<Self::User>, SshError> {
    if user != GIT_USER {
      return Ok(None);
    }

    // TODO: Add Connected user check

    Ok(Some(SshUser::Public))
  }
}

#[cfg(test)]
mod test {
  use russh_keys::key::{KeyPair, PublicKey};

  use super::*;

  fn unknown_key() -> PublicKey {
    let key = KeyPair::generate_ed25519().unwrap();
    key.clone_public_key().unwrap()
  }

  #[test]
  fn given_non_git_user_and_unknown_key_then_user_is_none() {
    let auth = DbAuthenticator::new();

    let user = auth.validate_public_key("unknown", &unknown_key());
    let user = user.expect("No error should be returned");
    assert_eq!(user, None);
  }

  #[test]
  fn given_git_user_and_unknown_key_then_user_is_none() {
    let auth = DbAuthenticator::new();

    let user = auth.validate_public_key("git", &unknown_key());
    let user = user.expect("No error should be returned");
    assert_eq!(user, Some(SshUser::Public));
  }
}
