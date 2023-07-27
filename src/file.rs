use crate::RwBuilder;
use anyhow::Result;
use std::{fs::OpenOptions, path::PathBuf};

/// Type for building readers and writers on top of a file handle.
/// It is itself an `RwBuilder`, but can't be created through one.
/// This is why we call it a source.
#[derive(Debug)]
pub struct Builder {
    /// The path of the file for which readers and writers can be created.
    path: PathBuf,
}

impl Builder {
    /// Factory function to create a builder holding on to a file path
    #[must_use]
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl RwBuilder for Builder {
    type Reader = std::fs::File;

    fn reader(&self) -> Result<Self::Reader> {
        let options = OpenOptions::new().read(true).open(&self.path)?;
        Ok(options)
    }

    type Writer = std::fs::File;

    fn writer(&self) -> Result<Self::Writer> {
        let options = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.path)?;
        Ok(options)
    }
}
