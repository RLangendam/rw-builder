use anyhow::Result;

use crate::{RwBuilder, SerDe};

/// Type returned by the `bincode` function on the `RwBuilder` trait.
/// It is itself not an `RwBuilder` so can't be chained further.
/// This is why we call it a sink.
#[derive(Debug)]
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
    #[must_use]
    pub const fn new(builder: B) -> Self {
        Self { builder }
    }
}

impl<B> SerDe for Builder<B>
where
    B: RwBuilder,
    B::Reader: std::io::Read,
    B::Writer: std::io::Write,
{
    fn load<T>(&self) -> Result<T>
    where
        T: for<'de> serde::de::Deserialize<'de>,
    {
        let result = bincode::deserialize_from(self.builder.reader()?)?;
        Ok(result)
    }

    fn save<T>(&self, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        bincode::serialize_into(self.builder.writer()?, value)?;
        Ok(())
    }
}
