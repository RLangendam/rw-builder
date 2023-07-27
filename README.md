# Reader Writer Builder


[![Test Status](https://github.com/rlangendam/rw-builder/workflows/Tests/badge.svg?event=push)](https://github.com/rlangendam/rw-builder/actions)
[![Crate](https://img.shields.io/crates/v/rw-builder.svg)](https://crates.io/crates/rw-builder)
[![API](https://img.shields.io/badge/api-master-yellow.svg)](https://rlangendam.github.io/rw-builder/index.html)
[![API](https://docs.rs/rw-builder/badge.svg)](https://docs.rs/rw-builder)

- [Reader Writer Builder](#reader-writer-builder)
  - [Introduction](#introduction)
  - [Usage](#usage)
  - [Warning](#warning)
  - [Example](#example)
  - [Sources and Sinks](#sources-and-sinks)
  - [Features](#features)
  - [Contributing](#contributing)
  - [License](#license)

## Introduction

This crate provides a convenient way to build `std::io::Read`ers and `std::io::Write`rs by chaining transformations. Since readers and writers are defined simultaneously through the same builder they can be used as inverses of each other.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rw-builder = "0.0.1"
```

## Warning

This crate is provided "as is" without warranty of any kind. Furthermore, this crate is still in its infancy and lacks significant real world exposure. We try our best to ensure the quality of this crate, but we should warn you that it's more a proof of concept than something to be used in production at the moment.

## Example

Let's say you have some application state you want to encrypt and store on disk. Once the application starts up you want to read that state back into memory. A good practice when encrypting is to compress the data beforehand so you may feel the desire to chain some readers and writers together.

```rust
use anyhow::Result;
use flate2::Compression;
use rw_builder::{FileBuilder, RwBuilder, SerDe};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ApplicationState {
    ...
}

fn main() -> Result<()> {
    let key = [0x42; 32];
    let nonce = [0x24; 12];
    let builder = FileBuilder::new("/some/file".into())
        .buffered()
        .chacha20(key.into(), nonce.into())
        .deflate(Compression::fast())
        .bincode();
    let mut state: ApplicationState = builder.load()?;
    // Change the state
    builder.save(&state)
}
```
The builder ensures the order of the readers will match the order of the writers so there's no opportunity for mistakes.

Writing something similar in the usual way is much more verbose and error prone.
```rust
use anyhow::Result;
use cipher::{KeyIvInit, StreamCipher};
use flate2::{Compression, Status};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ApplicationState {
    ...
}

fn main() -> Result<()> {
    let mut state = load_application_state()?;
    // Change the state
    save_application_state(&state)
}

fn save_application_state(state: &ApplicationState) -> Result<()> {
    let serialized = bincode::serialize(&state)?;
    let mut compressor = flate2::Compress::new(Compression::fast(), false);
    let mut output = vec![];
    assert_eq!(
        compressor.compress_vec(&serialized, &mut output, flate2::FlushCompress::Sync)?,
        Status::StreamEnd
    );
    let key = [0x42; 32];
    let nonce = [0x24; 12];
    let mut decoder = chacha20::ChaCha20::new(&key.into(), &nonce.into());
    decoder.apply_keystream(&mut output);
    Ok(std::fs::write("/some/file", output)?)
}

fn load_application_state() -> Result<ApplicationState> {
    let mut buffer = std::fs::read("/some/file")?;
    let key = [0x42; 32];
    let nonce = [0x24; 12];
    let mut encoder = chacha20::ChaCha20::new(&key.into(), &nonce.into());
    encoder.apply_keystream(&mut buffer);
    let mut decompress = flate2::Decompress::new(false);
    let mut output = vec![];
    assert_eq!(
        decompress.decompress_vec(
            buffer.as_slice(),
            &mut output,
            flate2::FlushDecompress::Sync,
        )?,
        Status::StreamEnd
    );
    Ok(bincode::deserialize(output.as_slice())?)
}
```
This second example doesn't support streaming, which is necessary in the case of large files. Notice how the save and load functionality is described in reverse order from each other and that configuration options like the file location and the encryption key need to be made available in multiple locations. This is needlessly challenging to maintain when compared to the former example.

## Sources and Sinks

You may have noticed that the `FileBuilder` struct and the `bincode` function have a special role. They are examples of a source and a sink respectively. Sources are a typical starting point for chaining builders, since they can be constructed without an inner builder. Sinks are a typical ending point for chaining builders, since they can interface with other types than `&[u8]` which `Read` and `Write` are restricted to.

## Features

To provide the functionality of many different readers and writers this crate has many optional dependencies which are enabled through a predefined set of features. The example above requires the `bincode`, `chacha20` and `flate2` features. Currently, the following features are available:
* `bincode`: includes the `serde` and `bincode` crates and enables the `SerDe` trait and the `bincode` function on the `RwBuilder` trait.
* `chacha20`: includes the `cipher` and `chacha20` crates and enables the `chacha20` function on the `RwBuilder` trait.
* `salsa20`: includes the `cipher` and `salsa20` crates and enables the `salsa20` function on the `RwBuilder` trait.
* `flate2`: includes the `flate2` crate and enables the `crc`, `deflate`, `gz` and `zlib` functions on the `RwBuilder` trait.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

The rw-builder crate is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0), with portions covered by various BSD-like licenses.

See [LICENSE-APACHE.txt](LICENSE-APACHE.txt), [LICENSE-MIT.txt](LICENSE-MIT.txt), and [COPYRIGHT.txt](COPYRIGHT.txt) for details.