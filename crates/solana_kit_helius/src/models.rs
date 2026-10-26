//! Declarative wire schemas keep spelling, optionality and public fields together.

mod object;
pub(crate) use object::deserialize as deserialize_object;

macro_rules! model {
    ($doc:literal, $name:ident {
        $($(#[$attribute:meta])* $field:ident: $ty:ty => $wire:literal),* $(,)?
    }
        optional { $($optional:ident: $optional_ty:ty => $optional_wire:literal),* $(,)? }) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, serde::Serialize)]
        pub struct $name {
            $(#[doc = concat!("The `", $wire, "` field.")]
            $(#[$attribute])*
            #[serde(rename = $wire)]
            pub $field: $ty,)*
            $(#[doc = concat!("The optional `", $optional_wire, "` field, retaining null.")]
            #[serde(rename = $optional_wire, default,
                skip_serializing_if = "crate::Field::is_missing")]
            pub $optional: crate::Field<$optional_ty>,)*
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    $($(#[$attribute])* #[serde(rename = $wire)] $field: $ty,)*
                    $(#[serde(rename = $optional_wire, default)]
                    $optional: crate::Field<$optional_ty>,)*
                }

                crate::models::deserialize_object::<D, Fields>(decoder).map(|fields| Self {
                    $($field: fields.$field,)*
                    $($optional: fields.$optional,)*
                })
            }
        }
    };
}

macro_rules! request {
    ($(#[$attribute:meta])* $doc:literal, $name:ident {
        $($field:ident: $ty:ty => $wire:literal),* $(,)? }
        optional { $($optional:ident: $optional_ty:ty => $optional_wire:literal),* $(,)? }) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        $(#[$attribute])*
        pub struct $name {
            $(#[doc = concat!("The `", $wire, "` parameter.")]
            #[serde(rename = $wire)]
            pub $field: $ty,)*
            $(#[doc = concat!("The optional `", $optional_wire, "` parameter; None omits it.")]
            #[serde(rename = $optional_wire, default, skip_serializing_if = "Option::is_none")]
            pub $optional: Option<$optional_ty>,)*
        }
    };
}

/// Two positional arguments, where the trailing configuration may be absent.
#[derive(serde::Deserialize)]
#[serde(bound(deserialize = "T: serde::Deserialize<'de>"))]
pub(crate) struct AddressArguments<T>(pub(crate) String, #[serde(default)] pub(crate) Option<T>);

/// Serializes one address and an optional trailing argument without allocating a JSON value.
pub(crate) fn serialize_address<S: serde::Serializer, T: serde::Serialize>(
    address: &str,
    config: Option<&T>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use serde::Serialize;

    match config {
        Some(config) => (address, config).serialize(serializer),
        None => [address].serialize(serializer),
    }
}

macro_rules! address_request {
    ($doc:literal, $name:ident, $address:ident, $config:ty) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            /// Address in the first positional argument.
            pub $address: String,
            /// Optional trailing configuration; None emits only the address.
            pub config: Option<$config>,
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
                let crate::models::AddressArguments(address, config) =
                    serde::Deserialize::deserialize(decoder)?;

                Ok(Self {
                    $address: address,
                    config,
                })
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                crate::models::serialize_address(&self.$address, self.config.as_ref(), serializer)
            }
        }
    };
}

macro_rules! method {
    ($name:ty, $wire:literal, $response:ty, object) => {
        impl solana_kit_client::methods::Method for $name {
            type Response = $response;
            const NAME: &'static str = $wire;

            fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
                crate::models::decode_object(result)
            }
        }
    };
    ($name:ty, $wire:literal, $response:ty, $check:expr) => {
        impl solana_kit_client::methods::Method for $name {
            type Response = $response;
            const NAME: &'static str = $wire;

            fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
                let check: fn(&Self, &Self::Response) -> bool = $check;
                crate::models::decode_checked(result, |response| check(self, response))
            }
        }
    };
    ($name:ty, $wire:literal, $response:ty) => {
        impl solana_kit_client::methods::Method for $name {
            type Response = $response;
            const NAME: &'static str = $wire;

            fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
                serde_json::from_str(result).map_err(solana_kit_client::Error::Schema)
            }
        }
    };
}

/// Rejects sequence representations for provider results whose wire contract is an object.
pub(crate) fn decode_object<Output: serde::de::DeserializeOwned>(
    result: &str,
) -> Result<Output, solana_kit_client::Error> {
    if !result.trim_start().starts_with('{') {
        return Err(solana_kit_client::Error::Schema(
            <serde_json::Error as serde::de::Error>::custom("expected a provider result object"),
        ));
    }

    serde_json::from_str(result).map_err(solana_kit_client::Error::Schema)
}

/// Applies request-dependent response checks before exposing a typed provider result.
pub(crate) fn decode_checked<Output: serde::de::DeserializeOwned>(
    result: &str,
    check: impl FnOnce(&Output) -> bool,
) -> Result<Output, solana_kit_client::Error> {
    checked_json(result.as_bytes(), check).map_err(solana_kit_client::Error::Schema)
}

/// Checks request-dependent identities after typed decoding for either RPC or REST.
pub(crate) fn checked_json<Output: serde::de::DeserializeOwned>(
    result: &[u8],
    check: impl FnOnce(&Output) -> bool,
) -> Result<Output, serde_json::Error> {
    let response = serde_json::from_slice(result)?;

    if !check(&response) {
        return Err(<serde_json::Error as serde::de::Error>::custom(
            "provider response does not match its requested entries",
        ));
    }

    Ok(response)
}

macro_rules! rest {
    ($name:ty, $response:ty, $service:ident, $verb:ident, $body:ident,
        |$value:ident| $path:expr) => {
        impl crate::service::Operation for $name {
            type Response = $response;
            const SERVICE: crate::service::Service = crate::service::Service::$service;
            const VERB: crate::service::Verb = crate::service::Verb::$verb;
            const BODY: crate::service::Body = crate::service::Body::$body;

            fn path(&self) -> Vec<String> {
                let $value = self;
                $path
            }
        }
    };
}
