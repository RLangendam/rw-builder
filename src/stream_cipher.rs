use std::{
    io::{Read, Write},
    marker::PhantomData,
};

use anyhow::Result;
#[cfg(feature = "chacha20")]
use chacha20::ChaCha20;
use cipher::{KeyIvInit, StreamCipher};
#[cfg(feature = "salsa20")]
use salsa20::Salsa20;

use crate::RwBuilder;

/// Type returned by the `chacha20` and `salsa20` functions on the `RwBuilder` trait.
/// It is itself an `RwBuilder` so can be chained further.
#[derive(Debug)]
pub struct Builder<B, C, K, N>
where
    B: RwBuilder,
    C: StreamCipher,
{
    /// The inner builder it wraps
    builder: B,
    /// The key used for encryption and decryption
    key: K,
    /// The nonce used for encryption and decryption
    nonce: N,
    /// We need `Builder` to be generic over the `StreamCipher`
    _marker: PhantomData<C>,
}

impl<B, C, K, N> Builder<B, C, K, N>
where
    B: RwBuilder,
    C: StreamCipher,
{
    /// Create a new cipher builder from a key and a nonce
    pub fn new(builder: B, key: K, nonce: N) -> Self {
        Self {
            builder,
            key,
            nonce,
            _marker: PhantomData::default(),
        }
    }
}

/// The key type for the chacha20 cipher
#[cfg(feature = "chacha20")]
pub type ChaCha20Key = chacha20::Key;

/// The nonce type for the chacha20 cipher
#[cfg(feature = "chacha20")]
pub type ChaCha20Nonce = chacha20::Nonce;

/// The type returned by the `chacha20` function in the `RwBuilder` trait
#[cfg(feature = "chacha20")]
pub type ChaCha20Builder<B> = Builder<B, ChaCha20, ChaCha20Key, ChaCha20Nonce>;

/// The key type for the salsa20 cipher
#[cfg(feature = "salsa20")]
pub type Salsa20Key = salsa20::Key;

/// The nonce type for the salsa20 cipher
#[cfg(feature = "salsa20")]
pub type Salsa20Nonce = salsa20::Nonce;

/// The type returned by the `salsa20` function in the `RwBuilder` trait
#[cfg(feature = "salsa20")]
pub type Salsa20Builder<B> = Builder<B, Salsa20, Salsa20Key, Salsa20Nonce>;

/// Recipe for how to create a cipher
trait CipherFactory<C> {
    /// Create the cipher from the key and the nonce stored in self
    fn create_cipher(&self) -> C;
}

#[cfg(feature = "chacha20")]
impl<B> CipherFactory<ChaCha20> for Builder<B, ChaCha20, ChaCha20Key, ChaCha20Nonce>
where
    B: RwBuilder,
{
    fn create_cipher(&self) -> ChaCha20 {
        ChaCha20::new(&self.key, &self.nonce)
    }
}

#[cfg(feature = "salsa20")]
impl<B> CipherFactory<Salsa20> for Builder<B, Salsa20, Salsa20Key, Salsa20Nonce>
where
    B: RwBuilder,
{
    fn create_cipher(&self) -> Salsa20 {
        Salsa20::new(&self.key, &self.nonce)
    }
}

impl<B, C, K, N> RwBuilder for Builder<B, C, K, N>
where
    B: RwBuilder,
    C: StreamCipher,
    Self: CipherFactory<C>,
{
    type Reader = Reader<B::Reader, C>;

    fn reader(&self) -> Result<Self::Reader> {
        let reader = self.builder.reader()?;
        let cipher = self.create_cipher();
        Ok(Reader { cipher, reader })
    }

    type Writer = Writer<B::Writer, C>;

    fn writer(&self) -> Result<Self::Writer> {
        let writer = self.builder.writer()?;
        let cipher = self.create_cipher();
        Ok(Writer { cipher, writer })
    }
}

/// Generic Reader type for multiple ciphers
#[derive(Debug)]
pub struct Reader<R, C>
where
    R: Read,
    C: StreamCipher,
{
    /// The cipher to use for reading
    cipher: C,
    /// The wrapped reader
    reader: R,
}

impl<R, C> Read for Reader<R, C>
where
    R: Read,
    C: StreamCipher,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        self.cipher
            .try_apply_keystream(buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(bytes_read)
    }
}

/// Generic Writer type for multiple ciphers
#[derive(Debug)]
pub struct Writer<W, C>
where
    W: Write,
    C: StreamCipher,
{
    /// The cipher to use for writing
    cipher: C,
    /// The wrapped writer
    writer: W,
}

impl<W, C> Write for Writer<W, C>
where
    W: Write,
    C: StreamCipher,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buffer = buf.to_owned();
        self.cipher
            .try_apply_keystream(buffer.as_mut_slice())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        self.writer.write(buffer.as_slice())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
