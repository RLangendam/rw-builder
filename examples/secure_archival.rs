use flate2::Compression;
use rw_builder::{FileBuilder, RwBuilderExt};
use std::env::temp_dir;

fn main() {
    let file_path = temp_dir().join("secure_archive.bin");
    let key = [0x42; 32]; // AES-256 Key
    let nonce = [0x24; 16]; // AES Nonce

    // Create a builder chain: File -> AES-256 -> Deflate Compression -> Postcard Binary Sink
    let builder = FileBuilder::new(file_path.clone())
        .aes256_ctr(key.into(), nonce.into())
        .deflate(Compression::fast())
        .postcard();

    // The data to archive
    let sensitive_data = vec!["Secret Agent 007", "Project X Details", "Launch Codes"];

    // 1. Save data (Serialization -> Compression -> Encryption -> File)
    builder
        .save(&sensitive_data)
        .expect("Failed to secure archive data");
    println!("Successfully archived sensitive data to {:?}", file_path);

    // 2. Load data (File -> Decryption -> Decompression -> Deserialization)
    let extracted_data: Vec<String> = builder.load().expect("Failed to extract archive");
    println!("Extracted data: {:?}", extracted_data);

    std::fs::remove_file(file_path).unwrap();
}
