use crate::{BoxFuture, Runtime};
use std::future::Future;
use tokio_lib::task::JoinError;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct TokioGlobalRuntime;

#[cfg(feature = "tokio")]
impl Runtime for TokioGlobalRuntime {
    type Error = JoinError;
    fn spawn<F: Future + 'static + Send>(&self, future: F)
    where
        F::Output: Send,
    {
        tokio_lib::spawn(future);
    }

    fn unblock<R, F>(&self, ret: F) -> BoxFuture<'static, Result<R, JoinError>>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        Box::pin(async move {
            let ret = tokio_lib::task::spawn_blocking(ret).await;
            ret
        })
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        futures_lite::future::block_on(future)
    }
}

impl Runtime for tokio_lib::runtime::Runtime {
    type Error = JoinError;
    fn spawn<F: Future + 'static + Send>(&self, future: F)
    where
        F::Output: Send,
    {
        self.spawn(future);
    }

    fn unblock<R, F>(&self, ret: F) -> BoxFuture<'static, Result<R, JoinError>>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let future = self.spawn_blocking(ret);
        Box::pin(async move {
            let ret = future.await;
            ret
        })
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.block_on(future)
    }
}
