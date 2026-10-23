//! Hashes actual bytes and commits archive metadata with an atomic rename.

use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Serialize, Deserialize)]
pub(super) struct FileHash {
    pub path: PathBuf,
    pub sha256: String,
}

impl FileHash {
    /// Reads the bytes rather than trusting filenames, times or old inventories.
    pub fn inspect(path: &Path) -> Result<Self> {
        let mut input = fs::File::open(path).with_context(|| format!("read {}", path.display()))?;
        let mut digest = Sha256::new();
        let mut buffer = [0; 65536];

        loop {
            let count = input.read(&mut buffer)?;

            if count == 0 {
                break;
            }

            digest.update(&buffer[..count]);
        }

        Ok(Self {
            path: path.to_owned(),
            sha256: format!("{:x}", digest.finalize()),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct Producer {
    pub file: FileHash,
    pub version: String,
}

impl Producer {
    /// Uses real toolchain files, avoiding rustup proxies and outer-only labels.
    pub fn inspect(path: &Path, expected: &str) -> Result<Self> {
        let file = FileHash::inspect(&path.canonicalize()?)?;
        let output = Command::new(&file.path).arg("--version").output()?;
        let version = String::from_utf8(output.stdout)?;
        ensure!(output.status.success(), "producer version failed");
        ensure!(
            version.split_whitespace().nth(1) == Some(expected),
            "producer selection mismatch: {version}"
        );
        Ok(Self { file, version })
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct Artifact {
    pub original: PathBuf,
    pub package: String,
    pub target: String,
    pub is_test: bool,
    pub copy: FileHash,
    pub tests: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub(super) struct Manifest {
    pub source: PathBuf,
    pub source_files: Vec<FileHash>,
    pub producers: Vec<Producer>,
    pub command: Vec<String>,
    pub target: PathBuf,
    pub artifacts: Vec<Artifact>,
    pub cargo_status: Option<i32>,
    pub complete: bool,
}

/// A completed manifest is authoritative only while its actual copied bytes match.
pub(super) fn verify(directory: &Path) -> Result<()> {
    let manifest: Manifest = serde_json::from_slice(&fs::read(directory.join("manifest.json"))?)?;
    ensure!(manifest.complete, "prior archive is incomplete");

    for artifact in &manifest.artifacts {
        let path = safe_copy_path(directory, &artifact.copy.path)?;
        ensure!(
            FileHash::inspect(&path)?.sha256 == artifact.copy.sha256,
            "prior artifact bytes are corrupt: {}",
            artifact.copy.path.display()
        );
    }

    Ok(())
}

/// Rejects absolute paths, traversal and symlink escapes in a receipt.
fn safe_copy_path(directory: &Path, relative: &Path) -> Result<PathBuf> {
    ensure!(
        relative
            .components()
            .all(|part| matches!(part, Component::Normal(_))),
        "archive path escape"
    );
    let canonical = directory.join(relative).canonicalize()?;
    ensure!(
        canonical.starts_with(directory.canonicalize()?),
        "archive symlink escape"
    );
    Ok(canonical)
}

/// Lists only Cargo test harnesses and streams their complete output to disk.
pub(super) fn list_tests(directory: &Path, manifest: &mut Manifest) -> Result<()> {
    for (index, artifact) in manifest.artifacts.iter_mut().enumerate() {
        if !artifact.is_test {
            continue;
        }

        let list = directory.join(format!("{index}.list"));
        let status = Command::new(directory.join(&artifact.copy.path))
            .arg("--list")
            .stdout(fs::File::create(&list)?)
            .stderr(fs::File::create(
                directory.join(format!("{index}.list.stderr")),
            )?)
            .stdin(Stdio::null())
            .status()?;
        ensure!(status.success(), "test inventory command failed");
        artifact.tests = Some(
            std::io::BufRead::lines(std::io::BufReader::new(fs::File::open(list)?))
                .try_fold(0, |count, row| -> Result<usize> {
                    Ok(count + usize::from(row?.ends_with(": test")))
                })?,
        );
    }

    Ok(())
}

/// Renames a fully flushed sibling file so readers never observe partial JSON.
pub(super) fn atomic_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let temporary = path.with_extension("json.pending");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    fs::rename(temporary, path)?;
    Ok(())
}
