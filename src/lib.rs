use std::{future::Future, pin::Pin};

pub type BoxFuture<'a, O> = Pin<Box<dyn Future<Output = O> + 'a>>;

pub trait Runtime {
    fn spawn<F: Future + 'static + Send>(&self, future: F)
    where
        F::Output: Send;

    fn unblock<R, F>(&self, ret: F) -> BoxFuture<'static, R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static;
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
}

#[cfg(feature = "smol")]
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
}

#[cfg(feature = "tokio")]
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
}
