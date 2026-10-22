//! Exact Token declaration inventory with bounded behavior and residual contracts.
use crate::schema::Error;
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};
const TOKEN_IDS: usize = 415;
const SOURCE_HASH_COLUMN: usize = 12;

/// Preserve the exact canonical inventory and verify every defining source file.
pub(crate) fn run() -> Result<(), Error> {
    let arguments: Vec<_> = std::env::args().collect();
    let source = Path::new(
        arguments
            .get(2)
            .ok_or("original Dart source root required")?,
    );
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ledger = fs::read_to_string(repository.join("docs/parity/capabilities.tsv"))?;
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut output="id\tpackage\tpath\tsymbol\tkind\tline\towner\tphase\tstatus\tevidence\tnotes\tunit\tsha256\tprovenance\tunit150_status\trust_api\tbehavior\tresidual\n".to_owned();

    for row in ledger.lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();

        if fields.get(1) != Some(&"solana_kit_token") {
            continue;
        }

        if !ids.insert(fields[0]) {
            return Err("duplicate canonical Token ID".into());
        }

        if paths.insert(fields[2]) {
            verify_source(source, &fields)?;
        }

        let (status, api, behavior, residual) = disposition(fields[2], fields[3]);
        output.push_str(&format!("{row}\t{status}\t{api}\t{behavior}\t{residual}\n"));
    }

    if ids.len() != TOKEN_IDS {
        return Err("Token canonical ID denominator changed".into());
    }

    fs::write(repository.join("docs/parity/token.tsv"), output)?;
    println!(
        "{} exact Token IDs; {} defining-file hashes verified; no renderer IDs promoted",
        ids.len(),
        paths.len()
    );
    Ok(())
}
fn verify_source(source: &Path, fields: &[&str]) -> Result<(), Error> {
    let actual = format!("{:x}", Sha256::digest(fs::read(source.join(fields[2]))?));

    if actual != fields[SOURCE_HASH_COLUMN] {
        return Err(format!("defining-file digest mismatch: {}", fields[2]).into());
    }

    Ok(())
}
/// Separate generated wire evidence from unimplemented handwritten contracts.
fn disposition(
    path: &str,
    symbol: &str,
) -> (&'static str, &'static str, &'static str, &'static str) {
    if path.contains("/generated/instructions/") {
        return instruction(symbol);
    }

    if path.contains("/generated/accounts/") {
        return (
            "partial",
            "Mint/Token/Multisig + codecs",
            "93-case original Dart matrix; all byte/option/array fields; exact82/165/355 strict frames",
            "fetch wrappers, Display/hashCode, mutable wrappers and validator execution remain pending",
        );
    }

    if path.contains("/generated/types/") {
        return (
            "partial",
            "AccountState/AuthorityType + codecs",
            "source variant order and strict known u8 tags exercised in account/instruction oracles",
            "source wrapper traits and unrestricted decoder behavior remain distinct",
        );
    }

    if path.contains("/generated/errors/") {
        return (
            "partial",
            "TokenError::{from_code,code,message}",
            "all20 known and fullwidth unknown codes exercised; exact source message retained",
            "source error wrapper/detail recognition contracts remain pending",
        );
    }

    if path.contains("/generated/programs/") {
        return (
            "partial",
            "PROGRAM + identify_instruction + identify_account",
            "trusted canonical identity and custom identities; explicit tags and unique account sizes",
            "async wrappers, complete source instruction-kind wrappers and fetch helpers pending",
        );
    }

    (
        "pending",
        "-",
        "no implementation claim",
        "handwritten helper/config/PDA/fetch/wrapper behavior requires separate public tests",
    )
}
/// Keep source-specific decoder and builder differences visible per declaration.
fn instruction(symbol: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    if symbol.contains("UiAmountToAmount") {
        return (
            "partial",
            "UiAmountToAmount + UiAmountToAmountCodec",
            "all three original Dart encoding cases match; Rust retains complete raw UTF8",
            "original Dart decoder removes an initial BOM; Rust preserves it; mutable wrappers and data-only parser semantics remain distinct",
        );
    }
    if symbol == "getBatchInstruction" {
        return (
            "partial",
            "Batch::{instruction,instruction_with_program}",
            "three original Dart nested-record/u8-length byte and empty-meta fixtures match",
            "IDL and Dart declare empty accounts; arbitrary batch account assembly and validator execution remain pending",
        );
    }
    if symbol.starts_with("get") && symbol.ends_with("Instruction") {
        return (
            "partial",
            "*::instruction[_with_program]",
            "all28 original Dart constructors ×3 values compare every byte/meta/address/program",
            "IDL signer-either and remaining lists supported explicitly; Dart forces signer and omits lists; defaults caller-owned",
        );
    }

    if symbol.starts_with("parse") {
        return (
            "partial",
            "*::parse[_with_program]",
            "strict identity/roles/frame/truncation/budget/remaining/duplicate checks",
            "Dart parses data only and accepts mutable discriminator/trailing behavior",
        );
    }

    if symbol.ends_with("discriminator") {
        return (
            "partial",
            "fixed validated codec discriminator",
            "all28 tags tested plus unknown domain rejection",
            "mutable source discriminator field intentionally absent",
        );
    }

    (
        "partial",
        "concrete instruction records + codecs",
        "independent actual Dart literals for zero/nonzero/fullwidth and absent/present values",
        "mutable constructor defaults, wrapper equality/Display/hashCode and validator behavior remain pending",
    )
}
