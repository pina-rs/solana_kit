//! Explicit version-tolerant revoke snapshots retain every unknown appended byte.
use crate::{
    AccountKind, FixedDelegation, FixedDelegationCodec, RecurringDelegation,
    RecurringDelegationCodec, SubscriptionDelegation, SubscriptionDelegationCodec,
    identify_account,
};
use alloc::vec::Vec;
use solana_kit_core::codecs::{Decoder, Error, Limits, Reader};

/// A complete pinned V1 value plus uninterpreted future trailing bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithTail<T> {
    /// Complete source-defined V1 fields.
    pub value: T,
    /// Unknown appended data, preserved byte-exact instead of silently discarded.
    pub trailing: Vec<u8>,
}

fn read<T>(
    bytes: &[u8],
    kind: AccountKind,
    codec: impl Decoder<T>,
    limits: Limits,
) -> Result<WithTail<T>, Error> {
    if identify_account(bytes)? != kind {
        return Err(Error::InvalidDiscriminator);
    }

    let mut reader = Reader::new(bytes, limits)?;
    let value = codec.read(&mut reader)?;
    let count = reader.remaining().len();
    reader.claim(count)?;
    let mut trailing = Vec::new();
    trailing
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;
    trailing.extend_from_slice(reader.take(count)?);
    Ok(WithTail { value, trailing })
}

/// Reads the frozen 187-byte fixed-delegation V1 prefix and retains future tails.
/// # Errors
/// Rejects a wrong discriminator, incomplete V1 state, limits or allocation failure.
pub fn fixed_delegation_for_revoke(
    bytes: &[u8],
    limits: Limits,
) -> Result<WithTail<FixedDelegation>, Error> {
    read(
        bytes,
        AccountKind::FixedDelegation,
        FixedDelegationCodec,
        limits,
    )
}

/// Reads the frozen 211-byte recurring-delegation V1 prefix and retains future tails.
/// # Errors
/// Rejects a wrong discriminator, incomplete V1 state, limits or allocation failure.
pub fn recurring_delegation_for_revoke(
    bytes: &[u8],
    limits: Limits,
) -> Result<WithTail<RecurringDelegation>, Error> {
    read(
        bytes,
        AccountKind::RecurringDelegation,
        RecurringDelegationCodec,
        limits,
    )
}

/// Reads the frozen 155-byte subscription-delegation V1 prefix and retains future tails.
/// # Errors
/// Rejects a wrong discriminator, incomplete V1 state, limits or allocation failure.
pub fn subscription_delegation_for_revoke(
    bytes: &[u8],
    limits: Limits,
) -> Result<WithTail<SubscriptionDelegation>, Error> {
    read(
        bytes,
        AccountKind::SubscriptionDelegation,
        SubscriptionDelegationCodec,
        limits,
    )
}
