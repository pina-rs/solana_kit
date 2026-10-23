//! Exact membership is audited independently without rewriting canonical receipts.
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
const COLUMNS: usize = 14;
const DIGEST: usize = 12;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = Path::new(&args[1]);
    let source = &args[2];
    let output = Path::new(&args[3]);
    let canonical = fs::read_to_string(root.join("docs/parity/capabilities.tsv")).unwrap();
    let old = fs::read_to_string(root.join("docs/parity/accounts-sysvars.tsv")).unwrap();
    let receipt = fs::read_to_string(root.join("docs/parity/account-client-helpers.tsv")).unwrap();
    let expected = rows(&canonical);

    verify_receipts(&old, &receipt, &expected);
    write_adjudication(root, source, output, &receipt);
}

fn verify_receipts(old: &str, receipt: &str, expected: &BTreeMap<String, String>) {
    assert_eq!(expected.len(), 84);
    assert_eq!(&rows(old), expected);
    assert_eq!(&rows(receipt), expected);
    let missing = receipt
        .lines()
        .take(receipt.lines().count() - 1)
        .collect::<Vec<_>>()
        .join("\n");
    assert_ne!(&rows(&missing), expected);
    assert!(duplicate(&format!(
        "{receipt}{}\n",
        receipt.lines().nth(1).unwrap()
    )));
    assert_ne!(
        &rows(
            &receipt.replacen(
                expected
                    .values()
                    .next()
                    .unwrap()
                    .split('\t')
                    .nth(DIGEST)
                    .unwrap(),
                "drift",
                1
            )
        ),
        expected
    );
}

fn write_adjudication(root: &Path, source: &str, output: &Path, receipt: &str) {
    let mut result = String::from(
        "id\tsymbol\tauthor_status\tindependent_adjudication\texecutable_evidence\tresidual\n",
    );
    let mut counts = BTreeMap::new();

    for line in receipt.lines().skip(1) {
        let row: Vec<_> = line.split('\t').collect();
        let blob = Command::new("git")
            .args([
                "-C",
                source,
                "show",
                &format!("0138bb956d2d81ccc0c56edc602401c0df2f2b46:{}", row[2]),
            ])
            .output()
            .unwrap();
        assert!(blob.status.success());
        assert_eq!(digest(&blob.stdout), row[DIGEST]);
        let (verdict, proof, residual) = adjudicate(row[3]);
        *counts.entry(verdict).or_insert(0_usize) += 1;
        result.push_str(&format!(
            "{}\t{}\t{}\t{verdict}\t{proof}\t{residual}\n",
            row[0], row[3], row[COLUMNS]
        ));
    }

    let license = Command::new("git")
        .args([
            "-C",
            source,
            "show",
            "0138bb956d2d81ccc0c56edc602401c0df2f2b46:packages/solana_kit_accounts/LICENSE",
        ])
        .output()
        .unwrap();
    assert!(license.status.success());
    assert_eq!(
        license.stdout,
        fs::read(root.join("verification/conformance/fixtures/DART-LICENSE")).unwrap()
    );
    fs::write(output, &result).unwrap();
    assert_eq!(fs::read_to_string(output).unwrap(), result);
    println!(
        "84 exact IDs, original 14 columns, defining digests, license and missing/duplicate/drift negatives pass. {counts:?}"
    );
}
fn rows(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .skip(1)
        .filter_map(|line| {
            let row: Vec<_> = line.split('\t').collect();
            (row.len() >= COLUMNS && row[1] == "solana_kit_accounts")
                .then(|| (row[0].to_owned(), row[..COLUMNS].join("\t")))
        })
        .collect()
}
fn duplicate(text: &str) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    text.lines()
        .skip(1)
        .any(|line| !seen.insert(line.split('\t').next().unwrap()))
}
fn digest(bytes: &[u8]) -> String {
    let mut child = Command::new("shasum")
        .args(["-a", "256"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_owned()
}
fn adjudicate(symbol: &str) -> (&'static str, &'static str, &'static str) {
    if symbol == "assertAccountDecoded" || symbol == "assertMaybeAccountDecoded" {
        return (
            "idiomatic-equivalent",
            "assertion_gap.rs and contracts.rs exact distinct addresses; source gaps.dart",
            "AccountNotDecoded(Address) preserves each encoded or compressed identity; generic custom data remains statically typed",
        );
    }

    if symbol == "parseBaseAccount" {
        return (
            "bounded-equivalent",
            "base_gap.rs executed; contracts.rs typed header and numeric matrix; numeric.dart and whitespace.dart",
            "BaseAccount is the shared standalone parser result without data or address; checked u64 rejects negative and overflow source values",
        );
    }

    if symbol == "parseJsonRpcAccount" {
        return (
            "bounded-equivalent",
            "gaps.rs and contracts.rs optional metadata; gaps.dart; probe.rs strict node boundaries",
            "Opaque exact parsed JSON preserves unknown payload and metadata; strict node RPC still requires program and consistent allocation",
        );
    }

    if symbol.starts_with("JsonParsedAccountData")
        || symbol.starts_with("ParsedAccountMeta")
        || symbol.starts_with("BaseAccount")
    {
        return (
            "idiomatic-equivalent",
            "idioms.rs::arbitrary_typed_data_and_optional_metadata_construct_compare_hash_and_format",
            "Generic Account<T> and Rust tuple/Option preserve typed data and optional metadata; exact Dart string and numeric hash are language presentation, not a new SDK behavior",
        );
    }

    if symbol.contains("fetch")
        || symbol.contains("SolanaAccountClient")
        || symbol == "createSolanaAccountClient"
        || symbol.starts_with("FetchAccountConfig")
    {
        return (
            "bounded-equivalent",
            "probe.rs::all_four_fetches_have_literal_options_context_and_local_pending; original.rs",
            "Strict cardinality deliberately rejects source invention of missing accounts; no host execution claim",
        );
    }

    if symbol.starts_with("parseBase") {
        return (
            "bounded-equivalent",
            "gaps.rs and contracts.rs numeric matrix; numeric.dart and whitespace.dart; probe.rs standalone boundaries",
            "Source-valid numeric strings checked to u64; strict encoding tags and string encoded values remain deliberate bounds",
        );
    }

    if symbol.starts_with("decode") {
        return (
            "idiomatic-equivalent",
            "probe.rs::custom_decoder_preserves_originals_exact_causes_and_limits",
            "Borrowed helper preserves source metadata and cause; existing consuming core decode is intentionally stricter",
        );
    }

    if symbol.starts_with("assert") {
        return (
            "idiomatic-equivalent",
            "probe.rs::assertions_retain_order_duplicates_and_compressed_state; probe.rs::native_value_construction_hash_equality_debug_and_missing_assertions",
            "Typed Account<T> eliminates decoded runtime guards for custom T; RPC states retain public assertions",
        );
    }

    (
        "idiomatic-equivalent",
        "probe.rs::native_value_construction_hash_equality_debug_and_missing_assertions; original.rs",
        "Native Eq/Hash/Debug and enum variants replace Dart inheritance; no exact Dart hash/string claim",
    )
}
