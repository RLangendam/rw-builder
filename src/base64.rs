use crate::{Result, RwBuilder};

/// Builder for Base64 encoding/decoding
#[derive(Debug)]
pub struct Base64Builder<B> {
    /// Inner builder
    builder: B,
}

impl<B: RwBuilder> Base64Builder<B> {
    /// Creates a new base64 builder wrapper
    pub const fn new(builder: B) -> Self {
        Self { builder }
    }
}

impl<B: RwBuilder> RwBuilder for Base64Builder<B> {
    type Reader = base64::read::DecoderReader<'static, base64::engine::GeneralPurpose, B::Reader>;
    type Writer = base64::write::EncoderWriter<'static, base64::engine::GeneralPurpose, B::Writer>;

    fn reader(&self) -> Result<Self::Reader> {
        Ok(base64::read::DecoderReader::new(
            self.builder.reader()?,
            &base64::engine::general_purpose::STANDARD,
        ))
    }

    fn writer(&self) -> Result<Self::Writer> {
        Ok(base64::write::EncoderWriter::new(
            self.builder.writer()?,
            &base64::engine::general_purpose::STANDARD,
        ))
    }
}
