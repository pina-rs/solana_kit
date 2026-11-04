//! Filter and preflight caller inputs before any RPC capability is exercised.

use super::{SmartTransactionError, SmartTransactionInput, SmartTransactionOptions};
use crate::transactions::{ResolvedPriorityFee, buffer_compute_units};
use core::num::NonZeroU32;
use solana_kit_client::methods::BlockhashValue;
use solana_kit_compute_budget::{PROGRAM, SetComputeUnitLimit, SetComputeUnitPrice};
use solana_kit_core::{
    Address, Instruction,
    codecs::Limits,
    transaction::{Lifetime, LookupTable, Message, Transaction, Version},
};
use std::collections::BTreeSet;

const BUDGET_CODEC_LIMITS: Limits = Limits::new(9, 0);
const PREFLIGHT_FEE: ResolvedPriorityFee = ResolvedPriorityFee {
    rate: 0,
    lamports: 0,
};

pub(super) struct Draft<'a> {
    payer: Address,
    instructions: Vec<Instruction>,
    lookup_tables: &'a [LookupTable],
    pub(super) explicit_units: Option<NonZeroU32>,
    pub(super) wire: Vec<u8>,
    pub(super) account_keys: Vec<Address>,
}

impl<'a> Draft<'a> {
    /// Owns bounded stripped instructions, the simulation wire and fee identities.
    pub(super) fn new(
        mut input: SmartTransactionInput<'a>,
        options: SmartTransactionOptions<'_>,
    ) -> Result<Self, SmartTransactionError> {
        let (payer, explicit_units) = prepare_input(&mut input, options)?;
        let wire = simulation_wire(payer, &input)?;
        let account_keys = account_keys(payer, &input.instructions);
        let draft = Self {
            payer,
            instructions: input.instructions,
            lookup_tables: input.lookup_tables,
            explicit_units,
            wire,
            account_keys,
        };

        draft.preflight(input.simulation_lifetime)?;

        Ok(draft)
    }

    /// Prepends maintained price/limit builders, retaining user instruction order.
    pub(super) fn priced_message(
        &self,
        lifetime: &BlockhashValue,
        budget: (NonZeroU32, ResolvedPriorityFee),
    ) -> Result<Message, SmartTransactionError> {
        let mut instructions = budget_instructions(budget)?;
        instructions.extend_from_slice(&self.instructions);
        compile_message(self.payer, lifetime, &instructions, self.lookup_tables)
    }

    /// Rejects a priced layout the caller's simulation lifetime cannot encode.
    fn preflight(&self, lifetime: &BlockhashValue) -> Result<(), SmartTransactionError> {
        // Budget instruction sizes are fixed; this catches final-layout failures
        // before spending RPC calls, using the caller's actual simulation lifetime.
        let units = self.explicit_units.unwrap_or(NonZeroU32::MIN);
        self.priced_message(lifetime, (units, PREFLIGHT_FEE))?;

        Ok(())
    }
}

/// Strips caller budget instructions and resolves the payer and compute policy.
fn prepare_input(
    input: &mut SmartTransactionInput<'_>,
    options: SmartTransactionOptions<'_>,
) -> Result<(Address, Option<NonZeroU32>), SmartTransactionError> {
    input
        .instructions
        .retain(|instruction| instruction.program_address != PROGRAM);
    let explicit_units = input
        .compute_unit_limit
        .map(|units| {
            NonZeroU32::new(units).ok_or(SmartTransactionError::NonPositiveComputeUnitLimit)
        })
        .transpose()?;
    let payer = input
        .fee_payer
        .or_else(|| input.signers.first().copied())
        .ok_or(SmartTransactionError::MissingFeePayer)?;

    if explicit_units.is_none() {
        buffer_compute_units(0, options.compute_buffer)?;
    }

    Ok((payer, explicit_units))
}

/// Compiles the exact simulation wire before fee identity collection.
fn simulation_wire(
    payer: Address,
    input: &SmartTransactionInput<'_>,
) -> Result<Vec<u8>, SmartTransactionError> {
    let message = compile_message(
        payer,
        input.simulation_lifetime,
        &input.instructions,
        input.lookup_tables,
    )?;

    Ok(Transaction::new(message).encode(false)?)
}

/// Builds the maintained price-then-limit prefix for one positive budget.
fn budget_instructions(
    (units, fee): (NonZeroU32, ResolvedPriorityFee),
) -> Result<Vec<Instruction>, SmartTransactionError> {
    Ok(vec![
        SetComputeUnitPrice {
            micro_lamports: fee.rate,
        }
        .instruction(BUDGET_CODEC_LIMITS)?,
        SetComputeUnitLimit { units: units.get() }.instruction(BUDGET_CODEC_LIMITS)?,
    ])
}

/// Compiles a v0 message against the caller's blockhash and lookup tables.
fn compile_message(
    payer: Address,
    lifetime: &BlockhashValue,
    instructions: &[Instruction],
    lookup_tables: &[LookupTable],
) -> Result<Message, SmartTransactionError> {
    Ok(Message::compile(
        Version::V0,
        payer,
        Lifetime::Blockhash {
            blockhash: lifetime.blockhash,
            last_valid_block_height: lifetime.last_valid_block_height,
        },
        instructions,
        lookup_tables,
    )?)
}

/// Keeps payer first and original account encounter order without adding program identities.
fn account_keys(payer: Address, instructions: &[Instruction]) -> Vec<Address> {
    let mut seen = BTreeSet::from([payer]);
    let mut keys = vec![payer];

    for account in instructions
        .iter()
        .flat_map(|instruction| &instruction.accounts)
    {
        if seen.insert(account.address) {
            keys.push(account.address);
        }
    }

    keys
}
