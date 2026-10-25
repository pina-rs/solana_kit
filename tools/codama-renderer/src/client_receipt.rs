//! Complete scoped accounting for the public System client slice.
use crate::schema::Error;
use std::{fs, path::Path};

type Disposition = (&'static str, &'static str, &'static str);

pub(crate) fn run() -> Result<(), Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let source = fs::read_to_string(root.join("docs/parity/capabilities.tsv"))?;
    let mut output = "id\tunit52_status\trust_api\tboundary\tevidence\n".to_owned();

    for row in source.lines().skip(1) {
        let columns: Vec<_> = row.split('\t').collect();

        if columns.get(1) != Some(&"solana_kit_system") {
            continue;
        }

        let id = columns.first().ok_or("missing id")?;
        let symbol = columns.get(3).ok_or("missing symbol")?;
        let (status, api, boundary) = disposition(symbol);
        output.push_str(&format!(
            "{id}\t{status}\t{api}\t{boundary}\tdocs/verification/system-client.md\n"
        ));
    }

    fs::write(root.join("docs/parity/system-client.tsv"), output)?;
    Ok(())
}

/// Mirrors actual API families while retaining every deliberate source discrepancy.
fn disposition(symbol: &str) -> Disposition {
    match symbol {
        symbol if symbol.contains("InstructionData") => data(symbol),
        symbol if symbol.starts_with("get") && symbol.ends_with("Instruction") => (
            "implemented",
            "solana_kit_system::*::instruction + *Accounts",
            "canonical fixed System identity/sysvars; exact fields; explicit payer/base; official constructor parity",
        ),
        symbol => other(symbol),
    }
}

/// Source parser wrappers collapse into the owned data enum; errors retain numeric identity.
fn other(symbol: &str) -> Disposition {
    match symbol {
        symbol if symbol.starts_with("parse") || symbol.starts_with("Parsed") => (
            "implemented",
            "solana_kit_system::*::parse + parse_instruction + SystemInstructionData",
            "strict canonical positional contract; richer validation than reference data-only parsing",
        ),

        symbol if symbol.starts_with("systemError") => errors(),
        "getSystemErrorMessage" | "isSystemError" => errors(),
        symbol => nonce(symbol),
    }
}

fn data(symbol: &str) -> Disposition {
    if symbol.ends_with(".new") || symbol.ends_with(".discriminator") {
        return (
            "partial",
            "solana_kit_system::*Codec",
            "fixed tags and explicit fields replace mutable discriminators and implicit defaults",
        );
    }

    (
        "implemented",
        "solana_kit_system::*Codec",
        "all14 bounded data layouts; unchanged independent wire oracle",
    )
}

fn errors() -> Disposition {
    (
        "implemented",
        "solana_kit_system::SystemError::{from_code,code,message}",
        "all9 known codes; unknown u32 preserved",
    )
}

/// Separates account snapshot evidence from scalar factory and source-shape gaps.
fn nonce(symbol: &str) -> Disposition {
    match symbol {
        symbol if symbol.starts_with("getNonceState") || symbol.starts_with("getNonceVersion") => {
            missing()
        }
        "Nonce.hashCode" | "SystemAccount" | "SystemAccount.nonce" => missing(),
        "nonceSize" | "decodeNonce" => account(),

        symbol if symbol.starts_with("getNonce") => account(),
        symbol if symbol.starts_with("Nonce") => (
            "partial",
            "solana_kit_system::{Nonce,NonceVersion,NonceState,NonceData}",
            "owned initialized payload; uninitialized storage has no authority/hash; legacy not usable for durable signing",
        ),
        _ => (
            "partial",
            "solana_kit_system::{identify_instruction,SystemInstructionData}",
            "known tags0..13 identified; dedicated standalone instruction-kind enum remains absent",
        ),
    }
}

fn missing() -> Disposition {
    (
        "pending",
        "-",
        "standalone scalar enum codecs, account-kind enum and Hash trait still absent",
    )
}

fn account() -> Disposition {
    (
        "implemented",
        "solana_kit_system::{NONCE_SIZE,decode_nonce,NonceCodec}",
        "exact80-byte allocated snapshots; strict owner/space/trailing checks; native official state oracle",
    )
}
