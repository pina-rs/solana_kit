//! BPF upgradeable loader payloads.
//!
//! Source:
//! `packages/solana_kit_rpc_parsed_types/lib/src/bpf_upgradeable_loader_accounts.dart`.
//! The pinned model declares only `program` and `programData`, so the node's
//! `uninitialized` and `buffer` discriminators are retained as unknown payloads.

use super::{ParsedAccountData, scalars};
use crate::Error;
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;
use solana_kit_core::Address;

pub(super) fn decode(program: &str, parsed: &RawValue) -> Result<ParsedAccountData, Error> {
    let envelope = super::envelope(parsed)?;

    let account = match envelope.type_name {
        "program" => BpfUpgradeableLoaderProgramAccount::Program(super::info(&envelope)?),
        "programData" => BpfUpgradeableLoaderProgramAccount::ProgramData(super::info(&envelope)?),
        other => return super::unknown_type(program, other, parsed),
    };

    Ok(ParsedAccountData::BpfUpgradeableLoader(account))
}

/// Reference `JsonParsedBpfUpgradeableLoaderProgramAccount`.
#[derive(Debug, PartialEq)]
pub enum BpfUpgradeableLoaderProgramAccount {
    /// `type: "program"`.
    Program(BpfProgramInfo),
    /// `type: "programData"`.
    ProgramData(BpfProgramDataInfo),
}

/// Reference `JsonParsedBpfProgramInfo`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BpfProgramInfo {
    /// Address of the program data account.
    #[serde(deserialize_with = "scalars::required_address")]
    pub program_data: Address,
}

/// Reference `JsonParsedBpfProgramDataInfo`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BpfProgramDataInfo {
    /// Optional upgrade authority.
    #[serde(default, deserialize_with = "scalars::optional_address")]
    pub authority: Option<Address>,
    /// Encoded program bytes exactly as the node reported them.
    pub data: Base64EncodedData,
    /// Slot of the last deploy or upgrade.
    pub slot: u64,
}

/// A two-member encoded data response, matching `Base64EncodedDataResponse`.
///
/// The base64 text stays encoded, exactly like the reference
/// `Base64EncodedBytes` branded string; this projection never decodes or
/// re-encodes executable bytes.
#[derive(Debug, PartialEq, Eq)]
pub struct Base64EncodedData {
    /// Exact base64 text reported by the node.
    pub data: String,
    /// Reported encoding label, which must be `base64`.
    pub encoding: Base64Encoding,
}

impl<'de> Deserialize<'de> for Base64EncodedData {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (data, encoding) = <(String, Base64Encoding)>::deserialize(deserializer)?;

        Ok(Self { data, encoding })
    }
}

/// The only encoding the reference `programData` payload declares.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Base64Encoding {
    /// Canonical padded base64 text.
    #[serde(rename = "base64")]
    Base64,
}
