use rw_builder::{FileBuilder, ProcessBuilder, RwBuilderExt};
use std::{env::temp_dir, process::Command};
mod common;
use common::write_and_read_string;

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
    use rw_builder::AdhocWriter;
    let command = Command::new("tee");
    let builder = ProcessBuilder::new(command).spawn().expect("Couldn't spawn process").string();
    builder.write_string("Hello world.\n").expect("Couldn't write string.");
    assert_eq!(builder.to_string(), "Hello world.\n");
}
