//! Batch results retain the count, order and identities requested by the application.

use super::{locators::*, models::*, requests::*};
use solana_kit_client::methods::Contextual;

fn entries_match<Query, Entry>(
    requested: &[Query],
    returned: &[Entry],
    same: impl Fn(&Query, &Entry) -> bool,
) -> bool {
    requested.len() == returned.len()
        && requested
            .iter()
            .zip(returned)
            .all(|(query, entry)| same(query, entry))
}

pub(super) fn account(
    request: &GetCompressedAccountRequest,
    response: &GetCompressedAccountResponse,
) -> bool {
    response
        .value
        .as_ref()
        .is_none_or(|account| match &request.0 {
            AccountLocator::Hash(hash) => &account.hash == hash,
            AccountLocator::Address(address) => account.address.value() == Some(address),
        })
}

pub(super) fn accounts(
    request: &GetMultipleCompressedAccountsRequest,
    response: &Contextual<CompressedItems<Option<CompressedAccount>>>,
) -> bool {
    let returned = &response.value.items;

    match request {
        GetMultipleCompressedAccountsRequest::Hashes(hashes) => {
            entries_match(hashes, returned, |hash, account| {
                account.as_ref().is_none_or(|account| &account.hash == hash)
            })
        }
        GetMultipleCompressedAccountsRequest::Addresses(addresses) => {
            entries_match(addresses, returned, |address, account| {
                account
                    .as_ref()
                    .is_none_or(|account| account.address.value() == Some(address))
            })
        }
    }
}

pub(super) fn proofs(
    request: &GetMultipleCompressedAccountProofsRequest,
    response: &Contextual<Vec<CompressedAccountProof>>,
) -> bool {
    entries_match(&request.0, &response.value, |hash, proof| {
        hash == &proof.hash
    })
}

pub(super) fn proof(
    request: &GetCompressedAccountProofRequest,
    response: &Contextual<CompressedAccountProof>,
) -> bool {
    request.hash == response.value.hash
}

pub(super) fn new_addresses(
    request: &GetMultipleNewAddressProofsRequest,
    response: &Contextual<Vec<NewAddressProof>>,
) -> bool {
    entries_match(&request.0, &response.value, |address, proof| {
        address == &proof.address
    })
}

pub(super) fn new_addresses_with_trees(
    request: &GetMultipleNewAddressProofsV2Request,
    response: &Contextual<Vec<NewAddressProof>>,
) -> bool {
    entries_match(&request.0, &response.value, |query, proof| {
        query.address == proof.address && query.tree == proof.merkle_tree
    })
}
