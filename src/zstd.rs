use crate::{Result, RwBuilder};

/// Type returned by the `zstd` function on the `RwBuilderExt` trait.
/// It is itself an `RwBuilder` so can be chained further.
#[derive(Debug)]
#[must_use]
pub struct ZstdBuilder<B>
where
    B: RwBuilder,
{
    /// Inner builder
    builder: B,
    /// Compression level for zstd
    level: i32,
}

impl<B> ZstdBuilder<B>
where
    B: RwBuilder,
{
    /// Factory function to wrap an inner builder
    pub const fn new(builder: B, level: i32) -> Self {
        Self { builder, level }
    }
}

impl<B> RwBuilder for ZstdBuilder<B>
where
    B: RwBuilder,
{
    type Reader = zstd::stream::read::Decoder<'static, std::io::BufReader<B::Reader>>;

    fn reader(&self) -> Result<Self::Reader> {
        let reader = self.builder.reader()?;
        zstd::stream::read::Decoder::new(reader).map_err(Into::into)
    }

    type Writer = zstd::stream::write::Encoder<'static, B::Writer>;

    fn writer(&self) -> Result<Self::Writer> {
        let writer = self.builder.writer()?;
        zstd::stream::write::Encoder::new(writer, self.level).map_err(Into::into)
    }
}
