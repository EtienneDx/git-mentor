use async_trait::async_trait;
use ssh_server::{user::User, wrapper::HandleWrapper};

use crate::repository::{Repository, RepositoryProvider};

pub struct SimpleRepositoryProvider {
  pub find_repository: bool,
  pub has_permission: bool,
}

impl RepositoryProvider for SimpleRepositoryProvider {
  type User = SimpleUser;

  type Repository = SimpleRepository;

  fn find_repository(&self, _user: &Self::User, path: &str) -> Option<Self::Repository> {
    if self.find_repository {
      Some(SimpleRepository(self.has_permission, path.to_string()))
    } else {
      None
    }
  }
}

pub struct SimpleRepository(pub bool, pub String);

impl Repository for SimpleRepository {
  type User = SimpleUser;

  fn has_permission(
    &self,
    _user: &Self::User,
    _permission: crate::repository::RepositoryPermission,
  ) -> bool {
    self.0
  }

  fn get_path(&self) -> &str {
    self.1.as_str()
  }
}

pub struct SimpleUser;
impl User for SimpleUser {}

#[derive(Clone)]
pub struct MockHandle;

#[async_trait]
impl HandleWrapper for MockHandle {
  type ChannelId = u32;

  async fn extended_data(
    &self,
    _id: Self::ChannelId,
    _ext: u32,
    _data: russh::CryptoVec,
  ) -> Result<(), russh::CryptoVec> {
    Ok(())
  }
  async fn close(&self, _id: Self::ChannelId) -> Result<(), ()> {
    Ok(())
  }

  async fn data(
    &self,
    _id: Self::ChannelId,
    _data: russh::CryptoVec,
  ) -> Result<(), russh::CryptoVec> {
    Ok(())
  }

  async fn eof(&self, _id: Self::ChannelId) -> Result<(), ()> {
    Ok(())
  }

  async fn exit_status_request(&self, _id: Self::ChannelId, _exit_status: u32) -> Result<(), ()> {
    Ok(())
  }
}
