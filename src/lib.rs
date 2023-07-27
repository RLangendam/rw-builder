//! rw-builder provides a convenient way to build `std::io::Read`ers and
//! `std::io::Write`rs by chaining transformations. Since readers and writers
//! are defined simultaneously through the same builder they can be used as
//! inverses of each other.
#![deny(
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    unused,
    warnings
)]
#![deny(
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    rust_2021_incompatible_or_patterns,
    pointer_structural_match,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::missing_safety_doc,
    clippy::missing_docs_in_private_items
)]
#![deny(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::private_intra_doc_links
)]

use anyhow::Result;

/// Provides the `BincodeBuilder` type which acts as a sink to (de)serialize a
/// `&[u8]` as bincode.
#[cfg(feature = "bincode")]
mod bincode;
#[cfg(feature = "bincode")]
pub use crate::bincode::Builder as BincodeBuilder;

/// Provides the `BufferedBuilder` type which helps build `BufReader` and
/// `BufWriter` instances.
mod buffered;
pub use buffered::Builder as BufferedBuilder;

/// Provides the `FileBuilder` type which acts as a source to read from and
/// write to a file.
mod file;
pub use file::Builder as FileBuilder;

/// Provides several wrapper types around the streaming compression algorithms
/// provided by the flate2 crate.
#[cfg(feature = "flate2")]
mod flate2;
#[cfg(feature = "flate2")]
pub use ::flate2::Compression;

#[cfg(feature = "flate2")]
pub use crate::flate2::{CompressionBuilder, Constructor, CrcBuilder};

/// Provides the `ProcessBuilder` type which acts as a source to read from
/// stdout and write to stdin of a running process.
mod process;
pub use process::Builder as ProcessBuilder;

/// Provides several wrapper types around the streaming cipher algorithms
/// provided by the flate2 crate.
#[cfg(any(feature = "chacha20", feature = "salsa20"))]
mod stream_cipher;
#[cfg(feature = "chacha20")]
pub use stream_cipher::{ChaCha20Builder, ChaCha20Key, ChaCha20Nonce};
#[cfg(feature = "salsa20")]
pub use stream_cipher::{Salsa20Builder, Salsa20Key, Salsa20Nonce};

/// Provides the `StringBuilder` type which is a sink without serde
mod string;
pub use string::AdhocWriter;

/// Provides the `TcpStreamBuilder` type which acts as a source to read from and
/// write to a TCP stream.
mod tcp_stream;
pub use tcp_stream::Builder as TcpStreamBuilder;

/// Provides the `VecBuilder` type which acts as a source to read from and write
/// to a memory buffer.
mod vec;
pub use vec::Builder as VecBuilder;

/// The trait that can construct readers and writers, but also has chainable
/// functions to create more complex builders
pub trait RwBuilder
where
    Self: Sized,
    Self::Reader: std::io::Read,
    Self::Writer: std::io::Write,
{
    /// The reader type that will be constructed by the reader function
    type Reader;

    /// Construct a reader from this builder
    /// # Errors
    /// In case the construction of any of the intermediate readers fails this
    /// will return the error associated to the first one that failed.
    fn reader(&self) -> Result<Self::Reader>;

    /// The writer type that will be constructed by the reader function
    type Writer;

    /// Construct a writer from this builder
    /// # Errors
    /// In case the construction of any of the intermediate writers fails this
    /// will return the error associated to the first one that failed.
    fn writer(&self) -> Result<Self::Writer>;

    /// Buffers the underlying readers and writers by wrapping them in a
    /// `BufReader` or `BufWriter`
    fn buffered(self) -> BufferedBuilder<Self> {
        BufferedBuilder::new(self)
    }

    /// Sink that provides a bridge between `String` instances and underlying
    /// readers and writers.
    fn string(self) -> string::Builder<Self> {
        string::Builder::new(self)
    }

    /// Sink that provides a bridge between serde and the underlying readers and
    /// writer by transforming from and to bincode.
    #[cfg(feature = "bincode")]
    fn bincode(self) -> BincodeBuilder<Self> {
        BincodeBuilder::new(self)
    }

    /// Transformation that decrypts while reading and encrypts while writing
    /// using the chacha20 cipher
    #[cfg(feature = "chacha20")]
    fn chacha20(self, key: ChaCha20Key, nonce: ChaCha20Nonce) -> ChaCha20Builder<Self> {
        ChaCha20Builder::<Self>::new(self, key, nonce)
    }

    /// Transformation that decrypts while reading and encrypts while writing
    /// using the salsa20 cipher
    #[cfg(feature = "salsa20")]
    fn salsa20(self, key: Salsa20Key, nonce: Salsa20Nonce) -> Salsa20Builder<Self> {
        Salsa20Builder::<Self>::new(self, key, nonce)
    }

    /// Non-commutative transformation that hashes using the CRC algorithm
    #[cfg(feature = "flate2")]
    fn crc(self) -> CrcBuilder<Self> {
        CrcBuilder::new(self)
    }

    /// Transformation that decompresses while reading and compresses while
    /// writing using the Deflate algorithm
    #[cfg(feature = "flate2")]
    fn deflate(self, compression: Compression) -> CompressionBuilder<Self, flate2::Deflate> {
        flate2::Deflate::new(self, compression)
    }

    /// Transformation that decompresses while reading and compresses while
    /// writing using the Gz algorithm
    #[cfg(feature = "flate2")]
    fn gz(self, compression: Compression) -> CompressionBuilder<Self, flate2::Gz> {
        flate2::Gz::new(self, compression)
    }

    /// Transformation that decompresses while reading and compresses while
    /// writing using the Zlib algorithm
    #[cfg(feature = "flate2")]
    fn zlib(self, compression: Compression) -> CompressionBuilder<Self, flate2::Zlib> {
        flate2::Zlib::new(self, compression)
    }
}

/// Trait to wrap serialization and deserialization functionality behind uniform
/// load and save functions
#[cfg(feature = "bincode")]
pub trait SerDe {
    /// Deserialize into a specified type
    /// # Errors
    /// In case the deserialization or the reading fails the return value will
    /// contain the first error that occurred.
    fn load<T>(&self) -> Result<T>
    where
        T: for<'de> serde::de::Deserialize<'de>;

    /// Serialize into the type of the last sink specified
    /// # Errors
    /// In case the serialization or the writing fails the return value will
    /// contain the first error that occurred.
    fn save<T>(&self, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize;
}

#[cfg(test)]
mod tests;
