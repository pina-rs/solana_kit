//! Checks the exact scoped signer inventory, source hashes, license and core graph.
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::Path,
    process::Command,
};

const STATUS_COLUMN: usize = 6;

fn audit_receipt(inventory: &str, receipt: &str) -> BTreeMap<String, String> {
    let rows: BTreeMap<_, _> = inventory
        .lines()
        .skip(1)
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .filter(|columns| columns[1] == "solana_kit_signers")
        .map(|columns| (columns[0], columns))
        .collect();
    let mut seen = BTreeSet::new();
    let mut files = BTreeMap::new();
    let mut partial = 0;
    let mut pending = 0;

    for line in receipt.lines().skip(1) {
        let columns: Vec<_> = line.split('\t').collect();
        assert_eq!(columns.len(), 9);
        assert!(seen.insert(columns[0]));
        let source = rows.get(columns[0]).expect("every receipt ID is canonical");
        assert_eq!(&columns[..4], &source[..4]);
        assert_eq!(columns[4], source[12]);
        assert_eq!(columns[5], source[13]);
        assert!(columns[5].starts_with("0138bb956d2d81ccc0c56edc602401c0df2f2b46:"));
        files.insert(source[2].to_string(), source[12].to_string());

        match columns[STATUS_COLUMN] {
            "partial" => partial += 1,
            "pending" => pending += 1,
            _ => panic!("unexpected completion claim"),
        }
    }

    assert_eq!(seen.len(), rows.len());
    assert_eq!((seen.len(), partial, pending), (85, 39, 46));
    assert_eq!(files.len(), 17);
    files
}

fn audit_sources(
    repository: &Path,
    reference: &Path,
    files: &BTreeMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    for (file, expected) in files {
        let output = Command::new("shasum")
            .args(["-a", "256"])
            .arg(reference.join(file))
            .output()?;
        assert!(output.status.success());
        let output = String::from_utf8(output.stdout)?;
        assert_eq!(
            output.split_whitespace().next().unwrap(),
            expected,
            "{file}"
        );
    }

    let license = fs::read(reference.join("packages/solana_kit_signers/LICENSE"))?;
    assert_eq!(
        license,
        fs::read(repository.join("crates/solana_kit_signers/DART-LICENSE"))?
    );
    println!(
        "PASS 85 exact unique IDs, 39 partial, 46 pending, zero complete, 17 source hashes and exact license"
    );
    Ok(())
}

fn audit_graph(path: &Path) -> Result<(), Box<dyn Error>> {
    let graph = fs::read_to_string(path)?;
    let forbidden = [
        "tokio",
        "js-sys",
        "web-sys",
        "getrandom",
        "rand ",
        "solana-client",
        "agave",
    ];
    let clean = |text: &str| forbidden.iter().all(|needle| !text.contains(needle));
    assert!(clean(&graph));
    assert!(!clean("solana_kit_signers\n tokio v1.54.1"));
    assert_eq!(graph.lines().count(), 3);
    assert!(graph.contains("bs58 v0.5.1"));
    assert!(graph.contains("solana_kit_core v0.1.0"));
    println!(
        "PASS exact three-package default normal/build graph; Tokio negative control rejected"
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(
        args.len(),
        4,
        "usage: signer-audit <repository> <reference> <default-graph>"
    );
    let repository = Path::new(&args[1]);
    let reference = Path::new(&args[2]);
    let inventory = fs::read_to_string(repository.join("docs/parity/capabilities.tsv"))?;
    let receipt = fs::read_to_string(repository.join("docs/parity/signer-capabilities.tsv"))?;
    let files = audit_receipt(&inventory, &receipt);
    audit_sources(repository, reference, &files)?;
    audit_graph(Path::new(&args[3]))
}
