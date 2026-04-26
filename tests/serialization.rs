use rw_builder::{RwBuilderExt, VecBuilder};

#[cfg(feature = "wincode")]
#[test]
fn wincode() {
    let builder = VecBuilder::default().wincode();
    let text = "This string is serialized and deserialized using wincode.";
    builder.save(&text).expect("Serialization failed.");
    let actual: String = builder.load().expect("Deserialization failed.");
    assert_eq!(actual, text);
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

#[cfg(feature = "serde_json")]
#[test]
fn serde_json_test() {
    let builder = VecBuilder::default().serde_json();
    let text = "This string is serialized and deserialized using serde_json.";
    builder.save(&text).expect("Serialization failed.");
    let actual: String = builder.load().expect("Deserialization failed.");
    assert_eq!(actual, text);
}

#[cfg(feature = "postcard")]
#[test]
fn postcard_test() {
    let builder = VecBuilder::default().postcard();
    let text = "This string is serialized and deserialized using postcard.";
    builder.save(&text).expect("Serialization failed.");
    let actual: String = builder.load().expect("Deserialization failed.");
    assert_eq!(actual, text);
}
