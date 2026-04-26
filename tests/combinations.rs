use rw_builder::{FileBuilder, RwBuilder, RwBuilderExt, VecBuilder};

#[cfg(all(
    feature = "wincode",
    feature = "chacha20",
    feature = "flate2",
    feature = "salsa20"
))]
#[test]
fn roundtrip_combinations() {
    use flate2::Compression;

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
    let actual: String = builder
        .load()
        .expect("Complex deserialization failed.");
    assert_eq!(actual, text);
}

#[cfg(all(feature = "aes_ctr", feature = "flate2"))]
#[test]
fn compression_encryption_order() {
    use flate2::Compression;
    use std::env::temp_dir;
    use std::io::Write;

    let data = vec![0u8; 10000];
    let key = [0x42; 16];
    let nonce = [0x24; 16];

    let path_correct = temp_dir().join("test_correct.bin");
    // Correct order: AES wraps Deflate (Deflate runs first on write)
    let builder_correct = FileBuilder::new(path_correct.clone())
        .aes128_ctr(key.into(), nonce.into())
        .deflate(Compression::fast());
    let mut writer_correct = builder_correct.writer().unwrap();
    writer_correct.write_all(&data).unwrap();
    writer_correct.flush().unwrap();
    drop(writer_correct);
    let len_correct = std::fs::metadata(&path_correct).unwrap().len();

    let path_incorrect = temp_dir().join("test_incorrect.bin");
    // Incorrect order: Deflate wraps AES (AES runs first on write)
    let builder_incorrect = FileBuilder::new(path_incorrect.clone())
        .deflate(Compression::fast())
        .aes128_ctr(key.into(), nonce.into());
    let mut writer_incorrect = builder_incorrect.writer().unwrap();
    writer_incorrect.write_all(&data).unwrap();
    writer_incorrect.flush().unwrap();
    drop(writer_incorrect);
    let len_incorrect = std::fs::metadata(&path_incorrect).unwrap().len();

    // Encrypting before compressing results in no compression
    assert!(len_correct < len_incorrect);
    // Correct order should compress it massively
    assert!(len_correct < 1000);
    // Incorrect order keeps it near 10000 bytes
    assert!(len_incorrect >= 10000);

    let _ = std::fs::remove_file(path_correct);
    let _ = std::fs::remove_file(path_incorrect);
}

#[cfg(all(feature = "sha2", feature = "rmp_serde"))]
#[test]
fn hashing_with_sink_workaround() {
    let data = "serialize me";
    let builder = VecBuilder::default().sha256();

    // To get the hash, we must construct the writer manually instead of using builder.rmp_serde().save()
    // because sinks consume and drop the writer.
    let mut writer = builder.writer().unwrap();
    ::rmp_serde::encode::write(&mut writer, &data).unwrap();
    let hash = writer.finalize();
    assert_eq!(hash.len(), 32);
}
