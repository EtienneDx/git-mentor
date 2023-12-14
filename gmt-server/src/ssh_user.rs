use ssh_server::user::User;

/// Different types of possible connections
#[derive(Debug, PartialEq)]
pub enum SshUser {
  /// A user which does not exist in the database
  Public,
  /// A connected user
  /// TODO: Add DB user data here
  #[allow(dead_code)]
  Connected(),
  /// An admin user, with full access to the system.
  /// This field might be used in the long run to provide access to a bash accesspoint or other ssh functions.
  #[allow(dead_code)]
  Admin,
}

impl User for SshUser {}
