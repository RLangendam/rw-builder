use crate::RwBuilder;
use anyhow::Result;
use flate2::{Compression, CrcReader, CrcWriter};

/// Type returned by the `deflate`, `gz` and `zlib` functions on the `RwBuilder` trait.
/// It is itself an `RwBuilder` so can be chained further.
#[derive(Debug)]
pub struct CompressionBuilder<B, C>
where
    B: RwBuilder,
    C: CoderBuilder<B::Reader, B::Writer>,
{
    /// The inner builder it wraps
    builder: B,
    /// The compression used for the encoder
    compression: Compression,
    /// The builder for the encoder and decoder
    coder: C,
}

impl<B, C> RwBuilder for CompressionBuilder<B, C>
where
    B: RwBuilder,
    B::Reader: std::io::Read,
    B::Writer: std::io::Write,
    C: CoderBuilder<B::Reader, B::Writer>,
    C::Decoder: std::io::Read,
    C::Encoder: std::io::Write,
{
    type Reader = C::Decoder;

    fn reader(&self) -> Result<Self::Reader> {
        let reader = self.builder.reader()?;
        Ok(self.coder.decoder(reader))
    }

    type Writer = C::Encoder;

    fn writer(&self) -> Result<Self::Writer> {
        let writer = self.builder.writer()?;
        Ok(self.coder.encoder(writer, self.compression))
    }
}

/// Implementors like `Deflate`, `Gz` and `Zlib` create the associated encoders and decoders.
pub trait CoderBuilder<R, W> {
    /// The type of encoder created
    type Encoder;

    /// Create an encoder on top of a writer
    fn encoder(&self, writer: W, compression: Compression) -> Self::Encoder;

    /// The type of decoder created
    type Decoder;

    /// Create a decoder on top of a reader
    fn decoder(&self, reader: R) -> Self::Decoder;
}

/// The Zlib encoder and decoder builder
#[derive(Default, Debug, Copy, Clone)]
pub struct Zlib;

impl<R, W> CoderBuilder<R, W> for Zlib
where
    R: std::io::Read,
    W: std::io::Write,
{
    type Encoder = flate2::write::ZlibEncoder<W>;

    fn encoder(&self, writer: W, compression: Compression) -> Self::Encoder {
        flate2::write::ZlibEncoder::new(writer, compression)
    }

    type Decoder = flate2::read::ZlibDecoder<R>;

    fn decoder(&self, reader: R) -> Self::Decoder {
        flate2::read::ZlibDecoder::new(reader)
    }
}

/// Convenience trait for creating a new encoder/decoder builder
pub trait Constructor<B>
where
    Self: Sized + CoderBuilder<B::Reader, B::Writer> + Default,
    B: RwBuilder,
{
    /// Create the encoder/decoder builder
    fn new(builder: B, compression: Compression) -> CompressionBuilder<B, Self> {
        CompressionBuilder {
            builder,
            compression,
            coder: Self::default(),
        }
    }
}

impl<B> Constructor<B> for Zlib where B: RwBuilder {}

/// The Gz encoder and decoder builder
#[derive(Default, Debug, Copy, Clone)]
pub struct Gz;

impl<R, W> CoderBuilder<R, W> for Gz
where
    R: std::io::Read,
    W: std::io::Write,
{
    type Encoder = flate2::write::GzEncoder<W>;

    fn encoder(&self, writer: W, compression: Compression) -> Self::Encoder {
        flate2::write::GzEncoder::new(writer, compression)
    }

    type Decoder = flate2::read::GzDecoder<R>;

    fn decoder(&self, reader: R) -> Self::Decoder {
        flate2::read::GzDecoder::new(reader)
    }
}

impl<B> Constructor<B> for Gz where B: RwBuilder {}

/// The Deflate encoder and decoder builder
#[derive(Default, Debug, Copy, Clone)]
pub struct Deflate;

impl<R, W> CoderBuilder<R, W> for Deflate
where
    R: std::io::Read,
    W: std::io::Write,
{
    type Encoder = flate2::write::DeflateEncoder<W>;

    fn encoder(&self, writer: W, compression: Compression) -> Self::Encoder {
        flate2::write::DeflateEncoder::new(writer, compression)
    }

    type Decoder = flate2::read::DeflateDecoder<R>;

    fn decoder(&self, reader: R) -> Self::Decoder {
        flate2::read::DeflateDecoder::new(reader)
    }
}

impl<B> Constructor<B> for Deflate where B: RwBuilder {}

/// Type returned by the `crc` function on the `RwBuilder` trait.
/// It is itself an `RwBuilder` so can be chained further, although this is an uncommon scenario
#[derive(Debug)]
pub struct CrcBuilder<B>
where
    B: RwBuilder,
{
    /// The inner builder it wraps
    builder: B,
}

impl<B> CrcBuilder<B>
where
    B: RwBuilder,
{
    /// Factory function to wrap an inner builder
    #[must_use]
    pub const fn new(builder: B) -> Self {
        Self { builder }
    }
}

impl<B> RwBuilder for CrcBuilder<B>
where
    B: RwBuilder,
    B::Reader: std::io::Read,
    B::Writer: std::io::Write,
{
    type Reader = CrcReader<B::Reader>;

    fn reader(&self) -> Result<Self::Reader> {
        Ok(CrcReader::new(self.builder.reader()?))
    }

    type Writer = CrcWriter<B::Writer>;

    fn writer(&self) -> Result<Self::Writer> {
        Ok(CrcWriter::new(self.builder.writer()?))
    }
}
