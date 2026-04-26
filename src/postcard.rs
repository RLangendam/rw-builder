use crate::{Result, RwBuilder};
use serde::{de::DeserializeOwned, Serialize};

/// Type returned by the `postcard` function on the `RwBuilderExt` trait.
/// It acts as a sink/source for serde operations using postcard binary format.
#[derive(Debug)]
#[must_use]
pub struct Builder<B>
where
    B: RwBuilder,
{
    /// Inner builder
    builder: B,
}

impl<B> Builder<B>
where
    B: RwBuilder,
{
    /// Factory function to wrap an inner builder
    pub const fn new(builder: B) -> Self {
        Self { builder }
    }
}

impl<B> Builder<B>
where
    B: RwBuilder,
    B::Reader: std::io::Read,
    B::Writer: std::io::Write,
{
    /// Load an item by executing the configured reader chain and decoding via Postcard
    /// # Errors
    /// Returns an error if the underlying reader fails or if the deserialization fails.
    pub fn load<T: DeserializeOwned>(&self) -> Result<T> {
        let reader = self.builder.reader()?;
        let mut buffer = vec![0u8; 8192];
        let (val, _) = postcard::from_io((reader, buffer.as_mut_slice()))
            .map_err(|e| crate::error::Error::Other(e.to_string()))?;
        Ok(val)
    }

    /// Save an item by executing the configured writer chain and encoding via Postcard
    /// # Errors
    /// Returns an error if the underlying writer fails or if the serialization fails.
    pub fn save<T: Serialize>(&self, item: &T) -> Result<()> {
        let writer = self.builder.writer()?;
        postcard::to_io(item, writer)
            .map(|_| ())
            .map_err(|e| crate::error::Error::Other(e.to_string()))
    }
}
