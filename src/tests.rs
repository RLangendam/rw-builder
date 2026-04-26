use std::{env::temp_dir, process::Command};

#[cfg(feature = "flate2")]
use ::flate2::Compression;

use super::*;
use crate::string::AdhocWriter;

fn write_and_read_string<B>(builder: B, input: &str) -> Result<String>
where
    B: RwBuilder,
{
    let string = builder.string();
    string.write_string(input)?;
    Ok(string.to_string())
}

#[cfg(any(
    feature = "flate2",
    feature = "chacha20",
    feature = "salsa20",
    feature = "zstd",
    feature = "bzip2",
    feature = "lz4_flex",
    feature = "aes_ctr"
))]
fn test_string<B>(builder: B)
where
    B: RwBuilder,
{
    let text = String::from("This text is written from a String and read back into a String.");
    let actual = write_and_read_string(builder, &text).expect("String couldn't be written");
    assert_eq!(actual, text);
}

#[cfg(feature = "flate2")]
#[test]
fn compression() {
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

#[cfg(feature = "chacha20")]
#[test]
fn chacha20() {
    let key = [0x42; 32];
    let nonce = [0x24; 12];
    test_string(VecBuilder::default().chacha20(key.into(), nonce));
}

#[cfg(feature = "salsa20")]
#[test]
fn salsa20() {
    let key = [0x42; 32];
    let nonce = [0x24; 8];
    test_string(VecBuilder::default().salsa20(key.into(), nonce.into()));
}

#[test]
fn file() {
    let path = temp_dir().join("test_file.txt");
    let text = String::from("This text is written from a String and read back into a String.");
    let builder = FileBuilder::new(path.clone()).buffered();
    let result = write_and_read_string(builder, &text);
    std::fs::remove_file(path).expect("File couldn't be removed.");
    let actual = result.expect("String couldn't be written");
    assert_eq!(actual, text);
}

#[test]
fn process_stdout() {
    let mut command = Command::new("rustc");
    let _ = command.arg("--help");
    let help = ProcessBuilder::new(command).string().to_string();
    assert!(help.starts_with("Usage: rustc"));
}

#[cfg(target_os = "linux")]
#[test]
fn process_child() {
    let command = Command::new("tee");
    let builder = ProcessBuilder::new(command).spawn().expect("Couldn't spawn process").string();
    builder.write_string("Hello world.\n").expect("Couldn't write string.");
    assert_eq!(builder.to_string(), "Hello world.\n");
}

#[cfg(feature = "wincode")]
#[test]
fn wincode() {
    let builder = VecBuilder::default().wincode();
    let text = "This string is serialized and deserialized using wincode.";
    builder.save(&text).expect("Serialization failed.");
    let actual: String = builder.load().expect("Deserialization failed.");
    assert_eq!(actual, text);
}

#[cfg(all(feature = "wincode", feature = "chacha20", feature = "flate2", feature = "salsa20"))]
#[test]
fn roundtrip_combinations() {
    let key1 = [0x42; 32];
    let nonce1 = [0x24; 12];
    let key2 = [0x11; 32];
    let nonce2 = [0x22; 8];

    let builder = VecBuilder::default()
        .deflate(Compression::fast())
        .chacha20(key1.into(), nonce1)
        .salsa20(key2.into(), nonce2.into())
        .wincode();

    let text = "This is a complex roundtrip test chaining compression, multiple encryptions, and wincode serialization.";
    builder.save(&text).expect("Complex serialization failed.");
    let actual: String = builder.load().expect("Complex deserialization failed.");
    assert_eq!(actual, text);
}

#[cfg(feature = "aes_ctr")]
#[test]
fn aes128_ctr() {
    let key = [0x42; 16];
    let nonce = [0x24; 16];
    test_string(VecBuilder::default().aes128_ctr(key.into(), nonce.into()));
}

#[cfg(feature = "rmp_serde")]
#[test]
fn rmp_serde() {
    let builder = VecBuilder::default().rmp_serde();
    let text = "This string is serialized and deserialized using rmp-serde.";
    builder.save(&text).expect("Serialization failed.");
    let actual: String = builder.load().expect("Deserialization failed.");
    assert_eq!(actual, text);
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
        assert_eq!(format!("{:x}", hash), "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    }
    {
        let mut reader = builder.reader().unwrap();
        let mut buffer = Vec::new();
        let _bytes = reader.read_to_end(&mut buffer).unwrap();
        let hash = reader.finalize();
        assert_eq!(format!("{:x}", hash), "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
        assert_eq!(buffer, data);
    }
}
