use futures_core::ready;
use futures_io::{AsyncRead, AsyncSeek, AsyncWrite};
use pin_project_lite::pin_project;
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};
use tokiolib::io::ReadBuf;

pin_project! {
    pub struct TokioIO<T> {
        #[pin]
        inner:T
    }

}

impl<T> AsyncRead for TokioIO<T>
where
    T: tokiolib::io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.project();
        let mut buf = ReadBuf::new(buf);
        match ready!(this.inner.poll_read(cx, &mut buf)) {
            Ok(_) => Poll::Ready(Ok(buf.filled().len())),
            Err(err) => Poll::Ready(Err(err)),
        }
    }
}

impl<T> AsyncWrite for TokioIO<T>
where
    T: tokiolib::io::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.project().inner.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_shutdown(cx)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        self.project().inner.poll_write_vectored(cx, bufs)
    }
}

impl<T> AsyncSeek for TokioIO<T>
where
    T: tokiolib::io::AsyncSeek,
{
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: io::SeekFrom,
    ) -> Poll<io::Result<u64>> {
        // let mut this = self.project();
        match (unsafe { Pin::new_unchecked(&mut self.inner) }).start_seek(pos) {
            Ok(_) => {}
            Err(err) => return Poll::Ready(Err(err)),
        };

        self.project().inner.poll_complete(cx)
    }
}
