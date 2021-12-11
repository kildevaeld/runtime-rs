// mod backend;

use std::{future::Future, pin::Pin, sync::Arc};

pub type BoxFuture<'a, O> = Pin<Box<dyn Future<Output = O> + Send + 'a>>;

pub trait Runtime: Send + Sync {
    fn spawn<F: Future + 'static + Send>(&self, future: F)
    where
        F::Output: Send;

    fn unblock<R, F>(&self, ret: F) -> BoxFuture<'static, R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static;

    fn block_on<F: Future>(&self, future: F) -> F::Output;
}

impl<T> Runtime for Arc<T>
where
    T: Runtime,
{
    fn spawn<F: Future + 'static + Send>(&self, future: F)
    where
        F::Output: Send,
    {
        (&**self).spawn(future)
    }

    fn unblock<R, F>(&self, ret: F) -> BoxFuture<'static, R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        (&**self).unblock(ret)
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        (&**self).block_on(future)
    }
}

#[cfg(feature = "smol")]
impl<'a> Runtime for smol_lib::Executor<'a> {
    fn spawn<F: Future + 'static + Send>(&self, future: F)
    where
        F::Output: Send,
    {
        self.spawn(future).detach()
    }

    fn unblock<R, F>(&self, ret: F) -> BoxFuture<'static, R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        Box::pin(blocking::unblock(ret))
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        async_io::block_on(future)
    }
}

#[cfg(feature = "smol")]
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct SmolGlobalRuntime;

#[cfg(feature = "smol")]
impl Runtime for SmolGlobalRuntime {
    fn spawn<F: Future + 'static + Send>(&self, future: F)
    where
        F::Output: Send,
    {
        smol_lib::spawn(future).detach()
    }

    fn unblock<R, F>(&self, ret: F) -> BoxFuture<'static, R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        Box::pin(blocking::unblock(ret))
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        smol_lib::block_on(future)
    }
}

#[cfg(feature = "tokio")]
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct TokioGlobal;

#[cfg(feature = "tokio")]
impl Runtime for TokioGlobal {
    fn spawn<F: Future + 'static + Send>(&self, future: F)
    where
        F::Output: Send,
    {
        tokio_lib::spawn(future);
    }

    fn unblock<R, F>(&self, ret: F) -> BoxFuture<'static, R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        Box::pin(async move {
            let ret = tokio_lib::task::spawn_blocking(ret).await;
            ret.unwrap()
        })
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        futures_lite::future::block_on(future)
    }
}

#[cfg(feature = "tokio")]
impl Runtime for tokio_lib::runtime::Runtime {
    fn spawn<F: Future + 'static + Send>(&self, future: F)
    where
        F::Output: Send,
    {
        self.spawn(future);
    }

    fn unblock<R, F>(&self, ret: F) -> BoxFuture<'static, R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let future = self.spawn_blocking(ret);
        Box::pin(async move {
            let ret = future.await;
            ret.unwrap()
        })
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.block_on(future)
    }
}
