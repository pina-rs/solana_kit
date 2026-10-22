//! Retains every canonical Compute Budget declaration without claiming renderer parity.
use crate::schema::Error;
use std::{fs, path::Path};

pub(super) fn run() -> Result<(), Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let input = fs::read_to_string(root.join("docs/parity/capabilities.tsv"))?;
    let mut output = "id\tunit78_status\trust_api\tboundary\tevidence\n".to_owned();
    let mut count = 0;

    for row in input.lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();

        if fields.get(1) != Some(&"solana_kit_compute_budget") {
            continue;
        }

        let symbol = *fields.get(3).ok_or("missing symbol")?;
        let (status, api, boundary) = disposition(symbol);
        output.push_str(&format!(
            "{}\t{status}\t{api}\t{boundary}\tdocs/verification/compute-budget.md\n",
            fields[0]
        ));
        count += 1;
    }

    if count != 84 {
        return Err("expected exact84 Compute Budget IDs".into());
    }

    fs::write(root.join("docs/parity/compute-budget.tsv"), output)?;
    Ok(())
}

fn disposition(symbol: &str) -> (&'static str, &'static str, &'static str) {
    if symbol.ends_with(".discriminator") || symbol.ends_with("InstructionData.new") {
        return (
            "partial",
            "solana_kit_compute_budget::*Codec",
            "fixed validated tags replace mutable source tags and constructor defaults",
        );
    }

    if symbol.starts_with("ComputeBudgetInstruction")
        || symbol == "ParsedComputeBudgetInstruction.instructionType"
    {
        return (
            "partial",
            "ComputeBudgetInstructionData + identify_instruction",
            "owned payload sum type and u8 tag; standalone source instruction-kind enum remains absent",
        );
    }

    if symbol.contains("TransactionMessage")
        || symbol.contains("ProvisorySet")
        || symbol.starts_with("updateOrAppend")
        || symbol.starts_with("findSet")
    {
        return (
            "partial",
            "solana_kit_compute_budget instruction-list helpers",
            "same executed ordering and numeric semantics; operates before compilation on instruction slices rather than a mutable message API",
        );
    }

    (
        "implemented",
        "solana_kit_compute_budget::*",
        "five bounded data codecs/builders/parsers; exact constants and typed callback/detail equivalents; native Dart/official oracles",
    )
}
