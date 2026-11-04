//! Token program account, mint and multisig payloads.
//!
//! Source: `packages/solana_kit_rpc_parsed_types/lib/src/token_accounts.dart`.
//! Token-2022 extension members stay opaque because the pinned model declares
//! them as `List<Object?>`, and every parsed member here mirrors that model.

use super::{ParsedAccountData, scalars};
use crate::Error;
use serde::Deserialize;
use serde_json::value::RawValue;
use solana_kit_core::Address;

pub(super) fn decode(program: &str, parsed: &RawValue) -> Result<ParsedAccountData, Error> {
    let envelope = super::envelope(parsed)?;

    let account = match envelope.type_name {
        "account" => TokenProgramAccount::Account(super::info(&envelope)?),
        "mint" => TokenProgramAccount::Mint(super::info(&envelope)?),
        "multisig" => TokenProgramAccount::Multisig(super::info(&envelope)?),
        other => return super::unknown_type(program, other, parsed),
    };

    Ok(ParsedAccountData::Token(account))
}

/// Reference `JsonParsedTokenProgramAccount`, discriminated by the node type.
#[derive(Debug, PartialEq)]
pub enum TokenProgramAccount {
    /// `type: "account"`.
    Account(TokenAccountInfo),
    /// `type: "mint"`.
    Mint(MintInfo),
    /// `type: "multisig"`.
    Multisig(MultisigInfo),
}

/// Reference `JsonParsedTokenAccount`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAccountInfo {
    /// Optional authority allowed to close the account.
    #[serde(default, deserialize_with = "scalars::optional_address")]
    pub close_authority: Option<Address>,
    /// Optional delegate permitted to move tokens.
    #[serde(default, deserialize_with = "scalars::optional_address")]
    pub delegate: Option<Address>,
    /// Amount approved to the delegate, with its display annotations.
    #[serde(default)]
    pub delegated_amount: Option<TokenAmount>,
    /// Program extension objects retained as their exact JSON array.
    #[serde(default, deserialize_with = "scalars::optional_raw_array")]
    pub extensions: Option<Box<RawValue>>,
    /// Whether this is a wrapped native SOL account.
    pub is_native: bool,
    /// Mint address of the token held by this account.
    #[serde(deserialize_with = "scalars::required_address")]
    pub mint: Address,
    /// Address that owns the token account.
    #[serde(deserialize_with = "scalars::required_address")]
    pub owner: Address,
    /// Rent-exempt reserve for a wrapped native SOL account.
    #[serde(default)]
    pub rent_exempt_reserve: Option<TokenAmount>,
    /// Token account state reported by the program.
    pub state: TokenAccountState,
    /// Exact token balance with node display annotations.
    pub token_amount: TokenAmount,
}

/// Reference `TokenAccountState`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TokenAccountState {
    /// The account is frozen and cannot be used.
    Frozen,
    /// The account is initialized and can be used.
    Initialized,
    /// The account has not been initialized.
    Uninitialized,
}

/// Reference `TokenAmount` from the shared RPC types.
///
/// The node's approximate `uiAmount` number is not part of the reference model,
/// so it is never parsed and no float path exists for the exact amount.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TokenAmount {
    /// Exact raw quantity reported as unsigned decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub amount: u64,
    /// Decimal scale applied to the raw quantity.
    pub decimals: u8,
    /// Exact node-supplied display spelling.
    pub ui_amount_string: String,
}

/// Reference `JsonParsedMintInfo`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintInfo {
    /// Decimal scale of the mint.
    pub decimals: u8,
    /// Program extension objects retained as their exact JSON array.
    #[serde(default, deserialize_with = "scalars::optional_raw_array")]
    pub extensions: Option<Box<RawValue>>,
    /// Optional authority allowed to freeze accounts.
    #[serde(default, deserialize_with = "scalars::optional_address")]
    pub freeze_authority: Option<Address>,
    /// Whether the mint was initialized.
    pub is_initialized: bool,
    /// Optional authority allowed to mint new supply.
    #[serde(default, deserialize_with = "scalars::optional_address")]
    pub mint_authority: Option<Address>,
    /// Exact total supply reported as unsigned decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub supply: u64,
}

/// Reference `JsonParsedMultisigInfo`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MultisigInfo {
    /// Whether the multisig was initialized.
    pub is_initialized: bool,
    /// Number of signers required to approve a transfer.
    pub num_required_signers: u8,
    /// Number of configured signer slots.
    pub num_valid_signers: u8,
    /// Addresses occupying the configured signer slots.
    #[serde(deserialize_with = "scalars::addresses")]
    pub signers: Vec<Address>,
}

impl PartialEq for TokenAccountInfo {
    fn eq(&self, other: &Self) -> bool {
        super::raw_optional_eq(&self.extensions, &other.extensions)
            && self.close_authority == other.close_authority
            && self.delegate == other.delegate
            && self.delegated_amount == other.delegated_amount
            && self.is_native == other.is_native
            && self.mint == other.mint
            && self.owner == other.owner
            && self.rent_exempt_reserve == other.rent_exempt_reserve
            && self.state == other.state
            && self.token_amount == other.token_amount
    }
}

impl PartialEq for MintInfo {
    fn eq(&self, other: &Self) -> bool {
        super::raw_optional_eq(&self.extensions, &other.extensions)
            && self.decimals == other.decimals
            && self.freeze_authority == other.freeze_authority
            && self.is_initialized == other.is_initialized
            && self.mint_authority == other.mint_authority
            && self.supply == other.supply
    }
}
