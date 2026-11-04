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
const NAME_LENGTH: usize = 100;
const LONG_NAME_LIMIT: usize = 4096;
const LONG_LINK_NAME: &str = "././@LongLink";

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

/// A validated header that either names a regular file or introduces its long name.
enum Header {
    Regular { name: String, size: usize },
    LongName { size: usize },
}

/// Accepts actual Cargo GNU records and ordinary USTAR, rejecting other extensions.
fn entries(bytes: &[u8]) -> Result<Vec<(String, &[u8])>> {
    let mut entries = Vec::new();
    let mut offset = 0usize;
    let mut root = None;
    let mut seen = BTreeSet::new();
    let mut pending_long_name = None;

    loop {
        let header = bytes
            .get(offset..offset + BLOCK)
            .context("truncated tar header")?;

        if header.iter().all(|byte| *byte == 0) {
            ensure!(pending_long_name.is_none(), "orphan GNU long-name metadata");
            ensure!(
                bytes.len() >= offset + 2 * BLOCK && bytes[offset..].iter().all(|byte| *byte == 0),
                "invalid tar end marker"
            );

            return Ok(entries);
        }

        let next = match parse_header(header)? {
            Header::LongName { size } => {
                ensure!(
                    pending_long_name.is_none(),
                    "repeated GNU long-name metadata"
                );
                let (payload, next) = payload(bytes, offset, size)?;
                pending_long_name = Some(long_name(payload)?);

                next
            }
            Header::Regular { name, size } => {
                let name = effective_name(name, pending_long_name.take())?;
                validate_member(&name, &mut root, &mut seen)?;
                let (content, next) = payload(bytes, offset, size)?;
                entries.push((name, content));

                next
            }
        };
        offset = next;
    }
}

/// Applies one long-name record only when the next header carries its truncated name.
fn effective_name(name: String, long: Option<String>) -> Result<String> {
    let Some(long) = long else {
        return Ok(name);
    };
    ensure!(
        name.as_bytes() == &long.as_bytes()[..NAME_LENGTH],
        "ambiguous GNU long-name metadata"
    );

    Ok(long)
}

/// Requires one root, one safe relative path and one effective member name.
fn validate_member(
    name: &str,
    root: &mut Option<String>,
    seen: &mut BTreeSet<String>,
) -> Result<()> {
    let prefix = name.split('/').next().context("archive has no root")?;
    let expected_root = root.get_or_insert_with(|| prefix.to_owned());
    member_path(name, expected_root)?;
    ensure!(
        seen.insert(name.to_owned()),
        "duplicate archive member: {name}"
    );

    Ok(())
}

/// Validates a complete header before interpreting its name, size or type.
fn parse_header(header: &[u8]) -> Result<Header> {
    let (ustar, gnu) = header_format(header)?;
    let kind = header[TYPE_OFFSET];
    ensure!(
        kind != b'K',
        "archive uses unsupported GNU long-link extension records"
    );
    ensure!(
        matches!(kind, 0 | b'0' | b'L'),
        "archive contains a nonregular member"
    );
    ensure!(
        octal(&header[148..156])? == header_checksum(header),
        "tar header checksum mismatch"
    );

    let name = field(&header[..NAME_LENGTH])?;
    let size = usize::try_from(octal(&header[124..136])?)?;
    ensure!(
        u64::try_from(size)? <= MAX_BYTES,
        "archive member exceeds size bound"
    );

    if kind == b'L' {
        return long_name_header(gnu, name, size);
    }

    regular_header(ustar, name, size, header)
}

/// Accepts USTAR and GNU headers while rejecting unknown GNU extension data.
fn header_format(header: &[u8]) -> Result<(bool, bool)> {
    let ustar = &header[257..263] == b"ustar\0" && &header[263..265] == b"00";
    let gnu = &header[257..265] == b"ustar  \0";
    ensure!(ustar || gnu, "unsupported tar header format");
    ensure!(
        !gnu || header[345..].iter().all(|byte| *byte == 0),
        "unsupported GNU extension data"
    );

    Ok((ustar, gnu))
}

/// Requires the exact GNU metadata marker and its bounded payload size.
fn long_name_header(gnu: bool, name: &str, size: usize) -> Result<Header> {
    ensure!(gnu, "GNU long-name metadata requires a GNU header");
    ensure!(
        name == LONG_LINK_NAME,
        "unexpected GNU long-name metadata header"
    );
    ensure!(
        (2..=LONG_NAME_LIMIT + 1).contains(&size),
        "GNU long-name metadata exceeds its bound"
    );

    Ok(Header::LongName { size })
}

/// Joins an actual USTAR prefix without accepting GNU prefix data.
fn regular_header(ustar: bool, name: &str, size: usize, header: &[u8]) -> Result<Header> {
    let prefix = if ustar { field(&header[345..500])? } else { "" };
    let name = if prefix.is_empty() {
        name.to_owned()
    } else {
        format!("{prefix}/{name}")
    };

    Ok(Header::Regular { name, size })
}

/// Locates one bounded payload and its following header without reading past the archive.
fn payload(bytes: &[u8], offset: usize, size: usize) -> Result<(&[u8], usize)> {
    let start = offset.checked_add(BLOCK).context("tar offset overflow")?;
    let end = start.checked_add(size).context("tar size overflow")?;
    let content = bytes.get(start..end).context("truncated tar member")?;
    let next = end
        .checked_add((BLOCK - size % BLOCK) % BLOCK)
        .context("tar padding overflow")?;

    Ok((content, next))
}

/// Requires one NUL-terminated, bounded, unambiguous UTF-8 name for the next file.
fn long_name(bytes: &[u8]) -> Result<String> {
    let (terminator, name) = bytes.split_last().context("empty GNU long-name metadata")?;
    ensure!(*terminator == 0, "unterminated GNU long-name metadata");
    ensure!(!name.contains(&0), "ambiguous GNU long-name metadata");
    let name = std::str::from_utf8(name)?;
    ensure!(name.len() > NAME_LENGTH, "redundant GNU long-name metadata");
    validate_name(name)?;

    Ok(name.to_owned())
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
    validate_name(name)?;
    let relative = name
        .strip_prefix(prefix)
        .and_then(|name| name.strip_prefix('/'))
        .context("conflicting archive root")?;
    ensure!(
        !relative.is_empty()
            && Path::new(relative)
                .components()
                .all(|part| matches!(part, Component::Normal(_))),
        "archive path escape"
    );
    Ok(relative)
}

/// Allows ordinary interior ASCII spaces while rejecting controls and ambiguous components.
fn validate_name(name: &str) -> Result<()> {
    ensure!(
        name.bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"/._-+ ".contains(&byte)),
        "unsupported archive filename: {name}"
    );
    ensure!(
        name.split('/').all(|part| {
            !part.is_empty()
                && part != "."
                && part != ".."
                && !part.starts_with(' ')
                && !part.ends_with(' ')
        }),
        "archive path escape: {name}"
    );
    Ok(())
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
