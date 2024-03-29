pub trait PasswordAuth {
  fn generate_hash(password: impl AsRef<[u8]>) -> String;
  fn verify_password(password: impl AsRef<[u8]>, hash: &str) -> bool;
}

pub struct PasswordAuthImpl;

impl PasswordAuth for PasswordAuthImpl {
  fn generate_hash(password: impl AsRef<[u8]>) -> String {
    password_auth::generate_hash(password)
  }

  fn verify_password(password: impl AsRef<[u8]>, hash: &str) -> bool {
    let res = password_auth::verify_password(password, hash);
    if let Err(e) = res {
      log::warn!("Error verifying password: {}", e);
      return false;
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate_hash() {
    let password = "password";
    let hash = PasswordAuthImpl::generate_hash(password);
    assert_eq!(hash.len(), "$argon2id$v=19$m=19456,t=2,p=1$KmhCTks8Qx0nfomuy+AHRw$IlnT6WL3hBzR2gHsWeE7vAr9ccprFP5a43GFi1WJEcM".len());
  }

  #[test]
  fn test_verify_password() {
    let password = "password";
    let hash = "$argon2id$v=19$m=19456,t=2,p=1$KmhCTks8Qx0nfomuy+AHRw$IlnT6WL3hBzR2gHsWeE7vAr9ccprFP5a43GFi1WJEcM";
    let result = PasswordAuthImpl::verify_password(password, hash);
    assert!(result, "Password should be verified");
  }

  #[test]
  fn test_verify_password_fail() {
    let password = "not password";
    let hash = "$argon2id$v=19$m=19456,t=2,p=1$KmhCTks8Qx0nfomuy+AHRw$IlnT6WL3hBzR2gHsWeE7vAr9ccprFP5a43GFi1WJEcM";
    let result = PasswordAuthImpl::verify_password(password, hash);
    assert!(!result, "Password should not be verified");
  }
}
