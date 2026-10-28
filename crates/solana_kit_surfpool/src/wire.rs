//! Shared positional encoding and typed decoding without a second RPC envelope.

use serde::{Serialize, Serializer, ser::SerializeSeq};

/// Writes a required positional argument and an optional trailing argument.
pub(crate) fn trailing<S: Serializer, First: Serialize, Last: Serialize>(
    first: First,
    last: Option<&Last>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut sequence = serializer.serialize_seq(Some(1 + usize::from(last.is_some())))?;
    sequence.serialize_element(&first)?;

    if let Some(last) = last {
        sequence.serialize_element(last)?;
    }

    sequence.end()
}

/// Writes zero or one positional argument without introducing a null placeholder.
pub(crate) fn optional<S: Serializer, Value: Serialize>(
    value: Option<&Value>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut sequence = serializer.serialize_seq(Some(usize::from(value.is_some())))?;

    if let Some(value) = value {
        sequence.serialize_element(value)?;
    }

    sequence.end()
}

macro_rules! method {
    ($name:ty, $wire:literal, $response:ty) => {
        impl solana_kit_client::methods::Method for $name {
            type Response = crate::Response<$response>;
            const NAME: &'static str = $wire;

            fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
                crate::response::decode(result).map_err(solana_kit_client::Error::Schema)
            }
        }
    };
}

/// Rejects sequence representations before deriving field-level object schemas.
pub(crate) fn object<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    use serde::Deserialize;
    let raw = Box::<serde_json::value::RawValue>::deserialize(deserializer)?;

    if !raw.get().trim_start().starts_with('{') {
        return Err(serde::de::Error::custom(
            "expected a Surfpool result object",
        ));
    }

    serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)
}

macro_rules! object_model {
    ($(#[$outer:meta])* pub struct $name:ident {
        $($(#[$attribute:meta])* pub $field:ident: $ty:ty),* $(,)?
    }) => {
        $(#[$outer])*
        #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            $($(#[$attribute])* pub $field: $ty,)*
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                #[derive(serde::Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct Fields {
                    $($(#[$attribute])* $field: $ty,)*
                }
                crate::wire::object::<D, Fields>(deserializer).map(|fields| Self {
                    $($field: fields.$field,)*
                })
            }
        }
    };
}

macro_rules! empty {
    ($name:ident, $wire:literal, $response:ty, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $name;

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                <[u8; 0] as serde::Serialize>::serialize(&[], serializer)
            }
        }

        method!($name, $wire, $response);
    };
}
