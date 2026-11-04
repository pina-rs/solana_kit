//! Appends the reviewed 73-declaration error-contract batch to the manifest.
//!
//! Reads the canonical achieved ledger, requires every unadjudicated
//! `solana_kit_errors` row, and appends one reviewed claim keyed by its complete
//! canonical ID and pinned source hash. Existing rows are preserved byte for
//! byte. Run from the repository root:
//!
//! ```sh
//! rustc --edition=2024 -D warnings verification/shared-errors/adjudicate.rs \
//!   -o /tmp/solana-kit-adjudicate
//! /tmp/solana-kit-adjudicate
//! ```

use std::{collections::BTreeSet, fs, process::ExitCode};

const CAPABILITIES: &str = "docs/parity/capabilities.tsv";
const MANIFEST: &str = "docs/parity/adjudications.tsv";
const EVIDENCE: &str = "verification/conformance/evidence/native-success.txt";
const HEADER: &str = "id\tsha256\tstatus\tevidence\tcases\tnotes\tdecision";
const EXISTING: usize = 389;
const ADDED: usize = 73;
const MAX_NOTES: usize = 200;
const MAX_DECISION: usize = 320;

const PACKAGE_COLUMN: usize = 1;
const STATUS_COLUMN: usize = 8;
const HASH_COLUMN: usize = 12;
const PACKAGE: &str = "solana_kit_errors";
const PENDING: &str = "pending";

const ENUM: &str = "error-code-enum-boundaries";
const DOMAIN: &str = "error-domain-classification-25";
const DIAGNOSTIC: &str = "error-diagnostic-source-contract";
const CONTEXTS: &str = "error-context-wire-literals;error-context-wire-mounted-values";
const CONTEXT: &str = "error-context-wire-mounted-values";
const KEYS: &str = "error-context-key-conventions";
const EXECUTION: &str = "error-execution-source-oracle";
const JSON_RPC: &str = "error-json-rpc-projection";
const MESSAGE: &str = "error-message-template-interpolation";
const RPC_ENUM: &str = "error-rpc-enum-offset-callback";
const SIMULATION: &str = "error-simulation-unwrap";

struct Claim {
    status: &'static str,
    cases: &'static str,
    notes: &'static str,
    decision: &'static str,
}

const fn claim(
    status: &'static str,
    cases: &'static str,
    notes: &'static str,
    decision: &'static str,
) -> Claim {
    Claim {
        status,
        cases,
        notes,
        decision,
    }
}

/// Pinned `SolanaErrorDomain` values in source declaration order.
const DOMAINS: [(&str, &str, &str); 25] = [
    ("general", "General", "[1,10]"),
    ("jsonRpc", "JsonRpc", "[-32768,-32000]"),
    ("addresses", "Addresses", "[2800000,2800999]"),
    ("accounts", "Accounts", "[3230000,3230999] and 32300001"),
    ("subtleCrypto", "SubtleCrypto", "[3610000,3610999]"),
    ("crypto", "Crypto", "[3611000,3611050]"),
    ("keys", "Keys", "[3704000,3704999]"),
    ("instruction", "Instruction", "[4128000,4128999]"),
    ("instructionError", "InstructionError", "[4615000,4615999]"),
    ("signer", "Signer", "[5508000,5508999]"),
    ("offchainMessage", "OffchainMessage", "[5607000,5607999]"),
    ("transaction", "Transaction", "[5663000,5663999]"),
    ("transactionError", "TransactionError", "[7050000,7050999]"),
    (
        "transactionIntrospection",
        "TransactionIntrospection",
        "[5664000,5664999]",
    ),
    ("instructionPlans", "InstructionPlans", "[7618000,7618999]"),
    ("codecs", "Codecs", "[8078000,8078999]"),
    ("rpc", "Rpc", "[8100000,8100999]"),
    ("rpcSubscriptions", "RpcSubscriptions", "[8190000,8190999]"),
    ("subscribable", "Subscribable", "[8195000,8195999]"),
    ("programClients", "ProgramClients", "[8500000,8500999]"),
    (
        "mobileWalletAdapter",
        "MobileWalletAdapter",
        "[8400000,8400199]",
    ),
    ("helius", "Helius", "[8600000,8600099]"),
    ("wallet", "Wallet", "[8900000,8900999]"),
    (
        "invariantViolation",
        "InvariantViolation",
        "[9900000,9900999]",
    ),
    ("unknown", "Unknown", ""),
];

/// The ten pinned `SolanaErrorContextKeys` literals.
const CONTEXT_KEYS: [&str; 10] = [
    "address",
    "cause",
    "causeCode",
    "causeContext",
    "causeType",
    "methodName",
    "operation",
    "path",
    "statusCode",
    "url",
];

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("adjudication failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let capabilities = fs::read_to_string(CAPABILITIES).map_err(|error| error.to_string())?;
    let pending = pending_rows(&capabilities)?;
    let manifest = fs::read_to_string(MANIFEST).map_err(|error| error.to_string())?;
    let (header, existing) = split_manifest(&manifest)?;
    let (rendered, complete, partial) = render(&pending)?;
    write_manifest(header, &existing, &rendered)?;
    println!(
        "{ADDED} reviewed rows appended after {EXISTING}: {complete} complete, {partial} partial"
    );

    Ok(())
}

/// Renders every reviewed row and counts the two allowed statuses.
fn render(pending: &[(String, String, String)]) -> Result<(Vec<String>, usize, usize), String> {
    let mut rendered = Vec::new();
    let mut complete = 0;
    let mut partial = 0;

    for (id, sha256, suffix) in pending {
        let claim = reviewed(suffix)?;
        check_status(id, claim.status)?;
        check_text(id, &claim)?;

        if claim.status == "complete" {
            complete += 1;
        } else {
            partial += 1;
        }

        rendered.push(format!(
            "{id}\t{sha256}\t{}\t{EVIDENCE}\t{}\t{}\t{}",
            claim.status, claim.cases, claim.notes, claim.decision
        ));
    }

    Ok((rendered, complete, partial))
}

/// Rejects a status the manifest parser does not accept.
fn check_status(id: &str, status: &str) -> Result<(), String> {
    if matches!(status, "complete" | "partial") {
        return Ok(());
    }

    Err(format!("invalid status for {id}"))
}

/// Enforces the reviewed text bounds and single-line contract.
fn check_text(id: &str, claim: &Claim) -> Result<(), String> {
    if claim.notes.chars().count() > MAX_NOTES {
        return Err(format!("notes exceed {MAX_NOTES} for {id}"));
    }

    if claim.decision.chars().count() > MAX_DECISION {
        return Err(format!("decision exceed {MAX_DECISION} for {id}"));
    }

    if claim.notes.contains('\t') || claim.notes.contains('\n') {
        return Err(format!("control character in {id}"));
    }

    if claim.decision.contains('\t') || claim.decision.contains('\n') {
        return Err(format!("control character in {id}"));
    }

    Ok(())
}

/// Writes the preserved prefix followed by the reviewed batch.
fn write_manifest(header: &str, existing: &[&str], rendered: &[String]) -> Result<(), String> {
    let mut output = String::from(header);
    output.push('\n');
    output.push_str(&existing.join("\n"));
    output.push('\n');
    output.push_str(&rendered.join("\n"));
    output.push('\n');
    fs::write(MANIFEST, &output).map_err(|error| error.to_string())
}

/// Reads every still-pending `solana_kit_errors` declaration in canonical order.
fn pending_rows(capabilities: &str) -> Result<Vec<(String, String, String)>, String> {
    let rows: Vec<_> = capabilities
        .lines()
        .skip(1)
        .filter(|line| is_pending(line))
        .map(pending_row)
        .collect::<Result<_, _>>()?;

    if rows.len() != ADDED {
        return Err(format!("expected {ADDED} pending rows, found {}", rows.len()));
    }

    Ok(rows)
}

/// Selects one package's unadjudicated canonical declaration.
fn is_pending(line: &str) -> bool {
    let fields: Vec<_> = line.split('\t').collect();

    fields.get(PACKAGE_COLUMN) == Some(&PACKAGE)
        && fields.get(STATUS_COLUMN) == Some(&PENDING)
}

/// Splits one pending row into its complete identity and symbol suffix.
fn pending_row(line: &str) -> Result<(String, String, String), String> {
    let fields: Vec<_> = line.split('\t').collect();
    let id = fields[0].to_owned();
    let sha256 = fields[HASH_COLUMN].to_owned();
    let suffix = id
        .split_once('#')
        .map(|(_, suffix)| suffix.to_owned())
        .ok_or_else(|| format!("non-canonical id: {id}"))?;

    Ok((id, sha256, suffix))
}

/// Splits the existing manifest and requires its reviewed prefix.
fn split_manifest(manifest: &str) -> Result<(&str, Vec<&str>), String> {
    let body = manifest
        .strip_suffix('\n')
        .ok_or_else(|| "manifest must end with a newline".to_owned())?;
    let mut lines = body.split('\n');
    let header = lines.next().ok_or_else(|| "manifest is empty".to_owned())?;

    if header != HEADER {
        return Err("manifest header differs".to_owned());
    }

    let existing: Vec<_> = lines.filter(|line| !line.is_empty()).collect();
    let unique: BTreeSet<_> = existing.iter().collect();

    if existing.len() != EXISTING {
        return Err(format!(
            "expected {EXISTING} existing rows, found {}",
            existing.len()
        ));
    }

    if unique.len() != existing.len() {
        return Err("existing manifest has duplicates".to_owned());
    }

    Ok((header, existing))
}

/// Resolves one reviewed claim by canonical symbol suffix.
fn reviewed(suffix: &str) -> Result<Claim, String> {
    reviewed_domain(suffix)
        .or_else(|| reviewed_key(suffix))
        .or_else(|| reviewed_code(suffix))
        .or_else(|| reviewed_enum(suffix))
        .or_else(|| reviewed_diagnostic(suffix))
        .or_else(|| reviewed_context(suffix))
        .or_else(|| reviewed_helper(suffix))
        .or_else(|| reviewed_execution(suffix))
        .or_else(|| reviewed_rpc(suffix))
        .ok_or_else(|| format!("unreviewed declaration: {suffix}"))
}

/// Resolves a pinned `SolanaErrorDomain` constant.
fn reviewed_domain(suffix: &str) -> Option<Claim> {
    let name = suffix.strip_prefix("SolanaErrorDomain.")?;

    for (domain, variant, range) in DOMAINS {
        if name != domain {
            continue;
        }

        return Some(domain_claim(domain, variant, range));
    }

    None
}

/// Builds the reviewed claim for one domain constant.
fn domain_claim(domain: &str, variant: &str, range: &str) -> Claim {
    if domain == "unknown" {
        return claim(
            "complete",
            DOMAIN,
            "Fallback for integers outside the 24 pinned intervals, including named unclassified codes.",
            "Complete for classification identity: unclassified integers and named codes execute on five hosts and return the unknown variant. Rust also classifies unrepresentable integers.",
        );
    }

    claim(
        "complete",
        DOMAIN,
        leak(format!(
            "Pinned {range} and every named code classify to {variant}."
        )),
        leak(format!(
            "Complete for classification identity: range endpoints, gaps and all catalog codes return {variant} on five hosts. Unrepresentable integers classify identically in Rust."
        )),
    )
}

/// Resolves one pinned `SolanaErrorContextKeys` literal.
fn reviewed_key(suffix: &str) -> Option<Claim> {
    let name = suffix.strip_prefix("SolanaErrorContextKeys.")?;

    if !CONTEXT_KEYS.contains(&name) {
        return None;
    }

    Some(claim(
        "partial",
        KEYS,
        leak(format!(
            "The exact \"{name}\" key literal is accepted and preserved as a public context fact."
        )),
        "The string value is exact and usable. No exported SolanaErrorContextKeys constant exists, so the named declaration is substituted by a literal.",
    ))
}

/// Resolves the `SolanaErrorCode` declaration boundary.
fn reviewed_code(suffix: &str) -> Option<Claim> {
    Some(match suffix {
        "SolanaErrorCode" => claim(
            "partial",
            ENUM,
            "All 365 members, exact integers, declaration order and public constants proven.",
            "Set and order are exact. Rust substitutes a struct plus associated constants for the enum, so values(), name and index reflection are absent.",
        ),
        "SolanaErrorCode.fromValue" => claim(
            "partial",
            ENUM,
            "Every pinned integer round-trips through from_value and definition().",
            "Exact for all 365 literals. The Dart enum cannot represent other integers and returns null; Rust preserves any i64 and reports definition() None.",
        ),
        "SolanaErrorCode.new" => claim(
            "complete",
            ENUM,
            "Every public associated constant binds its exact pinned integer in const context.",
            "Complete for construction of all 365 values: const bindings, value() and derived equality execute on five hosts.",
        ),
        "SolanaErrorCode.value" => claim(
            "complete",
            ENUM,
            "value() returns the exact signed integer for every known and preserved unknown code.",
            "Complete: the 64-bit signed value is identical to the pinned int for all 365 constants and is not truncated at the boundary.",
        ),
        _ => return None,
    })
}

/// Resolves the `SolanaErrorDomain` enum, extensions and helpers.
fn reviewed_enum(suffix: &str) -> Option<Claim> {
    Some(match suffix {
        "SolanaErrorDomain" => claim(
            "partial",
            DOMAIN,
            "All 25 unit variants and their pinned declaration order execute.",
            "Variant set, order and classification are exact. Rust has no .values iteration, .name or .index reflection, so the enum class contract is bounded.",
        ),
        "SolanaErrorCodeDomainExtension" => claim(
            "partial",
            DOMAIN,
            "Extension member behavior is provided by inherent ErrorCode methods.",
            "Code domain and membership operations are exact. Dart extension dispatch identity is substituted by inherent methods.",
        ),
        "SolanaErrorCodeDomainExtension.solanaErrorDomain" => claim(
            "complete",
            DOMAIN,
            "ErrorCode::domain returns the pinned variant for every catalog code and probe.",
            "Complete for behavior: the getter composes ErrorCode::domain, which classifies all 365 codes identically on five hosts.",
        ),
        "SolanaErrorCodeDomainExtension.isSolanaErrorDomain" => claim(
            "complete",
            DOMAIN,
            "code.domain() == domain proves true and false membership for pinned codes.",
            "Complete for behavior: membership is delegated to the identical classification and returns the same boolean for every catalog code.",
        ),
        "SolanaErrorDomainExtension" => claim(
            "partial",
            DOMAIN,
            "Extension member behavior is provided by Diagnostic::code().domain().",
            "Diagnostic domain and membership are exact. Dart extension dispatch identity is substituted by inherent composition.",
        ),
        "SolanaErrorDomainExtension.domain" => claim(
            "complete",
            DOMAIN,
            "Diagnostic::code().domain() returns the pinned variant for the diagnostic code.",
            "Complete for behavior: the extension getter is exactly the code classification, executed for diagnostics on five hosts.",
        ),
        "SolanaErrorDomainExtension.isInDomain" => claim(
            "complete",
            DOMAIN,
            "Diagnostic code domain equality proves true and false membership.",
            "Complete for behavior: the extension predicate is exactly classification equality and returns the same boolean.",
        ),
        "isSolanaErrorInDomain" => claim(
            "partial",
            DOMAIN,
            "Diagnostic membership composition matches for codes with typed facts.",
            "Diagnostic composition is exact. The Object? runtime guard becomes typed ErrorFacts, so non-diagnostic objects cannot be queried at all.",
        ),
        "getSolanaErrorDomain" => claim(
            "complete",
            DOMAIN,
            "Every pinned interval endpoint, gap, special value and catalog code classifies identically.",
            "Complete for the source rule: all 24 ranges, the accounts typo, endpoints, gaps and 365 named codes execute on five hosts.",
        ),
        "isSolanaErrorCodeInDomain" => claim(
            "complete",
            DOMAIN,
            "True and false membership execute for pinned codes across distinct domains.",
            "Complete: the helper is exactly classification equality and is proven for matching and non-matching domains on five hosts.",
        ),
        _ => return None,
    })
}

/// Resolves `SolanaError` storage, display and guard declarations.
fn reviewed_diagnostic(suffix: &str) -> Option<Claim> {
    Some(match suffix {
        "SolanaError" => claim(
            "partial",
            DIAGNOSTIC,
            "Diagnostic stores the exact code and context and implements core Error.",
            "Code and context storage are exact. Dart Exception identity and the eager private message field are substituted by std Error and on-demand rendering.",
        ),
        "SolanaError.code" => claim(
            "complete",
            DIAGNOSTIC,
            "code() returns the exact pinned or unknown ErrorCode supplied at construction.",
            "Complete: the field accessor returns the identical signed value for known and unknown codes and is exercised on five hosts.",
        ),
        "SolanaError.context" => claim(
            "complete",
            DIAGNOSTIC,
            "context() returns every inserted fact unchanged and cannot be mutated through the error.",
            "Complete: null, integer, boolean, text, list and object facts survive construction; the map is immutable by borrow.",
        ),
        "SolanaError.new" => claim(
            "complete",
            DIAGNOSTIC,
            "Construction stores an immutable context and accepts an omitted empty context.",
            "Complete for storage: code and context are retained exactly. Message text is produced by the separate bounded renderer.",
        ),
        "SolanaError.toString" => claim(
            "partial",
            DIAGNOSTIC,
            "Display emits the exact SolanaError#<code> identity for known and unknown codes.",
            "The identity prefix is exact. Dart appends \": <message>\"; Rust Display omits context and message and exposes them only through explicit rendering.",
        ),
        "isSolanaError" => claim(
            "partial",
            DIAGNOSTIC,
            "ErrorFacts::error_code returns Some for diagnostics and supports optional code equality.",
            "Typed facts identify diagnostics. Other Rust errors that implement ErrorFacts are not SolanaError values, unlike the source Object? guard.",
        ),
        _ => return None,
    })
}

/// Resolves the context wire pair.
fn reviewed_context(suffix: &str) -> Option<Claim> {
    Some(match suffix {
        "encodeContextObject" => claim(
            "partial",
            CONTEXTS,
            "Eight independent source encodings match, including unicode, booleans and nesting.",
            "Encoding is byte-exact for pinned values. Deterministic key order, ASCII key validation and size limits are stricter than the source Map.",
        ),
        "decodeEncodedContext" => claim(
            "partial",
            CONTEXT,
            "Lossy typing, duplicates, plus signs, bare keys and malformed escapes match source outcomes.",
            "Source-visible decoding matches. Rust keeps 128-bit integers where Dart int is 64-bit, and rejects fractional JSON numbers instead of returning them.",
        ),
        _ => return None,
    })
}

/// Resolves `error_helpers.dart` declarations.
fn reviewed_helper(suffix: &str) -> Option<Claim> {
    Some(match suffix {
        "SolanaErrorContextKeys" => claim(
            "partial",
            KEYS,
            "All ten exact key literals are accepted and preserved by Context::insert.",
            "Literal values are exact. Rust exports no SolanaErrorContextKeys namespace; callers supply the same strings explicitly.",
        ),
        "createSolanaError" => claim(
            "partial",
            KEYS,
            "Null top-level facts are dropped and the exact code is retained.",
            "Normalization matches. Dart renders the message eagerly; Rust returns a Diagnostic whose renderer is separate and bounded.",
        ),
        "createSolanaErrorContext" => claim(
            "partial",
            KEYS,
            "Null dropping, caller cause preservation and nested cause code/context are proven.",
            "Normalization matches. Dart stores cause, causeType, causeCode and causeContext strings; Rust retains a typed source with the same code and context.",
        ),
        "wrapSolanaError" => claim(
            "partial",
            KEYS,
            "A required cause becomes the typed error source and remains downcastable.",
            "Cause retention is exact for errors. Dart accepts any Object? cause; Rust requires an Error, so scalar causes cannot be wrapped.",
        ),
        _ => return None,
    })
}

/// Resolves instruction and transaction mapping declarations.
fn reviewed_execution(suffix: &str) -> Option<Claim> {
    Some(match suffix {
        "getSolanaErrorFromInstructionError" => claim(
            "partial",
            EXECUTION,
            "All 54 ordered names, custom codes, indices and future payloads map exactly.",
            "54 names and the 93-row oracle are exact. Dart accepts any num index and non-numeric custom payloads; Rust requires u8 indices and u32 custom codes.",
        ),
        "getSolanaErrorFromTransactionError" => claim(
            "partial",
            EXECUTION,
            "All 36 ordered names and every one of the 93 independent oracle rows map exactly.",
            "Names, nested instruction indices and contexts are exact. Dart accepts any num index; Rust validates u8 indices and strict one-key shapes.",
        ),
        "getSolanaErrorFromJsonRpcError" => claim(
            "partial",
            JSON_RPC,
            "Preflight defaults, typed cause, server message and data projection execute exactly.",
            "Pinned shapes are exact. Unknown codes are preserved instead of becoming malformedJsonRpcError, and fractional codes or missing codes are rejected rather than coerced.",
        ),
        "unwrapSimulationError" => claim(
            "partial",
            SIMULATION,
            "The three simulation codes expose exactly one typed cause and otherwise return self.",
            "Typed one-level unwrapping is exact. Dart returns any stored context value; Rust only unwraps typed causes and distinguishes boundary origins.",
        ),
        _ => return None,
    })
}

/// Resolves message, RPC enum and template declarations.
fn reviewed_rpc(suffix: &str) -> Option<Claim> {
    Some(match suffix {
        "getErrorMessage" => claim(
            "partial",
            MESSAGE,
            "All 365 empty-context renderings, substitutions, lists, objects and missing variables execute.",
            "Templates and interpolation are exact. Rust caps rendering at 4096 bytes and sorts object keys, where Dart is unbounded and insertion-ordered.",
        ),
        "solanaErrorMessages" => claim(
            "complete",
            MESSAGE,
            "All 365 code-to-template entries match exact bytes and reverse lookup.",
            "Complete for the map content: 365 exact template bytes, definition lookup and empty-context rendering execute on five hosts.",
        ),
        "RpcEnumErrorConfig" => claim(
            "partial",
            RPC_ENUM,
            "The three required fields drive ordered offsets and a typed callback.",
            "Offset behavior is exact. Rust uses a defined first ErrorCode, a borrowed name slice and a typed FnOnce instead of an int, List and nullable Map callback.",
        ),
        "RpcEnumErrorConfig.errorCodeBaseOffset" => claim(
            "partial",
            RPC_ENUM,
            "The base is the first code and indexes 0 and 1 produce the exact next codes.",
            "The numeric base value is identical for valid configs. Rust types it as ErrorCode rather than a bare int, so arbitrary offsets are not accepted.",
        ),
        "RpcEnumErrorConfig.orderedErrorNames" => claim(
            "partial",
            RPC_ENUM,
            "Ordered first-match lookup and the -1 unknown offset execute exactly.",
            "Ordering semantics are exact. Dart stores a mutable List of owned Strings; Rust borrows an immutable slice for the call.",
        ),
        "RpcEnumErrorConfig.getErrorContext" => claim(
            "partial",
            RPC_ENUM,
            "The callback runs once with the exact code, name and payload; explicit null is absent.",
            "Callback facts match. Rust returns a typed Context or ContextError instead of a nullable Map, and rejects limits the source ignores.",
        ),
        "RpcEnumErrorConfig.new" => claim(
            "partial",
            RPC_ENUM,
            "All required fields are enforced at construction.",
            "Field enforcement is exact. Rust substitutes a struct literal for the const named constructor, so const construction is not offered.",
        ),
        "getSolanaErrorFromRpcError" => claim(
            "partial",
            RPC_ENUM,
            "Strings, one-key maps, scalars, unknown names and undefined offsets are proven.",
            "Pinned inputs match. Multi-key maps are rejected instead of taking the first key, and raw JSON is retained where Dart rebuilds a message.",
        ),
        _ => return None,
    })
}

/// Leaks a generated reviewed note, matching the manifest's static contract.
fn leak(text: String) -> &'static str {
    Box::leak(text.into_boxed_str())
}
