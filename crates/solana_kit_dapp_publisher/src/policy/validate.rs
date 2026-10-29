//! Common checks run before inspecting either publication-specific payload.

use super::ExpectedTransaction;
use crate::Error;
use solana_kit_core::{
    Address, Signature, address_constants::COMPUTE_BUDGET_PROGRAM, crypto, transaction::Transaction,
};
use solana_kit_mpl_token_metadata::PROGRAM;

/// Establishes lookup, lifetime, payer, local signer, signature and program invariants.
pub(super) fn common(
    transaction: &Transaction,
    expected: ExpectedTransaction,
) -> Result<(), Error> {
    let message = transaction.message();

    if !message.lookups().is_empty() {
        return Err(Error::UnresolvedLookups);
    }

    if message.lifetime().token() != expected.blockhash {
        return Err(Error::BlockhashMismatch);
    }

    if message.static_accounts().first() != Some(&expected.payer) {
        return Err(Error::PayerMismatch);
    }

    if !message.signers().contains(&expected.signer) {
        return Err(Error::SignerNotRequired);
    }

    signatures(transaction)?;
    programs(transaction)
}

/// Missing caller slots are allowed here; every present slot must already authenticate.
fn signatures(transaction: &Transaction) -> Result<(), Error> {
    for (address, signature) in transaction
        .message()
        .signers()
        .iter()
        .zip(transaction.signatures())
    {
        if let Some(signature) = signature {
            crypto::verify(address, transaction.message().bytes(), signature)
                .map_err(|_| Error::InvalidSignature(*address))?;
        }
    }

    Ok(())
}

/// Every occupied duplicate identity slot must agree before core can fill its empty slots.
pub(super) fn occupied_signature(
    transaction: &Transaction,
    signer: Address,
    supplied: Signature,
) -> Result<(), Error> {
    for (address, occupied) in transaction
        .message()
        .signers()
        .iter()
        .zip(transaction.signatures())
    {
        if *address == signer && occupied.is_some_and(|existing| existing != supplied) {
            return Err(Error::OccupiedSignatureMismatch(signer));
        }
    }

    Ok(())
}

/// Only canonical Compute Budget and Token Metadata invocations are admitted.
fn programs(transaction: &Transaction) -> Result<(), Error> {
    let message = transaction.message();
    let mut has_metadata = false;

    for instruction in message.instructions() {
        let program = message.static_accounts()[usize::from(instruction.program_index())];

        if program != COMPUTE_BUDGET_PROGRAM && program != PROGRAM {
            return Err(Error::ProgramNotAllowed(program));
        }

        has_metadata |= program == PROGRAM;
    }

    if !has_metadata {
        return Err(Error::MissingMetadataInstruction);
    }

    Ok(())
}

/// Dart compares address sets: repeated equal identities do not add a new signer.
pub(super) fn signer_set(actual: &[Address], expected: [Address; 2]) -> Result<(), Error> {
    if !actual.iter().all(|address| expected.contains(address))
        || !expected.iter().all(|address| actual.contains(address))
    {
        return Err(Error::SignerSetMismatch);
    }

    Ok(())
}
