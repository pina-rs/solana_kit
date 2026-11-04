//! Pinned, hash-checked program artifacts supplied by the caller's local tree.
//!
//! No artifact is downloaded or rebuilt here. The exact bytes are verified before
//! the node starts and again against the node's ProgramData after readiness.

use solana_kit_core::Address;
use std::{
    env, fmt, fs, io,
    path::{Path, PathBuf},
};

/// A canonical external program and the exact artifact bytes it must load.
#[derive(Clone, Copy, Debug)]
pub struct ExternalProgram {
    /// Artifact metadata name in `artifacts.json`.
    pub name: &'static str,
    /// File name below the fixture directory.
    pub artifact: &'static str,
    /// Canonical program ID the artifact is built for.
    pub program_id: &'static str,
    /// Expected SHA-256 of the complete `.so` file.
    pub sha256: &'static str,
    /// Whether `artifacts.json` lists this artifact with its canonical ID.
    pub in_manifest: bool,
}

impl ExternalProgram {
    /// Parses the canonical program ID after a successful fixture load.
    #[must_use]
    pub fn address(&self) -> Address {
        self.program_id
            .parse()
            .expect("canonical fixture program IDs are valid")
    }
}

/// Pinned Config program artifact, recorded for inspection.
///
/// Agave 4.2 already deploys this program in genesis, so the suite does not
/// repeat it with `--bpf-program`; the bytes are still hash-checked on request.
pub const CONFIG: ExternalProgram = ExternalProgram {
    name: "config",
    artifact: "config-v3.0.0.so",
    program_id: "Config1111111111111111111111111111111111111",
    sha256: "78c5f88e528337a33937da050a8d883175ec02ec39e27a9b21ec34f72465a5bd",
    in_manifest: true,
};

/// Pinned Anchor compatibility fixture with an IDL in the same directory.
pub const ANCHOR_COMPATIBILITY: ExternalProgram = ExternalProgram {
    name: "anchor_compatibility",
    artifact: "anchor_compatibility-v0.1.0.so",
    program_id: "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS",
    sha256: "11df9f013b8e757ae128a929091b524d7cf3c0f172fd46b1484b640576cacaf8",
    in_manifest: false,
};

/// Pinned Anchor foreign-event provenance fixture.
pub const ANCHOR_EVENT_IMPOSTER: ExternalProgram = ExternalProgram {
    name: "anchor_event_imposter",
    artifact: "anchor_event_imposter-v0.1.0.so",
    program_id: "8MyZkLi7NVstEPYwQoSS9VtKAcaRzGNLqxwQX4VtwW1e",
    sha256: "b201f9eec0ab9084de0ba7eaf89876542948396f30bfb8ebbf9083ff32c404b5",
    in_manifest: true,
};

/// Pinned Subscriptions program.
pub const SUBSCRIPTIONS: ExternalProgram = ExternalProgram {
    name: "subscriptions",
    artifact: "subscriptions-v0.5.0.so",
    program_id: "De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44",
    sha256: "b398830321078473ac728435d8c11f667b95d1db342635ab3df892662b351d8d",
    in_manifest: true,
};

/// Pinned SPL Account Compression program.
pub const ACCOUNT_COMPRESSION: ExternalProgram = ExternalProgram {
    name: "spl_account_compression",
    artifact: "spl_account_compression-v0.3.3.so",
    program_id: "cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK",
    sha256: "8b1bbb64e086e96fcf4a7db8bebd2db661eaf61d5692e0330517f1a96e75ee1f",
    in_manifest: true,
};

/// Pinned Noop log wrapper.
pub const NOOP: ExternalProgram = ExternalProgram {
    name: "noop",
    artifact: "noop-v0.2.0.so",
    program_id: "noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV",
    sha256: "7e5c4e0bc5eec7f1b0aa273f55f3c86b12c1c1de40e2a3d43f3e9624dba737dc",
    in_manifest: true,
};

/// Pinned MPL Bubblegum program.
pub const MPL_BUBBLEGUM: ExternalProgram = ExternalProgram {
    name: "mpl_bubblegum",
    artifact: "mpl_bubblegum-v0.12.0.so",
    program_id: "BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY",
    sha256: "5b86d4092287e989893ef8bc039a7f02229cd65ac1d1aa96201a1ead99ef51ff",
    in_manifest: true,
};

/// Pinned MPL Core program.
pub const MPL_CORE: ExternalProgram = ExternalProgram {
    name: "mpl_core",
    artifact: "mpl_core-v0.2.0.so",
    program_id: "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d",
    sha256: "431b31ed52280f87f78e53369eb8c43f9f687becc14e07cd1f82f11823aae4f6",
    in_manifest: true,
};

/// Pinned MPL Token Metadata program.
pub const MPL_TOKEN_METADATA: ExternalProgram = ExternalProgram {
    name: "mpl_token_metadata",
    artifact: "mpl_token_metadata-v1.14.0.so",
    program_id: "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
    sha256: "5603d356486af7f7c4f94cc8f1a57f1028478e0b597f0d08b5598e7e7fdd39a6",
    in_manifest: true,
};

/// Pinned Squads V4 multisig program.
pub const SQUADS_MULTISIG: ExternalProgram = ExternalProgram {
    name: "squads_multisig",
    artifact: "squads_multisig-v2.1.0.so",
    program_id: "SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf",
    sha256: "26e2ba150db168da272b350b01aac45f20d2f3443fa4cebb9a0eb406c49b4769",
    in_manifest: true,
};

/// Exact local fixture directory selected by this run.
#[derive(Debug)]
pub struct FixtureDirectory {
    path: PathBuf,
}

impl FixtureDirectory {
    /// Resolves the explicitly supplied `SOLANA_KIT_PROGRAM_FIXTURES` directory.
    pub fn resolve() -> Result<Self, FixtureError> {
        if let Some(path) = env::var_os("SOLANA_KIT_PROGRAM_FIXTURES") {
            let path = PathBuf::from(path);

            if path.is_dir() {
                return Ok(Self {
                    path: path.canonicalize().map_err(FixtureError::Io)?,
                });
            }

            return Err(FixtureError::Directory(format!(
                "SOLANA_KIT_PROGRAM_FIXTURES={} is not a directory",
                path.display()
            )));
        }

        Err(FixtureError::Directory(
            "set SOLANA_KIT_PROGRAM_FIXTURES to the pinned config/programs directory".to_owned(),
        ))
    }

    /// Directory containing the artifacts and `artifacts.json`.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Loads, ID-checks and hashes exactly the requested artifacts.
    pub fn load(&self, selection: &[ExternalProgram]) -> Result<Vec<LoadedProgram>, FixtureError> {
        let artifacts = fs::read(self.path.join("artifacts.json")).map_err(FixtureError::Io)?;
        let manifest: serde_json::Value =
            serde_json::from_slice(&artifacts).map_err(FixtureError::Manifest)?;
        let entries = manifest
            .get("artifacts")
            .and_then(serde_json::Value::as_array)
            .ok_or(FixtureError::ManifestText("missing artifacts array"))?;

        selection
            .iter()
            .map(|program| self.load_one(program, entries))
            .collect()
    }

    fn load_one(
        &self,
        program: &ExternalProgram,
        entries: &[serde_json::Value],
    ) -> Result<LoadedProgram, FixtureError> {
        if program.in_manifest {
            let entry = entries
                .iter()
                .find(|entry| {
                    entry.get("name").and_then(serde_json::Value::as_str) == Some(program.name)
                })
                .ok_or(FixtureError::ManifestText("artifact name is absent"))?;
            let manifest_id = entry
                .get("programId")
                .and_then(serde_json::Value::as_str)
                .ok_or(FixtureError::ManifestText("artifact has no programId"))?;

            if manifest_id != program.program_id {
                return Err(FixtureError::Id {
                    artifact: program.artifact,
                    expected: program.program_id.to_owned(),
                    actual: manifest_id.to_owned(),
                });
            }
        }

        let path = self.path.join(program.artifact);
        let bytes = fs::read(&path).map_err(FixtureError::Io)?;
        let digest = sha256_hex(&bytes);

        if digest != program.sha256 {
            return Err(FixtureError::Hash {
                artifact: program.artifact,
                expected: program.sha256.to_owned(),
                actual: digest,
            });
        }

        Ok(LoadedProgram {
            program: *program,
            deployed_at: program.address(),
            path,
            sha256: digest,
            elf: bytes,
        })
    }
}

/// One verified artifact and its exact bytes.
#[derive(Debug)]
pub struct LoadedProgram {
    /// Canonical descriptor.
    pub program: ExternalProgram,
    /// Address where this node actually deploys the exact artifact.
    pub deployed_at: Address,
    /// Exact source path used for `--bpf-program`.
    pub path: PathBuf,
    /// Recomputed SHA-256, equal to the pinned value.
    pub sha256: String,
    /// Complete ELF bytes, retained for the post-readiness ProgramData check.
    pub elf: Vec<u8>,
}

/// Artifact discovery or integrity failed before any node was spawned.
#[derive(Debug)]
pub enum FixtureError {
    /// A required filesystem operation failed.
    Io(io::Error),
    /// `artifacts.json` is malformed.
    Manifest(serde_json::Error),
    /// `artifacts.json` lacks a required field.
    ManifestText(&'static str),
    /// No fixture directory was selected.
    Directory(String),
    /// The manifest and the SDK disagree on the canonical program ID.
    Id {
        /// Artifact file name.
        artifact: &'static str,
        /// SDK canonical ID.
        expected: String,
        /// Manifest ID.
        actual: String,
    },
    /// The file bytes differ from the pinned hash.
    Hash {
        /// Artifact file name.
        artifact: &'static str,
        /// Recorded hash.
        expected: String,
        /// Recomputed hash.
        actual: String,
    },
}

impl fmt::Display for FixtureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "fixture I/O failed: {error}"),
            Self::Manifest(error) => write!(formatter, "fixture manifest is invalid: {error}"),
            Self::ManifestText(message) => write!(formatter, "fixture manifest: {message}"),
            Self::Directory(message) => write!(formatter, "fixture directory: {message}"),
            Self::Id {
                artifact,
                expected,
                actual,
            } => write!(
                formatter,
                "fixture {artifact} program ID mismatch: SDK {expected}, manifest {actual}"
            ),
            Self::Hash {
                artifact,
                expected,
                actual,
            } => write!(
                formatter,
                "fixture {artifact} hash mismatch: expected {expected}, computed {actual}"
            ),
        }
    }
}

impl std::error::Error for FixtureError {}

/// Lowercase SHA-256 of the supplied bytes.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};

    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Loader v3 executable owner for every deployed external program.
pub const BPF_LOADER_UPGRADEABLE: Address = Address::from_bytes([
    2, 168, 246, 145, 78, 136, 161, 176, 226, 16, 21, 62, 247, 99, 174, 43, 0, 194, 185, 61, 22,
    193, 36, 210, 192, 83, 122, 16, 4, 128, 0, 0,
]);

/// One caller-owned genesis account injected with `--account`.
///
/// The caller supplies exact lamports, owner and bytes; no account is read from
/// an ambient ledger or a remote cluster.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalAccount {
    /// Account address.
    pub address: Address,
    /// Starting lamports.
    pub lamports: u64,
    /// Owning program.
    pub owner: Address,
    /// Exact account data bytes.
    pub data: Vec<u8>,
}
