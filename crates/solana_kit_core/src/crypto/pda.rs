//! Solana's hash domain and curve checks for derived account addresses.

use alloc::vec::Vec;
use core::fmt;

use curve25519_dalek::edwards::CompressedEdwardsY;

use crate::{Address, crypto::hash};

/// Maximum seeds accepted by direct PDA creation, including a supplied bump.
pub const MAX_SEEDS: usize = 16;
/// Maximum byte length of each seed.
pub const MAX_SEED_LENGTH: usize = 32;
const PDA_MARKER: &[u8] = b"ProgramDerivedAddress";

/// Why a Solana address derivation was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PdaError {
    /// Too many seed slices; bump search reserves one of the 16 slots.
    TooManySeeds,
    /// At least one seed exceeds 32 bytes.
    SeedTooLong,
    /// The hash is on the Ed25519 curve and cannot be a PDA.
    OnCurve,
    /// All 256 bump candidates produced on-curve addresses.
    NoViableBump,
    /// Seeded-address ownership would collide with the PDA domain separator.
    IllegalOwner,
}

impl fmt::Display for PdaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            Self::TooManySeeds => "too many address seeds",
            Self::SeedTooLong => "address seed exceeds 32 bytes",
            Self::OnCurve => "derived address is on the Ed25519 curve",
            Self::NoViableBump => "no off-curve bump candidate",
            Self::IllegalOwner => "owner ends with the program-derived address marker",
        };

        formatter.write_str(description)
    }
}

impl core::error::Error for PdaError {}

/// A validated off-curve address and the bump used by canonical bump search.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProgramDerivedAddress {
    /// The derived address.
    pub address: Address,
    /// The highest valid bump, searched from 255 through 0.
    pub bump: u8,
}

/// Whether the address bytes decompress as an Ed25519 curve point.
///
/// This is the Solana PDA curve test, not an Ed25519 signing-key validity test.
#[must_use]
pub fn is_on_curve(address: &Address) -> bool {
    CompressedEdwardsY(address.to_bytes())
        .decompress()
        .is_some()
}

/// Validates the seed count and each seed length before hashing.
fn validate_seeds(seeds: &[&[u8]], maximum: usize) -> Result<(), PdaError> {
    if seeds.len() > maximum {
        return Err(PdaError::TooManySeeds);
    }

    if seeds.iter().any(|seed| seed.len() > MAX_SEED_LENGTH) {
        return Err(PdaError::SeedTooLong);
    }

    Ok(())
}

/// Derives an off-curve PDA from explicit seeds, optionally including a bump.
///
/// Seeds are concatenated without lengths or separators, matching Solana.
/// Callers must choose unambiguous application seed schemas.
///
/// # Errors
/// Rejects more than 16 seeds, any seed longer than 32 bytes, or an on-curve hash.
pub fn create_program_address(seeds: &[&[u8]], program: &Address) -> Result<Address, PdaError> {
    validate_seeds(seeds, MAX_SEEDS)?;
    let mut input = Vec::with_capacity(seeds.len() + 2);
    input.extend_from_slice(seeds);
    input.push(program.as_ref());
    input.push(PDA_MARKER);
    let address = Address::from_bytes(hash(&input).to_bytes());

    if is_on_curve(&address) {
        return Err(PdaError::OnCurve);
    }

    Ok(address)
}

/// Finds the highest bump that produces a valid off-curve PDA.
///
/// # Errors
/// Rejects more than 15 input seeds (the bump occupies a slot), seeds longer
/// than 32 bytes, or the exceptional case where no bump is viable.
pub fn find_program_address(
    seeds: &[&[u8]],
    program: &Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    validate_seeds(seeds, MAX_SEEDS - 1)?;

    for bump in (0..=u8::MAX).rev() {
        let bump_seed = [bump];
        let mut input = Vec::with_capacity(seeds.len() + 1);
        input.extend_from_slice(seeds);
        input.push(&bump_seed);
        let candidate = create_program_address(&input, program);

        match candidate {
            Ok(address) => return Ok(ProgramDerivedAddress { address, bump }),
            Err(PdaError::OnCurve) => {}
            Err(error) => return Err(error),
        }
    }

    Err(PdaError::NoViableBump)
}

/// Derives a seeded address from a base, at most 32 seed bytes, and an owner.
///
/// Unlike a PDA, this address may be on curve. The owner must not end in the
/// `ProgramDerivedAddress` marker, preserving separation of the two schemes.
///
/// # Errors
/// Returns [`PdaError::SeedTooLong`] or [`PdaError::IllegalOwner`].
pub fn create_address_with_seed(
    base: &Address,
    seed: &[u8],
    owner: &Address,
) -> Result<Address, PdaError> {
    validate_seeds(&[seed], 1)?;

    if owner.as_ref().ends_with(PDA_MARKER) {
        return Err(PdaError::IllegalOwner);
    }

    Ok(Address::from_bytes(
        hash(&[base.as_ref(), seed, owner.as_ref()]).to_bytes(),
    ))
}
