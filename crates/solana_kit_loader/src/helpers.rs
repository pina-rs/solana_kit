//! Source deployment plans with explicit prepared account ownership.
use crate::v3;
use solana_kit_core::{
    Address,
    codecs::{Error, Limits},
    plans::{InstructionPlan, Plan},
};

/// The pinned client's default write payload size.
pub const DEFAULT_WRITE_CHUNK_SIZE: usize = 900;
const WRITE_PREFIX_SIZE: usize = 16;

/// Prepared buffer identity and authority shared by every write.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BufferWriter {
    /// Existing buffer address.
    pub buffer: Address,
    /// Current buffer authority.
    pub authority: Address,
}

impl v3::DeployWithMaxDataLenAccounts {
    /// Writes the prepared buffer, then deploys with an explicit or byte-length capacity.
    /// # Errors
    /// Preserves chunk, allocation, offset and instruction limit errors.
    pub fn plan(
        &self,
        bytes: &[u8],
        max_data_len: Option<u64>,
        chunk_size: usize,
        limits: Limits,
    ) -> Result<InstructionPlan, Error> {
        let max_data_len =
            max_data_len.unwrap_or(u64::try_from(bytes.len()).map_err(|_| Error::Overflow)?);
        let action = self.instruction(
            v3::PROGRAM,
            &v3::DeployWithMaxDataLen { max_data_len },
            &[],
            limits,
        )?;
        BufferWriter {
            buffer: self.buffer_account,
            authority: self.authority,
        }
        .plan(bytes, chunk_size, action, limits)
    }
}

impl v3::UpgradeAccounts {
    /// Writes the prepared buffer, then upgrades only after every write.
    /// # Errors
    /// Preserves chunk, allocation, offset and instruction limit errors.
    pub fn plan(
        &self,
        bytes: &[u8],
        chunk_size: usize,
        limits: Limits,
    ) -> Result<InstructionPlan, Error> {
        let action = self.instruction(v3::PROGRAM, &v3::Upgrade {}, &[], limits)?;
        BufferWriter {
            buffer: self.buffer_account,
            authority: self.authority,
        }
        .plan(bytes, chunk_size, action, limits)
    }
}

impl BufferWriter {
    /// Writes chunks in increasing offset order, then performs the final action.
    ///
    /// The caller supplies an already prepared deploy or upgrade instruction.
    /// No accounts are created and no transaction is sent. Empty bytes produce
    /// only the final action, matching the source's empty write sequence.
    /// # Errors
    /// Rejects zero chunk size, u32 offset overflow, allocation or codec limits.
    pub fn plan(
        self,
        bytes: &[u8],
        chunk_size: usize,
        final_action: solana_kit_core::Instruction,
        limits: Limits,
    ) -> Result<InstructionPlan, Error> {
        if chunk_size == 0 {
            return Err(Error::InvalidValue);
        }

        let count = bytes
            .len()
            .div_ceil(chunk_size)
            .checked_add(1)
            .ok_or(Error::Overflow)?;

        if count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let mut plans = alloc::vec::Vec::new();
        plans
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        let mut offset = 0_u32;

        for chunk in bytes.chunks(chunk_size) {
            let instruction = v3::WriteAccounts {
                buffer_account: self.buffer,
                buffer_authority: self.authority,
            }
            .instruction(
                v3::PROGRAM,
                &v3::Write {
                    offset,
                    bytes: write_payload(chunk, limits)?,
                },
                &[],
                limits,
            )?;
            plans.push(instruction.into());
            offset = offset
                .checked_add(u32::try_from(chunk.len()).map_err(|_| Error::Overflow)?)
                .ok_or(Error::Overflow)?;
        }

        plans.push(final_action.into());
        Ok(Plan::sequential(plans))
    }
}

fn write_payload(bytes: &[u8], limits: Limits) -> Result<alloc::vec::Vec<u8>, Error> {
    let size = bytes
        .len()
        .checked_add(WRITE_PREFIX_SIZE)
        .ok_or(Error::Overflow)?;

    if size > limits.max_bytes {
        return Err(Error::LimitExceeded);
    }

    let mut output = alloc::vec::Vec::new();
    output
        .try_reserve_exact(bytes.len())
        .map_err(|_| Error::AllocationFailed)?;
    output.extend_from_slice(bytes);
    Ok(output)
}
