# Errors

Solana Kit uses plain Rust error enums at each boundary, and each enum keeps the original facts. A codec error says which constant failed and at which offset. An RPC error keeps the node's code and message, and its `data` field as raw JSON. A program error keeps the custom code without guessing which instruction raised it. A shared layer, `solana_kit_core::errors`, maps these onto the stable numeric codes from the reference Solana Kit, for logging and cross-language diagnostics.

## Error types by layer

| Crate and module | Type | Covers |
| --- | --- | --- |
| `solana_kit_core::codecs` | `Error` | Truncation, budgets, trailing bytes, invalid tags, booleans, constants and UTF-8. See [Codecs](codecs.md) |
| `solana_kit_core` | `ParseError` | Address, hash and signature text or byte length |
| `solana_kit_core::accounts` | `AccountError` | `Missing`, `OwnerMismatch`, `DataSizeMismatch`, `Decode`. See [Accounts and sysvars](accounts.md) |
| `solana_kit_core::transaction` | `Error` | Compile, wire and signature failures. See [Messages and transactions](transactions.md) |
| `solana_kit_core::offchain` | `Error` | Off-chain message and envelope validation |
| `solana_kit_core::crypto` | `PdaError`, `KeypairImportError`, `VerificationError` | Derivation, key import and verification |
| `solana_kit_signers` | `Error` | Rejection, cancellation, conflicting or foreign signers, changed snapshots. See [Signers](signers.md) |
| `solana_kit_client` | `Error`, `TransportError`, `RemoteError` | Transport, HTTP, JSON, correlation and remote JSON-RPC failures. See [RPC](rpc.md) |
| `solana_kit_client::subscriptions` | `Failure`, `SessionError<E>` | Subscription protocol and socket failures. See [Subscriptions](subscriptions.md) |
| `solana_kit_client::confirmation` | `Failure`, `WaitError` | Confirmation outcomes, kept separate from cleanup failures |

All of these implement `core::error::Error`. Wrapping variants such as `AccountError::Decode` and `client::Error::Transport` expose the original error through `source()`.

## Program errors

When a transaction fails inside a program, the client decodes the node's execution error into `methods::submission::TransactionFailure`. Known variants are validated. Unknown future variants keep their exact raw JSON, and `Debug` redacts that JSON.

`solana_kit_core::program_error` works with program failures directly:

- `ProgramError` is `Custom(u32)`, `Known(KnownProgramError)` or `Unknown(String)`.
- `InstructionFailure { index, error }` ties an error to the failing instruction position. `is_program_error(&programs, program, code)` matches only a custom error at a valid index whose program is the one you name.
- `parse_failure_log(line)` parses exactly `Program <address> failed: <description>`. It keeps the original line, recognizes known descriptions, keeps unknown ones verbatim, and rejects bad addresses or overflowing custom codes.

```rust
use solana_kit_core::{
    Address,
    program_error::{InstructionFailure, ProgramError, parse_failure_log},
};

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let line = "Program 11111111111111111111111111111111 failed: custom program error: 0x2a";
    let parsed = parse_failure_log(line)?.expect("a failure line");
    assert_eq!(parsed.error, ProgramError::Custom(42));

    let system = Address::from_bytes([0; 32]);
    let failure = InstructionFailure { index: 0, error: ProgramError::Custom(42) };
    assert!(failure.is_program_error(&[system], system, Some(42)));
    Ok(())
}
```

Log parsing never infers an instruction index or an outer program from the order of log lines. Take the index from the node's structured error.

Each program client maps custom codes to its own enum. For example, `solana_kit_system::SystemError::from_code(0)` is `SystemError::AccountAlreadyInUse`. Unknown numbers stay as `SystemError::Unknown(code)`. See [Program clients](../programs/index.md).

## Shared error codes

`solana_kit_core::errors` holds the shared code catalog:

- `ErrorCode` is a stable numeric code. Named constants such as `ErrorCode::CODECS_INVALID_BOOLEAN` cover the 365 codes defined by the pinned source. `ErrorCode::from_value(i64)` keeps any other integer without truncating it.
- `code.domain()` returns an `ErrorDomain` such as `Codecs`, `Accounts` or `JsonRpc`. `code.definition()` returns the source name and message template.
- `code.message(&context)` renders the template with public context, up to 4096 bytes.
- The `ErrorFacts` trait is implemented by error types that can report a code. `error_code()` returns `Some` only when the code is an exact equivalent. `error_context()` returns public `Context` facts.
- `Diagnostic` is an owned code plus context that keeps its original cause through `with_source`.

```rust
use solana_kit_core::{
    codecs::{Decoder, Limits, Number},
    errors::{ErrorCode, ErrorFacts},
};

fn example() {
    let error = Number::<u32>::LE.decode(&[1], Limits::new(8, 0)).unwrap_err();
    assert_eq!(error.error_code(), Some(ErrorCode::CODECS_INVALID_BYTE_LENGTH));
}
```

`ErrorFacts` is implemented for the core `codecs::Error`, `PairError`, `ParseError`, `AccountError` and `PdaError`, and for the client `Error` and `RemoteError`. A remote error reports the node's code as its `ErrorCode`, and an HTTP status error reports `RPC_TRANSPORT_HTTP_ERROR`.

`solana_kit_client::diagnostics` turns strict RPC values into a `Diagnostic`. `json_rpc_error(json)` handles a JSON-RPC error object, and `transaction_error(json)` handles an execution error. `TransactionFailure::into_diagnostic` converts a failure you already have.

`Context` accepts public facts: scalars (null, integers, booleans and text) and structured `List` and `Object` values. Keys that look secret, such as `privateKey`, are rejected, and so is text over the size limit. `Debug` and `Display` output redact payloads. Message rendering is always an explicit call.

## Status

The [core contracts receipt](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/core-contracts.md) covers the shared code catalog and diagnostic conversions. They run on native Rust, Chrome, both WASI previews and the LeptosKit interpreter. The receipt does not change any capability's parity status. See [Parity and status](../reference/parity.md).
