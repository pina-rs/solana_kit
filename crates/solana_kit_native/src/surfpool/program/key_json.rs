//! Fixed-size key decoding never creates a generic JSON tree or secret string.

use serde::{
    Deserialize, Deserializer,
    de::{self, SeqAccess, Visitor},
};
use serde_json::value::RawValue;
use std::fmt;
use zeroize::Zeroizing;

pub(super) struct KeyBytes(pub Zeroizing<[u8; 64]>);

struct KeyByte(u8);

impl<'de> Deserialize<'de> for KeyByte {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = <&RawValue>::deserialize(deserializer)?;

        // JSON's integer negative zero is zero, as with the original byte visitor.
        let byte = if raw.get() == "-0" {
            Ok(0)
        } else {
            raw.get().parse::<u8>()
        };

        byte.map(Self)
            .map_err(|_| de::Error::custom("keypair element must be an integer in 0..255"))
    }
}

impl<'de> Deserialize<'de> for KeyBytes {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = <&RawValue>::deserialize(deserializer)?;

        if !raw.get().starts_with('[') {
            return Err(de::Error::custom("keypair JSON must be a byte array"));
        }

        serde_json::Deserializer::from_str(raw.get())
            .deserialize_seq(KeyVisitor)
            .map_err(de::Error::custom)
    }
}

struct KeyVisitor;

impl<'de> Visitor<'de> for KeyVisitor {
    type Value = KeyBytes;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("exactly 64 unsigned byte integers")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<KeyBytes, A::Error> {
        let mut bytes = Zeroizing::new([0; 64]);

        for (index, byte) in bytes.iter_mut().enumerate() {
            *byte = sequence
                .next_element::<KeyByte>()?
                .ok_or_else(|| de::Error::invalid_length(index, &self))?
                .0;
        }

        if sequence.next_element::<de::IgnoredAny>()?.is_some() {
            return Err(de::Error::invalid_length(bytes.len() + 1, &self));
        }

        Ok(KeyBytes(bytes))
    }
}
