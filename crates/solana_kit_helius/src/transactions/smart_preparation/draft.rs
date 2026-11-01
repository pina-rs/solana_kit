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

        let message = Message::compile(
            Version::V0,
            payer,
            Lifetime::Blockhash {
                blockhash: input.simulation_lifetime.blockhash,
                last_valid_block_height: input.simulation_lifetime.last_valid_block_height,
            },
            &input.instructions,
            input.lookup_tables,
        )?;
        let wire = Transaction::new(message).encode(false)?;
        let account_keys = account_keys(payer, &input.instructions);
        let draft = Self {
            payer,
            instructions: input.instructions,
            lookup_tables: input.lookup_tables,
            explicit_units,
            wire,
            account_keys,
        };

        // Budget instruction sizes are fixed; this catches final-layout failures
        // before spending RPC calls, using the caller's actual simulation lifetime.
        draft.priced_message(
            input.simulation_lifetime,
            (explicit_units.unwrap_or(NonZeroU32::MIN), PREFLIGHT_FEE),
        )?;

        Ok(draft)
    }

    /// Prepends maintained price/limit builders, retaining user instruction order.
    pub(super) fn priced_message(
        &self,
        lifetime: &BlockhashValue,
        budget: (NonZeroU32, ResolvedPriorityFee),
    ) -> Result<Message, SmartTransactionError> {
        let (units, fee) = budget;
        let mut instructions = vec![
            SetComputeUnitPrice {
                micro_lamports: fee.rate,
            }
            .instruction(BUDGET_CODEC_LIMITS)?,
            SetComputeUnitLimit { units: units.get() }.instruction(BUDGET_CODEC_LIMITS)?,
        ];
        instructions.extend_from_slice(&self.instructions);
        Ok(Message::compile(
            Version::V0,
            self.payer,
            Lifetime::Blockhash {
                blockhash: lifetime.blockhash,
                last_valid_block_height: lifetime.last_valid_block_height,
            },
            &instructions,
            self.lookup_tables,
        )?)
    }
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
