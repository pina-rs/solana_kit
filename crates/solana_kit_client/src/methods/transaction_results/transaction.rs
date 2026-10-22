//! Encoding-selected transaction views and version-aware envelopes.
use super::{
    AddressTableLookup, CompiledInstruction, ParsedAccount, ParsedOrPartialInstruction,
    TransactionMeta, addresses, parse, schema, signatures,
};
use crate::{Error, Presence};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use solana_kit_core::{Address, Hash, Signature};

/// Explicit transaction response encoding.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TransactionEncoding {
    /// Base58 wire bytes.
    Base58,
    /// Canonical padded standard base64 wire bytes.
    Base64,
    /// Index-based structured JSON.
    #[default]
    Json,
    /// Parsed JSON with typed fallback instructions.
    JsonParsed,
}
/// Reported transaction version. Unknown numeric versions are rejected.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum TransactionVersion {
    /// Unversioned transaction marker.
    Legacy(LegacyVersion),
    /// Current numbered message version, validated against request maximum.
    Number(u8),
}
/// Exact legacy marker.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum LegacyVersion {
    /// Legacy transaction.
    #[serde(rename = "legacy")]
    Legacy,
}
/// Header privilege counts in an unparsed JSON message.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeader {
    /// Required signatures.
    pub num_required_signatures: u8,
    /// Readonly signed accounts.
    pub num_readonly_signed_accounts: u8,
    /// Readonly unsigned accounts.
    pub num_readonly_unsigned_accounts: u8,
}
/// V1 execution configuration, present only when the node provides it.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionConfig {
    /// Whole-lamport priority fee.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub priority_fee: Option<u64>,
    /// Compute-unit limit.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub compute_unit_limit: Option<u32>,
    /// Loaded-account byte limit.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub loaded_accounts_data_size_limit: Option<u32>,
    /// Heap bytes.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub heap_size: Option<u32>,
}
/// Unparsed index-based JSON message, independently of core wire verification.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawMessage {
    /// Header privilege counts.
    pub header: MessageHeader,
    /// Static account addresses.
    #[serde(deserialize_with = "addresses")]
    pub account_keys: Vec<Address>,
    /// Blockhash or nonce bytes; no lifetime inference is made.
    #[serde(deserialize_with = "crate::wire::hash")]
    pub recent_blockhash: Hash,
    /// Compiled program invocations.
    pub instructions: Vec<CompiledInstruction>,
    /// Address lookup table references.
    #[serde(default)]
    pub address_table_lookups: Presence<Vec<AddressTableLookup>>,
    /// V1 execution configuration.
    #[serde(default)]
    pub transaction_config: Presence<TransactionConfig>,
}
/// JSON message with resolved accounts and parsed instruction routing.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedMessage {
    /// Resolved account privileges and sources.
    pub account_keys: Vec<ParsedAccount>,
    /// Blockhash or nonce bytes.
    #[serde(deserialize_with = "crate::wire::hash")]
    pub recent_blockhash: Hash,
    /// Parsed or partially decoded invocations.
    pub instructions: Vec<ParsedOrPartialInstruction>,
    /// Versioned lookup references.
    #[serde(default)]
    pub address_table_lookups: Presence<Vec<AddressTableLookup>>,
    /// V1 execution configuration.
    #[serde(default)]
    pub transaction_config: Presence<TransactionConfig>,
}
/// JSON transaction with an encoding-selected typed message.
#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "M: Deserialize<'de>"))]
pub struct JsonTransaction<M> {
    /// Validated Ed25519 signatures in transaction order.
    #[serde(deserialize_with = "signatures")]
    pub signatures: Vec<Signature>,
    /// Typed message view.
    pub message: M,
}
/// Accounts-only transaction view returned by getBlock.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountsTransaction {
    /// Validated transaction signatures.
    #[serde(deserialize_with = "signatures")]
    pub signatures: Vec<Signature>,
    /// Resolved account annotations.
    pub account_keys: Vec<ParsedAccount>,
}
/// Owned response representation selected and checked by the request.
#[derive(Debug)]
pub enum TransactionView {
    /// Opaque wire bytes with no packet-size or version inference.
    Binary(Vec<u8>),
    /// Index-based JSON message.
    Json(JsonTransaction<RawMessage>),
    /// Resolved parsed JSON message.
    JsonParsed(JsonTransaction<ParsedMessage>),
    /// Accounts-only view.
    Accounts(AccountsTransaction),
}
/// Transaction plus execution metadata and explicitly reported version.
#[derive(Debug)]
pub struct TransactionResult {
    /// Encoding-selected owned transaction view.
    pub transaction: TransactionView,
    /// Null for historical transactions without metadata.
    pub meta: Option<TransactionMeta>,
    /// Omitted for historical missing metadata or an omitted supported version.
    pub version: Presence<TransactionVersion>,
}
#[derive(Deserialize)]
pub(crate) struct WireTransactionResult {
    transaction: Box<RawValue>,
    #[serde(deserialize_with = "crate::json::required_nullable")]
    meta: Option<TransactionMeta>,
    #[serde(default)]
    version: Presence<TransactionVersion>,
}
/// The historical RPC serializers omit versions for transactions without metadata.
#[derive(Clone, Copy)]
pub(crate) enum VersionRequirement {
    RequiredWhenRequested,
    AllowHistoricalMissingMetadata,
}
impl WireTransactionResult {
    pub(crate) fn decode(
        self,
        encoding: TransactionEncoding,
        max: Option<u8>,
        accounts: bool,
        version_requirement: VersionRequirement,
    ) -> Result<TransactionResult, Error> {
        validate_version(&self.version, max, self.meta.as_ref(), version_requirement)?;
        let transaction = if accounts {
            TransactionView::Accounts(parse(self.transaction.get())?)
        } else {
            decode_transaction(self.transaction.get(), encoding)?
        };

        if let Some(meta) = &self.meta {
            validate_inner_encoding(meta, encoding, accounts)?;
            let success = matches!(
                meta.status,
                crate::methods::submission::ExecutionStatus::Ok(())
            );

            if success != meta.err.is_none() || meta.pre_balances.len() != meta.post_balances.len()
            {
                return Err(schema("contradictory transaction execution metadata"));
            }
        }

        Ok(TransactionResult {
            transaction,
            meta: self.meta,
            version: self.version,
        })
    }
}
fn validate_version(
    version: &Presence<TransactionVersion>,
    max: Option<u8>,
    meta: Option<&TransactionMeta>,
    requirement: VersionRequirement,
) -> Result<(), Error> {
    let requirement = if meta.is_some() {
        VersionRequirement::RequiredWhenRequested
    } else {
        requirement
    };

    match (version, max, requirement) {
        (Presence::Missing, None, _)
        | (Presence::Missing, Some(0..=1), VersionRequirement::AllowHistoricalMissingMetadata)
        | (Presence::Present(TransactionVersion::Legacy(_)), Some(0..=1), _) => Ok(()),

        (Presence::Present(TransactionVersion::Number(version)), Some(max), _)
            if *version <= max && *version <= 1 =>
        {
            Ok(())
        }

        _ => Err(schema("transaction version contradicts requested maximum")),
    }
}
fn decode_transaction(raw: &str, encoding: TransactionEncoding) -> Result<TransactionView, Error> {
    match encoding {
        TransactionEncoding::Json => parse(raw).map(TransactionView::Json),
        TransactionEncoding::JsonParsed => parse(raw).map(TransactionView::JsonParsed),
        TransactionEncoding::Base58 | TransactionEncoding::Base64 => {
            let (text, tag): (String, String) = parse(raw)?;
            let bytes = match (encoding, tag.as_str()) {
                (TransactionEncoding::Base58, "base58") => bs58::decode(text)
                    .into_vec()
                    .map_err(|_| schema("invalid base58 transaction"))?,
                (TransactionEncoding::Base64, "base64") => STANDARD
                    .decode(text)
                    .map_err(|_| schema("invalid base64 transaction"))?,
                _ => return Err(schema("transaction encoding contradicts request")),
            };

            Ok(TransactionView::Binary(bytes))
        }
    }
}

fn validate_inner_encoding(
    meta: &TransactionMeta,
    encoding: TransactionEncoding,
    accounts: bool,
) -> Result<(), Error> {
    if accounts {
        return Ok(());
    }

    let Presence::Present(Some(groups)) = &meta.inner_instructions else {
        return Ok(());
    };
    let parsed = encoding == TransactionEncoding::JsonParsed;
    let mismatch = groups
        .iter()
        .flat_map(|group| &group.instructions)
        .any(|instruction| {
            let resolved = matches!(instruction, super::InnerInstruction::Resolved(_));
            parsed != resolved
        });

    if mismatch {
        return Err(schema("inner instruction encoding contradicts request"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_version_contract_does_not_inherit_the_historical_exception() {
        let raw = r#"{"transaction":["AAE=","base64"],"meta":null}"#;

        for max in [None, Some(0), Some(1)] {
            let result = parse::<WireTransactionResult>(raw).unwrap().decode(
                TransactionEncoding::Base64,
                max,
                false,
                VersionRequirement::RequiredWhenRequested,
            );
            assert_eq!(result.is_ok(), max.is_none());
        }
    }
}
