use async_trait::async_trait;
use russh::{ChannelId, CryptoVec};

#[async_trait]
pub trait HandleWrapper: Clone {
  type ChannelId;

  async fn extended_data(
    &self,
    id: Self::ChannelId,
    ext: u32,
    data: CryptoVec,
  ) -> Result<(), CryptoVec>;

  async fn close(&self, id: ChannelId) -> Result<(), ()>;

  async fn data(&self, id: ChannelId, data: CryptoVec) -> Result<(), CryptoVec>;

  async fn eof(&self, id: ChannelId) -> Result<(), ()>;

  async fn exit_status_request(&self, id: ChannelId, exit_status: u32) -> Result<(), ()>;
}

#[derive(Clone)]
pub struct WrappedHandle(pub russh::server::Handle);

#[async_trait]
impl HandleWrapper for WrappedHandle {
  type ChannelId = ChannelId;

  async fn extended_data(
    &self,
    id: Self::ChannelId,
    ext: u32,
    data: CryptoVec,
  ) -> Result<(), CryptoVec> {
    self.0.extended_data(id, ext, data).await
  }

  async fn close(&self, id: ChannelId) -> Result<(), ()> {
    self.0.close(id).await
  }

  async fn data(&self, id: ChannelId, data: CryptoVec) -> Result<(), CryptoVec> {
    self.0.data(id, data).await
  }

  async fn eof(&self, id: ChannelId) -> Result<(), ()> {
    self.0.eof(id).await
  }

  async fn exit_status_request(&self, id: ChannelId, exit_status: u32) -> Result<(), ()> {
    self.0.exit_status_request(id, exit_status).await
  }
}
