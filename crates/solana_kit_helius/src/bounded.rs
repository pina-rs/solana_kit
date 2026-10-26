//! Optional per-operation item/string budgets supplement the shared RPC frame limits.

use serde::{Serialize, Serializer};
use solana_kit_client::rest::{Budgets, JsonError};
use solana_kit_client::{Error, methods::Method};

/// A typed RPC operation with explicit request/result JSON budgets.
///
/// Client frame byte/depth limits still apply. This wrapper additionally bounds
/// individual strings and total JSON values/object keys before typed allocation.
pub struct Bounded<M> {
    /// The real typed operation and decoder.
    pub method: M,
    /// Request parameters and success-result allocation budgets.
    pub budgets: Budgets,
}

impl<M: Serialize> Serialize for Bounded<M> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let body = self
            .budgets
            .serialize(&self.method)
            .map_err(|_| serde::ser::Error::custom("Helius request serialization budget failed"))?;
        let text = String::from_utf8(body)
            .map_err(|_| serde::ser::Error::custom("Helius serialized request was not UTF-8"))?;
        let raw =
            serde_json::value::RawValue::from_string(text).map_err(serde::ser::Error::custom)?;
        raw.serialize(serializer)
    }
}

impl<M: Method> Method for Bounded<M> {
    type Response = M::Response;
    const NAME: &'static str = M::NAME;

    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        self.budgets
            .validate_json(result.as_bytes())
            .map_err(rpc_error)?;
        self.method.decode(result)
    }
}

fn rpc_error(error: JsonError) -> Error {
    match error {
        JsonError::Bytes => Error::BodyLimit,
        JsonError::Depth => Error::DepthLimit,
        JsonError::Schema(error) => Error::Schema(error),
        _ => Error::Schema(<serde_json::Error as serde::de::Error>::custom(
            "Helius result item or string budget exceeded",
        )),
    }
}
