use rw_builder::{RwBuilderExt, VecBuilder};
mod common;
use common::test_string;

#[cfg(feature = "flate2")]
#[test]
fn compression_flate2() {
    use flate2::Compression;
    test_string(VecBuilder::default().zlib(Compression::fast()));
    test_string(VecBuilder::default().gz(Compression::fast()));
    test_string(VecBuilder::default().deflate(Compression::fast()));
}

#[cfg(feature = "zstd")]
#[test]
fn zstd_compression() {
    test_string(VecBuilder::default().zstd(3));
}

#[cfg(feature = "bzip2")]
#[test]
fn bzip2_compression() {
    test_string(VecBuilder::default().bzip2(bzip2::Compression::fast()));
}

#[cfg(feature = "lz4_flex")]
#[test]
fn lz4_flex_compression() {
    test_string(VecBuilder::default().lz4_flex());
}
