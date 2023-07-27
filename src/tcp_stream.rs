use std::net::{TcpStream, ToSocketAddrs};

use anyhow::Result;

use crate::RwBuilder;

/// Type for building readers and writers on top of a connected TCP socket.
/// It is itself an `RwBuilder`, but can't be created through one.
/// This is why we call it a source.
#[derive(Debug)]
pub struct Builder<A>
where
    A: ToSocketAddrs,
{
    /// The address to connect to
    addr: A,
}

impl<A> Builder<A>
where
    A: ToSocketAddrs,
{
    /// Factory function to create a builder holding on to a socket address
    pub const fn new(addr: A) -> Self {
        Self { addr }
    }
}

impl<A> RwBuilder for Builder<A>
where
    A: ToSocketAddrs,
{
    type Reader = TcpStream;
    type Writer = TcpStream;

    fn reader(&self) -> Result<Self::Reader> {
        let stream = TcpStream::connect(&self.addr)?;
        Ok(stream)
    }

    fn writer(&self) -> Result<Self::Writer> {
        let stream = TcpStream::connect(&self.addr)?;
        Ok(stream)
    }
}
