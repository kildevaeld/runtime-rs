#[cfg(feature = "fs")]
use super::fs::TokioFS;
use super::runtime::TokioGlobalRuntime;
use crate::{Backend, Listener, Transport};

use async_compat::Compat;
use async_trait::async_trait;
use futures_lite::{AsyncRead, AsyncWrite};
use std::{net::SocketAddr, path::PathBuf};
use tokio_lib::net::{TcpListener, TcpStream};
#[cfg(unix)]
use tokio_lib::net::{UnixListener, UnixStream};

pub struct Tokio;

#[async_trait]
impl Listener for TcpListener {
    type Error = std::io::Error;
    type Transport = Compat<TcpStream>;
    type Addr = SocketAddr;

    async fn bind(addr: Self::Addr) -> Result<Self, Self::Error> {
        TcpListener::bind(addr).await
    }
    async fn accept(&self) -> Result<(Self::Transport, Option<Self::Addr>), Self::Error> {
        let (stream, addr) = self.accept().await?;
        Ok((Compat::new(stream), Some(addr)))
    }
}

#[cfg(unix)]
#[async_trait]
impl Listener for UnixListener {
    type Error = std::io::Error;
    type Transport = Compat<UnixStream>;
    type Addr = PathBuf;

    async fn bind(addr: Self::Addr) -> Result<Self, Self::Error> {
        UnixListener::bind(addr)
    }
    async fn accept(&self) -> Result<(Self::Transport, Option<Self::Addr>), Self::Error> {
        let (stream, addr) = self.accept().await?;
        Ok((
            Compat::new(stream),
            addr.as_pathname().map(|m| m.to_path_buf()),
        ))
    }
}

impl Backend for Tokio {
    type TcpListener = TcpListener;
    #[cfg(unix)]
    type UnixListener = UnixListener;
    type Runtime = TokioGlobalRuntime;

    #[cfg(feature = "fs")]
    type FS = TokioFS;

    fn runtime() -> Self::Runtime {
        TokioGlobalRuntime
    }
}

impl<S> Transport for Compat<S> where Self: AsyncRead + AsyncWrite {}
