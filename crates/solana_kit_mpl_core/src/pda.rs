//! Explicit context and deterministic PDA derivation for external plugin accounts.
use crate::generated::*;
use alloc::vec::Vec;
use core::fmt;
use solana_kit_core::{
    Address,
    crypto::{PdaError, ProgramDerivedAddress, find_program_address},
};

/// Context addresses used by preconfigured and custom seed variants.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExtraAccountContext {
    /// Current asset, when the seed recipe requires it.
    pub asset: Option<Address>,
    /// Current collection, when the seed recipe requires it.
    pub collection: Option<Address>,
    /// Current asset owner, when the seed recipe requires it.
    pub owner: Option<Address>,
    /// Proposed recipient, when the seed recipe requires it.
    pub recipient: Option<Address>,
}

/// Missing context and core derivation errors remain distinguishable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExtraAccountError {
    /// A named source seed requires a missing context address.
    MissingContext(&'static str),
    /// Core PDA validation rejected the recipe.
    Derivation(PdaError),
    /// Storage for bounded resolved seed references could not be reserved.
    AllocationFailed,
}

impl fmt::Display for ExtraAccountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "extra account derivation error: {self:?}")
    }
}

impl core::error::Error for ExtraAccountError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Derivation(error) => Some(error),
            Self::MissingContext(_) | Self::AllocationFailed => None,
        }
    }
}

impl From<PdaError> for ExtraAccountError {
    fn from(error: PdaError) -> Self {
        Self::Derivation(error)
    }
}

/// Derives the asset signer on the canonical Core program.
/// # Errors
/// Preserves seed and bump-search failures.
pub fn find_asset_signer(asset: &Address) -> Result<ProgramDerivedAddress, PdaError> {
    find_asset_signer_with_program(asset, &PROGRAM)
}

/// Derives an asset signer against an explicit program identity.
/// # Errors
/// Preserves seed and bump-search failures.
pub fn find_asset_signer_with_program(
    asset: &Address,
    program: &Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[b"mpl-core-execute", asset.as_bytes()], program)
}

/// Derives the source preconfigured program identity using `mpl-core`.
/// # Errors
/// Preserves seed and bump-search failures.
pub fn find_preconfigured_program(program: &Address) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[b"mpl-core"], program)
}

/// Derives collection, owner, recipient or asset configuration using its explicit key.
/// # Errors
/// Preserves seed and bump-search failures.
pub fn find_preconfigured(
    key: &Address,
    program: &Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[b"mpl-core", key.as_bytes()], program)
}

fn require<'a>(
    value: &'a Option<Address>,
    name: &'static str,
) -> Result<&'a Address, ExtraAccountError> {
    value
        .as_ref()
        .ok_or(ExtraAccountError::MissingContext(name))
}

/// Resolves every ExtraAccount variant with caller-owned context and program identity.
/// # Errors
/// Rejects missing context, more than 15 custom seeds, long seeds and invalid derivations.
pub fn derive_extra_account(
    account: &ExtraAccount,
    program: &Address,
    context: &ExtraAccountContext,
) -> Result<Address, ExtraAccountError> {
    let key = match account {
        ExtraAccount::PreconfiguredProgram(_) => {
            return Ok(find_preconfigured_program(program)?.address());
        }
        ExtraAccount::PreconfiguredCollection(_) => require(&context.collection, "collection")?,
        ExtraAccount::PreconfiguredOwner(_) => require(&context.owner, "owner")?,
        ExtraAccount::PreconfiguredRecipient(_) => require(&context.recipient, "recipient")?,
        ExtraAccount::PreconfiguredAsset(_) => require(&context.asset, "asset")?,
        ExtraAccount::Address(value) => return Ok(value.address),
        ExtraAccount::CustomPda(value) => {
            return custom_seeds(
                &value.seeds,
                value.custom_program_id.as_ref().unwrap_or(program),
                context,
            );
        }
    };

    Ok(find_preconfigured(key, program)?.address())
}

fn custom_seeds(
    seeds: &[Seed],
    program: &Address,
    context: &ExtraAccountContext,
) -> Result<Address, ExtraAccountError> {
    if seeds.len() >= solana_kit_core::crypto::MAX_SEEDS {
        return Err(PdaError::TooManySeeds.into());
    }

    let mut resolved = Vec::new();
    resolved
        .try_reserve_exact(seeds.len())
        .map_err(|_| ExtraAccountError::AllocationFailed)?;

    for seed in seeds {
        let bytes: &[u8] = match seed {
            Seed::Collection => require(&context.collection, "collection")?.as_bytes(),
            Seed::Owner => require(&context.owner, "owner")?.as_bytes(),
            Seed::Recipient => require(&context.recipient, "recipient")?.as_bytes(),
            Seed::Asset => require(&context.asset, "asset")?.as_bytes(),
            Seed::Address(value) => value.as_bytes(),
            Seed::Bytes(value) => value,
        };

        resolved.push(bytes);
    }

    Ok(find_program_address(&resolved, program)?.address())
}

/// Returns the oracle base directly or resolves its extra-account configuration on that program.
/// # Errors
/// Preserves context, allocation and deterministic derivation failures.
pub fn find_oracle_account(
    base: &Address,
    config: Option<&ExtraAccount>,
    context: &ExtraAccountContext,
) -> Result<Address, ExtraAccountError> {
    config.map_or(Ok(*base), |config| {
        derive_extra_account(config, base, context)
    })
}
