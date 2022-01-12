use crate::{BoxStream, Runtime};
use async_stream::stream;
use async_trait::async_trait;
use futures_lite::{AsyncRead, AsyncSeek, AsyncWrite, Stream};
use std::{
    fs::{Metadata, OpenOptions},
    io,
    path::{Path, PathBuf},
};

pub trait Transport: AsyncRead + AsyncWrite {
    // fn remote_addr(&self) -> Option<SocketAddr>;
}

pub trait File: AsyncRead + AsyncWrite + AsyncSeek {}

#[async_trait]
pub trait Listener: Sized {
    type Error;
    type Transport: Transport;
    type Addr;

    async fn bind(addr: Self::Addr) -> Result<Self, Self::Error>;
    async fn accept(&self) -> Result<(Self::Transport, Option<Self::Addr>), Self::Error>;

    fn incoming(self) -> BoxStream<'static, Result<Self::Transport, Self::Error>>
    where
        Self: Send + Sync + 'static,
        Self::Addr: Send,
        Self::Error: Send,
        Self::Transport: Send,
    {
        Box::pin(stream! {
            loop {
                match self.accept().await {
                    Ok(ret) => yield Ok(ret.0),
                    Err(err) => yield Err(err)
                }
            }
        })
    }
}

#[async_trait]
pub trait DirEntry: Send {
    fn path(&self) -> PathBuf;
    async fn metadata(&self) -> io::Result<Metadata>;
}

#[async_trait]
pub trait FS: Send + Sync {
    type ReadDir: Stream<Item = Result<Self::DirEntry, io::Error>> + Send;
    type DirEntry: DirEntry;
    type File: File;

    async fn open<P: AsRef<Path> + Send>(path: P, opts: OpenOptions) -> io::Result<Self::File>;

    async fn read_dir<P: AsRef<Path> + Send>(path: P) -> io::Result<Self::ReadDir>;
    async fn read<P: AsRef<Path> + Send>(path: P) -> io::Result<Vec<u8>>;
    async fn metadata<P: AsRef<Path> + Send>(path: P) -> io::Result<Metadata>;
}

pub trait Backend: Sized + Send + Sync {
    type TcpListener: Listener;
    #[cfg(unix)]
    type UnixListener: Listener;
    // type File: File;
    type Runtime: Runtime;

    type FS: FS;

    fn runtime() -> Self::Runtime;
}

#[async_trait]
pub trait BackendExt: Backend {
    async fn tcp_bind(
        addr: <Self::TcpListener as Listener>::Addr,
    ) -> Result<Self::TcpListener, <Self::TcpListener as Listener>::Error>
    where
        Self: Sized + Send,
        <Self::TcpListener as Listener>::Addr: Send,
    {
        Self::TcpListener::bind(addr).await
    }
    #[cfg(unix)]
    async fn unix_bind(
        addr: <Self::UnixListener as Listener>::Addr,
    ) -> Result<Self::UnixListener, <Self::UnixListener as Listener>::Error>
    where
        Self: Sized + Send,
        <Self::UnixListener as Listener>::Addr: Send,
    {
        Self::UnixListener::bind(addr).await
    }
}

#[async_trait]
impl<T> BackendExt for T where T: Backend {}
