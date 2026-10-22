//! Simulation reports execution; it never signs, sends, retries or confirms.
use super::{
    Commitment, Contextual, Method, ReadConfig,
    accounts::{AccountConfig, AccountData, AccountEncoding, data::WireAccount},
    submission::{TransactionFailure, WireEncoding},
    transaction_results::{
        InnerInstructions, LoadedAddresses, ReturnData, TokenBalance, parse, schema,
    },
};
use crate::{Error, Presence};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize, Serializer};
use solana_kit_core::{Address, Hash, accounts::MaybeAccount};

/// Simulated account representation. Base58 account responses are unsupported by the node.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SimulationAccountEncoding {
    /// Canonical base64 data.
    #[default]
    Base64,
    /// Base64-wrapped compressed bytes; no decompression is implied.
    #[serde(rename = "base64+zstd")]
    Base64Zstd,
    /// Parsed program data with base64 fallback.
    JsonParsed,
}
impl SimulationAccountEncoding {
    fn account(self) -> AccountEncoding {
        match self {
            Self::Base64 => AccountEncoding::Base64,
            Self::Base64Zstd => AccountEncoding::Base64Zstd,
            Self::JsonParsed => AccountEncoding::JsonParsed,
        }
    }
}
/// Requested post-simulation account snapshots.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SimulationAccounts {
    /// Positional identities retained in the response, including null entries.
    pub addresses: Vec<Address>,
    /// Omission preserves the node's base64 default.
    pub encoding: Option<SimulationAccountEncoding>,
}
impl Serialize for SimulationAccounts {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wire {
            addresses: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            encoding: Option<SimulationAccountEncoding>,
        }
        Wire {
            addresses: self.addresses.iter().map(ToString::to_string).collect(),
            encoding: self.encoding,
        }
        .serialize(s)
    }
}
/// Complete simulation options. Conflicting signature/blockhash flags fail before exchange.
#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SimulationConfig {
    /// Confirmation level used for simulation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Minimum evaluation slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    /// Request recorded inner calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_instructions: Option<bool>,
    /// Verify the supplied transaction signatures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sig_verify: Option<bool>,
    /// Replace the supplied recent blockhash; incompatible with signature verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_recent_blockhash: Option<bool>,
    /// Optional post-simulation account snapshots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<SimulationAccounts>,
}
impl SimulationConfig {
    fn validate(&self) -> Result<(), Error> {
        if self.sig_verify == Some(true) && self.replace_recent_blockhash == Some(true) {
            return Err(schema("sigVerify conflicts with replaceRecentBlockhash"));
        }

        Ok(())
    }
}
/// Simulates opaque caller-owned wire bytes without a legacy packet-size assumption.
#[derive(Debug)]
pub struct SimulateTransaction<'a> {
    /// Signed or unsigned bytes as required by the selected node verification policy.
    pub wire: &'a [u8],
    /// Explicit transaction encoding, independent of account encoding.
    pub encoding: WireEncoding,
    /// Optional node execution policies.
    pub config: SimulationConfig,
}
impl Serialize for SimulateTransaction<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wire<'a> {
            encoding: WireEncoding,
            #[serde(flatten)]
            options: &'a SimulationConfig,
        }
        self.config.validate().map_err(serde::ser::Error::custom)?;

        let bytes = match self.encoding {
            WireEncoding::Base58 => bs58::encode(self.wire).into_string(),
            WireEncoding::Base64 => STANDARD.encode(self.wire),
        };

        (
            bytes,
            Wire {
                encoding: self.encoding,
                options: &self.config,
            },
        )
            .serialize(s)
    }
}
/// Replacement blockhash lifetime explicitly returned by the node.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacementBlockhash {
    /// Replacement hash; no client-derived expiry is inferred.
    #[serde(deserialize_with = "crate::wire::hash")]
    pub blockhash: Hash,
    /// Last valid block height reported by the node.
    pub last_valid_block_height: u64,
}
/// Typed execution fields. Required nullable members reject absence.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulationExecution {
    /// Execution failure, distinct from remote JSON-RPC failure.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub err: Option<TransactionFailure>,
    /// Fee the transaction would pay.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub fee: Option<u64>,
    /// Versioned loaded-account byte count.
    #[serde(default)]
    pub loaded_accounts_data_size: Presence<Option<u32>>,
    /// Lookup addresses, null if unavailable.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub loaded_addresses: Option<LoadedAddresses>,
    /// Logs, null when execution did not reach logging.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub logs: Option<Vec<String>>,
    /// Exact lamport balances before execution.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub pre_balances: Option<Vec<u64>>,
    /// Exact lamport balances after execution.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub post_balances: Option<Vec<u64>>,
    /// Token metadata before execution.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub pre_token_balances: Option<Vec<TokenBalance>>,
    /// Token metadata after execution.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub post_token_balances: Option<Vec<TokenBalance>>,
    /// Last program return value, or null.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub return_data: Option<ReturnData>,
    /// Replacement hash metadata, or null when not requested.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub replacement_blockhash: Option<ReplacementBlockhash>,
    /// Versioned actual compute usage.
    #[serde(default)]
    pub units_consumed: Presence<Option<u64>>,
    /// Recorded inner calls are versioned and may be absent or null.
    #[serde(default)]
    pub inner_instructions: Presence<Option<Vec<InnerInstructions>>>,
}
/// Simulation result retaining requested positional account identities.
#[derive(Debug)]
pub struct SimulationResult {
    /// Typed execution metadata.
    pub execution: SimulationExecution,
    /// Null when not requested or execution cannot produce snapshots.
    pub accounts: Option<Vec<MaybeAccount<AccountData>>>,
}
#[derive(Deserialize)]
struct WireValue {
    #[serde(deserialize_with = "crate::json::required_nullable")]
    accounts: Option<Vec<Option<WireAccount>>>,
}
impl Method for SimulateTransaction<'_> {
    type Response = Contextual<SimulationResult>;
    const NAME: &'static str = "simulateTransaction";
    fn decode(&self, json: &str) -> Result<Self::Response, Error> {
        self.config.validate()?;
        let wire: Contextual<WireValue> = parse(json)?;
        #[derive(Deserialize)]
        struct Envelope {
            value: SimulationExecution,
        }
        let execution = parse::<Envelope>(json)?.value;
        validate_execution(&execution, &self.config)?;
        let accounts = decode_accounts(wire.value.accounts, self.config.accounts.as_ref())?;

        Ok(Contextual {
            context: wire.context,
            value: SimulationResult {
                execution,
                accounts,
            },
        })
    }
}
// Node policy changes which fields may be populated; historical optional fields
// remain distinguishable from an explicit failure-time null.
fn validate_execution(value: &SimulationExecution, config: &SimulationConfig) -> Result<(), Error> {
    if value.replacement_blockhash.is_some() != config.replace_recent_blockhash.unwrap_or(false) {
        return Err(schema("replacement blockhash contradicts request"));
    }

    if let (Some(pre), Some(post)) = (&value.pre_balances, &value.post_balances)
        && pre.len() != post.len()
    {
        return Err(schema("simulation balance cardinality differs"));
    }

    if config.inner_instructions != Some(true)
        && matches!(value.inner_instructions, Presence::Present(Some(_)))
    {
        return Err(schema("unrequested simulation inner instructions"));
    }

    Ok(())
}
fn decode_accounts(
    wire: Option<Vec<Option<WireAccount>>>,
    requested: Option<&SimulationAccounts>,
) -> Result<Option<Vec<MaybeAccount<AccountData>>>, Error> {
    let Some(wire) = wire else {
        return Ok(None);
    };
    let config = requested.ok_or_else(|| schema("unrequested simulation accounts"))?;

    if wire.len() != config.addresses.len() {
        return Err(schema("simulation account cardinality differs"));
    }

    let account_config = AccountConfig {
        read: ReadConfig::default(),
        encoding: Some(config.encoding.unwrap_or_default().account()),
        data_slice: None,
    };
    wire.into_iter()
        .zip(&config.addresses)
        .map(|(account, address)| {
            account.map_or(Ok(MaybeAccount::Missing(*address)), |wire| {
                wire.owned(*address, account_config)
                    .map(MaybeAccount::Present)
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}
