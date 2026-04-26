use std::{
    io::{Read, Write},
    marker::PhantomData,
};

use crate::Result;
#[cfg(feature = "aes_ctr")]
use aes::{Aes128, Aes256};
#[cfg(feature = "chacha20")]
use chacha20::ChaCha20;
use cipher::{KeyIvInit, StreamCipher};
#[cfg(feature = "aes_ctr")]
use ctr::Ctr128BE;
#[cfg(feature = "salsa20")]
use salsa20::Salsa20;

use crate::RwBuilder;

/// Type returned by the `chacha20` and `salsa20` functions on the `RwBuilder`
/// trait. It is itself an `RwBuilder` so can be chained further.
#[derive(Debug)]
#[must_use]
pub struct Builder<B, C, K, N>
where
    B: RwBuilder,
    C: StreamCipher,
{
    /// The inner builder it wraps
    wrapped: B,
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
    pub const fn new(builder: B, key: K, nonce: N) -> Self {
        Self { wrapped: builder, key, nonce, _marker: PhantomData }
    }
}

/// The key type for the chacha20 cipher
#[cfg(feature = "chacha20")]
pub type ChaCha20Key = chacha20::Key;

/// The nonce type for the chacha20 cipher (12 bytes for IETF variant)
#[cfg(feature = "chacha20")]
pub type ChaCha20Nonce = [u8; 12];

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

/// Internal type for AES-128-CTR cipher
#[cfg(feature = "aes_ctr")]
pub type Aes128Ctr = Ctr128BE<Aes128>;
/// Internal type for AES-256-CTR cipher
#[cfg(feature = "aes_ctr")]
pub type Aes256Ctr = Ctr128BE<Aes256>;

/// The key type for the AES-128-CTR cipher
#[cfg(feature = "aes_ctr")]
pub type Aes128Key = [u8; 16];
/// The key type for the AES-256-CTR cipher
#[cfg(feature = "aes_ctr")]
pub type Aes256Key = [u8; 32];
/// The nonce type for AES-CTR cipher
#[cfg(feature = "aes_ctr")]
pub type AesNonce = [u8; 16];

/// The type returned by the `aes128_ctr` function in the `RwBuilder` trait
#[cfg(feature = "aes_ctr")]
pub type Aes128CtrBuilder<B> = Builder<B, Aes128Ctr, Aes128Key, AesNonce>;
/// The type returned by the `aes256_ctr` function in the `RwBuilder` trait
#[cfg(feature = "aes_ctr")]
pub type Aes256CtrBuilder<B> = Builder<B, Aes256Ctr, Aes256Key, AesNonce>;

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
        ChaCha20::new(&self.key, (&self.nonce).into())
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

#[cfg(feature = "aes_ctr")]
impl<B> CipherFactory<Aes128Ctr> for Builder<B, Aes128Ctr, Aes128Key, AesNonce>
where
    B: RwBuilder,
{
    fn create_cipher(&self) -> Aes128Ctr {
        Aes128Ctr::new((&self.key).into(), (&self.nonce).into())
    }
}

#[cfg(feature = "aes_ctr")]
impl<B> CipherFactory<Aes256Ctr> for Builder<B, Aes256Ctr, Aes256Key, AesNonce>
where
    B: RwBuilder,
{
    fn create_cipher(&self) -> Aes256Ctr {
        Aes256Ctr::new((&self.key).into(), (&self.nonce).into())
    }
}

impl<B, C, K, N> RwBuilder for Builder<B, C, K, N>
where
    B: RwBuilder,
    C: StreamCipher,
    Self: CipherFactory<C>,
{
    type Reader = Reader<B::Reader, C>;
    type Writer = Writer<B::Writer, C>;

    fn reader(&self) -> Result<Self::Reader> {
        let reader = self.wrapped.reader()?;
        let cipher = self.create_cipher();
        Ok(Reader { cipher, reader })
    }

    fn writer(&self) -> Result<Self::Writer> {
        let writer = self.wrapped.writer()?;
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
        self.cipher.try_apply_keystream(buf).map_err(std::io::Error::other)?;
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
        self.cipher.try_apply_keystream(buffer.as_mut_slice()).map_err(std::io::Error::other)?;
        self.writer.write(buffer.as_slice())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
