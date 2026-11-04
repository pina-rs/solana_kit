//! Key grinding that consumes export capability before returning signers.
//!
//! Core grinding returns [`ExportableKeypair`] values because a caller may have
//! requested backups. These wrappers convert every match with
//! `ExportableKeypair::into_signer`, so the yielded [`Keypair`] signers cannot
//! export key material. They implement the existing message and transaction
//! partial signer traits.

use alloc::vec::Vec;

use solana_kit_core::{
    Address,
    crypto::{
        Entropy, ExportableKeypair, GrindBudget, GrindError, GrindPattern, Keypair, grind_keypairs,
        grind_keypairs_with,
    },
};

/// Generates exactly `budget.amount()` non-exporting signers matching a literal.
///
/// Cancellation, entropy failure and budget exhaustion keep the core contract:
/// every retained exportable key is dropped with its zeroizing secret.
///
/// # Errors
/// Returns the core entropy or exhaustion failure unchanged.
pub async fn grind_keypair_signers<E: Entropy>(
    entropy: &mut E,
    pattern: GrindPattern<'_>,
    budget: GrindBudget,
) -> Result<Vec<Keypair>, GrindError<E::Error>> {
    let keys = grind_keypairs(entropy, pattern, budget).await?;

    Ok(into_signers(keys))
}

/// Generates exactly `budget.amount()` non-exporting signers satisfying a predicate.
///
/// The predicate receives each candidate address by value and owns its matching
/// strategy, including any flattening of a source regular expression.
///
/// # Errors
/// Returns the same failure contract as [`grind_keypair_signers`].
pub async fn grind_keypair_signers_with<E: Entropy, F>(
    entropy: &mut E,
    budget: GrindBudget,
    matches: F,
) -> Result<Vec<Keypair>, GrindError<E::Error>>
where
    F: FnMut(Address) -> bool,
{
    let keys = grind_keypairs_with(entropy, budget, matches).await?;

    Ok(into_signers(keys))
}

fn into_signers(keys: Vec<ExportableKeypair>) -> Vec<Keypair> {
    keys.into_iter()
        .map(ExportableKeypair::into_signer)
        .collect()
}
