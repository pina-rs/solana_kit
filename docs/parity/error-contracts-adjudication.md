# Error-contract adjudication

This note records the third reviewed use of
[`adjudications.tsv`](adjudications.tsv). It covers the 73 `solana_kit_errors`
declarations that remained after the 365 `SolanaErrorCode` constant review. It
does not complete the package, Phase 2, or the SDK.

## Scope and identity

The pinned package declares 438 capabilities: 365 `SolanaErrorCode` constants,
25 `SolanaErrorDomain` constants, the two enum classes, their constructors,
getters and extensions, the dynamic error class, the context wire pair, the
helper conventions, five conversion helpers and the message map. The 365
constants were already adjudicated. This batch reviews exactly the other 73.

| Field               | Value                                                   |
| ------------------- | ------------------------------------------------------- |
| Reference pin       | `0138bb956d2d81ccc0c56edc602401c0df2f2b46`              |
| Defining files      | twelve files under `packages/solana_kit_errors/lib/src` |
| Canonical ID prefix | `dart:solana_kit_errors:lib/src/`                       |
| Rows                | 37 complete, 36 partial                                 |
| Evidence            | `verification/conformance/evidence/native-success.txt`  |
| Suite               | `solana-kit-core-v59`, 476 cases                        |

Every row is keyed by its complete canonical ID and defining source hash. The
per-ID [`error-contracts.tsv`](error-contracts.tsv) lists all 73 IDs with the
public Rust operation, executable cases, status and concrete residual. The 73
new manifest rows follow the 389 existing rows. Regeneration changes only `status`,
`evidence` and `notes` for the 73 target declarations. All 17,948 IDs, source
hashes, provenance values, `source-files.tsv`, `delivery-units.md` and
`docs/planning/parity.tsv` remain byte-identical; only `capabilities.tsv`
changes, and only on the 73 target rows.

## What was executed

The fifteen appended cases follow the preserved 461-case prefix. Each mismatch
returns a `CaseResult` error with a stable field explanation, so a guest never
panics and later cases still run.

| Case                                     | Source declarations proved                                                      |
| ---------------------------------------- | ------------------------------------------------------------------------------- |
| `error-domain-rejects-wrong-range`       | Control: a mutated pinned interval must fail before the exact case              |
| `error-code-enum-rejects-wrong-constant` | Control: a mutated catalog integer must fail before the exact case              |
| `error-execution-rejects-wrong-expected` | Control: a mutated expected code must fail, then the oracle must still pass     |
| `error-domain-classification-25`         | 25 domain values, every interval endpoint, gap, fallback and unknown integer    |
| `error-code-enum-boundaries`             | `fromValue`, `value`, `new`, ordered equality/hash and unknown integers         |
| `error-diagnostic-source-contract`       | `SolanaError` storage, accessors, display identity, typed source and cause      |
| `error-message-rejects-wrong-template`   | Control: one changed template byte must fail before the exact case              |
| `error-message-template-interpolation`   | 365 exact templates, substitutions, lists, objects, nulls, booleans and limits  |
| `error-context-key-conventions`          | Ten key literals, null dropping, caller causes and both constructors            |
| `error-context-wire-mounted-values`      | Three additional source encodings, nested values and strict decode rules        |
| `error-context-wire-literals`            | The five previously committed wire literals                                     |
| `error-rpc-enum-offset-callback`         | Ordered offsets, exactly-once callback, typed payloads and invalid shapes       |
| `error-execution-source-oracle`          | 93 independent execution rows, 54 instruction names and boundary rejections     |
| `error-json-rpc-projection`              | Preflight defaults/cause, server messages, data projection and strict envelopes |
| `error-simulation-unwrap`                | The three simulation codes, typed one-level causes and unchanged non-wrappers   |

The suite retains the existing corruption control: only `sha256-fips-abc` fails
in the 475/476 run.

## Domain classification

`domains.tsv` transcribes every pinned `getSolanaErrorDomain` interval. The
body checks each minimum, maximum, one-below, one-above and midpoint, the
historical accounts typo, the fallback and all 365 catalog codes. It also
cross-checks the fixture against the independent per-code domain column.

| Domain                     | Pinned range                   | Rust variant               |
| -------------------------- | ------------------------------ | -------------------------- |
| `general`                  | `1..10`                        | `General`                  |
| `jsonRpc`                  | `-32768..-32000`               | `JsonRpc`                  |
| `addresses`                | `2800000..2800999`             | `Addresses`                |
| `accounts`                 | `3230000..3230999`, `32300001` | `Accounts`                 |
| `subtleCrypto`             | `3610000..3610999`             | `SubtleCrypto`             |
| `crypto`                   | `3611000..3611050`             | `Crypto`                   |
| `keys`                     | `3704000..3704999`             | `Keys`                     |
| `instruction`              | `4128000..4128999`             | `Instruction`              |
| `instructionError`         | `4615000..4615999`             | `InstructionError`         |
| `signer`                   | `5508000..5508999`             | `Signer`                   |
| `offchainMessage`          | `5607000..5607999`             | `OffchainMessage`          |
| `transaction`              | `5663000..5663999`             | `Transaction`              |
| `transactionError`         | `7050000..7050999`             | `TransactionError`         |
| `transactionIntrospection` | `5664000..5664999`             | `TransactionIntrospection` |
| `instructionPlans`         | `7618000..7618999`             | `InstructionPlans`         |
| `codecs`                   | `8078000..8078999`             | `Codecs`                   |
| `rpc`                      | `8100000..8100999`             | `Rpc`                      |
| `rpcSubscriptions`         | `8190000..8190999`             | `RpcSubscriptions`         |
| `subscribable`             | `8195000..8195999`             | `Subscribable`             |
| `programClients`           | `8500000..8500999`             | `ProgramClients`           |
| `mobileWalletAdapter`      | `8400000..8400199`             | `MobileWalletAdapter`      |
| `helius`                   | `8600000..8600099`             | `Helius`                   |
| `wallet`                   | `8900000..8900999`             | `Wallet`                   |
| `invariantViolation`       | `9900000..9900999`             | `InvariantViolation`       |
| `unknown`                  | fallback                       | `Unknown`                  |

All 25 constants are `complete` for classification identity. `jsonRpc`,
`addresses` and the other intervals whose endpoints are not codes can only be
observed in Rust: the Dart `SolanaErrorCode` type cannot represent an arbitrary
integer, so `fromValue` returns `null` for every gap probe. The fixture records
that difference instead of pretending the Dart API classifies those integers.

## Complete and partial rows

The 37 complete rows are the 25 domain constants, `SolanaErrorCode.new`,
`SolanaErrorCode.value`, `SolanaError.context`, `SolanaError.code`,
`SolanaError.new`, `solanaErrorMessages`,
`SolanaErrorCodeDomainExtension.solanaErrorDomain`,
`SolanaErrorCodeDomainExtension.isSolanaErrorDomain`,
`SolanaErrorDomainExtension.domain`, `SolanaErrorDomainExtension.isInDomain`,
`getSolanaErrorDomain` and `isSolanaErrorCodeInDomain`.

The 36 partial rows keep concrete residuals:

- The Rust `ErrorCode` is a struct plus associated constants, not an enum, so
  `SolanaErrorCode` and `fromValue` omit `values()`, `name` and `index`
  reflection. Unknown integers are preserved instead of returning `null`.
- The domain enum and both extension declarations use inherent methods rather
  than Dart extension dispatch. `isSolanaErrorInDomain` loses the `Object?`
  guard because Rust errors are typed.
- `SolanaError` implements `std::error::Error`, not `Exception`. `Display`
  emits only `SolanaError#<code>`; the message is rendered separately and
  capped at 4096 bytes. `isSolanaError` uses `ErrorFacts` and therefore also
  matches non-diagnostic errors that expose facts.
- Context encoding preserves the five pinned literals and three additional
  unicode/boolean/nesting literals, but sorts keys, rejects non-ASCII keys and
  enforces explicit size limits. Decoding keeps 128-bit integers where Dart
  `int` is 64-bit, and rejects fractional JSON numbers.
- The ten context-key constants are exact and usable, but Rust exports no
  `SolanaErrorContextKeys` namespace. `createSolanaErrorContext`,
  `createSolanaError` and `wrapSolanaError` keep null dropping and the caller's
  existing `cause` fact; the nested cause is a typed source with the same code
  and context instead of `causeType`/`causeCode`/`causeContext` strings.
- `getSolanaErrorFromInstructionError` maps all 54 names and the 93-row oracle,
  but requires `u8` indices and `u32` custom payloads where Dart accepts any
  `num` and non-numeric custom values.
  `getSolanaErrorFromTransactionError` has the same index boundary and rejects
  multi-key and fractional shapes.
- `getSolanaErrorFromJsonRpcError` preserves unknown signed codes instead of
  converting them to `malformedJsonRpcError`, and rejects fractional or missing
  codes rather than coercing them. Preflight still projects 13 defaults and a
  typed execution cause; the source's `cause` map entry is the Rust source.
- `getErrorMessage` reproduces all 365 templates and interpolations, but the
  renderer is bounded and object keys are sorted.
- `RpcEnumErrorConfig` keeps the exact ordered offsets and a once-only typed
  callback. The base is a defined `ErrorCode`, the names are a borrowed slice
  and an explicit JSON null payload is absent, matching the source's nullable
  read. Multi-key maps and undefined offsets are rejected.
- `unwrapSimulationError` returns one typed cause for the three pinned codes
  and leaves other diagnostics unchanged. Dart can return any stored context
  value; Rust only unwraps typed causes and distinguishes boundary origins.

## Real fix

The only production change is in
`crates/solana_kit_client/src/diagnostics/rpc_enum.rs`. The earlier mapping
passed `{"Name": null}` to the callback as `Some(ContextValue::Null)`. The
source reads a nullable payload, so an explicit JSON null is indistinguishable
from an absent one. The mapper now yields `None`. The regression is covered by
`error-rpc-enum-offset-callback` and the native
`pinned_rpc_enum_offsets_and_nullable_payload` test. No other production source,
dependency, budget or security limit changed.

## Status and totals

The canonical ledger now has 404 complete, 59 partial and 17,485 pending rows
across 17,948 declarations. Phase 2 is 404 complete, 59 partial and 924
pending. Every package disposition remains `inventoried`; the 73 reviewed rows
do not claim `SolanaErrorCode` as an enum, any future-phase integration or the
package itself.

## Reproduction

```sh
cargo test --locked -p solana_kit_core --all-features --offline --test shared_errors
cargo test --locked -p solana_kit_client --all-features --offline --test shared_errors
cargo test --locked -p solana_kit_conformance --offline --lib
cargo run --locked --manifest-path tools/capability_inventory/Cargo.toml --offline -- \
  check <clean-pin-checkout> .
rustc --edition=2024 -D warnings verification/shared-errors/adjudicate.rs \
  -o /tmp/solana-kit-adjudicate
```
