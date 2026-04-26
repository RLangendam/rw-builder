use crate::{Result, RwBuilder};
use digest::Digest;
use std::io::{Read, Write};

/// A builder that computes a cryptographic hash or checksum on the stream.
#[derive(Debug)]
#[must_use]
pub struct DigestBuilder<B, D>
where
    B: RwBuilder,
    D: Digest,
{
    /// Inner builder
    builder: B,
    /// Phantom data
    _marker: std::marker::PhantomData<D>,
}

impl<B, D> DigestBuilder<B, D>
where
    B: RwBuilder,
    D: Digest,
{
    /// Factory function to wrap an inner builder for digesting
    pub const fn new(builder: B) -> Self {
        Self { builder, _marker: std::marker::PhantomData }
    }
}

/// The type returned by the `sha256` function in the `RwBuilder` trait
#[cfg(feature = "sha2")]
pub type Sha256Builder<B> = DigestBuilder<B, ::sha2::Sha256>;
/// The type returned by the `sha512` function in the `RwBuilder` trait
#[cfg(feature = "sha2")]
pub type Sha512Builder<B> = DigestBuilder<B, ::sha2::Sha512>;

/// The type returned by the `sha3_256` function in the `RwBuilder` trait
#[cfg(feature = "sha3")]
pub type Sha3_256Builder<B> = DigestBuilder<B, ::sha3::Sha3_256>;
/// The type returned by the `sha3_512` function in the `RwBuilder` trait
#[cfg(feature = "sha3")]
pub type Sha3_512Builder<B> = DigestBuilder<B, ::sha3::Sha3_512>;

impl<B, D> RwBuilder for DigestBuilder<B, D>
where
    B: RwBuilder,
    B::Reader: Read,
    B::Writer: Write,
    D: Digest,
{
    type Reader = DigestReader<B::Reader, D>;
    type Writer = DigestWriter<B::Writer, D>;

    fn reader(&self) -> Result<Self::Reader> {
        Ok(DigestReader::new(self.builder.reader()?))
    }

    fn writer(&self) -> Result<Self::Writer> {
        Ok(DigestWriter::new(self.builder.writer()?))
    }
}

/// A reader wrapper that computes a digest while reading.
#[derive(Debug)]
pub struct DigestReader<R, D>
where
    R: Read,
    D: Digest,
{
    /// Inner reader
    inner: R,
    /// Hasher instance
    hasher: D,
}

impl<R, D> DigestReader<R, D>
where
    R: Read,
    D: Digest,
{
    /// Creates a new `DigestReader`
    pub fn new(inner: R) -> Self {
        Self { inner, hasher: D::new() }
    }

    /// Consumes the reader and returns the computed hash.
    pub fn finalize(self) -> digest::Output<D> {
        self.hasher.finalize()
    }
    
    /// Unwraps the inner reader.
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R, D> Read for DigestReader<R, D>
where
    R: Read,
    D: Digest,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }
}

/// A writer wrapper that computes a digest while writing.
#[derive(Debug)]
pub struct DigestWriter<W, D>
where
    W: Write,
    D: Digest,
{
    /// Inner writer
    inner: W,
    /// Hasher instance
    hasher: D,
}

impl<W, D> DigestWriter<W, D>
where
    W: Write,
    D: Digest,
{
    /// Creates a new `DigestWriter`
    pub fn new(inner: W) -> Self {
        Self { inner, hasher: D::new() }
    }

    /// Consumes the writer and returns the computed hash.
    pub fn finalize(self) -> digest::Output<D> {
        self.hasher.finalize()
    }
    
    /// Unwraps the inner writer.
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W, D> Write for DigestWriter<W, D>
where
    W: Write,
    D: Digest,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
