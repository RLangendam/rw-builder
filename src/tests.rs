use std::{env::temp_dir, process::Command};

#[cfg(feature = "flate2")]
use ::flate2::Compression;

use crate::string::AdhocWriter;

use super::*;

fn write_and_read_string<B>(builder: B, input: &str) -> Result<String>
where
    B: RwBuilder,
{
    let string = builder.string();
    string.write_string(input)?;
    Ok(string.to_string())
}

#[cfg(any(feature = "flate2", feature = "chacha20", feature = "salsa20"))]
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

#[cfg(feature = "chacha20")]
#[test]
fn chacha20() {
    let key = [0x42; 32];
    let nonce = [0x24; 12];
    test_string(VecBuilder::default().chacha20(key.into(), nonce.into()));
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
    let builder = ProcessBuilder::new(command)
        .spawn()
        .expect("Couldn't spawn process")
        .string();
    builder
        .write_string("Hello world.\n")
        .expect("Couldn't write string.");
    assert_eq!(builder.to_string(), "Hello world.\n");
}

#[cfg(feature = "bincode")]
#[test]
fn bincode() {
    let builder = VecBuilder::default().bincode();
    let text = "This string is serialized and deserialized using bincode.";
    builder.save(&text).expect("Serialization failed.");
    let actual: String = builder.load().expect("Deserialization failed.");
    assert_eq!(actual, text);
}
