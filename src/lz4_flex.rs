use crate::{Result, RwBuilder};

/// Type returned by the `lz4_flex` function on the `RwBuilderExt` trait.
/// It is itself an `RwBuilder` so can be chained further.
#[derive(Debug)]
#[must_use]
pub struct Lz4Builder<B>
where
    B: RwBuilder,
{
    /// Inner builder
    builder: B,
}

impl<B> Lz4Builder<B>
where
    B: RwBuilder,
{
    /// Factory function to wrap an inner builder
    pub const fn new(builder: B) -> Self {
        Self { builder }
    }
}

impl<B> RwBuilder for Lz4Builder<B>
where
    B: RwBuilder,
{
    type Reader = lz4_flex::frame::FrameDecoder<B::Reader>;

    fn reader(&self) -> Result<Self::Reader> {
        let reader = self.builder.reader()?;
        Ok(lz4_flex::frame::FrameDecoder::new(reader))
    }

    type Writer = lz4_flex::frame::FrameEncoder<B::Writer>;

    fn writer(&self) -> Result<Self::Writer> {
        let writer = self.builder.writer()?;
        Ok(lz4_flex::frame::FrameEncoder::new(writer))
    }
}
