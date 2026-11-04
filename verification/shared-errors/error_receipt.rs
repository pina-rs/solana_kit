//! Emits the reviewed 73-row error-contract receipt from the canonical ledgers.
//!
//! Each row keeps the complete canonical ID, the public Rust operation, the
//! executable case names, the reviewed status and the concrete residual. Run
//! from the repository root:
//!
//! ```sh
//! rustc --edition=2024 -D warnings verification/shared-errors/error_receipt.rs \
//!   -o /tmp/solana-kit-error-receipt
//! /tmp/solana-kit-error-receipt
//! ```

use std::{collections::BTreeMap, fs, process::ExitCode};

const CAPABILITIES: &str = "docs/parity/capabilities.tsv";
const MANIFEST: &str = "docs/parity/adjudications.tsv";
const RECEIPT: &str = "docs/parity/error-contracts.tsv";
const HEADER: &str = "id\trust_operation\tcases\tstatus\tresidual";
const ROWS: usize = 73;
const PACKAGE: &str = "solana_kit_errors";

/// Rust variant names for the 25 pinned domain constants.
const DOMAIN_VARIANTS: [(&str, &str); 25] = [
    ("general", "General"),
    ("jsonRpc", "JsonRpc"),
    ("addresses", "Addresses"),
    ("accounts", "Accounts"),
    ("subtleCrypto", "SubtleCrypto"),
    ("crypto", "Crypto"),
    ("keys", "Keys"),
    ("instruction", "Instruction"),
    ("instructionError", "InstructionError"),
    ("signer", "Signer"),
    ("offchainMessage", "OffchainMessage"),
    ("transaction", "Transaction"),
    ("transactionError", "TransactionError"),
    ("transactionIntrospection", "TransactionIntrospection"),
    ("instructionPlans", "InstructionPlans"),
    ("codecs", "Codecs"),
    ("rpc", "Rpc"),
    ("rpcSubscriptions", "RpcSubscriptions"),
    ("subscribable", "Subscribable"),
    ("programClients", "ProgramClients"),
    ("mobileWalletAdapter", "MobileWalletAdapter"),
    ("helius", "Helius"),
    ("wallet", "Wallet"),
    ("invariantViolation", "InvariantViolation"),
    ("unknown", "Unknown"),
];

/// One reviewed manifest row.
struct Claim {
    status: String,
    cases: String,
    decision: String,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error receipt failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let capabilities = fs::read_to_string(CAPABILITIES).map_err(|error| error.to_string())?;
    let manifest = fs::read_to_string(MANIFEST).map_err(|error| error.to_string())?;
    let claims = claims(&manifest)?;
    let targets = targets(&capabilities)?;
    let mut output = String::from(HEADER);
    output.push('\n');

    for (id, suffix, kind) in &targets {
        let claim = claims
            .get(id)
            .ok_or_else(|| format!("missing adjudication: {id}"))?;
        let operation = operation(suffix, kind)?;
        output.push_str(&format!(
            "{id}\t{operation}\t{}\t{}\t{}\n",
            claim.cases, claim.status, claim.decision
        ));
    }

    fs::write(RECEIPT, output).map_err(|error| error.to_string())?;
    println!("{ROWS} error-contract rows written with full IDs and Rust operations");
    Ok(())
}

/// Reads every reviewed manifest claim by canonical ID.
fn claims(manifest: &str) -> Result<BTreeMap<String, Claim>, String> {
    let mut claims = BTreeMap::new();

    for line in manifest.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();
        let id = fields[0].to_owned();
        let claim = Claim {
            status: fields[2].to_owned(),
            cases: fields[4].to_owned(),
            decision: fields[6].to_owned(),
        };

        if claims.insert(id.clone(), claim).is_some() {
            return Err(format!("duplicate claim: {id}"));
        }
    }

    Ok(claims)
}

/// Selects the 73 non-catalog declarations in canonical order.
fn targets(capabilities: &str) -> Result<Vec<(String, String, String)>, String> {
    let mut rows = Vec::new();

    for line in capabilities.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();

        if fields[1] != PACKAGE || is_catalog_constant(fields[2], fields[4]) {
            continue;
        }

        let id = fields[0].to_owned();
        let suffix = id
            .split_once('#')
            .map(|(_, suffix)| suffix.to_owned())
            .ok_or_else(|| format!("non-canonical id: {id}"))?;
        rows.push((id, suffix, fields[4].to_owned()));
    }

    if rows.len() != ROWS {
        return Err(format!(
            "expected {ROWS} declarations, found {}",
            rows.len()
        ));
    }

    Ok(rows)
}

/// Excludes the 365 constants already covered by the catalog receipt.
fn is_catalog_constant(path: &str, kind: &str) -> bool {
    path.ends_with("codes.dart") && kind == "enum_constant"
}

/// Returns the public Rust operation for one reviewed declaration.
fn operation(suffix: &str, kind: &str) -> Result<&'static str, String> {
    if let Some(name) = suffix.strip_prefix("SolanaErrorDomain.") {
        return domain_operation(name);
    }

    if suffix.starts_with("SolanaErrorContextKeys.") {
        return Ok("errors::Context::{insert,get};ContextValue");
    }

    let operation = match suffix {
        "SolanaErrorCode" => "errors::ErrorCode;CodeDefinition;definitions",
        "SolanaErrorCode.fromValue" => "ErrorCode::from_value;definition",
        "SolanaErrorCode.new" => "ErrorCode associated const",
        "SolanaErrorCode.value" => "ErrorCode::value",
        "SolanaErrorDomain" => "errors::ErrorDomain",
        "SolanaErrorCodeDomainExtension" => "ErrorCode::domain;isSolanaErrorDomain",
        "SolanaErrorCodeDomainExtension.solanaErrorDomain" => "ErrorCode::domain",
        "SolanaErrorCodeDomainExtension.isSolanaErrorDomain" => "ErrorCode::domain equality",
        "SolanaErrorDomainExtension" => "Diagnostic::code().domain()",
        "SolanaErrorDomainExtension.domain" => "Diagnostic::code().domain()",
        "SolanaErrorDomainExtension.isInDomain" => "Diagnostic::code().domain() equality",
        "isSolanaErrorInDomain" => "Diagnostic::code().domain();ErrorFacts",
        "getSolanaErrorDomain" => "ErrorCode::domain",
        "isSolanaErrorCodeInDomain" => "ErrorCode::domain equality",
        "SolanaError" => "errors::Diagnostic;std::error::Error",
        "SolanaError.code" => "Diagnostic::code",
        "SolanaError.context" => "Diagnostic::context",
        "SolanaError.new" => "Diagnostic::new",
        "SolanaError.toString" => "Diagnostic Display;ErrorCode::message",
        "isSolanaError" => "ErrorFacts::error_code",
        "encodeContextObject" => "diagnostics::encode_context",
        "decodeEncodedContext" => "diagnostics::decode_context",
        "SolanaErrorContextKeys" => "errors::Context;ContextValue",
        "createSolanaError" => "Diagnostic::from_context",
        "createSolanaErrorContext" => "Diagnostic::wrap;Context::normalized",
        "wrapSolanaError" => "Diagnostic::wrap;Error::source",
        "getSolanaErrorFromInstructionError" => "diagnostics::instruction_error",
        "getSolanaErrorFromJsonRpcError" => "diagnostics::json_rpc_error;RemoteError",
        "getErrorMessage" => "ErrorCode::message;Context",
        "solanaErrorMessages" => "CodeDefinition::template;definitions",
        "RpcEnumErrorConfig" => "diagnostics::RpcEnumErrorConfig",
        "RpcEnumErrorConfig.errorCodeBaseOffset" => "RpcEnumErrorConfig::base",
        "RpcEnumErrorConfig.orderedErrorNames" => "RpcEnumErrorConfig::names",
        "RpcEnumErrorConfig.getErrorContext" => "rpc_enum_error callback",
        "RpcEnumErrorConfig.new" => "RpcEnumErrorConfig struct literal",
        "getSolanaErrorFromRpcError" => "diagnostics::rpc_enum_error;RpcEnumError",
        "unwrapSimulationError" => "Diagnostic::simulation_cause",
        "getSolanaErrorFromTransactionError" => "diagnostics::transaction_error",
        other => return Err(format!("unreviewed {kind}: {other}")),
    };

    Ok(operation)
}

/// Builds the domain constant's exact variant operation.
fn domain_operation(name: &str) -> Result<&'static str, String> {
    if !DOMAIN_VARIANTS.iter().any(|(domain, _)| *domain == name) {
        return Err(format!("unknown domain: {name}"));
    }

    Ok(match name {
        "general" => "ErrorCode::domain -> ErrorDomain::General",
        "jsonRpc" => "ErrorCode::domain -> ErrorDomain::JsonRpc",
        "addresses" => "ErrorCode::domain -> ErrorDomain::Addresses",
        "accounts" => "ErrorCode::domain -> ErrorDomain::Accounts",
        "subtleCrypto" => "ErrorCode::domain -> ErrorDomain::SubtleCrypto",
        "crypto" => "ErrorCode::domain -> ErrorDomain::Crypto",
        "keys" => "ErrorCode::domain -> ErrorDomain::Keys",
        "instruction" => "ErrorCode::domain -> ErrorDomain::Instruction",
        "instructionError" => "ErrorCode::domain -> ErrorDomain::InstructionError",
        "signer" => "ErrorCode::domain -> ErrorDomain::Signer",
        "offchainMessage" => "ErrorCode::domain -> ErrorDomain::OffchainMessage",
        "transaction" => "ErrorCode::domain -> ErrorDomain::Transaction",
        "transactionError" => "ErrorCode::domain -> ErrorDomain::TransactionError",
        "transactionIntrospection" => "ErrorCode::domain -> ErrorDomain::TransactionIntrospection",
        "instructionPlans" => "ErrorCode::domain -> ErrorDomain::InstructionPlans",
        "codecs" => "ErrorCode::domain -> ErrorDomain::Codecs",
        "rpc" => "ErrorCode::domain -> ErrorDomain::Rpc",
        "rpcSubscriptions" => "ErrorCode::domain -> ErrorDomain::RpcSubscriptions",
        "subscribable" => "ErrorCode::domain -> ErrorDomain::Subscribable",
        "programClients" => "ErrorCode::domain -> ErrorDomain::ProgramClients",
        "mobileWalletAdapter" => "ErrorCode::domain -> ErrorDomain::MobileWalletAdapter",
        "helius" => "ErrorCode::domain -> ErrorDomain::Helius",
        "wallet" => "ErrorCode::domain -> ErrorDomain::Wallet",
        "invariantViolation" => "ErrorCode::domain -> ErrorDomain::InvariantViolation",
        _ => "ErrorCode::domain -> ErrorDomain::Unknown",
    })
}
