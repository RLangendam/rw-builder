#![allow(unused_imports)]
use rw_builder::{RwBuilderExt, VecBuilder};

#[cfg(feature = "wincode")]
#[test]
fn wincode() {
    let builder = VecBuilder::default().wincode();
    assert!(format!("{:?}", builder).contains("Builder"));
    let text = "This string is serialized and deserialized using wincode.";
    builder.save(&text).expect("Serialization failed.");
    let actual: String = builder.load().expect("Deserialization failed.");
    assert_eq!(actual, text);

    let empty_builder = VecBuilder::default().wincode();
    assert!(empty_builder.load::<String>().is_err());
}

#[cfg(feature = "rmp_serde")]
#[test]
fn rmp_serde() {
    let builder = VecBuilder::default().rmp_serde();
    assert!(format!("{:?}", builder).contains("Builder"));
    let text = "This string is serialized and deserialized using rmp-serde.";
    builder.save(&text).expect("Serialization failed.");
    let actual: String = builder.load().expect("Deserialization failed.");
    assert_eq!(actual, text);

    let empty_builder = VecBuilder::default().rmp_serde();
    assert!(empty_builder.load::<String>().is_err());
}

#[cfg(feature = "serde_json")]
#[test]
fn serde_json_test() {
    let builder = VecBuilder::default().serde_json();
    assert!(format!("{:?}", builder).contains("Builder"));
    let text = "This string is serialized and deserialized using serde_json.";
    builder.save(&text).expect("Serialization failed.");
    let actual: String = builder.load().expect("Deserialization failed.");
    assert_eq!(actual, text);

    let empty_builder = VecBuilder::default().serde_json();
    assert!(empty_builder.load::<String>().is_err());
}

#[cfg(feature = "postcard")]
#[test]
fn postcard_test() {
    let builder = VecBuilder::default().postcard();
    assert!(format!("{:?}", builder).contains("Builder"));
    let text = "This string is serialized and deserialized using postcard.";
    builder.save(&text).expect("Serialization failed.");
    let actual: String = builder.load().expect("Deserialization failed.");
    assert_eq!(actual, text);

    let empty_builder = VecBuilder::default().postcard();
    assert!(empty_builder.load::<String>().is_err());
}
