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
        let bytes_read = reader
            .read_to_end(&mut buffer)
            .expect("Couldn't read into buffer.");
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
        let hash = writer.finalize();
        let hash_hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(
            hash_hex,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }
    {
        let mut reader = builder.reader().unwrap();
        let mut buffer = Vec::new();
        let _bytes = reader.read_to_end(&mut buffer).unwrap();
        let hash = reader.finalize();
        let hash_hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(
            hash_hex,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
        assert_eq!(buffer, data);
    }
}
