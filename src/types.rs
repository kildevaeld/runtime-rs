use std::{future::Future, pin::Pin};

use futures_lite::Stream;

pub type BoxFuture<'a, O> = Pin<Box<dyn Future<Output = O> + Send + 'a>>;

pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + Send + 'a>>;
