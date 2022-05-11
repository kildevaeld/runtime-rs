use std::{
    fs::{Metadata, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use async_compat::Compat;
use async_trait::async_trait;
use tokio_stream::wrappers::ReadDirStream;

use crate::{DirEntry, FS, File};

#[async_trait]
impl File for Compat<tokio_lib::fs::File>
where
    Self: AsyncRead + AsyncWrite + AsyncSeek,
{
    async fn metadata(&self) -> Result<Metadata, io::Error> {
        self.metadata().await
    }
}


pub struct TokioFS;

#[async_trait]
impl FS for TokioFS {
    type DirEntry = tokio_lib::fs::DirEntry;
    type ReadDir = ReadDirStream;

    type File = Compat<tokio_lib::fs::File>;

    async fn open<P: AsRef<Path> + Send>(path: P, opts: OpenOptions) -> io::Result<Self::File> {
        Ok(Compat::new(
            tokio_lib::fs::OpenOptions::from(opts).open(path).await?,
        ))
    }

    async fn read_dir<P: AsRef<Path> + Send>(path: P) -> io::Result<Self::ReadDir> {
        Ok(ReadDirStream::new(tokio_lib::fs::read_dir(path).await?))
    }
    async fn read<P: AsRef<Path> + Send>(path: P) -> io::Result<Vec<u8>> {
        tokio_lib::fs::read(path).await
    }

    async fn metadata<P: AsRef<Path> + Send>(path: P) -> io::Result<Metadata> {
        tokio_lib::fs::metadata(path).await
    }
}

#[async_trait]
impl DirEntry for tokio_lib::fs::DirEntry {
    fn path(&self) -> PathBuf {
        self.path()
    }
    async fn metadata(&self) -> io::Result<Metadata> {
        self.metadata().await
    }
}
