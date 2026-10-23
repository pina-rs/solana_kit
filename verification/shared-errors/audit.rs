use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::Path,
    process::Command,
};

fn main() {
    let arguments: Vec<_> = env::args().collect();
    let root = Path::new(&arguments[1]);
    let source = Path::new(&arguments[2]);
    let inventory =
        fs::read_to_string(root.join("docs/parity/capabilities.tsv")).expect("canonical inventory");
    let rows: Vec<_> = inventory
        .lines()
        .filter(|line| line.split('\t').nth(1) == Some("solana_kit_errors"))
        .collect();
    validate(&rows, &rows).expect("exact membership");
    let mut missing = rows.clone();

    missing.pop();
    assert!(validate(&rows, &missing).is_err());
    let mut duplicate = rows.clone();
    duplicate[1] = duplicate[0];
    assert!(validate(&rows, &duplicate).is_err());
    let changed = rows[0].replace(rows[0].split('\t').nth(12).expect("hash"), "invalid-digest");
    let mut invalid = rows.clone();
    invalid[0] = &changed;

    assert!(validate(&rows, &invalid).is_err());
    verify_sources(&rows, source);
    let mut output = String::from(
        "source_id\tstatus\trust_contract\tbehavior_proof\tresidual\tsource_sha256\tprovenance\n",
    );

    for row in &rows {
        let fields: Vec<_> = row.split('\t').collect();
        let (status, contract, residual) = contract(fields[2]);
        let contract = format!("{} => {contract}", fields[3]);
        output.push_str(&format!("{}\t{status}\t{contract}\tcrates/solana_kit_core/tests/shared_errors.rs;crates/solana_kit_client/tests/shared_errors.rs;verification/shared-errors/consumer\t{residual}\t{}\t{}\n", fields[0],fields[12],fields[13]));
    }

    fs::write(root.join("docs/parity/shared-errors.tsv"), &output).expect("scoped receipt");
    let actual =
        fs::read_to_string(root.join("docs/parity/shared-errors.tsv")).expect("actual receipt");
    assert_eq!(actual, output);
    assert_eq!(actual.lines().count(), 439);
    println!("438 exact IDs; source digests; missing, duplicate and changed-hash controls passed");
}

fn validate(expected: &[&str], actual: &[&str]) -> Result<(), &'static str> {
    let expected: BTreeSet<_> = expected.iter().copied().collect();
    let unique: BTreeSet<_> = actual.iter().copied().collect();

    if expected.len() != 438 || actual.len() != expected.len() || unique != expected {
        return Err("membership or original metadata changed");
    }

    Ok(())
}

fn verify_sources(rows: &[&str], source: &Path) {
    let mut files = BTreeMap::new();

    for row in rows {
        let fields: Vec<_> = row.split('\t').collect();
        files.insert(fields[2], fields[12]);
    }

    for (path, expected) in files {
        let output = Command::new("shasum")
            .args(["-a", "256"])
            .arg(source.join(path))
            .output()
            .expect("source hash");
        assert!(output.status.success());
        let actual = String::from_utf8(output.stdout).expect("hash output");
        assert_eq!(actual.split_whitespace().next(), Some(expected));
    }
}

fn contract(path: &str) -> (&'static str, &'static str, &'static str) {
    if path.ends_with("codes.dart") || path.ends_with("messages.dart") {
        return (
            "mapped-bounded",
            "errors::ErrorCode;CodeDefinition;definitions;message;Eq;Hash;Ord",
            "365 exact definitions and templates; named later-phase codes do not imply later-phase integration",
        );
    }

    if path.ends_with("error_domain.dart") {
        return (
            "mapped-bounded",
            "errors::ErrorCode::domain;ErrorDomain;Eq",
            "exact pinned ranges including known unclassified codes; typed equality replaces runtime guards",
        );
    }

    if path.ends_with("context.dart") {
        return (
            "mapped-bounded",
            "client::diagnostics::{encode_context,decode_context};errors::{Context,ContextValue}",
            "five independent Dart wire literals; deterministic key order; 64 keys/key bytes, 1024 text bytes, depth 8, aggregate 256 nodes/16KiB; sensitive keys and numeric overflow are errors",
        );
    }

    if path.ends_with("simulation_errors.dart") {
        return (
            "mapped-bounded",
            "errors::Diagnostic::simulation_cause;core::error::Error::source",
            "one-level typed cause identity; non-simulation and absent-cause diagnostics remain unchanged; arbitrary objects use native typed Error",
        );
    }

    let file = path.rsplit('/').next().expect("defining file");

    let (api, residual) = match file {
        "error.dart" => (
            "Diagnostic::{new,code,context}; ErrorFacts; Error::downcast_ref",
            "native typed Error replaces runtime guard; Display/Debug intentionally redact context; explicit ErrorCode::message renders bounded public facts",
        ),
        "error_helpers.dart" => (
            "Context::{insert,get,normalized}; Diagnostic::{from_context,wrap,with_source}; Error::source/downcast_ref",
            "independent Dart normalization/cause facts; typed source retains cause code/context/type instead of unsafe cause stringification; explicit context fields remain queryable",
        ),
        "instruction_error.dart" => (
            "client::diagnostics::instruction_error; TransactionFailure::into_diagnostic",
            "54 independent source mappings and future names/context; index u8/custom u32 validated; Borsh payload retained; no numeric truncation",
        ),
        "transaction_error.dart" => (
            "client::diagnostics::transaction_error; KnownTransactionFailure::error_code",
            "36 independent source mappings plus nested/future contexts; strict protocol integer/shape boundaries; additional runtime variants retain typed names",
        ),
        "json_rpc_error.dart" => (
            "RemoteError::into_diagnostic; client::diagnostics::json_rpc_error",
            "preflight err projection, 13 metadata defaults, typed execution cause, server-message/data rules; unknown i64 preserved rather than source malformed-code fallback; invalid shapes are explicit errors",
        ),
        "message_formatter.dart" => (
            "ErrorCode::message; ContextValue::Display",
            "365 exact templates and literal/context checks; 4096-byte pre-append rendering limit; deterministic object key ordering",
        ),
        "rpc_enum_errors.dart" => (
            "RpcEnumErrorConfig::{base,names}; rpc_enum_error callback; RpcEnumError::raw",
            "checked source ordered offsets, typed FnOnce callback and exact raw input; strict one-key maps and bounded context; integer overflow/undefined offsets rejected",
        ),
        _ => panic!("unadjudicated shared error source: {path}"),
    };

    ("mapped-bounded", api, residual)
}
