//! Explicit caller inputs and unsigned prepared observations.

use crate::{
    Decimal,
    transactions::{ComputeUnitBuffer, ResolvedPriorityFee},
};
use core::num::NonZeroU32;
use solana_kit_client::methods::{BlockhashValue, Context, Contextual, ReadConfig};
use solana_kit_core::{
    Address, Instruction,
    transaction::{LookupTable, Transaction},
};

/// Caller-owned instructions, payer identities and an actual simulation lifetime.
#[derive(Debug)]
pub struct SmartTransactionInput<'a> {
    /// Owned instructions; all caller Compute Budget invocations are removed.
    pub instructions: Vec<Instruction>,
    /// Explicit payer, taking precedence over the first signer address.
    pub fee_payer: Option<Address>,
    /// Caller-supplied signer identities; no signing capability or key is inferred.
    pub signers: &'a [Address],
    /// Known blockhash and real expiry used only to compile the simulation draft.
    pub simulation_lifetime: &'a BlockhashValue,
    /// Caller-owned lookup-table snapshots; preparation performs no table discovery.
    pub lookup_tables: &'a [LookupTable],
    /// Positive explicit limit overrides buffering; zero rejects before any exchange.
    pub compute_unit_limit: Option<u32>,
}

/// Shared node read constraints and exact fee/buffer choices.
#[derive(Debug, Clone, Copy, Default)]
pub struct SmartTransactionOptions<'a> {
    /// Applied to simulation and the refreshed blockhash read.
    pub read: ReadConfig,
    /// Used only when no explicit compute limit is supplied.
    pub compute_buffer: ComputeUnitBuffer,
    /// Optional exact per-unit fee ceiling.
    pub rate_cap: Option<&'a Decimal>,
    /// Optional exact total-lamport fee ceiling.
    pub lamports_cap: Option<&'a Decimal>,
}

/// Unsigned compiled v0 transaction and the observations used to prepare it.
#[derive(Debug)]
pub struct PreparedSmartTransaction {
    /// Final price-then-limit transaction with every signature slot still absent.
    pub transaction: Transaction,
    /// Requested positive compute limit encoded in the transaction.
    pub compute_units: NonZeroU32,
    /// Exact per-unit and total fee resolved for that limit.
    pub priority_fee: ResolvedPriorityFee,
    /// Fee-query identities: payer first, then unique instruction accounts; no added program IDs.
    pub account_keys: Vec<Address>,
    /// Raw compute observation and its evaluation context.
    pub simulation: Contextual<u64>,
    /// Context of the actual refreshed blockhash and expiry retained in the message lifetime.
    pub blockhash_context: Context,
}
