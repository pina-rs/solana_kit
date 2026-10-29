//! Only provisional or absent applicable limits spend a caller-owned estimate.

use super::{
    EstimateConfig, MAX_COMPUTE_UNIT_LIMIT, ResourceError, ResourceLimitsEstimate, ResourceMessage,
    TuningError, VersionContext, estimate,
};
use crate::{Client, Transport};
use core::future::Future;

/// Replaces provisional applicable limits with a caller-provided estimate.
///
/// Compute is provisional when absent, zero, or 1,400,000. Version-one loaded
/// bytes are provisional when absent or zero. If every applicable limit is
/// explicit, the estimator is never invoked. Legacy/v0 loaded limits survive.
/// A custom estimator may omit loaded bytes, leaving that field unchanged.
///
/// The closure receives a draft snapshot; its concrete future remains local or
/// Send according to the caller's captured values. No runtime task is created.
///
/// # Errors
/// Retains the estimator's exact error type, or reports a rejected applied limit.
pub async fn estimate_and_set<'a, E, F, O>(
    mut message: ResourceMessage<'a>,
    estimator: F,
) -> Result<ResourceMessage<'a>, TuningError<E>>
where
    F: FnOnce(ResourceMessage<'a>) -> O,
    O: Future<Output = Result<ResourceLimitsEstimate, E>>,
{
    let compute_explicit = message
        .compute_unit_limit()
        .is_some_and(|limit| limit != 0 && limit != MAX_COMPUTE_UNIT_LIMIT);
    let loaded_explicit = !matches!(message.version, VersionContext::V1(_))
        || message
            .loaded_accounts_data_size_limit()
            .is_some_and(|limit| limit != 0);

    if compute_explicit && loaded_explicit {
        return Ok(message);
    }

    let limits = estimator(message.clone())
        .await
        .map_err(TuningError::Estimation)?;

    if !compute_explicit {
        message = message
            .with_compute_unit_limit(Some(limits.compute_unit_limit))
            .map_err(TuningError::Message)?;
    }

    if !loaded_explicit && let Some(limit) = limits.loaded_accounts_data_size_limit {
        message = message
            .with_loaded_accounts_data_size_limit(Some(limit))
            .map_err(TuningError::Message)?;
    }

    Ok(message)
}

/// Tunes a draft with the standard one-shot RPC estimator and explicit node policy.
///
/// # Errors
/// Preserves simulation failures and any failure to apply the observed estimate.
pub async fn estimate_and_set_with_client<'a, T: Transport>(
    client: &Client<T>,
    message: ResourceMessage<'a>,
    config: EstimateConfig,
) -> Result<ResourceMessage<'a>, TuningError<ResourceError>> {
    estimate_and_set(message, |draft| async move {
        estimate(client, &draft, config).await
    })
    .await
}
