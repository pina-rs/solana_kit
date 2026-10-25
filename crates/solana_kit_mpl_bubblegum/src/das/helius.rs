//! Helius DAS uses the existing typed JSON-RPC client and its bounded transport.
use super::{
    DasApi, DasAsset, DasAssetProof, DasError,
    response::{RawAsset, RawProof},
};
use alloc::string::ToString;
use serde::{Serialize, Serializer};
use solana_kit_client::{Client, Error, Transport, methods::Method};
use solana_kit_core::{Address, codecs::Limits};

struct GetAsset {
    id: Address,
    limits: Limits,
}
struct GetAssetProof {
    id: Address,
    limits: Limits,
}

macro_rules! method {
    ($request:ident, $response:ty, $name:literal) => {
        impl Serialize for $request {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                [self.id.to_string()].serialize(serializer)
            }
        }

        impl Method for $request {
            type Response = $response;
            const NAME: &'static str = $name;
            fn decode(&self, result: &str) -> Result<Self::Response, Error> {
                decode(result, self.limits)
            }
        }
    };
}

fn decode<R: serde::de::DeserializeOwned>(result: &str, limits: Limits) -> Result<R, Error> {
    if result.len() > limits.max_bytes {
        return Err(Error::BodyLimit);
    }

    serde_json::from_str(result).map_err(Error::Schema)
}

method!(GetAsset, RawAsset, "getAsset");
method!(GetAssetProof, RawProof, "getAssetProof");

/// Caller-owned Helius-compatible DAS client. Endpoints, credentials and deadlines live in T.
pub struct HeliusDasClient<T> {
    client: Client<T>,
    limits: Limits,
}

impl<T: Transport> HeliusDasClient<T> {
    /// Reuses an already configured RPC owner and separate normalized-value budgets.
    #[must_use]
    pub const fn new(client: Client<T>, limits: Limits) -> Self {
        Self { client, limits }
    }

    /// Returns ownership of the RPC client and its transport namespace.
    #[must_use]
    pub fn into_client(self) -> Client<T> {
        self.client
    }
}

impl<T: Transport> DasApi for HeliusDasClient<T> {
    type Error = DasError<Error>;

    async fn get_asset(&self, id: Address) -> Result<DasAsset, Self::Error> {
        let value = self
            .client
            .call(GetAsset {
                id,
                limits: self.limits,
            })
            .await
            .map_err(DasError::Provider)?
            .normalize(self.limits)
            .map_err(DasError::Data)?;

        if value.id != id {
            return Err(DasError::Data(solana_kit_core::codecs::Error::InvalidValue));
        }

        Ok(value)
    }

    async fn get_asset_proof(&self, id: Address) -> Result<DasAssetProof, Self::Error> {
        self.client
            .call(GetAssetProof {
                id,
                limits: self.limits,
            })
            .await
            .map_err(DasError::Provider)?
            .normalize(self.limits)
            .map_err(DasError::Data)
    }
}
