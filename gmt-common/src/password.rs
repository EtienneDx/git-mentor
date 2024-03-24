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
