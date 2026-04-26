use crate::{Result, RwBuilder};
use serde::{de::DeserializeOwned, Serialize};

/// Type returned by the `serde_json` function on the `RwBuilderExt` trait.
/// It acts as a sink/source for serde operations using JSON format.
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
    /// Load an item by executing the configured reader chain and decoding via JSON
    /// # Errors
    /// Returns an error if the underlying reader fails or if the deserialization fails.
    pub fn load<T: DeserializeOwned>(&self) -> Result<T> {
        let reader = self.builder.reader()?;
        serde_json::from_reader(reader).map_err(|e| crate::error::Error::Other(e.to_string()))
    }

    /// Save an item by executing the configured writer chain and encoding via JSON
    /// # Errors
    /// Returns an error if the underlying writer fails or if the serialization fails.
    pub fn save<T: Serialize>(&self, item: &T) -> Result<()> {
        let writer = self.builder.writer()?;
        serde_json::to_writer(writer, item)
            .map_err(|e| crate::error::Error::Other(e.to_string()))
    }
}
