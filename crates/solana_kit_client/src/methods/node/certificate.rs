//! Alpenglow certificate JSON schema, without a cryptographic trust claim.
use serde::{Deserialize, Deserializer};

/// The genesis certificate returned by an Alpenglow-capable RPC node.
///
/// This validates the JSON shapes and byte widths only. It does not verify BLS
/// signatures, validator rank membership or consensus finality.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AlpenglowGenesisCertificate {
    /// Certified block identity and slot.
    pub block: CertifiedBlock,
    /// Aggregate signature and validator rank bitmap.
    pub signature: AggregateSignature,
}

/// Exact block identifier bytes and slot.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CertifiedBlock {
    /// 32-byte block identifier, represented as JSON byte numbers.
    pub block_id: [u8; 32],
    /// Certified block slot.
    pub slot: u64,
}

/// BLS aggregate bytes together with the rank bitmap.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AggregateSignature {
    /// Exactly 192 aggregate BLS signature bytes.
    #[serde(deserialize_with = "signature")]
    pub signature: [u8; 192],
    /// Rank bitmap bytes, bounded by the client's complete reply limit.
    pub bitmap: Vec<u8>,
}

fn signature<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 192], D::Error> {
    Vec::<u8>::deserialize(d)?.try_into().map_err(|_| {
        serde::de::Error::custom("aggregate BLS signature must contain exactly 192 bytes")
    })
}
