//! Audits exact declaration membership, defining source and retained program licenses.
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
type Error = Box<dyn std::error::Error>;
type Rows = BTreeMap<String, Vec<String>>;
const ORIGINAL_COLUMNS: usize = 14;
const DECLARATIONS: usize = 5501;
const PACKAGES: [&str; 8] = [
    "solana_kit_address_lookup_table",
    "solana_kit_config",
    "solana_kit_stake",
    "solana_kit_loader",
    "solana_kit_spl_account_compression",
    "solana_kit_mpl_core",
    "solana_kit_mpl_token_metadata",
    "solana_kit_mpl_bubblegum",
];
const CANONICAL: &str = include_str!("../../../../docs/parity/capabilities.tsv");
const RECEIPTS: [&str; 3] = [
    include_str!("../../../../docs/parity/alt-config.tsv"),
    include_str!("../../../../docs/parity/stake-loader.tsv"),
    include_str!("../../../../docs/parity/metaplex-source.tsv"),
];
const DART_LICENSE: &str = "dd0836fdc750ca3bc8f8ffab6b1b8fc7feac918ac41af286da46cd10cedd1b25";
const UPSTREAM_LICENSE: &str = "a6cba85bc92e0cff7a450b1d873c0eaa2e9fc96bf472df0247a26bec77bf3ff9";

/// Requires the caller's existing immutable pinned Dart export; never fetches inputs.
fn main() -> Result<(), Error> {
    let reference = std::env::args_os()
        .nth(1)
        .ok_or("pinned Dart source root required")?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    audit(&root, Path::new(&reference))?;
    println!(
        "PASS 5501 exact original rows, defining sources and all retained licenses; statuses unchanged"
    );
    Ok(())
}

/// Original fourteen fields remain exact; additional evidence cannot redefine scope.
fn records(source: &str) -> Result<BTreeMap<&str, Vec<&str>>, Error> {
    let mut rows = BTreeMap::new();

    for line in source.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();

        if fields.len() < 14 {
            return Err("short declaration row".into());
        }

        if PACKAGES.contains(&fields[1]) && rows.insert(fields[0], fields).is_some() {
            return Err("duplicate declaration".into());
        }
    }

    Ok(rows)
}

/// Rejects missing, duplicate, foreign and altered identities before reading source.
fn validate(receipts: &[&str]) -> Result<Rows, Error> {
    let expected = records(CANONICAL)?;
    let actual = merge(receipts)?;

    if actual.len() != DECLARATIONS
        || actual
            .keys()
            .map(String::as_str)
            .ne(expected.keys().copied())
    {
        return Err("receipt membership drift".into());
    }

    for (id, fields) in &actual {
        if fields[..ORIGINAL_COLUMNS]
            .iter()
            .map(String::as_str)
            .ne(expected[id.as_str()][..ORIGINAL_COLUMNS].iter().copied())
        {
            return Err("canonical declaration drift".into());
        }
    }

    Ok(actual)
}

/// Each receipt contributes unique declarations from this exact package scope.
fn merge(receipts: &[&str]) -> Result<Rows, Error> {
    let mut rows = BTreeMap::new();

    for line in receipts.iter().flat_map(|source| source.lines().skip(1)) {
        let fields = receipt_fields(line)?;

        if rows.insert(fields[0].clone(), fields).is_some() {
            return Err("duplicate receipt declaration".into());
        }
    }

    Ok(rows)
}

/// Scope validation precedes positional access to original declaration columns.
fn receipt_fields(line: &str) -> Result<Vec<String>, Error> {
    let fields: Vec<_> = line.split('\t').collect();

    if fields.len() < ORIGINAL_COLUMNS || !PACKAGES.contains(&fields[1]) {
        return Err("foreign receipt row".into());
    }

    Ok(fields.into_iter().map(str::to_owned).collect())
}

/// Binds every original defining path and every shipped copy of both license notices.
fn audit(root: &Path, reference: &Path) -> Result<(), Error> {
    let rows = validate(&RECEIPTS)?;
    let mut paths = BTreeSet::new();

    for fields in rows.values() {
        if paths.insert(&fields[2]) {
            digest(&reference.join(&fields[2]), &fields[12])?;
        }
    }

    digest(&reference.join("LICENSE"), DART_LICENSE)?;

    for package in PACKAGES {
        let rust_package = match package {
            "solana_kit_spl_account_compression" => "solana_kit_account_compression",
            package => package,
        };
        let folder = root.join("crates").join(rust_package);
        digest(&folder.join("DART-LICENSE"), DART_LICENSE)?;

        if let Some(expected) = upstream_license(package) {
            digest(&folder.join("UPSTREAM-LICENSE"), expected)?;
        }
    }

    Ok(())
}

/// MIT-only modeled clients retain their Dart notice; Apache IDL clients retain both notices.
fn upstream_license(package: &str) -> Option<&'static str> {
    match package {
        "solana_kit_spl_account_compression" => {
            Some("d8ddd6435eae4d5e7df65248f19a347fcf67e2d454d0826cb418a33648e2dd54")
        }
        "solana_kit_address_lookup_table"
        | "solana_kit_config"
        | "solana_kit_stake"
        | "solana_kit_loader" => Some(UPSTREAM_LICENSE),
        _ => None,
    }
}

/// Checks actual bytes with a fixed digest, preserving path-specific failures.
fn digest(path: &Path, expected: &str) -> Result<(), Error> {
    if format!("{:x}", Sha256::digest(fs::read(path)?)) != expected {
        return Err(format!("source or license digest drift: {}", path.display()).into());
    }

    Ok(())
}

#[cfg(all(test, feature = "verification"))]
mod tests {
    use super::*;

    #[test]
    fn complete_membership_and_original_rows_are_required() {
        assert_eq!(validate(&RECEIPTS).unwrap().len(), 5501);
        assert!(validate(&RECEIPTS[..1]).is_err());
        let mut duplicated = RECEIPTS.to_vec();
        duplicated.push(RECEIPTS[0]);
        assert!(validate(&duplicated).is_err());
        let altered = RECEIPTS[1].replacen("\tpending\t", "\tcomplete\t", 1);
        let mut changed = RECEIPTS.to_vec();
        changed[1] = &altered;
        assert!(validate(&changed).is_err());
        let foreign = format!(
            "{}\nforeign\tforeign\t{}",
            RECEIPTS[1],
            ["x"; 12].join("\t")
        );
        changed[1] = &foreign;
        assert!(validate(&changed).is_err());
    }

    #[test]
    fn actual_copied_defining_source_and_license_changes_reject() {
        let row = records(CANONICAL)
            .unwrap()
            .into_values()
            .find(|row| {
                row[2] == "packages/solana_kit_stake/lib/src/generated/instructions/initialize.dart"
            })
            .unwrap();
        changed_copy(
            "defining",
            include_bytes!("../../fixtures/stake-loader/dart-stake/initialize.dart.txt"),
            row[12],
        );
        changed_copy(
            "license",
            include_bytes!("../../../../crates/solana_kit_stake/DART-LICENSE"),
            DART_LICENSE,
        );
    }

    fn changed_copy(name: &str, bytes: &[u8], expected: &str) {
        let path = std::env::temp_dir().join(format!("kit-program-{name}-{}", std::process::id()));
        fs::write(&path, bytes).unwrap();
        digest(&path, expected).unwrap();
        let mut changed = bytes.to_vec();
        changed.push(0);
        fs::write(&path, &changed).unwrap();
        assert!(digest(&path, expected).is_err());
        fs::remove_file(path).unwrap();
    }
}
