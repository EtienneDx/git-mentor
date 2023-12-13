/// A simple user implementation that allows all users to login.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User;

impl ssh_server::user::User for User {}
