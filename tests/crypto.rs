#![allow(unused_imports)]
use rw_builder::{RwBuilderExt, VecBuilder};
mod common;
use common::test_string;

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

#[cfg(feature = "aes_ctr")]
#[test]
fn aes128_ctr() {
    let key = [0x42; 16];
    let nonce = [0x24; 16];
    test_string(VecBuilder::default().aes128_ctr(key.into(), nonce.into()));
}

#[cfg(feature = "base64")]
#[test]
fn base64_test() {
    test_string(VecBuilder::default().base64());
}
