//! Unsigned v0 preparation preserves simulation, fee, then blockhash observation order.

mod draft;
mod errors;
mod models;
mod observations;

pub use errors::{SmartTransactionBuildError, SmartTransactionError, SmartTransactionStage};
pub use models::{PreparedSmartTransaction, SmartTransactionInput, SmartTransactionOptions};

use super::{buffer_compute_units, get_compute_units};
use draft::Draft;
use observations::{recommended_fee, refreshed_blockhash};
use solana_kit_client::{Client, Transport};
use solana_kit_core::transaction::Transaction;

/// Prepares an unsigned v0 transaction; the caller signs and submits it separately.
///
/// Validates and strips caller Compute Budget instructions before I/O. One simulation
/// precedes recommended account-key pricing, then the final blockhash refresh.
/// Caller lookup snapshots are reused without discovery. The final message encodes
/// the resolved price and limit and retains the refreshed blockhash's real expiry.
/// Dropping this future drops its active exchange; preparation creates no task or retry.
pub async fn create_smart_transaction<T: Transport>(
    client: &Client<T>,
    input: SmartTransactionInput<'_>,
    options: SmartTransactionOptions<'_>,
) -> Result<PreparedSmartTransaction, SmartTransactionError> {
    let draft = Draft::new(input, options)?;
    let simulation = get_compute_units(client, &draft.wire, options.read).await?;

    let compute_units = match draft.explicit_units {
        Some(units) => units,
        None => buffer_compute_units(simulation.value, options.compute_buffer)?,
    };

    let priority_fee = recommended_fee(client, &draft.account_keys, compute_units, options).await?;
    let blockhash = refreshed_blockhash(client, options.read).await?;
    let message = draft.priced_message(&blockhash.value, (compute_units, priority_fee))?;
    Ok(PreparedSmartTransaction {
        transaction: Transaction::new(message),
        compute_units,
        priority_fee,
        account_keys: draft.account_keys,
        simulation,
        blockhash_context: blockhash.context,
    })
}
