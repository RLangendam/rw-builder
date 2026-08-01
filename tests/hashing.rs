#![allow(unused_imports)]
use rw_builder::{RwBuilder, RwBuilderExt, VecBuilder};

#[cfg(feature = "flate2")]
#[test]
fn crc() {
    use std::io::{Read, Write};
    let expected_crc = 1_191_942_644;
    let data = [1, 2, 3, 4, 5];
    let builder = VecBuilder::default().crc();
    {
        let mut writer = builder.writer().expect("Writer couldn't be created.");
        writer.write_all(&data).expect("Couldn't write data.");
        assert_eq!(writer.crc().sum(), expected_crc);
    }
    {
        let mut reader = builder.reader().expect("Reader couldn't be created.");
        let mut buffer = vec![];
        let bytes_read = reader.read_to_end(&mut buffer).expect("Couldn't read into buffer.");
        assert_eq!(bytes_read, 5);
        assert_eq!(reader.crc().sum(), expected_crc);
        assert_eq!(buffer, data);
    }
}

#[cfg(feature = "sha2")]
#[test]
fn sha256_hash() {
    use std::io::{Read, Write};
    let data = b"hello world";
    let builder = VecBuilder::default().sha256();
    {
        let mut writer = builder.writer().unwrap();
        writer.write_all(data).unwrap();
        writer.flush().unwrap();
        assert!(format!("{:?}", writer).contains("DigestWriter"));
        let hash = writer.finalize();
        let hash_hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(hash_hex, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    }
    {
        let mut reader = builder.reader().unwrap();
        let mut buffer = Vec::new();
        let _bytes = reader.read_to_end(&mut buffer).unwrap();
        assert!(format!("{:?}", reader).contains("DigestReader"));
        let hash = reader.finalize();
        let hash_hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(hash_hex, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
        assert_eq!(buffer, data);
    }
    {
        let writer = builder.writer().unwrap();
        let _inner = writer.into_inner();
        let reader = builder.reader().unwrap();
        let _inner = reader.into_inner();
    }
}

#[cfg(feature = "sha2")]
#[test]
fn sha512_hash() {
    use std::io::{Read, Write};
    let data = b"hello world";
    let builder = VecBuilder::default().sha512();
    let mut writer = builder.writer().unwrap();
    writer.write_all(data).unwrap();
    let hash = writer.finalize();
    assert_eq!(hash.len(), 64);

    let mut reader = builder.reader().unwrap();
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    let hash_r = reader.finalize();
    assert_eq!(hash_r.len(), 64);
}

#[cfg(feature = "sha3")]
#[test]
fn sha3_hashes() {
    use std::io::{Read, Write};
    let data = b"hello world";

    // Sha3-256
    let builder = VecBuilder::default().sha3_256();
    let mut writer = builder.writer().unwrap();
    writer.write_all(data).unwrap();
    let hash = writer.finalize();
    assert_eq!(hash.len(), 32);

    let mut reader = builder.reader().unwrap();
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    let hash_r = reader.finalize();
    assert_eq!(hash_r.len(), 32);

    // Sha3-512
    let builder512 = VecBuilder::default().sha3_512();
    let mut writer512 = builder512.writer().unwrap();
    writer512.write_all(data).unwrap();
    let hash512 = writer512.finalize();
    assert_eq!(hash512.len(), 64);

    let mut reader512 = builder512.reader().unwrap();
    let mut buffer512 = Vec::new();
    reader512.read_to_end(&mut buffer512).unwrap();
    let hash512_r = reader512.finalize();
    assert_eq!(hash512_r.len(), 64);
}

#[cfg(feature = "crc32fast")]
#[test]
fn crc32fast_hash() {
    use std::io::{Read, Write};
    let data = b"hello crc32fast";
    let builder = VecBuilder::default().crc32fast();
    assert!(format!("{:?}", builder).contains("Crc32FastBuilder"));

    let mut writer = builder.writer().unwrap();
    writer.write_all(data).unwrap();
    writer.flush().unwrap();
    assert!(format!("{:?}", writer).contains("Crc32FastWriter"));
    let crc_w = writer.finalize();

    let mut reader = builder.reader().unwrap();
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    assert!(format!("{:?}", reader).contains("Crc32FastReader"));
    let crc_r = reader.finalize();

    assert_eq!(crc_w, crc_r);
    assert_eq!(buffer, data);
}
