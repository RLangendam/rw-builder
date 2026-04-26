use crate::{Result, RwBuilder};
use std::io::{Read, Write};

/// A builder that computes a CRC32 checksum using `crc32fast`
#[derive(Debug)]
#[must_use]
pub struct Crc32FastBuilder<B>
where
    B: RwBuilder,
{
    /// Inner builder
    builder: B,
}

impl<B> Crc32FastBuilder<B>
where
    B: RwBuilder,
{
    /// Factory function to wrap an inner builder
    pub const fn new(builder: B) -> Self {
        Self { builder }
    }
}

impl<B> RwBuilder for Crc32FastBuilder<B>
where
    B: RwBuilder,
    B::Reader: Read,
    B::Writer: Write,
{
    type Reader = Crc32FastReader<B::Reader>;
    type Writer = Crc32FastWriter<B::Writer>;

    fn reader(&self) -> Result<Self::Reader> {
        Ok(Crc32FastReader::new(self.builder.reader()?))
    }

    fn writer(&self) -> Result<Self::Writer> {
        Ok(Crc32FastWriter::new(self.builder.writer()?))
    }
}

/// A reader wrapper that computes a CRC32 checksum
#[derive(Debug)]
pub struct Crc32FastReader<R: Read> {
    /// Inner reader
    inner: R,
    /// Hasher instance
    hasher: crc32fast::Hasher,
}

impl<R: Read> Crc32FastReader<R> {
    /// Creates a new `Crc32FastReader`
    pub fn new(inner: R) -> Self {
        Self { inner, hasher: crc32fast::Hasher::new() }
    }

    /// Consumes the reader and returns the computed CRC32 checksum.
    pub fn finalize(self) -> u32 {
        self.hasher.finalize()
    }
}

impl<R: Read> Read for Crc32FastReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }
}

/// A writer wrapper that computes a CRC32 checksum
#[derive(Debug)]
pub struct Crc32FastWriter<W: Write> {
    /// Inner writer
    inner: W,
    /// Hasher instance
    hasher: crc32fast::Hasher,
}

impl<W: Write> Crc32FastWriter<W> {
    /// Creates a new `Crc32FastWriter`
    pub fn new(inner: W) -> Self {
        Self { inner, hasher: crc32fast::Hasher::new() }
    }

    /// Consumes the writer and returns the computed CRC32 checksum.
    pub fn finalize(self) -> u32 {
        self.hasher.finalize()
    }
}

impl<W: Write> Write for Crc32FastWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
