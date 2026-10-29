//! Simulation uses maximum applicable budgets without signing or submitting.

use crate::{
    Client, Presence, Transport,
    methods::{
        Commitment,
        simulation::{SimulateTransaction, SimulationConfig, SimulationExecution},
        submission::WireEncoding,
    },
};
use solana_kit_core::transaction::{Lifetime, Transaction};

use super::{
    MAX_COMPUTE_UNIT_LIMIT, MAX_LOADED_ACCOUNTS_DATA_SIZE_LIMIT, ResourceError,
    ResourceLimitsEstimate, ResourceMessage, VersionContext,
};

/// Per-call node evaluation policy. Transport deadlines belong to its adapter.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EstimateConfig {
    /// Confirmation level used for simulation.
    pub commitment: Option<Commitment>,
    /// Require evaluation at or beyond this slot.
    pub min_context_slot: Option<u64>,
}

/// Observes actual resource use through one unsigned transaction simulation.
///
/// Compute is maximized for every version. Version one also maximizes loaded
/// bytes. A nonce lifetime keeps its exact token and disables RPC replacement;
/// blockhash lifetimes request replacement. The returned replacement hash never
/// changes caller metadata. Dropping this future drops the transport exchange.
///
/// # Errors
/// Preserves preparation and RPC failures. Missing or null compute usage takes
/// precedence over missing version-one loaded usage, then transaction errors.
/// Compute observations above u32::MAX are capped without applying a margin.
pub async fn estimate<T: Transport>(
    client: &Client<T>,
    message: &ResourceMessage<'_>,
    config: EstimateConfig,
) -> Result<ResourceLimitsEstimate, ResourceError> {
    let is_v1 = matches!(message.version, VersionContext::V1(_));
    let replace_recent_blockhash = !matches!(message.lifetime, Lifetime::DurableNonce(_));
    let mut prepared = message
        .clone()
        .with_compute_unit_limit(Some(MAX_COMPUTE_UNIT_LIMIT))?;

    if is_v1 {
        prepared = prepared
            .with_loaded_accounts_data_size_limit(Some(MAX_LOADED_ACCOUNTS_DATA_SIZE_LIMIT))?;
    }

    let wire = Transaction::new(prepared.compile()?)
        .encode(false)
        .map_err(ResourceError::Compilation)?;
    let result = client
        .call(SimulateTransaction {
            wire: &wire,
            encoding: WireEncoding::Base64,
            config: SimulationConfig {
                commitment: config.commitment,
                min_context_slot: config.min_context_slot,
                sig_verify: Some(false),
                replace_recent_blockhash: Some(replace_recent_blockhash),
                ..SimulationConfig::default()
            },
        })
        .await
        .map_err(ResourceError::Rpc)?;
    observed_limits(result.value.execution, is_v1)
}

// The pinned source validates usage before reporting a transaction failure.
fn observed_limits(
    execution: SimulationExecution,
    require_loaded: bool,
) -> Result<ResourceLimitsEstimate, ResourceError> {
    let compute = match execution.units_consumed {
        Presence::Present(Some(units)) => units,
        _ => return Err(ResourceError::MissingComputeUsage),
    };

    let loaded = match execution.loaded_accounts_data_size {
        Presence::Present(bytes) => bytes,
        Presence::Missing => None,
    };

    if require_loaded && loaded.is_none() {
        return Err(ResourceError::MissingLoadedAccountsDataSize);
    }

    if let Some(error) = execution.err {
        return Err(ResourceError::Transaction(error));
    }

    Ok(ResourceLimitsEstimate {
        compute_unit_limit: u32::try_from(compute).unwrap_or(u32::MAX),
        loaded_accounts_data_size_limit: loaded,
    })
}
