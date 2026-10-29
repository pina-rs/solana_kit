//! Ordered explicit target roots replace ambient project and environment searches.

use std::{
    fmt, io,
    path::{Path, PathBuf},
};
use tokio::fs;

/// One selected same-root program/keypair pair and its IDL-presence snapshot.
///
/// Discovery does not read file contents. Callers supply trusted paths; subsequent
/// filesystem changes are not prevented. Debug omits every supplied path.
pub struct ProgramArtifacts {
    program_path: PathBuf,
    keypair_path: PathBuf,
    idl_path: Option<PathBuf>,
    candidate_index: usize,
}

impl ProgramArtifacts {
    /// Explicit compiled program path in the selected target root.
    pub fn program_path(&self) -> &Path {
        &self.program_path
    }
    /// Explicit keypair JSON path in that same target root.
    pub fn keypair_path(&self) -> &Path {
        &self.keypair_path
    }
    /// IDL path only when it named a regular file at discovery time.
    pub fn idl_path(&self) -> Option<&Path> {
        self.idl_path.as_deref()
    }
    /// Zero-based position of the selected root in the caller's original order.
    pub const fn candidate_index(&self) -> usize {
        self.candidate_index
    }
}

impl fmt::Debug for ProgramArtifacts {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProgramArtifacts")
            .field("candidate_index", &self.candidate_index)
            .field("has_idl", &self.idl_path.is_some())
            .finish_non_exhaustive()
    }
}

/// Candidate file whose metadata failed, without disclosing its path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactRole {
    /// Compiled `.so` program.
    Program,
    /// Solana-keygen JSON keypair.
    Keypair,
    /// Optional Anchor IDL JSON.
    Idl,
}

/// Discovery failures redact paths while preserving explicit original I/O access.
pub enum DiscoveryError {
    /// Program name was not one nonempty filename stem.
    InvalidName,
    /// No target roots were supplied.
    NoRoots,
    /// A root was empty or not absolute; all roots are checked before I/O.
    InvalidRoot {
        /// Zero-based root position.
        index: usize,
    },
    /// No single root contained both regular program and keypair files.
    NotFound,
    /// Metadata failed for a reason other than a missing candidate.
    Io {
        /// Zero-based root position.
        index: usize,
        /// Candidate file role.
        role: ArtifactRole,
        /// Original I/O error; inspect deliberately because it may contain paths.
        cause: io::Error,
    },
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName => formatter.write_str("program name must be one filename stem"),
            Self::NoRoots => formatter.write_str("program discovery requires target roots"),
            Self::InvalidRoot { index } => {
                write!(formatter, "target root {index} must be absolute")
            }
            Self::NotFound => {
                formatter.write_str("no target root contains the program/keypair pair")
            }
            Self::Io { index, role, .. } => {
                write!(formatter, "target root {index} {role:?} metadata failed")
            }
        }
    }
}

impl fmt::Debug for DiscoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DiscoveryError")
            .field("message", &format_args!("{self}"))
            .finish_non_exhaustive()
    }
}

impl std::error::Error for DiscoveryError {}

/// Selects the first regular program/keypair pair in explicit caller root order.
///
/// Each root is a target directory, containing `deploy/<name>.so` and
/// `deploy/<name>-keypair.json`. Only the winning root's `idl/<name>.json` is
/// considered. Missing/nonregular candidates skip; no sorting, deduplication,
/// canonicalization, environment lookup or ancestor search occurs.
///
/// # Errors
/// Rejects invalid names or any nonabsolute/empty root before filesystem work.
/// Permission and other nonmissing metadata failures propagate with candidate
/// index and role, a stricter policy than Dart's existence checks. A selected
/// malformed key remains selected; content is not examined or retried here.
pub async fn discover_program_artifacts(
    program_name: &str,
    target_roots: &[PathBuf],
) -> Result<ProgramArtifacts, DiscoveryError> {
    validate(program_name, target_roots)?;

    for (index, root) in target_roots.iter().enumerate() {
        let program_path = root.join("deploy").join(format!("{program_name}.so"));
        let keypair_path = root
            .join("deploy")
            .join(format!("{program_name}-keypair.json"));

        if !regular(&program_path, index, ArtifactRole::Program).await?
            || !regular(&keypair_path, index, ArtifactRole::Keypair).await?
        {
            continue;
        }

        let idl = root.join("idl").join(format!("{program_name}.json"));
        let idl_path = regular(&idl, index, ArtifactRole::Idl)
            .await?
            .then_some(idl);

        return Ok(ProgramArtifacts {
            program_path,
            keypair_path,
            idl_path,
            candidate_index: index,
        });
    }

    Err(DiscoveryError::NotFound)
}

/// Reject platform-specific path syntax and every invalid root before any probe.
fn validate(name: &str, roots: &[PathBuf]) -> Result<(), DiscoveryError> {
    if name.is_empty()
        || matches!(name, "." | "..")
        || name
            .chars()
            .any(|character| character.is_control() || "/\\:".contains(character))
    {
        return Err(DiscoveryError::InvalidName);
    }

    if roots.is_empty() {
        return Err(DiscoveryError::NoRoots);
    }

    for (index, root) in roots.iter().enumerate() {
        if !root.is_absolute() {
            return Err(DiscoveryError::InvalidRoot { index });
        }
    }

    Ok(())
}

async fn regular(path: &Path, index: usize, role: ArtifactRole) -> Result<bool, DiscoveryError> {
    match fs::metadata(path).await {
        Ok(metadata) => Ok(metadata.is_file()),

        Err(cause) if cause.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(cause) => Err(DiscoveryError::Io { index, role, cause }),
    }
}
