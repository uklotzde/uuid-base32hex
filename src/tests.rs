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
fn new_from_into() {
    let uuid = Uuid::now_v7();
    assert_eq!(Uuid::new(uuid.into()), uuid);
    assert_eq!(Uuid::from(uuid.into()), uuid);
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
    let encoded = uuid.encode_str();
    assert!(Uuid::decode_str(&encoded).is_ok());

    // Append/prepend the first character of the alphabet to the encoded string.
    assert!(Uuid::decode_str(&[&encoded, "0"].concat()).is_err());
    assert!(Uuid::decode_str(&["0", &encoded].concat()).is_err());
}

#[test]
fn should_fail_to_decode_too_short_string() {
    let uuid = Uuid::now_v7();
    let encoded = uuid.encode_str();
    assert!(Uuid::decode_str(&encoded[..encoded.len() - 1]).is_err());
    assert!(Uuid::decode_str(&encoded[1..]).is_err());
}
