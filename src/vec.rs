use std::{cell::RefCell, cmp::min, rc::Rc};

use anyhow::Result;

use crate::RwBuilder;

/// Type for building readers and writers on top of a `Vec` in memory.
/// It is itself an `RwBuilder`, but can't be created through one.
/// This is why we call it a source.
#[derive(Debug, Default)]
pub struct Builder {
    /// The inner `Vec` which is used to write data to and read data from
    /// It is shared between the reader and the writer
    buffer: Rc<RefCell<Vec<u8>>>,
}

impl RwBuilder for Builder {
    type Reader = Reader;
    type Writer = Writer;

    fn reader(&self) -> Result<Self::Reader> {
        Ok(Reader { buffer: Rc::clone(&self.buffer), bytes_read: 0 })
    }

    fn writer(&self) -> Result<Self::Writer> {
        Ok(Writer { buffer: Rc::clone(&self.buffer) })
    }
}

/// Reading from a `Vec`
#[derive(Debug)]
pub struct Reader {
    /// The buffer shared between the builder, reader and writer
    buffer: Rc<RefCell<Vec<u8>>>,
    /// The read position in the buffer
    bytes_read: usize,
}

impl std::io::Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let borrowed = self.buffer.borrow();
        let byte_count = min(borrowed.len() - self.bytes_read, buf.len());
        buf[..byte_count].copy_from_slice(&borrowed[self.bytes_read..self.bytes_read + byte_count]);
        self.bytes_read += byte_count;
        Ok(byte_count)
    }
}

impl std::io::Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut borrowed = self.buffer.borrow_mut();
        let byte_count = buf.len();
        borrowed.extend_from_slice(buf);
        Ok(byte_count)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Writing to a `Vec`
#[derive(Debug)]
pub struct Writer {
    /// The buffer shared between the builder, reader and writer
    buffer: Rc<RefCell<Vec<u8>>>,
}
