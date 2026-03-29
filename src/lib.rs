// SPDX-FileCopyrightText: The uuid-base32hex authors
// SPDX-License-Identifier: MPL-2.0

#![expect(rustdoc::invalid_rust_codeblocks)]
#![doc = include_str!("../README.md")]

use std::{fmt, hash::Hash, str};

use data_encoding::{BASE32HEX_NOPAD, DecodePartial, Encoding};
use derive_more::{AsRef, Deref, Display, Error, From};

/// UUID with base32hex string representation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, Deref)]
#[repr(transparent)]
#[cfg_attr(
    feature = "json-schema",
    derive(schemars::JsonSchema),
    schemars(transparent)
)]
pub struct Uuid {
    #[cfg_attr(feature = "json-schema", schemars(with = "String"))]
    uuid: uuid::Uuid,
}

impl Uuid {
    const ENCODING: &'static Encoding = &BASE32HEX_NOPAD;

    // Only needed for safe initialization.
    const ENCODING_PAD_CHAR: u8 = b'=';

    /// Length of UUID encoded as ASCII string.
    pub const STR_LEN: usize = 26;

    pub const NIL: Self = Self {
        uuid: uuid::Uuid::nil(),
    };

    #[must_use]
    pub const fn as_ref(&self) -> &uuid::Uuid {
        let Self { uuid } = self;
        uuid
    }

    #[must_use]
    pub const fn is_nil(&self) -> bool {
        self.as_ref().is_nil()
    }

    #[cfg(all(feature = "std", feature = "v7"))]
    #[must_use]
    pub fn now_v7() -> Self {
        Self {
            uuid: uuid::Uuid::now_v7(),
        }
    }

    fn decode_ascii(input: &[u8]) -> Result<Self, DecodeError> {
        const DECODED_LEN: usize = 16;
        debug_assert_eq!(DECODED_LEN, uuid::Uuid::nil().as_bytes().len());
        if input.len() != Self::STR_LEN {
            return Err(DecodeInputError::Invalid.into());
        }
        let mut decode_buf = [0; DECODED_LEN];
        match Self::ENCODING.decode_mut(input, &mut decode_buf) {
            Ok(decode_len) => {
                debug_assert!(decode_len <= DECODED_LEN);
                if decode_len < DECODED_LEN {
                    return Err(DecodeInputError::Insufficient.into());
                }
            }
            Err(DecodePartial {
                #[cfg_attr(not(feature = "std"), expect(unused_variables))]
                error,
                read,
                written,
            }) => {
                debug_assert!(read <= input.len());
                debug_assert!(written <= decode_buf.len());
                #[cfg(feature = "std")]
                return Err(DecodeInputError::Superfluous(error).into());
                #[cfg(not(feature = "std"))]
                return Err(DecodeInputError::Superfluous.into());
            }
        }
        let uuid = uuid::Uuid::from_bytes(decode_buf);
        Ok(Self { uuid })
    }

    fn decode_str(input: &str) -> Result<Self, DecodeError> {
        Self::decode_ascii(input.as_bytes())
    }

    #[must_use]
    const fn encode_buf() -> [u8; Self::STR_LEN] {
        [Self::ENCODING_PAD_CHAR; Self::STR_LEN]
    }

    #[must_use]
    fn encode_str_impl<'a>(&self, output: &'a mut [u8; Self::STR_LEN]) -> &'a str {
        let Self { uuid } = self;
        let uuid_bytes = uuid.as_bytes();
        let encoded_str = Self::ENCODING.encode_mut_str(uuid_bytes, output);
        debug_assert_eq!(encoded_str.len(), Self::STR_LEN);
        encoded_str
    }

    #[must_use]
    pub fn encode_str(&self) -> UuidEncodedStr {
        let mut encode_buf = Self::encode_buf();
        let encode_len = self.encode_str_impl(&mut encode_buf).len();
        debug_assert_eq!(encode_buf.len(), encode_len);
        debug_assert!(encode_buf.is_ascii());
        UuidEncodedStr {
            ascii_chars: encode_buf,
        }
    }
}

#[derive(Debug, Display, Error, From)]
#[repr(transparent)]
pub struct DecodeError(DecodeInputError);

#[derive(Debug, Display, Error)]
enum DecodeInputError {
    #[display("invalid input")]
    Invalid,
    #[display("insufficient input")]
    Insufficient,
    #[cfg(feature = "std")]
    #[display("superfluous input: {_0:#}")]
    Superfluous(data_encoding::DecodeError),
    #[cfg(not(feature = "std"))]
    #[display("superfluous input")]
    Superfluous,
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UuidEncodedStr::encode(self).fmt(f)
    }
}

impl std::str::FromStr for Uuid {
    type Err = DecodeError;

    fn from_str(encoded: &str) -> Result<Self, Self::Err> {
        Self::decode_str(encoded)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(UuidDeserializeFromStr)
    }
}

#[cfg(feature = "serde")]
struct UuidDeserializeFromStr;

#[cfg(feature = "serde")]
impl serde::de::Visitor<'_> for UuidDeserializeFromStr {
    type Value = Uuid;

    fn visit_str<E>(self, input: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        input
            .parse()
            .map_err(|_| serde::de::Error::invalid_value(serde::de::Unexpected::Str(input), &self))
    }

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a base32hex-encoded UUID")
    }
}

/// Stringified [`Uuid`].
///
/// Only supposed to be used as a temporary in-memory representation
/// without support for serialization. Decoding the string back into
/// a [`Uuid`] is infallible.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UuidEncodedStr {
    ascii_chars: [u8; Uuid::STR_LEN],
}

impl UuidEncodedStr {
    pub const NIL: Self = Self {
        ascii_chars: [b'0'; Uuid::STR_LEN],
    };

    #[must_use]
    #[expect(unsafe_code, reason = "ASCII characters of base32hex serialization.")]
    pub const fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.ascii_chars) }
    }

    #[must_use]
    pub fn encode(uuid: &Uuid) -> Self {
        uuid.encode_str()
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc, reason = "Infallible.")]
    pub fn decode(&self) -> Uuid {
        Uuid::decode_ascii(&self.ascii_chars).unwrap()
    }
}

impl Default for UuidEncodedStr {
    fn default() -> Self {
        Self::NIL
    }
}

impl From<Uuid> for UuidEncodedStr {
    fn from(from: Uuid) -> Self {
        Self::encode(&from)
    }
}

impl From<UuidEncodedStr> for Uuid {
    fn from(from: UuidEncodedStr) -> Self {
        from.decode()
    }
}

impl AsRef<str> for UuidEncodedStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::ops::Deref for UuidEncodedStr {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for UuidEncodedStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(test)]
mod tests;
