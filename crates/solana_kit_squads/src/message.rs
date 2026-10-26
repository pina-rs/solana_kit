//! Checked client-side transaction-message structure, with no RPC or bank-state inference.
use crate::{VaultTransactionMessage, VaultTransactionMessageCodec};
use solana_kit_core::codecs::{Encoder, Error, Limits};

const INDEX_CAPACITY: usize = u8::MAX as usize + 1;

/// Validates indexed message structure before a client constructs a vault transaction.
/// This checks wire/index consistency, not current lookup-table contents or execution authority.
/// # Errors
/// Rejects inconsistent header partitions, out-of-range indexes and caller budgets.
pub fn validate_vault_message(
    message: &VaultTransactionMessage,
    limits: Limits,
) -> Result<(), Error> {
    let static_count = message.account_keys.len();
    let signers = usize::from(message.num_signers);

    if signers > static_count
        || message.num_writable_signers > message.num_signers
        || usize::from(message.num_writable_non_signers) > static_count - signers
    {
        return Err(Error::InvalidValue);
    }

    validate_item_budget(message, limits.max_items)?;

    if VaultTransactionMessageCodec.encoded_len(message)? > limits.max_bytes {
        return Err(Error::LimitExceeded);
    }

    let total = total_accounts(message, static_count)?;

    if total > INDEX_CAPACITY {
        return Err(Error::LimitExceeded);
    }

    validate_indexes(message, total)
}

fn total_accounts(message: &VaultTransactionMessage, static_count: usize) -> Result<usize, Error> {
    message
        .address_table_lookups
        .iter()
        .try_fold(static_count, |count, lookup| {
            count
                .checked_add(lookup.writable_indexes.len())
                .and_then(|count| count.checked_add(lookup.readonly_indexes.len()))
                .ok_or(Error::Overflow)
        })
}

fn validate_indexes(message: &VaultTransactionMessage, total: usize) -> Result<(), Error> {
    for instruction in &message.instructions {
        if usize::from(instruction.program_id_index) >= total
            || instruction
                .account_indexes
                .iter()
                .any(|index| usize::from(*index) >= total)
        {
            return Err(Error::InvalidValue);
        }
    }

    Ok(())
}

fn validate_item_budget(message: &VaultTransactionMessage, maximum: usize) -> Result<(), Error> {
    let mut items = 0;

    for count in [
        message.account_keys.len(),
        message.instructions.len(),
        message.address_table_lookups.len(),
    ] {
        claim_items(&mut items, count, maximum)?;
    }

    for instruction in &message.instructions {
        claim_items(&mut items, instruction.account_indexes.len(), maximum)?;
        claim_items(&mut items, instruction.data.len(), maximum)?;
    }

    for lookup in &message.address_table_lookups {
        claim_items(&mut items, lookup.writable_indexes.len(), maximum)?;
        claim_items(&mut items, lookup.readonly_indexes.len(), maximum)?;
    }

    Ok(())
}

fn claim_items(items: &mut usize, count: usize, maximum: usize) -> Result<(), Error> {
    *items = items.checked_add(count).ok_or(Error::Overflow)?;

    if *items > maximum {
        return Err(Error::LimitExceeded);
    }

    Ok(())
}
