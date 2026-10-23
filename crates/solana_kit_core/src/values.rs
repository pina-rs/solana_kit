//! Kit-owned byte values with canonical base58 display.

use core::{fmt, str::FromStr};

use crate::{ParseError, encoding};

/// Defines the same encoding contract without giving domain values interchangeable types.
macro_rules! fixed_value {
    ($name:ident, $size:literal, $maximum:literal, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name([u8; $size]);

        impl $name {
            /// Constructs a value from exactly the required bytes.
            #[must_use]
            pub const fn from_bytes(bytes: [u8; $size]) -> Self {
                Self(bytes)
            }

            /// Borrows the underlying fixed-byte representation.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; $size] {
                &self.0
            }

            /// Returns the underlying bytes by value.
            #[must_use]
            pub const fn to_bytes(self) -> [u8; $size] {
                self.0
            }
        }

        impl From<[u8; $size]> for $name {
            fn from(bytes: [u8; $size]) -> Self {
                Self(bytes)
            }
        }

        impl TryFrom<&[u8]> for $name {
            type Error = ParseError;

            fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
                encoding::from_slice(bytes, encoding::ValueKind::$name).map(Self)
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl FromStr for $name {
            type Err = ParseError;

            fn from_str(text: &str) -> Result<Self, Self::Err> {
                encoding::decode(text, $maximum, encoding::ValueKind::$name).map(Self)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&bs58::encode(self.0).into_string())
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}({self})", stringify!($name))
            }
        }
    };
}

fixed_value!(
    Address,
    32,
    44,
    "A 32-byte Solana account or program address."
);
fixed_value!(
    Hash,
    32,
    44,
    "A 32-byte hash, including a Solana blockhash."
);
fixed_value!(
    Signature,
    64,
    88,
    "A 64-byte Ed25519 signature; verification is a separate operation."
);
