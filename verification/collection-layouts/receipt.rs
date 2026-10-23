//! Reconcile exact optional-layout membership without promoting canonical status.

use std::{collections::BTreeSet, env, fs, path::Path, process::Command};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const PIN: &str = "0138bb956d2d81ccc0c56edc602401c0df2f2b46";
const COLUMNS: usize = 14;
const EXPECTED: usize = 73;
const DIGEST: usize = 12;
const PROVENANCE: usize = 13;

fn selected(fields: &[&str]) -> bool {
    if fields[1] == "solana_kit_options" {
        return true;
    }

    fields[1] == "solana_kit_codecs_data_structures"
        && ["/array.dart", "/boolean.dart", "/nullable.dart"]
            .iter()
            .any(|suffix| fields[2].ends_with(suffix))
}

fn validate(rows: &[String], original: &[String]) -> Result<()> {
    let expected: BTreeSet<_> = original.iter().collect();
    let actual: BTreeSet<_> = rows.iter().collect();

    if rows.len() != EXPECTED || actual.len() != rows.len() || actual != expected {
        return Err("missing, duplicate or altered canonical row".into());
    }

    Ok(())
}

fn check_source(reference: &Path, fields: &[&str]) -> Result<()> {
    let file = reference.join(fields[2]);
    let output = Command::new("shasum")
        .args(["-a", "256"])
        .arg(file)
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;

    if !output.status.success() || stdout.split_whitespace().next() != Some(fields[DIGEST]) {
        return Err("defining source digest differs".into());
    }

    if !fields[PROVENANCE].starts_with(PIN) {
        return Err("source pin differs".into());
    }

    Ok(())
}

fn contract(path: &str) -> (&str, &str) {
    if path.ends_with("/array.dart") {
        return (
            "Sequence; Counted; OptionalCount; ItemSentinel",
            "u32 resolved count domain; explicit minimum item width; metadata remains open",
        );
    }

    if path.ends_with("/boolean.dart") {
        return (
            "Bool; Boolean<P>; Mapped",
            "u32 normalized storage; runtime metadata and Dart error taxonomy remain open",
        );
    }

    if path.ends_with("/nullable.dart") || path.ends_with("/option_codec.dart") {
        return (
            "OptionCodec; Nullable; Absence; Rust Option<T>",
            "explicit zero width; metadata and arbitrary callback equivalence remain open",
        );
    }

    (
        "Rust Option<T>",
        "source helper and object contracts require separate focused equivalence proof",
    )
}

fn receipt(rows: &[String]) -> String {
    let mut output = String::from(
        "id\tpackage\tpath\tsymbol\tkind\tline\towner\tphase\tstatus\tevidence\tnotes\tunit\tsha256\tprovenance\tunit_status\trust_contract\tunit_evidence\tresidual\n",
    );

    for row in rows {
        let fields: Vec<_> = row.split('\t').collect();
        let (mut contract, mut residual) = contract(fields[2]);
        let mut status = "partial";

        if fields[2].contains("/solana_kit_options/") && !fields[2].ends_with("/option_codec.dart")
        {
            (status, contract, residual) = native_contract(fields[3]);
        }

        output.push_str(row);
        output.push_str(&format!("\t{status}\t{contract}\tcollection_layouts.rs; native_option_values.rs; collection-layouts-dart.tsv\t{residual}\n"));
    }

    output
}

fn native_contract(symbol: &str) -> (&'static str, &'static str, &'static str) {
    match symbol {
        "None" | "None.new" | "Option" | "Option.new" | "Some" | "Some.new" | "none" | "some" => (
            "mapped-native",
            "Option<T>; Some(T); None; const constructors",
            "Rust enum replaces sealed hierarchy and prevents constructing the abstract base",
        ),
        "Some.value" | "isSome" | "isNone" => (
            "mapped-native",
            "pattern extraction; Option::is_some; Option::is_none",
            "statically typed values preserve nested Some(None)",
        ),
        "None.operator==" | "Some.operator==" => (
            "mapped-native",
            "Option<T>: PartialEq and Eq",
            "same-type equality; cross-type comparisons rejected statically",
        ),
        "None.toString" => ("mapped-native", "Debug renders None", "exact None spelling"),
        "unwrapOption" | "wrapNullable" => (
            "mapped-native",
            "identity over Option<T>",
            "Option already is the Rust nullable representation; nested absence retained",
        ),
        "unwrapOptionOr" => (
            "mapped-native",
            "Option::unwrap_or_else",
            "fallback is lazy and called exactly once only for None",
        ),
        "None.hashCode" | "Some.hashCode" => (
            "partial",
            "Option<T>: Hash",
            "equal values hash equally; Dart numeric hash and runtime type identity differ",
        ),
        "Some.toString" => (
            "partial",
            "Debug renders Some(513) for numeric values",
            "generic Dart string formatting differs from Rust Debug",
        ),
        "isOption" => (
            "partial",
            "static Option<T> parameter contract",
            "arbitrary heterogeneous runtime Option introspection has no built-in generic equivalent",
        ),
        "unwrapOptionRecursively" => (
            "partial",
            "Option::flatten; typed iterator map",
            "homogeneous nested values tested; arbitrary heterogeneous recursive list/map traversal remains open",
        ),
        _ => panic!("unadjudicated option symbol: {symbol}"),
    }
}

fn controls(original: &[String]) -> Result<()> {
    let missing = original[1..].to_vec();
    let mut duplicate = original.to_vec();
    duplicate[0] = duplicate[1].clone();
    let mut drift = original.to_vec();
    drift[0].push_str("changed");

    for rows in [missing, duplicate, drift] {
        if validate(&rows, original).is_ok() {
            return Err("negative membership control escaped".into());
        }
    }

    Ok(())
}

fn canonical_rows(root: &Path, reference: &Path) -> Result<Vec<String>> {
    let canonical = fs::read_to_string(root.join("docs/parity/capabilities.tsv"))?;
    let mut rows = Vec::new();

    for line in canonical.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();

        if fields.len() != COLUMNS {
            return Err("canonical column drift".into());
        }

        if selected(&fields) {
            check_source(reference, &fields)?;
            rows.push(line.to_owned());
        }
    }

    Ok(rows)
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let root = Path::new(&args[1]);
    let reference = Path::new(&args[2]);
    let rows = canonical_rows(root, reference)?;

    validate(&rows, &rows)?;
    controls(&rows)?;
    let mut corrupted: Vec<_> = rows[0].split('\t').collect();
    corrupted[DIGEST] = "invalid-digest";

    if check_source(reference, &corrupted).is_ok() {
        return Err("source digest negative control escaped".into());
    }

    let output = receipt(&rows);
    fs::write(&args[3], &output)?;

    if fs::read_to_string(&args[3])? != output {
        return Err("receipt readback differs".into());
    }

    println!(
        "PASS 73 exact canonical rows; source hashes; missing/duplicate/drift controls; readback"
    );
    Ok(())
}
