//! Exact canonical System IDs, without mutating the ecosystem capability ledger.

use crate::schema::Error;
use std::{fs, path::Path};

pub(crate) fn run() -> Result<(), Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ledger = fs::read_to_string(root.join("docs/parity/capabilities.tsv"))?;
    let mut output = "id\tunit33_status\trust_api\tboundary\n".to_owned();

    for row in ledger.lines().skip(1) {
        let columns: Vec<_> = row.split('\t').collect();

        if columns.get(1) != Some(&"solana_kit_system") {
            continue;
        }

        let id = columns.first().ok_or("missing canonical id")?;
        let symbol = columns.get(3).ok_or("missing canonical symbol")?;
        let instruction = columns
            .get(2)
            .ok_or("missing canonical path")?
            .contains("/instructions/");
        let (status, api, boundary) = disposition(symbol, instruction);
        output.push_str(&format!("{id}\t{status}\t{api}\t{boundary}\n"));
    }

    fs::write(root.join("docs/parity/system-data.tsv"), output)?;
    Ok(())
}

fn disposition(symbol: &str, instruction: bool) -> (&'static str, &'static str, &'static str) {
    if instruction && symbol.contains("InstructionData") {
        if symbol.ends_with(".new") || symbol.ends_with(".discriminator") {
            return (
                "partial-data-slice",
                "solana_kit_system::*Codec",
                "validated fixed tags and explicit fields replace mutable tags/default constructors",
            );
        }

        return (
            "implemented-data-slice",
            "solana_kit_system::*Codec",
            "structural data only; fixed validated tag; explicit fields; no builder/default parity claim",
        );
    }

    (
        "pending",
        "-",
        "accounts/builders/parsers/errors/identification remain outside unit33",
    )
}
