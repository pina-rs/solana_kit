//! Sequential byte deployment is portable; artifact discovery stays host-owned.

use crate::{
    Object, Surfnet,
    cheatcodes::{RegisterIdl, WriteProgram},
    errors::SurfpoolRpcError,
    hex::HexBytes,
};
use solana_kit_client::Transport;
use solana_kit_core::Address;
use std::num::NonZeroUsize;

/// Pinned upstream deployment chunk size of 15 MiB.
pub const PROGRAM_CHUNK_BYTES: usize = 15 * 1024 * 1024;

/// Program byte deployment with an optional already parsed IDL.
#[derive(Clone, Debug)]
pub struct DeployOptions<'a> {
    /// Destination program address.
    pub program_id: Address,
    /// Caller-provided compiled program bytes, borrowed without file access.
    pub so_bytes: &'a [u8],
    /// Optional IDL object; its address is set to `program_id` during registration.
    pub idl: Option<Object>,
    /// Explicit nonzero chunk limit; the client JSON budget still applies.
    pub chunk_bytes: NonZeroUsize,
}

impl<'a> DeployOptions<'a> {
    /// Uses the pinned 15 MiB chunk size, requiring an adequate client body budget.
    pub const fn new(program_id: Address, so_bytes: &'a [u8]) -> Self {
        Self {
            program_id,
            so_bytes,
            idl: None,
            chunk_bytes: NonZeroUsize::new(PROGRAM_CHUNK_BYTES)
                .expect("pinned chunk size is nonzero"),
        }
    }
}

impl<T: Transport> Surfnet<T> {
    /// Writes each chunk once, in order, then registers the optional IDL.
    ///
    /// # Errors
    /// Stops on the first exchange failure or an unrepresentable byte offset.
    /// Partial writes remain visible upstream; the method never retries or rolls back.
    pub async fn deploy(&self, options: DeployOptions<'_>) -> Result<Address, DeployError> {
        for (index, chunk) in options
            .so_bytes
            .chunks(options.chunk_bytes.get())
            .enumerate()
        {
            let offset = index
                .checked_mul(options.chunk_bytes.get())
                .and_then(|offset| u64::try_from(offset).ok())
                .ok_or(DeployError::OffsetOverflow)?;
            self.execute(WriteProgram {
                program_id: options.program_id,
                data: HexBytes(chunk.to_vec()),
                offset,
                authority: None,
            })
            .await
            .map_err(DeployError::Rpc)?;
        }

        if let Some(mut idl) = options.idl {
            idl.insert("address".into(), options.program_id.to_string().into());
            self.execute(RegisterIdl { idl, slot: None })
                .await
                .map_err(DeployError::Rpc)?;
        }

        Ok(options.program_id)
    }
}

/// Deployment preserves its failing RPC call or exact offset boundary failure.
#[derive(Debug)]
pub enum DeployError {
    /// Program data offset cannot be represented by the wire's unsigned integer.
    OffsetOverflow,
    /// The original RPC failure.
    Rpc(SurfpoolRpcError),
}

impl core::fmt::Display for DeployError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OffsetOverflow => formatter.write_str("program byte offset exceeds u64"),
            Self::Rpc(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for DeployError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Rpc(error) => Some(error),
            Self::OffsetOverflow => None,
        }
    }
}
