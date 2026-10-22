//! Extracts every exact reference signer declaration without altering the ledger.

use std::{env, error::Error, fs};

const PARTIAL: &[&str] = &[
    "deduplicateSigners",
    "getSignerAddress",
    "KeyPairSigner",
    "KeyPairSigner.address",
    "KeyPairSigner.keyPair",
    "KeyPairSigner.signMessages",
    "KeyPairSigner.signTransactions",
    "createKeyPairSignerFromPrivateKeyBytes",
    "createSignerFromKeyPair",
    "MessagePartialSigner",
    "MessagePartialSigner.address",
    "MessagePartialSigner.signMessages",
    "partiallySignTransactionWithSigners",
    "signAndSendTransactionWithSigners",
    "signTransactionWithSigners",
    "TransactionModifyingSigner",
    "TransactionModifyingSigner.address",
    "TransactionModifyingSigner.modifyAndSignTransactions",
    "TransactionPartialSigner",
    "TransactionPartialSigner.address",
    "TransactionPartialSigner.signTransactions",
    "TransactionSendingSigner",
    "TransactionSendingSigner.address",
    "TransactionSendingSigner.signAndSendTransactions",
    "getTransactionSignerAddress",
    "assertContainsResolvableTransactionSendingSigner",
];

const COLUMN_COUNT: usize = 14;
const HASH_COLUMN: usize = 12;
const PROVENANCE_COLUMN: usize = 13;

fn coverage(covered: bool) -> (&'static str, &'static str, &'static str) {
    if covered {
        return (
            "partial",
            "crates/solana_kit_signers/tests;verification/signer-capabilities/tests",
            "automatic heterogeneous transaction roles and bounded ordered transaction batches; message batches, attachment and configuration remain pending",
        );
    }

    ("pending", "-", "not implemented by this unit")
}

fn scoped_row(row: &str) -> Result<Option<(String, bool)>, &'static str> {
    let columns: Vec<_> = row.split('\t').collect();

    if columns.get(1) != Some(&"solana_kit_signers") {
        return Ok(None);
    }

    if columns.len() != COLUMN_COUNT {
        return Err("unexpected canonical inventory schema");
    }

    let covered = PARTIAL.contains(&columns[3]);
    let (status, evidence, limit) = coverage(covered);
    let fields = [
        columns[0],
        columns[1],
        columns[2],
        columns[3],
        columns[HASH_COLUMN],
        columns[PROVENANCE_COLUMN],
        status,
        evidence,
        limit,
    ];
    Ok(Some((fields.join("\t"), covered)))
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<_> = env::args().collect();

    if arguments.len() != 3 {
        return Err("usage: signer-receipt <canonical.tsv> <scoped.tsv>".into());
    }

    let ledger = fs::read_to_string(&arguments[1])?;
    let mut output =
        String::from("id\tpackage\tpath\tsymbol\tsha256\tprovenance\tstatus\tevidence\tlimit\n");
    let mut count = 0;
    let mut partial = 0;

    for row in ledger.lines().skip(1) {
        if let Some((record, covered)) = scoped_row(row)? {
            output.push_str(&record);
            output.push('\n');
            count += 1;
            partial += usize::from(covered);
        }
    }

    if count == 0 {
        return Err("signer inventory is empty".into());
    }

    fs::write(&arguments[2], output)?;
    println!(
        "{count} exact IDs; {partial} partial; {} pending; zero complete",
        count - partial
    );
    Ok(())
}
