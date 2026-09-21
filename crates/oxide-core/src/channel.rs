use bytes::Bytes;
use futures_util::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

/// Default ring buffer capacity for token streaming.
pub const DEFAULT_TOKEN_CHANNEL_CAPACITY: usize = 512;

/// Zero-copy token chunk transmitter for the inference pipeline.
#[derive(Clone, Debug)]
pub struct TokenSender {
    inner: mpsc::Sender<Bytes>,
}

impl TokenSender {
    pub fn new(inner: mpsc::Sender<Bytes>) -> Self {
        Self { inner }
    }

    /// Send a token chunk as `Bytes` without copying.
    pub async fn send_chunk(&self, chunk: Bytes) -> Result<(), mpsc::error::SendError<Bytes>> {
        self.inner.send(chunk).await
    }

    /// Send a UTF-8 token string converted directly to static/borrowed bytes.
    pub async fn send_str(
        &self,
        text: impl Into<String>,
    ) -> Result<(), mpsc::error::SendError<Bytes>> {
        let b = Bytes::from(text.into());
        self.inner.send(b).await
    }
}

/// Stream receiver wrapping tokio mpsc for SSE and HTTP response pipelines.
pub struct TokenReceiver {
    inner: mpsc::Receiver<Bytes>,
}

impl TokenReceiver {
    pub fn new(inner: mpsc::Receiver<Bytes>) -> Self {
        Self { inner }
    }

    /// Asynchronously receive the next token chunk.
    pub async fn recv(&mut self) -> Option<Bytes> {
        self.inner.recv().await
    }
}

impl Stream for TokenReceiver {
    type Item = Bytes;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.poll_recv(cx)
    }
}

/// Create a bounded token stream channel with pre-allocated buffer slots.
pub fn create_token_channel(capacity: usize) -> (TokenSender, TokenReceiver) {
    let (tx, rx) = mpsc::channel(capacity);
    (TokenSender::new(tx), TokenReceiver::new(rx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_token_channel_streaming() {
        let (tx, mut rx) = create_token_channel(64);

        tokio::spawn(async move {
            tx.send_str("Hello").await.unwrap();
            tx.send_str(" ").await.unwrap();
            tx.send_str("World").await.unwrap();
        });

        let mut collected = Vec::new();
        while let Some(chunk) = rx.next().await {
            collected.push(String::from_utf8_lossy(&chunk).to_string());
        }

        assert_eq!(collected, vec!["Hello", " ", "World"]);
    }
}
