use crate::BoxFuture;
use futures_lite::Future;
use std::sync::Arc;

pub trait Runtime: Send + Sync {
    type Error;
    fn spawn<F: Future + 'static + Send>(&self, future: F)
    where
        F::Output: Send;

    fn unblock<R, F>(&self, ret: F) -> BoxFuture<'static, Result<R, Self::Error>>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static;

    fn block_on<F: Future>(&self, future: F) -> F::Output;
}

impl<T> Runtime for Arc<T>
where
    T: Runtime,
{
    type Error = T::Error;
    fn spawn<F: Future + 'static + Send>(&self, future: F)
    where
        F::Output: Send,
    {
        (&**self).spawn(future)
    }

    fn unblock<R, F>(&self, ret: F) -> BoxFuture<'static, Result<R, Self::Error>>
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
