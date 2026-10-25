//! Strict, standalone canonical coverage and immutable-source auditor.
use std::{collections::BTreeSet, error::Error, fs, path::Path, process::Command};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const PACKAGE: &str = "solana_kit_instruction_plans";
const DECLARATIONS: usize = 212;
const SOURCE_FILES: usize = 11;
const INVENTORY_COLUMNS: usize = 14;
const HASH_COLUMN: usize = 12;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let root = Path::new(args.get(1).ok_or("expected SDK root")?);
    let reference = Path::new(args.get(2).ok_or("expected immutable Dart root")?);
    let inventory = fs::read_to_string(root.join("docs/parity/capabilities.tsv"))?;
    let expected = generate(&inventory)?;
    let path = root.join("verification/instruction-plans/canonical.tsv");

    if args.get(3).is_some_and(|arg| arg == "--generate") {
        fs::write(path, expected)?;

        return Ok(());
    }

    let actual = fs::read_to_string(path)?;
    validate(&actual, &expected)?;
    verify_hashes(&actual, reference)?;
    negative_controls(&actual, &expected)?;
    println!(
        "{DECLARATIONS} canonical IDs, {SOURCE_FILES} exact source hashes; missing, duplicate and hash-drift controls reject"
    );

    Ok(())
}

/// Inventory rows stay authoritative so new or missing declarations cannot disappear silently.
fn generate(inventory: &str) -> Result<String> {
    let mut output = String::from(
        "id\tsource_sha256\tsource_path\trust_contract\tstatus\tevidence\tdifference\n",
    );
    let mut count = 0;

    for line in inventory.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();

        if fields.get(1) != Some(&PACKAGE) {
            continue;
        }

        if fields.len() != INVENTORY_COLUMNS {
            return Err("malformed canonical inventory row".into());
        }

        let (contract, evidence, difference) = mapping(fields[2]);
        output.push_str(&format!(
            "{}\t{}\t{}\t{contract}\tpartial\t{evidence}\t{difference}; portable planning suite executes 43 mapped bodies on five hosts; declaration-level parity remains partial\n",
            fields[0], fields[HASH_COLUMN], fields[2]
        ));
        count += 1;
    }

    if count != DECLARATIONS {
        return Err("canonical declaration count changed".into());
    }

    Ok(output)
}

/// Paths choose the bounded contract; the final branch preserves unmatched evidence.
fn mapping(path: &str) -> (&'static str, &'static str, &'static str) {
    let core = "crates/solana_kit_core/tests/plans.rs";
    let client = "crates/solana_kit_client/tests/plans.rs";

    if path.contains("transaction_plan_result") {
        return (
            "solana_kit_core::plans::LeafResult / Status / ResultSummary / Plan",
            client,
            "typed context and exhaustive status matching replace unchecked map access",
        );
    }

    if path.contains("max_instructions") || path.contains("message_packer_errors") {
        return (
            "solana_kit_core::plans::MaxInstructions / Error::requires_new_candidate",
            core,
            "validated instruction budget and compiler errors replace dynamic error codes",
        );
    }

    if path.contains("transaction_planner") {
        return (
            "solana_kit_core::plans::Planner / AsyncMessageBuilder / MessageBuilder",
            "crates/solana_kit_core/tests/plans.rs; crates/solana_kit_client/tests/async_planning.rs; verification/instruction-plans/async_oracle.dart",
            "per-candidate awaited owned construction/update futures; portable Pending/wake/drop proof; caller-owned factories and complete source-contract parity remain partial",
        );
    }

    if path.contains("append_instruction_plan") {
        return (
            "solana_kit_core::plans::append_instruction_plan",
            core,
            "transactional returned draft with explicit instruction budget",
        );
    }

    if path.contains("transaction_plan_executor") {
        return (
            "solana_kit_client::plans::Executor / execute / ExecutionMode / ExecutionError",
            client,
            "concrete associated futures; owned typed partial context; Result retains partial tree",
        );
    }

    if path.contains("transaction_execution_boundary") {
        return (
            "solana_kit_client::plans::Boundary / Signer / Sender / StageError / BoundaryOutcome",
            "crates/solana_kit_client/tests/plans.rs; crates/solana_kit_client/tests/async_planning.rs",
            "caller supplies signing implementation; no heterogeneous signer list or network defaults",
        );
    }

    remaining_mapping(path)
}

/// Less common planning modules retain their own evidence and limitations.
fn remaining_mapping(path: &str) -> (&'static str, &'static str, &'static str) {
    let core = "crates/solana_kit_core/tests/plans.rs";
    let client = "crates/solana_kit_client/tests/plans.rs";

    if path.contains("transaction_plan_errors") {
        return (
            "solana_kit_client::plans::FailureDetails / FailureReport / FailureAction / ExecutionError",
            client,
            "typed preflight payload and original cause replace untyped error-context unwrapping",
        );
    }

    (
        "solana_kit_core::plans::Plan / InstructionLeaf / PlanInput",
        core,
        "owned enums and typed inputs replace runtime class assertions",
    )
}

fn validate(actual: &str, expected: &str) -> Result<()> {
    if actual != expected {
        return Err(
            "coverage rows differ from the authoritative inventory and contract mapping".into(),
        );
    }

    let ids: BTreeSet<_> = actual
        .lines()
        .skip(1)
        .filter_map(|line| line.split('\t').next())
        .collect();

    if ids.len() != DECLARATIONS {
        return Err("missing or duplicate canonical declarations".into());
    }

    Ok(())
}

/// Each source is hashed once because many declarations share a defining file.
fn verify_hashes(actual: &str, reference: &Path) -> Result<()> {
    let mut seen = BTreeSet::new();

    for line in actual.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();

        if !seen.insert(fields[2]) {
            continue;
        }

        let output = Command::new("shasum")
            .args(["-a", "256"])
            .arg(reference.join(fields[2]))
            .output()?;

        if !output.status.success() {
            return Err("could not hash immutable source".into());
        }

        if String::from_utf8(output.stdout)?.split_whitespace().next() != Some(fields[1]) {
            return Err("immutable source hash mismatch".into());
        }
    }

    if seen.len() != SOURCE_FILES {
        return Err("defining source file count changed".into());
    }

    Ok(())
}

fn negative_controls(actual: &str, expected: &str) -> Result<()> {
    let lines: Vec<_> = actual.lines().collect();
    let missing = format!("{}\n", lines[..lines.len() - 1].join("\n"));
    let duplicate = format!("{actual}{}\n", lines[1]);
    let mut hash_drift = actual.to_owned();
    let hash_offset = actual.find('\n').ok_or("missing header")? + 1;
    let hash_offset = hash_offset
        + actual[hash_offset..]
            .find('\t')
            .ok_or("missing hash column")?
        + 1;
    let replacement = if &actual[hash_offset..hash_offset + 1] == "0" {
        "1"
    } else {
        "0"
    };

    hash_drift.replace_range(hash_offset..hash_offset + 1, replacement);

    for corrupted in [missing, duplicate, hash_drift] {
        if validate(&corrupted, expected).is_ok() {
            return Err("negative control was accepted".into());
        }
    }

    Ok(())
}
