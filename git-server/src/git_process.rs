use log::{debug, error};
use russh::{server::Handle, ChannelId, CryptoVec};
use tokio::{io::AsyncReadExt, process::Child};

/// A struct representing a git process.
///
/// This struct is used to forward the output of the git process to the client.
pub(crate) struct GitProcess {
  process: Child,
  handle: Handle,
  channel_id: ChannelId,
}

impl GitProcess {
  /// Forwards the output of the git process to the client.
  pub(crate) fn forward_output(process: Child, handle: Handle, channel_id: ChannelId) {
    let git_process = GitProcess {
      process,
      handle,
      channel_id,
    };
    git_process.forward_output_inner();
  }

  /// Forwards the output of the git process to the client. The need for a struct is explained by the utilities functions.
  fn forward_output_inner(mut self) {
    let mut git_stdout = self.process.stdout.take().unwrap();

    tokio::spawn(async move {
      const BUF_SIZE: usize = 1024 * 32;
      let mut buf = [0u8; BUF_SIZE];
      loop {
        let read = git_stdout.read(&mut buf).await.map_err(|e| {
          error!("Error reading from git process: {}", e);
        })?;
        if read == 0 {
          break;
        }
        self.data(&buf[..read]).await?;
      }

      let status = self
        .process
        .wait()
        .await
        .map_err(|e| {
          error!("Error waiting for git process: {}", e);
        })?
        .code()
        .unwrap_or(128) as u32;
      self.exit_status(status).await?;

      self.eof().await?;
      self.close().await?;

      debug!("Git process forwarding finished");
      Ok::<(), ()>(())
    });
    debug!("Git process forwarding started");
  }

  /// Closes the channel.
  async fn close(&self) -> Result<(), ()> {
    self.handle.close(self.channel_id).await.map_err(|_| {
      error!("Failed to close handle");
    })?;
    Ok(())
  }
  /// Sends data to the client.
  async fn data(&self, data: &[u8]) -> Result<(), ()> {
    let buf: russh::CryptoVec = CryptoVec::from_slice(data);
    self.handle.data(self.channel_id, buf).await.map_err(|_| {
      error!("Failed to write data to channel");
    })?;
    Ok(())
  }
  /// Sets the exit status of the process.
  async fn exit_status(&self, status: u32) -> Result<(), ()> {
    self
      .handle
      .exit_status_request(self.channel_id, status)
      .await
      .map_err(|_| {
        error!("Failed to set exit status");
      })?;
    Ok(())
  }
  /// Sends an EOF to the client.
  async fn eof(&self) -> Result<(), ()> {
    self.handle.eof(self.channel_id).await.map_err(|_| {
      error!("Failed to send EOF");
    })?;
    Ok(())
  }
}
