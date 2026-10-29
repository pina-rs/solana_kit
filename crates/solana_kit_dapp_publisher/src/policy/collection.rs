//! Source-defined collection checks reuse existing bounded codecs and canonical PDAs.

use super::{PublicationPolicy, validate::signer_set};
use crate::Error;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Limits},
    transaction::Transaction,
};
use solana_kit_mpl_token_metadata::{
    CreateArgs, CreateCodec, PROGRAM, find_master_edition, find_metadata,
};

const CREATE_INSTRUCTION: u8 = 42;

/// Applies the selected exact signer set and collection-specific account/data rules.
pub(super) fn validate(
    transaction: &Transaction,
    policy: PublicationPolicy,
    limits: Limits,
) -> Result<(), Error> {
    let payer = policy.expected().payer;

    match policy {
        PublicationPolicy::ReleaseMint {
            mint, collection, ..
        } => {
            signer_set(transaction.message().signers(), [payer, mint])?;
            release(transaction, mint, collection, limits)
        }
        PublicationPolicy::VerifyCollection {
            nft_mint,
            collection,
            authority,
            ..
        } => {
            signer_set(transaction.message().signers(), [payer, authority])?;
            verification(transaction, nft_mint, collection, authority)
        }
    }
}

/// The first Metadata Create is authoritative, matching the pinned Dart helper.
fn release(
    transaction: &Transaction,
    mint: Address,
    collection: Address,
    limits: Limits,
) -> Result<(), Error> {
    let message = transaction.message();
    let instruction = message
        .instructions()
        .iter()
        .find(|instruction| {
            message.static_accounts()[usize::from(instruction.program_index())] == PROGRAM
                && instruction.data().first() == Some(&CREATE_INSTRUCTION)
        })
        .ok_or(Error::CollectionMismatch)?;
    let decoded = CreateCodec
        .decode(instruction.data(), limits)
        .map_err(Error::Codec)?;
    let CreateArgs::V1(args) = decoded.create_args;

    if args.asset_data.collection.map(|value| value.key) != Some(collection) {
        return Err(Error::CollectionMismatch);
    }

    let signed_mint = message
        .signers()
        .iter()
        .zip(transaction.signatures())
        .any(|(address, signature)| *address == mint && signature.is_some());

    if !signed_mint {
        return Err(Error::MissingMintSignature);
    }

    Ok(())
}

/// Presence checks intentionally preserve the reference's narrower policy contract.
fn verification(
    transaction: &Transaction,
    mint: Address,
    collection: Address,
    authority: Address,
) -> Result<(), Error> {
    let required = [
        find_metadata(mint).map_err(Error::Pda)?.address(),
        collection,
        find_metadata(collection).map_err(Error::Pda)?.address(),
        find_master_edition(collection)
            .map_err(Error::Pda)?
            .address(),
        authority,
    ];

    for address in required {
        if !transaction.message().static_accounts().contains(&address) {
            return Err(Error::MissingAccount(address));
        }
    }

    Ok(())
}
