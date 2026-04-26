#![cfg(all(feature = "base64", feature = "serde_json"))]

use rw_builder::{RwBuilderExt, VecBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct UserConfig {
    username: String,
    theme: String,
    notifications_enabled: bool,
}

#[test]
fn json_streaming_test() {
    // We want to base64 encode our JSON so it's safely printable ASCII
    let builder = VecBuilder::default().base64().serde_json();

    let config = UserConfig {
        username: "admin".to_string(),
        theme: "dark".to_string(),
        notifications_enabled: true,
    };

    // 1. Save config to JSON string (JSON -> Base64 -> Vec<u8>)
    builder.save(&config).expect("Failed to serialize config");
    println!("Config successfully streamed to base64 encoded JSON bytes.");

    // 2. Load config back (Vec<u8> -> Base64 Decode -> JSON Decode -> Struct)
    let loaded_config: UserConfig = builder.load().expect("Failed to deserialize config");
    println!("Loaded config: {:#?}", loaded_config);

    assert_eq!(config, loaded_config);
}
