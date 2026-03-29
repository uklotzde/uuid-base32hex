// SPDX-FileCopyrightText: The uuid-base32hex authors
// SPDX-License-Identifier: MPL-2.0

use super::{Uuid, UuidEncodedStr};

#[test]
fn nil() {
    assert_eq!(UuidEncodedStr::encode(&Uuid::NIL), UuidEncodedStr::NIL);
    assert_eq!(UuidEncodedStr::NIL.decode(), Uuid::NIL);
}

#[test]
fn default() {
    assert_eq!(Uuid::default(), Uuid::NIL);
    assert_eq!(UuidEncodedStr::default(), UuidEncodedStr::NIL);
}

#[test]
fn should_encode_decode_uuid() {
    let uuid = Uuid::now_v7();
    let encoded_str = UuidEncodedStr::encode(&uuid);
    assert_eq!(encoded_str.len(), Uuid::STR_LEN);
    let decoded = encoded_str.decode();
    assert_eq!(uuid, decoded);
}

#[test]
fn should_fail_to_decode_too_long_string() {
    let uuid = Uuid::now_v7();

    // Test encode -> decode roundtrip
    let mut encoded = uuid.to_string();
    assert!(Uuid::decode_str(&encoded).is_ok());

    // Append the first character of the alphabet to the encoded string.
    encoded.push('0');
    assert!(Uuid::decode_str(&encoded).is_err());
}

#[test]
fn should_fail_to_decode_too_short_string() {
    let uuid = Uuid::now_v7();
    let mut encoded = uuid.to_string();
    encoded.truncate(Uuid::STR_LEN - 1);
    assert!(Uuid::decode_str(&encoded).is_err());
}
