//! Audits every transaction declaration and updates only bounded nonce mappings.
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::Path,
    process::Command,
};

const PIN: &str = "0138bb956d2d81ccc0c56edc602401c0df2f2b46";
const COUNT: usize = 746;
const PHASE_COLUMN: usize = 7;
const CANONICAL_COLUMNS: [usize; 10] = [0, 1, 2, 3, 4, 5, 6, PHASE_COLUMN, 12, 13];
const SYMBOLS: &[&str] = &[
    "DurableNonceConfig",
    "DurableNonceConfig.new",
    "DurableNonceConfig.nonce",
    "DurableNonceConfig.nonceAccountAddress",
    "DurableNonceConfig.nonceAuthorityAddress",
    "assertIsTransactionMessageWithDurableNonceLifetime",
    "isTransactionMessageWithDurableNonceLifetime",
    "setTransactionMessageLifetimeUsingDurableNonce",
    "createAdvanceNonceAccountInstruction",
    "isAdvanceNonceAccountInstruction",
    "DurableNonceLifetimeConstraint",
    "DurableNonceLifetimeConstraint.new",
    "DurableNonceLifetimeConstraint.nonce",
];

fn rows(text: &str) -> BTreeMap<String, Vec<String>> {
    let mut result = BTreeMap::new();

    for line in text.lines().skip(1) {
        let row: Vec<String> = line.split('\t').map(str::to_owned).collect();
        assert_eq!(row.len(), 14);
        assert!(result.insert(row[0].clone(), row).is_none(), "duplicate ID");
    }

    result
}

fn audit_sources(reference: &Path, canonical: &BTreeMap<String, Vec<String>>) {
    let sources: BTreeSet<_> = canonical.values().map(|row| (&row[2], &row[12])).collect();

    for (path, hash) in sources {
        let output = Command::new("git")
            .arg("-C")
            .arg(reference)
            .args(["show", &format!("{PIN}:{path}")])
            .output()
            .expect("read immutable source");
        assert!(output.status.success());
        assert_eq!(
            fs::read(reference.join(path)).expect("source"),
            output.stdout,
            "defining source changed"
        );
        let digest = Command::new("shasum")
            .args(["-a", "256"])
            .arg(reference.join(path))
            .output()
            .expect("hash source");
        assert!(digest.status.success());
        assert_eq!(
            String::from_utf8(digest.stdout)
                .unwrap()
                .split_whitespace()
                .next(),
            Some(hash.as_str())
        );
    }
}

fn mapped(row: &[String]) -> bool {
    row[1] == "solana_kit_transaction_messages"
        && (row[2].ends_with("/durable_nonce.dart")
            || row[2].ends_with("/durable_nonce_instruction.dart")
            || row[2].ends_with("/lifetime.dart"))
        && SYMBOLS.contains(&row[3].as_str())
}

fn update_mappings(
    actual: &mut BTreeMap<String, Vec<String>>,
    canonical: &BTreeMap<String, Vec<String>>,
    write: bool,
) -> usize {
    let mut partial = 0;

    for (id, row) in actual {
        let source = &canonical[id];

        for column in CANONICAL_COLUMNS {
            assert_eq!(row[column], source[column], "canonical column {column}");
        }

        assert!(matches!(row[8].as_str(), "pending" | "partial"));

        if mapped(row) && write {
            row[8] = "partial".into();
            row[9] = "verification/durable-nonce/README.md;verification/durable-nonce/src/lib.rs;verification/durable-nonce/tests/oracle.rs".into();
            row[10] = "explicit deterministic construction and structural inspection only;30/30 compiled Dart bytes and ordered metas match after shared comparator integration;prior158 partial evidence and3 original collation failures retained;lookup resolution,state authentication,validator and portable host execution remain unproved;Display/hash/equality wrappers not mapped".into();
            row[11] = "158-durable-nonce-transactions;166-nonce-collation-integration".into();
        }

        partial += usize::from(row[8] == "partial");
    }

    partial
}

fn write_rows(path: &Path, text: &str, actual: &BTreeMap<String, Vec<String>>) {
    let mut output = text.lines().next().unwrap().to_owned();
    output.push('\n');

    for row in actual.values() {
        output.push_str(&row.join("\t"));
        output.push('\n');
    }

    fs::write(path, output).unwrap();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let repository = Path::new(args.get(1).expect("repository argument"));
    let reference = Path::new(args.get(2).expect("reference argument"));
    let write = args.get(3).is_some_and(|arg| arg == "--write");
    let inventory =
        rows(&fs::read_to_string(repository.join("docs/parity/capabilities.tsv")).unwrap());
    let canonical: BTreeMap<_, _> = inventory
        .into_iter()
        .filter(|(_, row)| row[PHASE_COLUMN] == "3")
        .collect();

    let path = repository.join("docs/parity/message-transactions.tsv");
    let text = fs::read_to_string(&path).unwrap();
    let mut actual = rows(&text);

    assert_eq!(actual.len(), COUNT);
    assert_eq!(
        actual.keys().collect::<Vec<_>>(),
        canonical.keys().collect::<Vec<_>>()
    );
    audit_sources(reference, &canonical);
    let partial = update_mappings(&mut actual, &canonical, write);

    if write {
        write_rows(&path, &text, &actual);
    }

    println!(
        "{COUNT} canonical IDs and actual defining hashes match; {partial} partial; {} pending; none complete",
        COUNT - partial
    );
}
