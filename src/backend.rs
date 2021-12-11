use futures_lite::{AsyncRead, AsyncSeek, AsyncWrite};

use crate::{Runtime, SmolGlobalRuntime};

pub trait File: AsyncSeek + AsyncRead + AsyncWrite {}

pub trait Child {}

pub trait Backend {
    type Runtime: Runtime;
    type File: File;
}

#[cfg(feature = "smol")]
struct SmolBackend;

#[cfg(feature = "smol")]
impl File for smol_lib::fs::File {}

#[cfg(feature = "smol")]
impl Backend for SmolBackend {
    type Runtime = SmolGlobalRuntime;
    type File = smol_lib::fs::File;
}
