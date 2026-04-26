pub use bzip2::Compression;
use crate::{Result, RwBuilder};

/// Type returned by the `bzip2` function on the `RwBuilderExt` trait.
/// It is itself an `RwBuilder` so can be chained further.
#[derive(Debug)]
#[must_use]
pub struct BzBuilder<B>
where
    B: RwBuilder,
{
    /// Inner builder
    builder: B,
    /// Compression level for bzip2
    compression: Compression,
}

impl<B> BzBuilder<B>
where
    B: RwBuilder,
{
    /// Factory function to wrap an inner builder
    pub const fn new(builder: B, compression: Compression) -> Self {
        Self { builder, compression }
    }
}

impl<B> RwBuilder for BzBuilder<B>
where
    B: RwBuilder,
{
    type Reader = bzip2::read::BzDecoder<B::Reader>;

    fn reader(&self) -> Result<Self::Reader> {
        let reader = self.builder.reader()?;
        Ok(bzip2::read::BzDecoder::new(reader))
    }

    type Writer = bzip2::write::BzEncoder<B::Writer>;

    fn writer(&self) -> Result<Self::Writer> {
        let writer = self.builder.writer()?;
        Ok(bzip2::write::BzEncoder::new(writer, self.compression))
    }
}
