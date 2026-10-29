//! Transaction simulation helpers borrow caller-owned wire bytes and RPC capabilities.

use core::fmt;
use solana_kit_client::{
    Client, Presence, Transport,
    methods::{
        Context, Contextual, ReadConfig,
        simulation::{SimulateTransaction, SimulationConfig},
        submission::{TransactionFailure, WireEncoding},
    },
};

/// Compute estimation failures preserve typed causes through explicit access.
///
/// Debug and Display omit response details, including execution context and RPC bodies.
pub enum ComputeUnitsError {
    /// The shared RPC client rejected transport, framing, budgets or the simulation schema.
    Rpc(solana_kit_client::Error),
    /// The transaction failed during simulation, regardless of reported units.
    Execution {
        /// Node evaluation context retained for explicit inspection.
        context: Context,
        /// Typed execution failure, including any preserved future node variant.
        failure: TransactionFailure,
    },
    /// The node omitted unitsConsumed or returned null.
    MissingUnits {
        /// Node evaluation context retained for explicit inspection.
        context: Context,
    },
    /// The node evaluated the transaction below the caller's requested minimum slot.
    StaleContext {
        /// Rejected node evaluation context.
        context: Context,
        /// Caller-requested minimum evaluation slot.
        minimum_slot: u64,
    },
}

impl fmt::Debug for ComputeUnitsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Rpc(_) => "ComputeUnitsError::Rpc",
            Self::Execution { .. } => "ComputeUnitsError::Execution",
            Self::MissingUnits { .. } => "ComputeUnitsError::MissingUnits",
            Self::StaleContext { .. } => "ComputeUnitsError::StaleContext",
        };
        formatter.debug_struct(name).finish_non_exhaustive()
    }
}

impl fmt::Display for ComputeUnitsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Rpc(_) => "compute estimation RPC failed",
            Self::Execution { .. } => "compute estimation simulation failed",
            Self::MissingUnits { .. } => "compute estimation did not report unitsConsumed",
            Self::StaleContext { .. } => "compute estimation context is below the requested slot",
        })
    }
}

impl std::error::Error for ComputeUnitsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Rpc(error) => Some(error),
            Self::Execution { failure, .. } => Some(failure),
            Self::MissingUnits { .. } | Self::StaleContext { .. } => None,
        }
    }
}

/// Simulates caller-owned transaction wire once and returns exact reported units and context.
///
/// The shared RPC client encodes Base64, replaces the recent blockhash and disables
/// signature verification. Its byte and JSON-depth budgets bound the exchange.
/// Execution failures take precedence over units; missing or null units reject,
/// while a reported zero is valid. The returned slot must meet the requested minimum.
/// The caller owns wire construction, transport and deadlines. Dropping this future
/// drops its current exchange; it does not start a task or retry.
pub async fn get_compute_units<T: Transport>(
    client: &Client<T>,
    wire: &[u8],
    read: ReadConfig,
) -> Result<Contextual<u64>, ComputeUnitsError> {
    let Contextual { context, value } = client
        .call(SimulateTransaction {
            wire,
            encoding: WireEncoding::Base64,
            config: SimulationConfig {
                commitment: read.commitment,
                min_context_slot: read.min_context_slot,
                sig_verify: Some(false),
                replace_recent_blockhash: Some(true),
                ..SimulationConfig::default()
            },
        })
        .await
        .map_err(ComputeUnitsError::Rpc)?;

    if let Some(failure) = value.execution.err {
        return Err(ComputeUnitsError::Execution { context, failure });
    }

    if let Some(minimum_slot) = read.min_context_slot
        && context.slot < minimum_slot
    {
        return Err(ComputeUnitsError::StaleContext {
            context,
            minimum_slot,
        });
    }

    let Presence::Present(Some(units)) = value.execution.units_consumed else {
        return Err(ComputeUnitsError::MissingUnits { context });
    };

    Ok(Contextual {
        context,
        value: units,
    })
}
