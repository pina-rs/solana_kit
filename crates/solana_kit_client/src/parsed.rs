//! Bounded typed projection of node `jsonParsed` account payloads.
//!
//! The Solana node may answer a `jsonParsed` account request with program-defined
//! JSON, or with annotated base64 when it cannot parse the account. This module
//! projects that program-defined JSON into the reference account families without
//! touching the encoded fallback: [`crate::methods::accounts::AccountData::Bytes`]
//! and [`crate::methods::accounts::AccountData::Zstd`] stay exactly as returned.
//!
//! The node's `program` member is a parser name. It selects a payload family as
//! data and is never treated as proof that the account is owned by that program;
//! ownership policy remains the caller's decision.
//!
//! Supported families mirror the pinned Dart
//! `solana_kit_rpc_parsed_types` declarations at revision
//! `0138bb956d2d81ccc0c56edc602401c0df2f2b46`:
//!
//! | Node parser name        | Projected variants                                                     |
//! | ----------------------- | ---------------------------------------------------------------------- |
//! | `spl-token`, `spl-token-2022` | [`TokenProgramAccount`]: account, mint, multisig                 |
//! | `stake`                 | [`StakeProgramAccount`]: delegated, initialized                        |
//! | `vote`                  | [`VoteInfo`]                                                           |
//! | `nonce`                 | [`NonceInfo`]                                                          |
//! | `address-lookup-table`  | [`AddressLookupTableInfo`]                                             |
//! | `bpf-upgradeable-loader`| [`BpfUpgradeableLoaderProgramAccount`]: program, programData           |
//! | `config`                | [`ConfigProgramAccount`]: validatorInfo, legacy stakeConfig            |
//! | `sysvar`                | [`SysvarAccount`]: ten declared sysvar variants                        |
//!
//! Every other parser name becomes [`ParsedAccountData::UnknownProgram`]. A
//! supported parser name with a discriminator outside the reference model (for
//! example an uninitialized account or the `rewards` sysvar) becomes
//! [`ParsedAccountData::UnknownType`]. Both retain the complete original
//! `parsed` JSON value exactly, and the source [`crate::methods::accounts::ParsedAccount`]
//! is borrowed, so the caller always keeps the node payload unchanged.
//!
//! Decoding is bounded before typed allocation: [`decode`] applies the existing
//! [`crate::Limits`] byte and nesting admission, plus duplicate-key rejection, to
//! the exact payload first. Amounts reported as decimal text stay exact integers;
//! no numeric field passes through a floating-point conversion.

use crate::{Error, Limits};
use serde::Deserialize;
use serde_json::value::RawValue;

mod scalars;

mod address_lookup_table;
mod config;
mod loader;
mod nonce;
mod stake;
mod sysvar;
mod token;
mod vote;

pub use address_lookup_table::AddressLookupTableInfo;
pub use config::{ConfigProgramAccount, StakeConfigInfo, ValidatorInfoData, ValidatorInfoKey};
pub use loader::{
    Base64EncodedData, Base64Encoding, BpfProgramDataInfo, BpfProgramInfo,
    BpfUpgradeableLoaderProgramAccount,
};
pub use nonce::NonceInfo;
pub use stake::{
    StakeAccountInfo, StakeAuthorized, StakeData, StakeDelegation, StakeLockup, StakeMeta,
    StakeProgramAccount,
};
pub use sysvar::{
    ClockInfo, EpochRewardsInfo, EpochScheduleInfo, FeesInfo, LastRestartSlotInfo,
    RecentBlockhashEntry, RentInfo, SlotHashEntry, SlotHistoryInfo, StakeHistoryData,
    StakeHistoryEntry, SysvarAccount,
};
pub use token::{
    MintInfo, MultisigInfo, TokenAccountInfo, TokenAccountState, TokenAmount, TokenProgramAccount,
};
pub use vote::{AuthorizedVoter, EpochCredit, LastTimestamp, PriorVoter, Vote, VoteInfo};

use crate::methods::accounts::ParsedAccount;

/// A lamports-per-signature rate shared by the nonce and fee-sysvar payloads.
///
/// The pinned source declares this shape twice, as
/// `JsonParsedNonceFeeCalculator` and `JsonParsedFeeCalculator`. Both carry only
/// an exact `lamportsPerSignature` string, so one Rust type represents both.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeeCalculator {
    /// Exact lamports charged per signature, reported as unsigned decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub lamports_per_signature: u64,
}

/// One reference family selected by the node's parser name.
#[derive(Debug, PartialEq)]
pub enum ParsedAccountData {
    /// A token program account, mint or multisig.
    Token(TokenProgramAccount),
    /// A stake account in its delegated or initialized state.
    Stake(StakeProgramAccount),
    /// Complete vote account state.
    Vote(VoteInfo),
    /// Initialized nonce account data.
    Nonce(NonceInfo),
    /// Address lookup table metadata and addresses.
    AddressLookupTable(AddressLookupTableInfo),
    /// A BPF upgradeable loader program or program data account.
    BpfUpgradeableLoader(BpfUpgradeableLoaderProgramAccount),
    /// A config program account.
    Config(ConfigProgramAccount),
    /// A sysvar account declared by the reference model.
    Sysvar(SysvarAccount),
    /// The node parser name is outside every reference family.
    UnknownProgram(UnknownParsed),
    /// A reference parser name reported an undeclared discriminator.
    UnknownType(UnknownParsed),
}

/// A payload the reference model does not declare, retained without interpretation.
#[derive(Debug, Clone)]
pub struct UnknownParsed {
    /// Node parser name reported with the account.
    pub program: String,
    /// Node `type` discriminator when the payload declares a string one.
    pub type_name: Option<String>,
    /// Exact `parsed` JSON value, including every unknown member.
    pub value: Box<RawValue>,
}

impl UnknownParsed {
    pub(crate) fn new(
        program: &str,
        type_name: Option<String>,
        parsed: &RawValue,
    ) -> Result<Self, Error> {
        Ok(Self {
            program: program.to_owned(),
            type_name,
            value: RawValue::from_string(parsed.get().to_owned()).map_err(Error::Schema)?,
        })
    }
}

impl PartialEq for UnknownParsed {
    fn eq(&self, other: &Self) -> bool {
        self.program == other.program
            && self.type_name == other.type_name
            && self.value.get() == other.value.get()
    }
}

impl Eq for UnknownParsed {}

/// Projects one parsed account into a reference-derived typed family.
///
/// The input value is borrowed: `program`, `parsed` and `space` remain available
/// exactly as the node reported them. `limits` bounds the payload bytes and JSON
/// nesting, and duplicate object keys are rejected, before any typed allocation.
/// Encoded or compressed account data has no JSON to project and is not involved.
pub fn decode(account: &ParsedAccount, limits: Limits) -> Result<ParsedAccountData, Error> {
    decode_parts(&account.program, &account.parsed, limits)
}

/// Projects an already separated parser name and exact `parsed` value.
///
/// This is the same bounded projection as [`decode`] for callers that retain the
/// two members independently. No binary fallback is inferred from parsed JSON.
pub fn decode_parts(
    program: &str,
    parsed: &RawValue,
    limits: Limits,
) -> Result<ParsedAccountData, Error> {
    if program.len() > limits.bytes {
        return Err(Error::BodyLimit);
    }

    let body = parsed.get().as_bytes();
    crate::json::check_limits(body, limits)?;
    crate::validate::unique(body).map_err(Error::Schema)?;

    match program {
        "spl-token" | "spl-token-2022" => token::decode(program, parsed),
        "stake" => stake::decode(program, parsed),
        "vote" => vote::decode(program, parsed),
        "nonce" => nonce::decode(program, parsed),
        "address-lookup-table" => address_lookup_table::decode(program, parsed),
        "bpf-upgradeable-loader" => loader::decode(program, parsed),
        "config" => config::decode(program, parsed),
        "sysvar" => sysvar::decode(program, parsed),
        _ => Ok(ParsedAccountData::UnknownProgram(UnknownParsed::new(
            program,
            type_peek(parsed),
            parsed,
        )?)),
    }
}

/// A node `type` discriminator with its still-exact `info` member.
#[derive(Deserialize)]
pub(super) struct Envelope<'a> {
    #[serde(rename = "type", borrow)]
    pub(super) type_name: &'a str,
    #[serde(default, borrow)]
    pub(super) info: Option<&'a RawValue>,
}

pub(super) fn envelope(parsed: &RawValue) -> Result<Envelope<'_>, Error> {
    serde_json::from_str(parsed.get()).map_err(Error::Schema)
}

pub(super) fn info<T: serde::de::DeserializeOwned>(envelope: &Envelope<'_>) -> Result<T, Error> {
    let info = envelope.info.ok_or_else(|| {
        Error::Schema(<serde_json::Error as serde::de::Error>::missing_field(
            "info",
        ))
    })?;
    serde_json::from_str(info.get()).map_err(Error::Schema)
}

pub(super) fn unknown_type(
    program: &str,
    type_name: &str,
    parsed: &RawValue,
) -> Result<ParsedAccountData, Error> {
    Ok(ParsedAccountData::UnknownType(UnknownParsed::new(
        program,
        Some(type_name.to_owned()),
        parsed,
    )?))
}

pub(super) fn raw_optional_eq(left: &Option<Box<RawValue>>, right: &Option<Box<RawValue>>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => left.get() == right.get(),
        _ => false,
    }
}

fn type_peek(parsed: &RawValue) -> Option<String> {
    #[derive(Deserialize)]
    struct Peek<'a> {
        #[serde(rename = "type", default, borrow)]
        type_name: Option<&'a str>,
    }

    serde_json::from_str::<Peek<'_>>(parsed.get())
        .ok()
        .and_then(|peek| peek.type_name.map(str::to_owned))
}
