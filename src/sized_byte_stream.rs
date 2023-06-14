use futures_util::Stream;

use crate::Error;

/// A wrapper around a downloaded object's byte stream that provides a useful `size_hint`.
pub struct SizedByteStream<S: Stream<Item = Result<bytes::Bytes, Error>> + Unpin> {
    size: Option<u64>,
    bytes: S,
}

impl<S: Stream<Item = Result<bytes::Bytes, Error>> + Unpin> SizedByteStream<S> {
    pub(crate) fn new(bytes: S, size: Option<u64>) -> Self {
        Self { size, bytes }
    }
}

impl<S: Stream<Item = Result<bytes::Bytes, Error>> + Unpin> Stream for SizedByteStream<S> {
    type Item = Result<bytes::Bytes, Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        futures_util::StreamExt::poll_next_unpin(&mut self.bytes, cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self
            .size
            .and_then(|s| std::convert::TryInto::try_into(s).ok());
        (size.unwrap_or(0), size)
    }
}
