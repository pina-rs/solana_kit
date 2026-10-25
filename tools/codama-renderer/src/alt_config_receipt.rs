//! Strict scoped declaration and immutable source auditor; no IDL approximation.
use crate::{inputs, schema::Error};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

const RECEIPT: &str = include_str!("../fixtures/alt-config-prior-receipt.tsv");
const CANONICAL: &str = include_str!("../../../docs/parity/capabilities.tsv");
const NOTICE: &str = "a6cba85bc92e0cff7a450b1d873c0eaa2e9fc96bf472df0247a26bec77bf3ff9";
const PACKAGES: [&str; 2] = ["solana_kit_address_lookup_table", "solana_kit_config"];
const DEFINING_HASH: usize = 12;
const DEFINING_PATHS: usize = 13;

fn records(source: &str) -> Result<BTreeMap<&str, Vec<&str>>, Error> {
    let mut records = BTreeMap::new();

    for line in source.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();

        if fields
            .get(1)
            .is_some_and(|package| PACKAGES.contains(package))
            && records.insert(fields[0], fields).is_some()
        {
            return Err("duplicate declaration ID".into());
        }
    }

    Ok(records)
}

/// Canonical rows retain all fourteen original fields; partial evidence cannot rewrite scope.
fn validate(receipt: &str) -> Result<BTreeMap<&str, Vec<&str>>, Error> {
    if receipt.lines().skip(1).any(|line| {
        !line
            .split('\t')
            .nth(1)
            .is_some_and(|package| PACKAGES.contains(&package))
    }) {
        return Err("foreign receipt row".into());
    }

    let actual = records(receipt)?;
    let expected = records(CANONICAL)?;

    if actual.len() != 113 || actual.keys().ne(expected.keys()) {
        return Err("missing or foreign declaration ID".into());
    }

    for (id, fields) in &actual {
        if fields.len() != 18 || fields[..14] != expected[id][..] || fields[14] != "partial" {
            return Err("canonical declaration or partial-status drift".into());
        }
    }

    Ok(actual)
}

pub(crate) fn run() -> Result<(), Error> {
    let argument = std::env::args()
        .nth(2)
        .ok_or("expected immutable cache root")?;
    let cache = Path::new(&argument);
    let source = cache.join("dart/source");
    let rows = validate(RECEIPT)?;
    defining_hashes(&source, &rows)?;
    source_pins(cache)?;
    retained_notices(&source)?;
    println!(
        "PASS 113 exact IDs, 13 actual defining hashes, 3 archives, 2 IDLs and retained notices"
    );

    Ok(())
}

fn defining_hashes(source: &Path, rows: &BTreeMap<&str, Vec<&str>>) -> Result<(), Error> {
    let mut paths = BTreeSet::new();

    for fields in rows.values() {
        if paths.insert(fields[2])
            && inputs::hash(&source.join(fields[2]))? != fields[DEFINING_HASH]
        {
            return Err("actual defining source hash drift".into());
        }
    }

    if paths.len() != DEFINING_PATHS {
        return Err("defining path inventory drift".into());
    }

    Ok(())
}

/// IDL bytes alone cannot prove archive or license identity; each independent pin is checked.
fn source_pins(cache: &Path) -> Result<(), Error> {
    for (name, archive, idl, idl_hash) in [
        (
            "address-lookup-table",
            "cdfc8df0dd85efb676d981309e9978ba943b83e9f0bfed8363f36b424fe49de7",
            "idl.json",
            "f23e8397def424afc9b38648ca3d7b7ed7fbcfa1c690af2c910b9e556bfd4a8a",
        ),
        (
            "config",
            "a08b8af93fcbbb2f7ff200399247b1f54e1be854264a5ac02f108da3fd8c0e90",
            "program/idl.json",
            "b11fc73aa20bdd5199a5ae4193b835e16ad65f7b563844bd200baa097b209131",
        ),
    ] {
        if inputs::hash(&cache.join(name).join("source.tar.gz"))? != archive
            || inputs::hash(&cache.join(name).join("source").join(idl))? != idl_hash
            || inputs::hash(&cache.join(name).join("source/LICENSE"))? != NOTICE
        {
            return Err("archive, IDL or license pin drift".into());
        }
    }

    if inputs::hash(&cache.join("dart/source.tar.gz"))?
        != "b229acdc682bf9ea4b00e6f215551d7e72d5d1aa85a98ec0ae42ebd71cd3259d"
    {
        return Err("Dart archive drift".into());
    }
    Ok(())
}

fn retained_notices(source: &Path) -> Result<(), Error> {
    for package in PACKAGES {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates")
            .join(package);

        if inputs::hash(&root.join("UPSTREAM-LICENSE"))? != NOTICE {
            return Err("retained notice drift".into());
        }

        same_notice(
            &fs::read(root.join("DART-LICENSE"))?,
            &fs::read(source.join("LICENSE"))?,
        )?;
    }

    Ok(())
}

fn same_notice(actual: &[u8], expected: &[u8]) -> Result<(), Error> {
    if actual != expected {
        return Err("retained notice byte drift".into());
    }

    Ok(())
}

#[cfg(all(test, feature = "verification"))]
mod tests {
    use super::*;

    #[test]
    fn inventory_missing_duplicate_and_hash_drift_reject() {
        assert!(validate(RECEIPT).is_ok());
        let lines: Vec<_> = RECEIPT.lines().collect();
        assert!(validate(&lines[..lines.len() - 1].join("\n")).is_err());
        assert!(validate(&format!("{RECEIPT}{}\n", lines[1])).is_err());
        assert!(validate(&format!("{RECEIPT}foreign\tother\n")).is_err());
        let hash = lines[1].split('\t').nth(12).unwrap();
        assert!(validate(&RECEIPT.replacen(hash, &"0".repeat(64), 1)).is_err());
        assert!(
            same_notice(
                b"altered license",
                include_bytes!("../fixtures/DART-LICENSE")
            )
            .is_err()
        );
    }
}
