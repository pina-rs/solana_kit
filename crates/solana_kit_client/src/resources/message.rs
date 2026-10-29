//! Resource edits retain caller-owned instructions and explicit version context.

use std::borrow::Cow;

use solana_kit_compute_budget::{PROGRAM, SetComputeUnitLimit, SetLoadedAccountsDataSizeLimit};
use solana_kit_core::{
    Address, Instruction,
    codecs::Limits,
    transaction::{Lifetime, LookupTable, Message, V1Config, Version},
};

use super::{
    MAX_COMPUTE_UNIT_LIMIT, PROVISORY_COMPUTE_UNIT_LIMIT,
    PROVISORY_LOADED_ACCOUNTS_DATA_SIZE_LIMIT, ResourceError,
};

const LIMIT_INSTRUCTION_SIZE: usize = 5;
const COMPUTE_LIMIT_TAG: u8 = 2;
const LOADED_LIMIT_TAG: u8 = 4;

/// Version-specific compilation inputs; inline config and lookups cannot mix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VersionContext<'a> {
    /// Original unprefixed message, without lookup tables or inline config.
    Legacy,
    /// Version zero with caller-owned lookup table snapshots.
    V0(&'a [LookupTable]),
    /// Version one with inline resource configuration and no lookup tables.
    V1(V1Config),
}

/// An unsigned instruction draft with explicit compilation and lifetime inputs.
///
/// Borrowed inputs are cloned only when edited. Nonce advances stay in their
/// original position; compilation checks their identities against the lifetime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceMessage<'a> {
    /// Exact version and its applicable compilation options.
    pub version: VersionContext<'a>,
    /// Writable fee payer and required signer.
    pub payer: Address,
    /// Caller-established expiry or nonce metadata, never fetched implicitly.
    pub lifetime: Lifetime,
    /// Original ordered program invocations, including any nonce advance.
    pub instructions: Cow<'a, [Instruction]>,
}

impl ResourceMessage<'_> {
    /// First compute limit for legacy/v0, or the inline version-one value.
    #[must_use]
    pub fn compute_unit_limit(&self) -> Option<u32> {
        match self.version {
            VersionContext::V1(config) => config.compute_unit_limit(),
            _ => self.instruction_limit(COMPUTE_LIMIT_TAG),
        }
    }

    /// First loaded account limit for legacy/v0, or the version-one config value.
    #[must_use]
    pub fn loaded_accounts_data_size_limit(&self) -> Option<u32> {
        match self.version {
            VersionContext::V1(config) => config.loaded_accounts_data_limit(),
            _ => self.instruction_limit(LOADED_LIMIT_TAG),
        }
    }

    /// Sets or removes only the first applicable compute limit.
    ///
    /// Legacy/v0 append when absent, preserving instruction zero for a nonce.
    ///
    /// # Errors
    /// Rejects limits above 1,400,000 and instruction encoding failures. The
    /// observed estimate itself can exceed this supported construction range.
    pub fn with_compute_unit_limit(mut self, limit: Option<u32>) -> Result<Self, ResourceError> {
        if let Some(limit) = limit
            && limit > MAX_COMPUTE_UNIT_LIMIT
        {
            return Err(ResourceError::ComputeUnitLimitOutOfRange(limit));
        }

        if let VersionContext::V1(config) = self.version {
            self.version = VersionContext::V1(limit.map_or_else(
                || config.without_compute_unit_limit(),
                |limit| config.with_compute_unit_limit(limit),
            ));
        } else {
            let instruction = limit
                .map(|units| SetComputeUnitLimit { units }.instruction(instruction_limits()))
                .transpose()
                .map_err(ResourceError::Instruction)?;
            self.set_instruction_limit(COMPUTE_LIMIT_TAG, instruction);
        }

        Ok(self)
    }

    /// Sets or removes only the first applicable loaded account byte limit.
    ///
    /// Unlike tuning, this explicit setter supports every version. Trailing
    /// bytes and account metadata on an existing match are ignored, as in Dart.
    ///
    /// # Errors
    /// Returns an instruction encoding failure without changing the input.
    pub fn with_loaded_accounts_data_size_limit(
        mut self,
        limit: Option<u32>,
    ) -> Result<Self, ResourceError> {
        if let VersionContext::V1(config) = self.version {
            self.version = VersionContext::V1(limit.map_or_else(
                || config.without_loaded_accounts_data_limit(),
                |limit| config.with_loaded_accounts_data_limit(limit),
            ));
        } else {
            let instruction = limit
                .map(|account_data_size_limit| {
                    SetLoadedAccountsDataSizeLimit {
                        account_data_size_limit,
                    }
                    .instruction(instruction_limits())
                })
                .transpose()
                .map_err(ResourceError::Instruction)?;
            self.set_instruction_limit(LOADED_LIMIT_TAG, instruction);
        }

        Ok(self)
    }

    /// Reserves zero placeholders only for missing applicable resource limits.
    ///
    /// All existing values survive; loaded bytes are filled only for version one.
    ///
    /// # Errors
    /// Returns an instruction encoding failure while preparing a legacy/v0 limit.
    pub fn fill_provisory_resource_limits(mut self) -> Result<Self, ResourceError> {
        if self.compute_unit_limit().is_none() {
            self = self.with_compute_unit_limit(Some(PROVISORY_COMPUTE_UNIT_LIMIT))?;
        }

        if matches!(self.version, VersionContext::V1(_))
            && self.loaded_accounts_data_size_limit().is_none()
        {
            self = self.with_loaded_accounts_data_size_limit(Some(
                PROVISORY_LOADED_ACCOUNTS_DATA_SIZE_LIMIT,
            ))?;
        }

        Ok(self)
    }

    /// Freezes this draft using the existing version-specific compiler.
    ///
    /// # Errors
    /// Preserves compilation errors, including unresolved lifetime, invalid nonce
    /// prerequisite, lookup conflicts, privileges, or an oversized transaction.
    pub fn compile(&self) -> Result<Message, ResourceError> {
        let compiled = match self.version {
            VersionContext::Legacy => Message::compile(
                Version::Legacy,
                self.payer,
                self.lifetime,
                &self.instructions,
                &[],
            ),
            VersionContext::V0(tables) => Message::compile(
                Version::V0,
                self.payer,
                self.lifetime,
                &self.instructions,
                tables,
            ),
            VersionContext::V1(config) => {
                Message::compile_v1(self.payer, self.lifetime, &self.instructions, config)
            }
        };

        compiled.map_err(ResourceError::Compilation)
    }

    fn instruction_limit(&self, tag: u8) -> Option<u32> {
        let index = self.limit_index(tag)?;
        let data = &self.instructions[index].data;
        let bytes = [data[1], data[2], data[3], data[4]];

        Some(u32::from_le_bytes(bytes))
    }

    fn limit_index(&self, tag: u8) -> Option<usize> {
        self.instructions.iter().position(|instruction| {
            instruction.program_address == PROGRAM
                && instruction.data.len() >= LIMIT_INSTRUCTION_SIZE
                && instruction.data[0] == tag
        })
    }

    fn set_instruction_limit(&mut self, tag: u8, replacement: Option<Instruction>) {
        let index = self.limit_index(tag);

        // Equal values retain the original accounts and trailing bytes, matching Dart.
        if replacement.as_ref().is_some_and(|instruction| {
            index.is_some_and(|index| {
                &self.instructions[index].data[..LIMIT_INSTRUCTION_SIZE]
                    == instruction.data.as_slice()
            })
        }) {
            return;
        }

        match (index, replacement) {
            (Some(index), Some(instruction)) => self.instructions.to_mut()[index] = instruction,
            (Some(index), None) => {
                self.instructions.to_mut().remove(index);
            }
            (None, Some(instruction)) => self.instructions.to_mut().push(instruction),
            (None, None) => {}
        }
    }
}

fn instruction_limits() -> Limits {
    Limits::new(LIMIT_INSTRUCTION_SIZE, 0)
}
