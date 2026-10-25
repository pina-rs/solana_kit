//! Audits exact Token-2022 declarations, source files, notices and source pins.
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const PACKAGE: &str = "solana_kit_token_2022";
const IDS: usize = 1829;
const DEFINING_FILES: usize = 118;
const HASH_COLUMN: usize = 12;
const CANONICAL_COLUMNS: usize = 14;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let dart = Path::new(args.get(1).ok_or("original Dart root required")?);
    let upstream = Path::new(args.get(2).ok_or("original Token-2022 root required")?);
    let output = Path::new(args.get(3).ok_or("evidence output directory required")?);
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    pins(dart, upstream)?;
    let canonical = fs::read_to_string(repository.join("docs/parity/capabilities.tsv"))?;
    let (receipt, files) = inventory(&canonical, dart)?;

    let package = dart.join("packages").join(PACKAGE);
    let mut paths = Vec::new();
    collect(&package, &package, &mut paths)?;
    paths.sort();
    let mut manifest = "path\tsha256\n".to_owned();

    for relative in &paths {
        manifest.push_str(&format!(
            "{}\t{}\n",
            relative.display(),
            digest(&package.join(relative))?
        ));
    }

    fs::write(output.join("dart-package-files.tsv"), manifest)?;
    fs::write(repository.join("docs/parity/token-2022.tsv"), receipt)?;
    println!(
        "{IDS} canonical IDs; {files} defining hashes; {} package files; exact archive/IDL/notices verified",
        paths.len()
    );
    Ok(())
}

fn digest(path: &Path) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(fs::read(path)?)))
}

fn pins(dart: &Path, upstream: &Path) -> Result<()> {
    let sources = [
        (
            dart.parent()
                .ok_or("missing Dart cache")?
                .join("source.tar.gz"),
            "b229acdc682bf9ea4b00e6f215551d7e72d5d1aa85a98ec0ae42ebd71cd3259d",
        ),
        (
            upstream
                .parent()
                .ok_or("missing upstream cache")?
                .join("source.tar.gz"),
            "86a5c5a837fee85cb039d26eb64645f44f166ff4f68e739f0a5a07bd75afec97",
        ),
        (
            upstream.join("idl.json"),
            "13cbda7213599f3312e1b7ffd1c7f176f36ebd74dba9feabe6e9a6874dc1d5d2",
        ),
        (
            upstream.join("LICENSE"),
            "a6cba85bc92e0cff7a450b1d873c0eaa2e9fc96bf472df0247a26bec77bf3ff9",
        ),
    ];

    for (path, expected) in sources {
        if digest(&path)? != expected {
            return Err(format!("source pin mismatch: {}", path.display()).into());
        }
    }

    verify_notice(
        &fs::read(upstream.join("LICENSE"))?,
        include_bytes!("../../fixtures/TOKEN-2022-LICENSE"),
    )?;
    verify_notice(
        &fs::read(dart.join("LICENSE"))?,
        include_bytes!("../../fixtures/DART-LICENSE"),
    )
}

fn verify_notice(actual: &[u8], expected: &[u8]) -> Result<()> {
    if actual != expected {
        return Err("source notice mismatch".into());
    }

    Ok(())
}

/// Preserve canonical rows verbatim while verifying each defining file once.
fn inventory(canonical: &str, dart: &Path) -> Result<(String, usize)> {
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut lines = canonical.lines();
    let header = lines.next().ok_or("missing canonical header")?;
    let mut output = format!("{header}\tunit217_status\tbehavior\tresidual\n");

    for row in lines {
        let fields: Vec<_> = row.split('\t').collect();

        if fields.get(1) != Some(&PACKAGE) {
            continue;
        }

        if fields.len() != CANONICAL_COLUMNS || !ids.insert(fields[0]) {
            return Err("malformed or duplicate canonical Token-2022 ID".into());
        }

        if paths.insert(fields[2]) {
            verify_source(dart, &fields)?;
        }

        output.push_str(&format!("{row}\tpending\tindependent original Dart source/oracle groundwork only\tproduction typed emission and portable client behavior remain unimplemented\n"));
    }

    if ids.len() != IDS || paths.len() != DEFINING_FILES {
        return Err("Token-2022 canonical inventory denominator changed".into());
    }

    Ok((output, paths.len()))
}

fn verify_source(dart: &Path, fields: &[&str]) -> Result<()> {
    if digest(&dart.join(fields[2]))? != fields[HASH_COLUMN] {
        return Err("Token-2022 defining-source hash mismatch".into());
    }

    Ok(())
}

/// Reject symlinks before walking the complete immutable package tree.
fn collect(root: &Path, directory: &Path, output: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let kind = entry.file_type()?;

        if kind.is_symlink() {
            return Err("unexpected symlink in pinned package".into());
        }

        if kind.is_dir() {
            collect(root, &path, output)?;
        } else if kind.is_file() {
            output.push(path.strip_prefix(root)?.to_owned());
        }
    }

    Ok(())
}

#[cfg(all(test, feature = "verification"))]
mod tests {
    use super::*;

    #[test]
    fn notice_corruption_rejects() {
        assert!(verify_notice(b"Apache-2.0", b"MIT").is_err());
        assert!(verify_notice(b"MIT", b"MIT").is_ok());
    }

    #[test]
    fn missing_and_duplicate_ids_reject() {
        let directory = std::env::temp_dir().join(format!("kit217-audit-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        fs::write(directory.join("source"), b"fixture").unwrap();
        assert!(inventory("header\n", &directory).is_err());
        let hash = digest(&directory.join("source")).unwrap();
        let row = format!("id\t{PACKAGE}\tsource\t-\t-\t-\t-\t-\t-\t-\t-\t-\t{hash}\t-");
        let error = inventory(&format!("header\n{row}\n{row}"), &directory).unwrap_err();
        assert_eq!(
            error.to_string(),
            "malformed or duplicate canonical Token-2022 ID"
        );

        let fields: Vec<_> = row.split('\t').collect();
        assert!(verify_source(&directory, &fields).is_ok());
        fs::write(directory.join("source"), b"corrupt").unwrap();
        assert!(verify_source(&directory, &fields).is_err());
    }
}
