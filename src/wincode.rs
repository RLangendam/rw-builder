use crate::Result;

use crate::RwBuilder;
use wincode::io::{std_read::ReadAdapter, std_write::WriteAdapter, Writer};

/// Type returned by the `wincode` function on the `RwBuilder` trait.
/// It is itself not an `RwBuilder` so can't be chained further.
/// This is why we call it a sink.
///
/// Note: This builder provides `load` and `save` methods for serialization.
/// Due to wincode's schema trait requirements, types must implement both
/// `serde` traits and `wincode`'s `SchemaRead`/`SchemaWrite` traits.
#[derive(Debug)]
#[must_use]
pub struct Builder<B>
where
    B: RwBuilder,
{
    /// The inner builder it wraps
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

    /// Load a value from the builder
    ///
    /// # Errors
    /// Returns an error if deserialization or reading fails
    pub fn load<T>(&self) -> Result<T>
    where
        T: for<'de> serde::de::Deserialize<'de>
            + for<'de> wincode::SchemaRead<'de, wincode::config::DefaultConfig, Dst = T>,
    {
        let reader = self.builder.reader()?;
        let adapter = ReadAdapter::new(reader);
        let result = wincode::deserialize_from(adapter)?;
        Ok(result)
    }

    /// Save a value to the builder
    ///
    /// # Errors
    /// Returns an error if serialization or writing fails
    pub fn save<T>(&self, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize
            + wincode::SchemaWrite<wincode::config::DefaultConfig, Src = T>
            + ?Sized,
    {
        let writer = self.builder.writer()?;
        let mut adapter = WriteAdapter::new(writer);
        wincode::serialize_into(&mut adapter, value)?;
        adapter.finish()?;
        Ok(())
    }
}
