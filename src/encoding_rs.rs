use encoding_rs::Encoding;
use encoding_rs_io::{DecodeReaderBytes, EncodeWriterBytes};
use crate::{Result, RwBuilder};

/// Type returned by the `encoding_rs` function on the `RwBuilderExt` trait.
/// It is itself an `RwBuilder` so can be chained further.
#[derive(Debug)]
#[must_use]
pub struct EncodingRsBuilder<B>
where
    B: RwBuilder,
{
    builder: B,
    encoding: &'static Encoding,
}

impl<B> EncodingRsBuilder<B>
where
    B: RwBuilder,
{
    /// Factory function to wrap an inner builder
    pub const fn new(builder: B, encoding: &'static Encoding) -> Self {
        Self { builder, encoding }
    }
}

impl<B> RwBuilder for EncodingRsBuilder<B>
where
    B: RwBuilder,
{
    type Reader = DecodeReaderBytes<B::Reader, Vec<u8>>;

    fn reader(&self) -> Result<Self::Reader> {
        let reader = self.builder.reader()?;
        Ok(encoding_rs_io::DecodeReaderBytesBuilder::new()
            .encoding(Some(self.encoding))
            .build(reader))
    }

    type Writer = EncodeWriterBytes<B::Writer>;

    fn writer(&self) -> Result<Self::Writer> {
        let writer = self.builder.writer()?;
        Ok(encoding_rs_io::EncodeWriterBytesBuilder::new()
            .encoding(self.encoding)
            .build(writer))
    }
}
