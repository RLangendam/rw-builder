# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-04-27

### Added
- `base64` text encoding modifier stream.
- `serde_json` terminal sink for serialization.
- `postcard` terminal sink for binary serialization (`no_std` compatible alternative to `bincode`).
- Criterion benchmark suite demonstrating the near-zero overhead of the builder abstraction.
- Documentation and badges for docs.rs.

### Changed
- Converted all examples into heavily documented integration test suites within the `tests/` directory.
- Fixed unused imports warnings during feature combinatorial testing.
- Restored standard `rustfmt` configuration for improved CI compatibility.

## [0.1.0] - 2026-04-26

### Added
- Combinatorial tests for complex reader/writer chains.
- `lint.yml` GitHub Action for `cargo clippy`, `cargo fmt`, and `cargo audit` checks.
- Extension trait pattern via `RwBuilderExt` for better extensibility.
- `zstd`, `bzip2`, and `lz4_flex` compression algorithms.
- `digest`, `sha2`, `sha3`, and `crc32fast` hashing algorithms.
- `aes_ctr` symmetric encryption (`Aes128Ctr` and `Aes256Ctr`).
- `rmp_serde` MessagePack serialization sink.

### Changed
- Standardized MSRV to 1.89 across all configuration files and documentation.
- Enhanced `dependabot.yml` to group updates and use `chore(deps)` conventional commit prefixes.
- Replaced `anyhow` with `thiserror` for better library error handling.
- **Breaking**: Removed `SerDe` trait; `load` and `save` are now directly implemented on sinks.
- **Breaking**: Moved combinator methods (e.g. `wincode`, `chacha20`) from `RwBuilder` into a new `RwBuilderExt` extension trait.
- **Breaking**: Added `#[non_exhaustive]` to the `Error` enum to prevent future breakage on new error types.
- **Breaking**: Added `#[must_use]` to all builders and combinators to warn users against unused chains.

## [0.0.2] - 2026-04-26

### Changed
- Updated `chacha20` from 0.9.1 to 0.10.0
- Updated `cipher` from 0.4.4 to 0.5.1 for compatibility with chacha20 0.10.0
- Updated `salsa20` from 0.10.2 to 0.11.0 for compatibility with cipher 0.5.1
- Fixed ChaCha20 nonce type to use `[u8; 12]` array for IETF variant
- Updated GitHub Actions to v4 versions

### Added
- MSRV (Minimum Supported Rust Version) set to 1.70

## [0.0.1] - Initial Release

### Added
- Initial implementation of the reader/writer builder pattern
- Support for multiple transformations: compression, encryption, serialization
- Features for: `wincode`, `chacha20`, `flate2`, `salsa20`
- Multiple source builders: `FileBuilder`, `ProcessBuilder`, `TcpStreamBuilder`, `VecBuilder`
- Comprehensive test coverage
- GitHub Actions CI/CD pipeline
- Documentation and examples

[Unreleased]: https://github.com/RLangendam/rw-builder/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/RLangendam/rw-builder/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/RLangendam/rw-builder/compare/v0.0.2...v0.1.0
[0.0.2]: https://github.com/RLangendam/rw-builder/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/RLangendam/rw-builder/releases/tag/v0.0.1
