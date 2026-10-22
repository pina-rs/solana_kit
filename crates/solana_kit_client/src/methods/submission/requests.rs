//! Typed submission requests keep wire encoding explicit.
use super::SignatureStatus;
use crate::methods::history::{parse, signature};
use crate::{
    Error,
    methods::{Commitment, Contextual, Method, ReadConfig},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};
use solana_kit_core::{Address, Signature};

/// Wire encoding selected by the caller; independent of message version.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WireEncoding {
    /// Bitcoin base58 alphabet.
    Base58,
    /// Canonical padded standard base64.
    Base64,
}

/// Submits opaque wire bytes without imposing a legacy packet-size limit.
///
/// The node validates the transaction and version. The caller owns signing and
/// lifetime metadata; no decode or re-signing occurs in this operation.
#[derive(Debug)]
pub struct SendTransaction<'a> {
    /// Exact signed wire bytes, including any supported V1 envelope.
    pub wire: &'a [u8],
    /// Explicit canonical wire encoding.
    pub encoding: WireEncoding,
    /// Optional node preflight/retry policy; the client itself never retries.
    pub config: SendConfig,
}

/// Node-side submission options; absent fields preserve node defaults.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SendConfig {
    /// Skip the node's preflight verification/simulation when true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_preflight: Option<bool>,
    /// Commitment for the node's preflight simulation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preflight_commitment: Option<Commitment>,
    /// Maximum node-side retries, encoded as an exact integer on all targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u64>,
    /// Minimum evaluation slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}
impl Serialize for SendTransaction<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Config<'a> {
            encoding: WireEncoding,
            #[serde(flatten)]
            options: &'a SendConfig,
        }
        let encoded = match self.encoding {
            WireEncoding::Base58 => bs58::encode(self.wire).into_string(),
            WireEncoding::Base64 => STANDARD.encode(self.wire),
        };
        (
            encoded,
            Config {
                encoding: self.encoding,
                options: &self.config,
            },
        )
            .serialize(s)
    }
}
impl Method for SendTransaction<'_> {
    type Response = Signature;
    const NAME: &'static str = "sendTransaction";
    fn decode(&self, json: &str) -> Result<Self::Response, Error> {
        decode_signature(json)
    }
}
fn decode_signature(json: &str) -> Result<Signature, Error> {
    #[derive(Deserialize)]
    struct Wire(#[serde(deserialize_with = "signature")] Signature);
    parse::<Wire>(json).map(|wire| wire.0)
}

/// Estimates fees for the exact message bytes encoded canonically as base64.
#[derive(Debug)]
pub struct FeeForMessage<'a>(pub &'a [u8], pub Option<ReadConfig>);
impl Serialize for FeeForMessage<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(None)?;
        seq.serialize_element(&STANDARD.encode(self.0))?;

        if let Some(config) = self.1 {
            seq.serialize_element(&config)?;
        }

        seq.end()
    }
}
impl Method for FeeForMessage<'_> {
    type Response = Contextual<Option<u64>>;
    const NAME: &'static str = "getFeeForMessage";
    fn decode(&self, json: &str) -> Result<Self::Response, Error> {
        parse::<crate::wire::NullableContext<u64>>(json).map(|wire| Contextual {
            context: wire.context,
            value: wire.value,
        })
    }
}

/// Optional commitment for an airdrop request.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct AirdropConfig {
    /// Required commitment, or node default when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
}

/// Requests an airdrop; never automatically retried by the client.
#[derive(Debug)]
pub struct RequestAirdrop(pub Address, pub u64, pub Option<AirdropConfig>);
impl Serialize for RequestAirdrop {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(None)?;
        seq.serialize_element(&self.0.to_string())?;
        seq.serialize_element(&self.1)?;

        if let Some(config) = self.2 {
            seq.serialize_element(&config)?;
        }

        seq.end()
    }
}
impl Method for RequestAirdrop {
    type Response = Signature;
    const NAME: &'static str = "requestAirdrop";
    fn decode(&self, json: &str) -> Result<Self::Response, Error> {
        decode_signature(json)
    }
}

/// Optional historical-storage search for signature statuses.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StatusesConfig {
    /// False stays explicit; absence preserves the node default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_transaction_history: Option<bool>,
}

/// Reads positional statuses for at most 256 validated signatures.
#[derive(Debug)]
pub struct SignatureStatuses<'a>(pub &'a [Signature], pub Option<StatusesConfig>);
impl Serialize for SignatureStatuses<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self.0.len() > 256 {
            return Err(serde::ser::Error::custom(
                "at most 256 signatures are allowed",
            ));
        }

        let mut seq = s.serialize_seq(None)?;
        seq.serialize_element(&self.0.iter().map(ToString::to_string).collect::<Vec<_>>())?;

        if let Some(config) = self.1 {
            seq.serialize_element(&config)?;
        }

        seq.end()
    }
}
impl Method for SignatureStatuses<'_> {
    type Response = Contextual<Vec<Option<SignatureStatus>>>;
    const NAME: &'static str = "getSignatureStatuses";
    fn decode(&self, json: &str) -> Result<Self::Response, Error> {
        let result: Self::Response = parse(json)?;

        if result.value.len() != self.0.len() {
            return Err(super::responses::schema(
                "status result cardinality differs from request",
            ));
        }

        for status in result.value.iter().flatten() {
            status.validate()?;
        }

        Ok(result)
    }
}
