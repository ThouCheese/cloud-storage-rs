use futures_util::Stream;
use std::{
    io::{BufReader, Read},
    pin::Pin,
    task::{Context, Poll},
};

const BUF_CAP: usize = 8 * 1024;

pub struct ReaderStream<R>(BufReader<R>);

impl<R: std::io::Read> ReaderStream<R> {
    pub fn new(r: R) -> Self {
        Self(BufReader::with_capacity(BUF_CAP, r))
    }
}

impl<R: std::io::Read + Send + Sync + Unpin + 'static> Stream for ReaderStream<R> {
    type Item = Result<Vec<u8>, crate::Error>;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = vec![0; BUF_CAP];
        let res = Pin::into_inner(self).0.read(&mut buf);
        match res {
            Ok(0) => Poll::Ready(None),
            Ok(n) => {
                buf.truncate(n);
                Poll::Ready(Some(Ok(buf)))
            }
            Err(e) => Poll::Ready(Some(Err(e.into()))),
        }
    }
}
