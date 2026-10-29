//! Recommended pricing and refreshed lifetime observations preserve their typed causes.

use super::{SmartTransactionError, SmartTransactionOptions, SmartTransactionStage};
use crate::{
    priority_fee::{GetPriorityFeeEstimateRequest, PriorityFeeOptions},
    transactions::{ResolvePriorityFeeInput, ResolvedPriorityFee, resolve_priority_fee},
};
use core::num::NonZeroU32;
use solana_kit_client::{
    Client, Transport,
    methods::{BlockhashValue, Contextual, LatestBlockhash, ReadConfig},
};
use solana_kit_core::Address;

/// Samples one recommended fee by the already bounded, source-ordered account keys.
pub(super) async fn recommended_fee<T: Transport>(
    client: &Client<T>,
    account_keys: &[Address],
    compute_units: NonZeroU32,
    options: SmartTransactionOptions<'_>,
) -> Result<ResolvedPriorityFee, SmartTransactionError> {
    let response = client
        .call(GetPriorityFeeEstimateRequest {
            account_keys: Some(account_keys.iter().map(ToString::to_string).collect()),
            transaction: None,
            options: Some(PriorityFeeOptions {
                priority_level: None,
                include_all_priority_fee_levels: None,
                transaction_encoding: None,
                lookback_slots: None,
                include_vote: None,
                recommended: Some(true),
                evaluate_empty_slot_as_zero: None,
            }),
        })
        .await
        .map_err(|error| SmartTransactionError::Rpc {
            stage: SmartTransactionStage::FeeEstimate,
            error,
        })?;
    let estimate = response
        .priority_fee_estimate
        .value()
        .ok_or(SmartTransactionError::MissingFeeEstimate)?;
    Ok(resolve_priority_fee(ResolvePriorityFeeInput {
        estimate,
        units: u64::from(compute_units.get()),
        rate_cap: options.rate_cap,
        lamports_cap: options.lamports_cap,
    })?)
}

/// Retains the actual returned blockhash/expiry and enforces the explicit read minimum.
pub(super) async fn refreshed_blockhash<T: Transport>(
    client: &Client<T>,
    read: ReadConfig,
) -> Result<Contextual<BlockhashValue>, SmartTransactionError> {
    let config = (read != ReadConfig::default()).then_some(read);
    let response = client
        .call(LatestBlockhash(config))
        .await
        .map_err(|error| SmartTransactionError::Rpc {
            stage: SmartTransactionStage::Blockhash,
            error,
        })?;

    if let Some(minimum_slot) = read.min_context_slot
        && response.context.slot < minimum_slot
    {
        return Err(SmartTransactionError::StaleBlockhashContext {
            context: response.context,
            minimum_slot,
        });
    }

    Ok(response)
}
