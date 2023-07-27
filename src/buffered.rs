use std::io::{BufReader, BufWriter};

use anyhow::Result;

use crate::RwBuilder;

/// Type returned by the `buffered` function on the `RwBuilder` trait.
/// It is itself an `RwBuilder` so can be chained further.
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

impl<B> RwBuilder for Builder<B>
where
    B: RwBuilder,
    B::Reader: std::io::Read,
    B::Writer: std::io::Write,
{
    type Reader = BufReader<B::Reader>;
    type Writer = BufWriter<B::Writer>;

    fn reader(&self) -> Result<Self::Reader> {
        Ok(BufReader::new(self.builder.reader()?))
    }

    fn writer(&self) -> Result<Self::Writer> {
        Ok(BufWriter::new(self.builder.writer()?))
    }
}
