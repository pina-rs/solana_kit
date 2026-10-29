//! Explicit native inputs reuse Surfnet's sequential, cancel-on-drop deployment.

use solana_kit_client::Transport;
use solana_kit_core::Address;
use solana_kit_surfpool::{
    Surfnet,
    cheatcodes::RegisterIdl,
    deploy::{DeployError as ProgramDeployError, DeployOptions as ProgramDeployOptions},
};
use std::{borrow::Cow, fmt, io, num::NonZeroUsize, path::Path};
use tokio::{fs, io::AsyncReadExt};

mod artifacts;
mod idl;
pub use artifacts::{ArtifactRole, DiscoveryError, ProgramArtifacts, discover_program_artifacts};

#[cfg(feature = "keys")]
mod program;
#[cfg(feature = "keys")]
pub use program::{DeployProgramError, DeployProgramOptions, MAX_KEYPAIR_BYTES, deploy_program};

/// Default maximum compiled program input: 16 MiB.
pub const MAX_PROGRAM_BYTES: usize = 16 * 1024 * 1024;
/// Default maximum IDL JSON input: 1 MiB.
pub const MAX_IDL_BYTES: usize = 1024 * 1024;

/// Exactly one caller-provided program input, with no artifact discovery.
#[derive(Clone, Copy)]
pub enum ProgramSource<'a> {
    /// Compiled program bytes borrowed through the deployment.
    Bytes(&'a [u8]),
    /// Explicit path to a regular compiled program file.
    Path(&'a Path),
}

impl fmt::Debug for ProgramSource<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytes(bytes) => formatter
                .debug_struct("Bytes")
                .field("len", &bytes.len())
                .finish(),
            Self::Path(_) => formatter.debug_struct("Path").finish_non_exhaustive(),
        }
    }
}

/// Explicit program destination, inputs and finite native read policy.
#[derive(Clone, Copy)]
pub struct DeployOptions<'a> {
    /// Destination program address, supplied without reading a keypair file.
    pub program_id: Address,
    /// Mutually exclusive borrowed bytes or explicit file path.
    pub program: ProgramSource<'a>,
    /// Optional regular IDL JSON file, opened only after every program write succeeds.
    pub idl_path: Option<&'a Path>,
    /// Nonzero portable write chunk size; the existing client wire budget still applies.
    pub chunk_bytes: NonZeroUsize,
    /// Maximum program input length, including caller-borrowed bytes.
    pub max_program_bytes: usize,
    /// Maximum IDL input length, checked before parsing.
    pub max_idl_bytes: usize,
}

impl<'a> DeployOptions<'a> {
    /// Uses the portable chunk size and finite default program/IDL read limits.
    pub const fn new(program_id: Address, program: ProgramSource<'a>) -> Self {
        Self {
            program_id,
            program,
            idl_path: None,
            chunk_bytes: NonZeroUsize::new(solana_kit_surfpool::deploy::PROGRAM_CHUNK_BYTES)
                .expect("default chunk size is nonzero"),
            max_program_bytes: MAX_PROGRAM_BYTES,
            max_idl_bytes: MAX_IDL_BYTES,
        }
    }
}

impl fmt::Debug for DeployOptions<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeployOptions")
            .field("program", &self.program)
            .field("has_idl", &self.idl_path.is_some())
            .field("chunk_bytes", &self.chunk_bytes)
            .field("max_program_bytes", &self.max_program_bytes)
            .field("max_idl_bytes", &self.max_idl_bytes)
            .finish_non_exhaustive()
    }
}

/// Deploys explicit program input, then reads and registers an optional IDL object.
///
/// No project, home or environment search occurs. Each file must be regular;
/// metadata and incremental reads enforce its limit even if it grows. The vector
/// holds at most the selected limit, with a separate fixed 8 KiB read buffer.
/// Callers must supply trusted paths; concurrent path replacement is not prevented.
/// Tokio may finish an already submitted filesystem operation after cancellation,
/// but dropping this future never starts another RPC or IDL read.
///
/// # Errors
/// Stops at the first file, size, IDL-object parsing or RPC failure. Earlier
/// writes remain visible: there are no retries or rollback. Diagnostics redact
/// paths and data; inspect error variants explicitly for original typed causes.
pub async fn deploy<T: Transport>(
    surfnet: &Surfnet<T>,
    options: DeployOptions<'_>,
) -> Result<Address, DeployError> {
    let program = match options.program {
        ProgramSource::Bytes(bytes) => {
            check_length(bytes.len(), options.max_program_bytes, DeployFile::Program)?;
            Cow::Borrowed(bytes)
        }
        ProgramSource::Path(path) => {
            Cow::Owned(read_file(path, options.max_program_bytes, DeployFile::Program).await?)
        }
    };

    let mut portable = ProgramDeployOptions::new(options.program_id, &program);
    portable.chunk_bytes = options.chunk_bytes;
    surfnet
        .deploy(portable)
        .await
        .map_err(DeployError::Deploy)?;

    if let Some(path) = options.idl_path {
        let bytes = read_file(path, options.max_idl_bytes, DeployFile::Idl).await?;
        let mut idl = idl::object(&bytes)?;
        idl.insert("address".into(), options.program_id.to_string().into());
        surfnet
            .execute(RegisterIdl { idl, slot: None })
            .await
            .map_err(|cause| DeployError::Deploy(ProgramDeployError::Rpc(cause)))?;
    }

    Ok(options.program_id)
}

/// Which explicit file input failed, without revealing its path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeployFile {
    /// Compiled program input.
    Program,
    /// IDL JSON input.
    Idl,
    /// Explicit keypair JSON input used by key-backed deployment.
    Keypair,
}

/// Redacted deployment failure with original causes available by variant matching.
///
/// Implicit formatting and the standard error chain omit file paths, JSON content
/// and transport/provider payloads. The public variant fields retain those causes.
pub enum DeployError {
    /// Native file operation failed; the original I/O error remains available.
    Io {
        /// Input whose operation failed.
        input: DeployFile,
        /// Original native I/O cause.
        cause: io::Error,
    },
    /// The explicit input path did not name a regular file.
    NotRegular {
        /// Input whose file type was rejected.
        input: DeployFile,
    },
    /// Program or IDL input exceeded its caller-selected byte limit.
    TooLarge {
        /// Input whose byte limit was exceeded.
        input: DeployFile,
        /// Maximum accepted input length.
        limit: usize,
    },
    /// IDL was not a JSON object; retains the original JSON syntax or shape error.
    IdlJson(serde_json::Error),
    /// IDL reached serde_json's existing recursion ceiling on container entry.
    ///
    /// As with the original decoder, the root object counts and at most 127
    /// nested object/array containers are accepted along any path. Program writes
    /// have already completed when this error is returned.
    IdlNesting,
    /// Original portable program-write or IDL-registration failure.
    Deploy(ProgramDeployError),
}

impl fmt::Debug for DeployError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DeployError")
            .field("message", &format_args!("{self}"))
            .finish_non_exhaustive()
    }
}

impl fmt::Display for DeployError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { input, .. } => write!(formatter, "{input:?} file read failed"),
            Self::NotRegular { input } => {
                write!(formatter, "{input:?} input is not a regular file")
            }
            Self::TooLarge { input, limit } => {
                write!(formatter, "{input:?} input exceeds {limit} bytes")
            }
            Self::IdlJson(_) => formatter.write_str("IDL input must be a JSON object"),
            Self::IdlNesting => formatter.write_str("IDL input exceeds the JSON recursion limit"),
            Self::Deploy(cause) => cause.fmt(formatter),
        }
    }
}

impl std::error::Error for DeployError {}

async fn read_file(path: &Path, limit: usize, input: DeployFile) -> Result<Vec<u8>, DeployError> {
    let io_error = |cause| DeployError::Io { input, cause };
    let metadata = fs::metadata(path).await.map_err(io_error)?;
    check_metadata(&metadata, limit, input)?;
    let mut file = fs::File::open(path).await.map_err(io_error)?;
    check_metadata(&file.metadata().await.map_err(io_error)?, limit, input)?;
    let mut bytes = Vec::new();
    let mut buffer = [0; 8192];

    loop {
        let count = file.read(&mut buffer).await.map_err(io_error)?;

        if count == 0 {
            return Ok(bytes);
        }

        if count > limit.saturating_sub(bytes.len()) {
            return Err(DeployError::TooLarge { input, limit });
        }

        bytes.extend_from_slice(&buffer[..count]);
    }
}

fn check_metadata(
    metadata: &std::fs::Metadata,
    limit: usize,
    input: DeployFile,
) -> Result<(), DeployError> {
    if !metadata.is_file() {
        return Err(DeployError::NotRegular { input });
    }

    if u128::from(metadata.len()) > limit as u128 {
        return Err(DeployError::TooLarge { input, limit });
    }

    Ok(())
}

fn check_length(length: usize, limit: usize, input: DeployFile) -> Result<(), DeployError> {
    if length > limit {
        return Err(DeployError::TooLarge { input, limit });
    }

    Ok(())
}
