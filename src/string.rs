use std::{
    fmt::{Display, Error},
    io::{Read, Write},
};

use anyhow::Result;

use crate::RwBuilder;

/// Type returned by the `string` function on the `RwBuilder` trait.
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

impl<B> Display for Builder<B>
where
    B: RwBuilder,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /// The size with which the buffer is extended until the reader is empty
        const BLOCK_SIZE: usize = 1024;
        let mut reader = self.builder.reader().or(Err(Error))?;
        let mut buffer = vec![0u8; BLOCK_SIZE];
        let mut position: usize = 0;
        while let Ok(bytes) = reader.read(&mut buffer[position..]) {
            if bytes == BLOCK_SIZE {
                position += BLOCK_SIZE;
                buffer.extend_from_slice(&[0u8; BLOCK_SIZE]);
            } else {
                buffer.truncate(position + bytes);
                break;
            }
        }
        write!(f, "{}", String::from_utf8(buffer).or(Err(Error))?)
    }
}

/// Creates a writer and writes the string to it
pub trait AdhocWriter {
    /// Write a string to a built writer
    /// # Errors
    /// If either the writer creation or the write operation fails the error is
    /// propagated.
    fn write_string(&self, text: &str) -> Result<()>;
}

impl<B> AdhocWriter for Builder<B>
where
    B: RwBuilder,
{
    fn write_string(&self, text: &str) -> Result<()> {
        let mut writer = self.builder.writer()?;
        let byte_count = writer.write(text.as_bytes())?;
        assert_eq!(byte_count, text.len());
        Ok(writer.flush()?)
    }
}
