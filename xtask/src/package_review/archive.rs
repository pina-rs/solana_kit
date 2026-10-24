//! Inspects raw GNU/USTAR records rather than an extractor's filtered listing.

use super::{Config, MAX_BYTES, Member, quality, regular, sha256};
use anyhow::{Context, Result, ensure};
use std::{
    collections::BTreeSet,
    fs,
    io::Read,
    path::{Component, Path},
    process::{Command, Stdio},
};

const BLOCK: usize = 512;
const TYPE_OFFSET: usize = 156;
const OCTAL_RADIX: u32 = 8;

/// Bounds decompression and requires every actual header to be a regular-file record.
pub(super) fn review(config: &Config) -> Result<Vec<Member>> {
    let bytes = decompress(config)?;
    let entries = entries(&bytes)?;
    let prefix = entries
        .first()
        .context("archive is empty")?
        .0
        .split('/')
        .next()
        .context("missing archive root")?
        .to_owned();
    let mut members = Vec::new();
    let artifact = config.receipt.join("artifact");
    let analysis = config.receipt.join("analysis");
    fs::create_dir(&artifact)?;
    fs::create_dir(&analysis)?;

    for (name, bytes) in entries {
        let relative = member_path(&name, &prefix)?;
        write(&artifact.join(relative), bytes)?;
        let scored = quality::analysis_path(relative);

        if let Some(scored) = &scored {
            write(&analysis.join(scored), bytes)?;
        }

        members.push(Member {
            path: relative.to_owned(),
            kind: "regular",
            size: bytes.len(),
            sha256: sha256(bytes),
            analysis: scored,
            source: source_path(relative),
        });
    }

    compare_source(config, &members)?;

    Ok(members)
}

/// Stops an oversized decoder before reading or retaining unbounded archive bytes.
fn decompress(config: &Config) -> Result<Vec<u8>> {
    let mut child = Command::new(&config.gzip)
        .arg("-dc")
        .arg(&config.archive)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut bytes = Vec::new();
    child
        .stdout
        .take()
        .context("missing gzip output")?
        .take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)?;

    if u64::try_from(bytes.len())? > MAX_BYTES {
        child.kill()?;
    }

    let output = child.wait_with_output()?;
    ensure!(
        u64::try_from(bytes.len())? <= MAX_BYTES,
        "decompressed archive exceeds size bound"
    );
    ensure!(
        output.status.success(),
        "gzip failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(bytes)
}

/// Accepts actual Cargo GNU records and ordinary USTAR, rejecting extension records.
fn entries(bytes: &[u8]) -> Result<Vec<(String, &[u8])>> {
    let mut entries = Vec::new();
    let mut offset = 0usize;
    let mut root = None;
    let mut seen = BTreeSet::new();

    loop {
        let header = bytes
            .get(offset..offset + BLOCK)
            .context("truncated tar header")?;

        if header.iter().all(|byte| *byte == 0) {
            ensure!(
                bytes.len() >= offset + 2 * BLOCK && bytes[offset..].iter().all(|byte| *byte == 0),
                "invalid tar end marker"
            );
            return Ok(entries);
        }

        let (name, size) = parse_header(header)?;
        let prefix = name.split('/').next().context("archive has no root")?;
        let expected_root = root.get_or_insert_with(|| prefix.to_owned());
        member_path(&name, expected_root)?;
        ensure!(
            seen.insert(name.clone()),
            "duplicate archive member: {name}"
        );

        let start = offset.checked_add(BLOCK).context("tar offset overflow")?;
        let end = start.checked_add(size).context("tar size overflow")?;
        let content = bytes.get(start..end).context("truncated tar member")?;
        entries.push((name, content));
        offset = end
            .checked_add((BLOCK - size % BLOCK) % BLOCK)
            .context("tar padding overflow")?;
    }
}

/// Validates a complete regular-file header before interpreting its name and size.
fn parse_header(header: &[u8]) -> Result<(String, usize)> {
    let ustar = &header[257..263] == b"ustar\0" && &header[263..265] == b"00";
    let gnu = &header[257..265] == b"ustar  \0";
    ensure!(ustar || gnu, "unsupported tar header format");
    ensure!(
        !gnu || header[345..].iter().all(|byte| *byte == 0),
        "unsupported GNU extension data"
    );
    ensure!(
        matches!(header[TYPE_OFFSET], 0 | b'0'),
        "archive contains a nonregular member"
    );
    let checksum = header_checksum(header);
    ensure!(
        octal(&header[148..156])? == checksum,
        "tar header checksum mismatch"
    );
    let name = field(&header[..100])?;
    let prefix = if ustar { field(&header[345..500])? } else { "" };
    let name = if prefix.is_empty() {
        name.to_owned()
    } else {
        format!("{prefix}/{name}")
    };
    let size = usize::try_from(octal(&header[124..136])?)?;
    ensure!(
        u64::try_from(size)? <= MAX_BYTES,
        "archive member exceeds size bound"
    );

    Ok((name, size))
}

/// The tar checksum treats its own eight bytes as spaces.
fn header_checksum(header: &[u8]) -> u64 {
    header
        .iter()
        .enumerate()
        .map(|(index, byte)| {
            if (148..TYPE_OFFSET).contains(&index) {
                u64::from(b' ')
            } else {
                u64::from(*byte)
            }
        })
        .sum()
}

/// Rejects embedded nonzero suffix bytes and invalid UTF-8 in header text.
fn field(bytes: &[u8]) -> Result<&str> {
    let end = bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len());
    ensure!(
        bytes[end..].iter().all(|byte| *byte == 0),
        "ambiguous tar text field"
    );
    Ok(std::str::from_utf8(&bytes[..end])?)
}

/// Cargo uses ordinary octal numeric fields; binary extensions are rejected explicitly.
fn octal(bytes: &[u8]) -> Result<u64> {
    let value = std::str::from_utf8(bytes)?.trim_matches(['\0', ' ']);
    ensure!(
        !value.is_empty() && value.bytes().all(|byte| (b'0'..=b'7').contains(&byte)),
        "invalid tar octal field"
    );
    Ok(u64::from_str_radix(value, OCTAL_RADIX)?)
}

/// Confirms every non-Cargo-generated shipping member matches the reviewed source.
pub(super) fn compare_source(config: &Config, members: &[Member]) -> Result<()> {
    for member in members {
        let Some(source) = &member.source else {
            continue;
        };
        let path = config.source.join(source);
        let mut parent = config.source.clone();

        for component in Path::new(source).components() {
            parent.push(component);
            ensure!(
                !fs::symlink_metadata(&parent)?.is_symlink(),
                "source alias: {source}"
            );
        }

        let canonical = path.canonicalize()?;
        ensure!(
            canonical.starts_with(config.source.canonicalize()?),
            "source path escapes its root"
        );
        ensure!(
            sha256(&regular(&path)?) == member.sha256,
            "archive/source mismatch: {source}"
        );
    }

    Ok(())
}

/// Cargo rewrites the normalized manifest and lockfile and may add VCS metadata.
fn source_path(path: &str) -> Option<String> {
    match path {
        "Cargo.toml" | "Cargo.lock" | ".cargo_vcs_info.json" => None,
        "Cargo.toml.orig" => Some("Cargo.toml".to_owned()),
        other => Some(other.to_owned()),
    }
}

/// Rejects escaped, ambiguous and option-like filenames before creating files.
fn member_path<'a>(name: &'a str, prefix: &str) -> Result<&'a str> {
    ensure!(
        name.bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"/._-+".contains(&byte)),
        "unsupported archive filename: {name}"
    );
    ensure!(
        name.split('/')
            .all(|part| !part.is_empty() && part != "." && part != ".."),
        "archive path escape: {name}"
    );
    let relative = name
        .strip_prefix(prefix)
        .and_then(|name| name.strip_prefix('/'))
        .context("conflicting archive root")?;
    ensure!(
        Path::new(relative)
            .components()
            .all(|part| matches!(part, Component::Normal(_))),
        "archive path escape"
    );
    Ok(relative)
}

/// Creates only new regular files beneath the fresh, gate-owned extraction root.
fn write(path: &Path, bytes: &[u8]) -> Result<()> {
    use std::io::Write;
    fs::create_dir_all(path.parent().context("member has no parent")?)?;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests;
