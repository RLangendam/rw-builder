#![allow(unused_imports)]
use rw_builder::{RwBuilderExt, VecBuilder};
mod common;
use common::test_string;

#[cfg(feature = "chacha20")]
#[test]
fn chacha20() {
    use rw_builder::RwBuilder;
    use std::io::Write;
    let key = [0x42; 32];
    let nonce = [0x24; 12];
    {
        let builder = VecBuilder::default().chacha20(key.into(), nonce);
        let mut writer = builder.writer().unwrap();
        writer.write_all(b"test flush").unwrap();
        writer.flush().unwrap();
        assert!(format!("{:?}", writer).contains("Writer"));

        let reader = builder.reader().unwrap();
        assert!(format!("{:?}", reader).contains("Reader"));
    }
    test_string(VecBuilder::default().chacha20(key.into(), nonce));
}

#[cfg(feature = "salsa20")]
#[test]
fn salsa20() {
    use rw_builder::RwBuilder;
    use std::io::Write;
    let key = [0x42; 32];
    let nonce = [0x24; 8];
    {
        let builder = VecBuilder::default().salsa20(key.into(), nonce.into());
        let mut writer = builder.writer().unwrap();
        writer.write_all(b"test flush").unwrap();
        writer.flush().unwrap();

        let _reader = builder.reader().unwrap();
    }
    test_string(VecBuilder::default().salsa20(key.into(), nonce.into()));
}

#[cfg(feature = "aes_ctr")]
#[test]
fn aes128_ctr() {
    use rw_builder::RwBuilder;
    use std::io::Write;
    let key = [0x42; 16];
    let nonce = [0x24; 16];
    {
        let builder = VecBuilder::default().aes128_ctr(key, nonce);
        let mut writer = builder.writer().unwrap();
        writer.write_all(b"test flush").unwrap();
        writer.flush().unwrap();
        assert!(format!("{:?}", writer).contains("Writer"));

        let reader = builder.reader().unwrap();
        assert!(format!("{:?}", reader).contains("Reader"));
    }

    test_string(VecBuilder::default().aes128_ctr(key, nonce));
}

#[cfg(feature = "aes_ctr")]
#[test]
fn aes256_ctr() {
    use rw_builder::RwBuilder;
    use std::io::Write;
    let key = [0x42; 32];
    let nonce = [0x24; 16];
    {
        let builder = VecBuilder::default().aes256_ctr(key, nonce);
        let mut writer = builder.writer().unwrap();
        writer.write_all(b"test flush").unwrap();
        writer.flush().unwrap();
        assert!(format!("{:?}", writer).contains("Writer"));

        let reader = builder.reader().unwrap();
        assert!(format!("{:?}", reader).contains("Reader"));
    }
    test_string(VecBuilder::default().aes256_ctr(key, nonce));
}

#[cfg(feature = "base64")]
#[test]
fn base64_test() {
    test_string(VecBuilder::default().base64());
}
