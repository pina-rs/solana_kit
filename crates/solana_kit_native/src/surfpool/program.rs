//! Verified keypair addresses feed the existing explicit-path deployment owner.

use super::{DeployError, DeployFile, DeployOptions, ProgramArtifacts, ProgramSource, deploy};
use solana_kit_client::Transport;
use solana_kit_core::{
    Address,
    crypto::{Keypair, KeypairImportError},
};
use solana_kit_surfpool::Surfnet;
use std::{fmt, num::NonZeroUsize, path::Path};
use tokio::{fs, io::AsyncReadExt};
use zeroize::Zeroizing;

mod key_json;

/// Default maximum Solana-keygen keypair JSON input: 1 KiB.
pub const MAX_KEYPAIR_BYTES: usize = 1024;

/// Finite key-file and existing program/IDL deployment budgets.
#[derive(Clone, Copy, Debug)]
pub struct DeployProgramOptions {
    /// Maximum keypair JSON file bytes before decoding.
    pub max_keypair_bytes: usize,
    /// Maximum compiled program bytes.
    pub max_program_bytes: usize,
    /// Maximum optional IDL bytes.
    pub max_idl_bytes: usize,
    /// Nonzero write chunk size; the caller's client wire budget still applies.
    pub chunk_bytes: NonZeroUsize,
}

impl Default for DeployProgramOptions {
    fn default() -> Self {
        Self {
            max_keypair_bytes: MAX_KEYPAIR_BYTES,
            max_program_bytes: super::MAX_PROGRAM_BYTES,
            max_idl_bytes: super::MAX_IDL_BYTES,
            chunk_bytes: NonZeroUsize::new(solana_kit_surfpool::deploy::PROGRAM_CHUNK_BYTES)
                .expect("default chunk size is nonzero"),
        }
    }
}

/// Key validation or subsequent deployment failure, with explicit typed causes.
///
/// Debug, Display and the implicit error chain omit all paths and key material.
pub enum DeployProgramError {
    /// Bounded regular-key-file validation or read failed.
    KeyFile(DeployError),
    /// Key JSON was not exactly 64 unsigned byte integers.
    KeyJson(serde_json::Error),
    /// Core rejected the seed/public-key pair.
    KeyImport(KeypairImportError),
    /// Existing explicit deployment failed after key material was dropped.
    Deploy(DeployError),
}

impl fmt::Display for DeployProgramError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::KeyFile(_) => "program keypair file read failed",
            Self::KeyJson(_) => "program keypair JSON must contain 64 bytes",
            Self::KeyImport(_) => "program keypair verification failed",
            Self::Deploy(_) => "program artifact deployment failed",
        })
    }
}

impl fmt::Debug for DeployProgramError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeployProgramError")
            .field("message", &format_args!("{self}"))
            .finish_non_exhaustive()
    }
}

impl std::error::Error for DeployProgramError {}

/// Verifies the selected keypair, drops all owned key material, then deploys.
///
/// No fallback to later roots occurs on key failure. IDL presence follows the
/// discovery snapshot: a previously absent IDL is ignored; a selected IDL that
/// disappears fails only after program writes, following [`deploy`]. The raw input
/// vector, read scratch and decoded array use zeroizing storage on every exit/drop.
///
/// # Errors
/// Returns the original typed file, JSON, key import or deployment failure.
/// Cancellation starts no later filesystem or RPC operation; already submitted
/// Tokio filesystem work may finish independently, as with explicit deployment.
pub async fn deploy_program<T: Transport>(
    surfnet: &Surfnet<T>,
    artifacts: &ProgramArtifacts,
    options: DeployProgramOptions,
) -> Result<Address, DeployProgramError> {
    let address = {
        let raw = read_key(artifacts.keypair_path(), options.max_keypair_bytes)
            .await
            .map_err(DeployProgramError::KeyFile)?;
        let decoded: key_json::KeyBytes =
            serde_json::from_slice(&raw).map_err(DeployProgramError::KeyJson)?;
        let key = Keypair::from_bytes(&decoded.0[..]).map_err(DeployProgramError::KeyImport)?;

        key.address()
    };

    let mut explicit = DeployOptions::new(address, ProgramSource::Path(artifacts.program_path()));
    explicit.idl_path = artifacts.idl_path();
    explicit.chunk_bytes = options.chunk_bytes;
    explicit.max_program_bytes = options.max_program_bytes;
    explicit.max_idl_bytes = options.max_idl_bytes;

    deploy(surfnet, explicit)
        .await
        .map_err(DeployProgramError::Deploy)
}

async fn read_key(path: &Path, limit: usize) -> Result<Zeroizing<Vec<u8>>, DeployError> {
    let input = DeployFile::Keypair;
    let io_error = |cause| DeployError::Io { input, cause };
    let metadata = fs::metadata(path).await.map_err(io_error)?;
    super::check_metadata(&metadata, limit, input)?;
    let mut file = fs::File::open(path).await.map_err(io_error)?;
    super::check_metadata(&file.metadata().await.map_err(io_error)?, limit, input)?;

    let mut bytes = Zeroizing::new(Vec::new());
    // Reserve once so extending secret data never frees an unwiped earlier allocation.
    bytes
        .try_reserve_exact(limit)
        .map_err(|cause| io_error(std::io::Error::other(cause)))?;
    let mut buffer = Zeroizing::new([0; 8192]);

    loop {
        let count = file.read(&mut buffer[..]).await.map_err(io_error)?;

        if count == 0 {
            return Ok(bytes);
        }

        if count > limit.saturating_sub(bytes.len()) {
            return Err(DeployError::TooLarge { input, limit });
        }

        bytes.extend_from_slice(&buffer[..count]);
    }
}
