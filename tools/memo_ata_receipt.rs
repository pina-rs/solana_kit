//! Rebuilds the bounded client receipt from canonical declaration IDs and source hashes.

use std::{collections::BTreeMap, env, error::Error, fs, path::Path, process::Command};

type Failure = Box<dyn Error>;
const SOURCE_HASH_COLUMN: usize = 12;

fn source_hash(path: &Path) -> Result<String, Failure> {
    let output = Command::new("shasum")
        .args(["-a", "256"])
        .arg(path)
        .output()?;

    if !output.status.success() {
        return Err("source hash command failed".into());
    }

    Ok(String::from_utf8(output.stdout)?
        .split_whitespace()
        .next()
        .ok_or("missing source hash")?
        .to_owned())
}

fn contract(symbol: &str, memo: bool) -> (&'static str, &'static str, &'static str) {
    if memo {
        return memo_contract(symbol);
    }

    if symbol.starts_with("AssociatedTokenSeeds")
        || symbol.contains("AssociatedTokenAddress")
        || symbol.contains("AssociatedTokenPda")
    {
        return (
            "partial",
            "AssociatedTokenSeeds",
            "three address seeds; synchronous highest-bump derivation; no async wrapper or source Display",
        );
    }

    if symbol.contains("Error") {
        return (
            "partial",
            "ERROR_INVALID_OWNER,error_message",
            "known code and message; no separately named boolean wrapper",
        );
    }

    if symbol == "ataProgramAddress" {
        return ("implemented", "PROGRAM", "canonical ATA program identity");
    }

    (
        "partial",
        "AssociatedTokenInstruction,CreateAccounts,RecoverNestedAccounts",
        "all three canonical constructors; exact account metadata; full strict parsing; mutable discriminators and standalone data codecs absent",
    )
}

fn memo_contract(symbol: &str) -> (&'static str, &'static str, &'static str) {
    if symbol.starts_with("ExtractedMemo")
        || symbol == "getMemosFromInstructions"
        || symbol == "supportedMemoProgramAddresses"
    {
        return (
            "implemented",
            "ExtractedMemo,memos_from_instructions,SUPPORTED_PROGRAMS",
            "three historical identities; order/index/raw bytes; strict UTF8 and source BOM/null behavior; explicit budgets",
        );
    }

    (
        "partial",
        "add_memo,add_memo_with_program,decode_memo,parse_instruction",
        "raw UTF8 payload and ordered signer accounts; supported identities only; full metadata validation; no standalone source data class",
    )
}

fn main() -> Result<(), Failure> {
    let args: Vec<_> = env::args().collect();
    let dart = Path::new(args.get(1).ok_or("usage: memo_ata_receipt DART_SOURCE")?);
    let canonical = fs::read_to_string("docs/parity/capabilities.tsv")?;
    let mut hashes = BTreeMap::new();
    let mut receipt = String::from("id\tunit115_status\trust_api\tboundary\tevidence\n");
    let mut count = 0;

    for line in canonical.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();
        let package = fields[1];

        if package != "solana_kit_memo" && package != "solana_kit_associated_token_account" {
            continue;
        }

        let digest = hashes
            .entry(fields[2])
            .or_insert(source_hash(&dart.join(fields[2]))?);

        if digest != fields[SOURCE_HASH_COLUMN] {
            return Err(format!("canonical hash mismatch for {}", fields[2]).into());
        }

        let (status, api, boundary) = contract(fields[3], package == "solana_kit_memo");
        let evidence = "docs/verification/memo-associated-token.md";
        receipt.push_str(&format!(
            "{}\t{status}\t{package}::{{{api}}}\t{boundary}\t{evidence}\n",
            fields[0]
        ));
        count += 1;
    }

    fs::write("docs/parity/memo-associated-token.tsv", receipt)?;
    println!(
        "validated {count} exact canonical IDs across {} source files",
        hashes.len()
    );
    Ok(())
}
