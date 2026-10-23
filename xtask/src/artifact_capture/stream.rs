//! Parses compiler artifacts amid ordinary test output without guessing executable paths.

use super::archive::{Artifact, FileHash, Manifest};
use anyhow::{Context, Result, bail, ensure};
use serde::{
    Deserialize, Deserializer,
    de::{IgnoredAny, MapAccess, Visitor},
};
use serde_json::Value;
use std::{
    collections::HashSet,
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
};

const MAX_ROW: u64 = 8 * 1024 * 1024;

#[derive(Deserialize)]
struct CompilerArtifact {
    package_id: String,
    target: Target,
    profile: Profile,
    executable: Option<PathBuf>,
}

#[derive(Deserialize)]
struct Target {
    name: String,
}

#[derive(Deserialize)]
struct Profile {
    test: bool,
}

/// Drains even rejected streams so the child can exit and all failure logs survive.
pub(super) fn capture(
    input: impl Read,
    target: &Path,
    directory: &Path,
    manifest: &mut Manifest,
) -> Result<()> {
    let mut input = BufReader::new(input);
    let mut raw = fs::File::create(directory.join("cargo.stdout"))?;
    let mut row = Vec::new();
    let mut failure = None;
    let mut seen = HashSet::new();

    loop {
        row.clear();
        let count = input
            .by_ref()
            .take(MAX_ROW + 1)
            .read_until(b'\n', &mut row)?;

        if count == 0 {
            break;
        }

        raw.write_all(&row)?;

        if count as u64 > MAX_ROW {
            failure.get_or_insert_with(|| anyhow::anyhow!("Cargo row exceeds 8 MiB"));
            continue;
        }

        if let Err(error) = consume(&row, target, directory, manifest, &mut seen) {
            failure.get_or_insert(error);
        }
    }

    match failure {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

/// Null library artifacts are valid; a claimed malformed Cargo object is not.
fn consume(
    row: &[u8],
    target: &Path,
    directory: &Path,
    manifest: &mut Manifest,
    seen: &mut HashSet<PathBuf>,
) -> Result<()> {
    let Some(artifact) = parse(row)? else {
        return Ok(());
    };
    let Some(executable) = &artifact.executable else {
        return Ok(());
    };
    let original = executable
        .canonicalize()
        .context("reported executable is missing")?;
    ensure!(
        original.starts_with(target),
        "executable outside owned target"
    );
    ensure!(
        fs::metadata(&original)?.is_file(),
        "executable is not a regular file"
    );
    ensure!(
        seen.insert(original.clone()),
        "ambiguous repeated executable path"
    );
    let relative = PathBuf::from(format!("{}.executable", manifest.artifacts.len()));
    let mut copy = copy(&original, &directory.join(&relative))?;
    copy.path = relative;
    manifest.artifacts.push(Artifact {
        original,
        package: artifact.package_id,
        target: artifact.target.name,
        is_test: artifact.profile.test,
        copy,
        tests: None,
    });
    Ok(())
}

/// Interprets only Cargo's artifact protocol amid unrelated stdout rows.
fn parse(row: &[u8]) -> Result<Option<CompilerArtifact>> {
    let value: Value = match serde_json::from_slice(row) {
        Ok(value) => value,

        Err(error) if claims_artifact(row) => {
            bail!("malformed Cargo artifact: {error}")
        }
        Err(_) => return Ok(None),
    };

    if value.get("reason").and_then(Value::as_str) != Some("compiler-artifact") {
        return Ok(None);
    }

    Ok(Some(
        serde_json::from_value(value).context("invalid Cargo artifact")?,
    ))
}

struct ArtifactClaim<'a>(&'a mut bool);

impl<'de> Visitor<'de> for ArtifactClaim<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Cargo object")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> std::result::Result<(), M::Error> {
        while let Some(key) = map.next_key::<String>()? {
            if key == "reason" {
                *self.0 = map.next_value::<Value>()?.as_str() == Some("compiler-artifact");

                return Ok(());
            }

            map.next_value::<IgnoredAny>()?;
        }

        Ok(())
    }
}

/// Reads the top-level discriminator before a malformed artifact's remaining fields.
fn claims_artifact(row: &[u8]) -> bool {
    let mut claimed = false;
    // The full parse already failed; only a decoded discriminator before that failure matters.
    let _ = serde_json::Deserializer::from_slice(row).deserialize_map(ArtifactClaim(&mut claimed));

    claimed
}

/// Opens a unique destination and compares both actual files after copying.
fn copy(original: &Path, destination: &Path) -> Result<FileHash> {
    let mut input = fs::File::open(original)?;
    let mut output = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination)?;
    std::io::copy(&mut input, &mut output)?;
    output.sync_all()?;
    fs::set_permissions(destination, fs::metadata(original)?.permissions())?;
    let copy = FileHash::inspect(destination)?;
    ensure!(
        copy.sha256 == FileHash::inspect(original)?.sha256,
        "executable changed during copy"
    );
    Ok(copy)
}
