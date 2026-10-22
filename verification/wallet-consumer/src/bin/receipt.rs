use sha2::{Digest, Sha256};
use std::{env, error::Error, fs, path::Path};

type Result<Value> = std::result::Result<Value, Box<dyn Error>>;
const SOURCE_HASH_COLUMN: usize = 12;

fn main() -> Result<()> {
    let arguments: Vec<_> = env::args().collect();
    let repository = Path::new(arguments.get(1).ok_or("missing repository")?);
    let reference = Path::new(arguments.get(2).ok_or("missing immutable reference")?);
    let catalog = fs::read_to_string(repository.join("docs/parity/capabilities.tsv"))?;
    let mut receipt = String::from("id\tsource_sha256\tstatus\tevidence\tremaining\n");
    let rows: Vec<_> = catalog
        .lines()
        .skip(1)
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .filter(|columns| columns.get(1) == Some(&"solana_kit_wallet_standard"))
        .collect();

    if rows.len() != 203 {
        return Err("wallet declaration catalog changed".into());
    }

    for columns in &rows {
        append_row(&mut receipt, columns, reference)?;
    }

    fs::write(repository.join("docs/parity/wallet-standard.tsv"), receipt)?;
    println!(
        "{} exact canonical declaration IDs and verified source hashes; all partial",
        rows.len()
    );
    Ok(())
}

fn append_row(receipt: &mut String, columns: &[&str], reference: &Path) -> Result<()> {
    let path = columns.get(2).ok_or("missing source path")?;
    let expected_hash = columns
        .get(SOURCE_HASH_COLUMN)
        .ok_or("missing source hash")?;
    let source = fs::read(reference.join(path))?;
    let hash = format!("{:x}", Sha256::digest(&source));

    if &hash != expected_hash {
        return Err("immutable source hash mismatch".into());
    }

    let remaining = if path.ends_with("logos.dart") {
        "independent upstream asset provenance review pending"
    } else {
        "independent review and platform integration pending"
    };

    receipt.push_str(&format!(
        "{}\t{}\tpartial\tcrates/solana_kit_wallet/tests;verification/wallet-consumer\t{}\n",
        columns[0], hash, remaining
    ));
    Ok(())
}
