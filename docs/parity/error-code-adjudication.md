# Error-code adjudication

This note records the reviewed adjudication of the 365 `SolanaErrorCode` enum
constants in the pinned `solana_kit_errors` package. It does not complete the
package, the phase, or the SDK.

## Scope and exclusions

The pinned source defines 438 declarations in `solana_kit_errors`: 365
`SolanaErrorCode` constants in `lib/src/codes.dart` and 25 `SolanaErrorDomain`
constants in `lib/src/error_domain.dart`, plus enum classes, constructors,
getters, functions and extensions. This unit adjudicates exactly the 365
`SolanaErrorCode` constants. The 25 domain constants, the enum class, its
constructors and getters, dynamic error construction, and every other
`solana_kit_errors` declaration remain outside the claim.

A complete row here means the constant's exact pinned **name** and **integer**
are executed against an independent literal table on every promised host. It is
not a claim about `SolanaError`, `getSolanaErrorFromTransactionError`, message
interpolation with caller context, or any other declaration in the package.

## Identity

| Field               | Value                                                              |
| ------------------- | ------------------------------------------------------------------ |
| Reference pin       | `0138bb956d2d81ccc0c56edc602401c0df2f2b46`                         |
| Defining source     | `packages/solana_kit_errors/lib/src/codes.dart`                    |
| Source SHA-256      | `c83928e4ac7fe6c4a50d8e1a1922342a88e560ff93f4cad60d06edf0ab560e36` |
| Canonical ID prefix | `dart:solana_kit_errors:lib/src/codes.dart#SolanaErrorCode.`       |
| Rows                | 365 complete                                                       |
| Evidence            | `verification/conformance/evidence/native-success.txt`             |
| Case                | `error-code-catalog-exact-365`                                     |

Every row is keyed by its complete canonical ID plus that source hash in
[`adjudications.tsv`](adjudications.tsv). Regeneration changes only `status`,
`evidence` and `notes`. The other 17,583 capability rows, every ID, every source
hash, `source-files.tsv` and `delivery-units.md` remain byte-identical.

## Independent literal table

[`catalog.tsv`](../../crates/solana_kit_core/tests/shared_errors/catalog.tsv)
contains 365 rows in `codes.dart` declaration order. Each row stores the exact
name, signed integer, lower-camel domain and hex message-template bytes derived
from the pinned source.

A disposable clean clone at the reference pin was parsed independently of the
Rust implementation. The check read 365 enum constants from `codes.dart`, 365
template literals from `messages.dart`, and the 23 numeric domain ranges plus the
historical accounts typo from `error_domain.dart`. All 365 catalog rows matched
the extracted name, integer, domain and template bytes exactly. That extraction
was scratch verification; the production fixture was not regenerated from the
Rust catalog.

## Executed proof

`error-code-catalog-exact-365` executes the shared body in
`crates/solana_kit_core/tests/shared_errors/exact_catalog.rs`. The native
`shared_errors` integration test and the portable conformance suite include the
same file; there is no second expected table. For every definition and literal
row, in order, the body checks:

- the exact `CodeDefinition::name`;
- the exact signed `ErrorCode::value()`;
- integer uniqueness across all 365 rows;
- the domain returned by `ErrorCode::domain()` against the pinned rule;
- the exact template bytes, decoded from hex;
- `ErrorCode::definition()` returns that same definition;
- the default-context message renders and is non-empty;
- definition count, fixture row count and unique-integer count are all 365.

Mismatches return a `CaseResult` error with a stable field explanation. They do
not panic, abort the guest, or discard later case results.

`error-code-catalog-rejects-wrong-literal` is the negative control. It first
proves the unmodified fixture passes, then flips the last row's integer with
`value ^ 1` and requires the same comparison to report `catalog integer differs`
for that name. The full evidence case runs after the control, so a broken
comparison is visible without suppressing the later result. Native assertions
are unchanged in substance and still fail the `shared_errors` test on any
mismatch.

## Execution

The cases execute in `solana-kit-core-v58` at 461/461 on native Rust, real
Chrome, both Wasmtime 50 previews and the pinned LeptosKit host. The corruption
control is 460/461 with only `sha256-fips-abc` failing. The checked-in reports
hash as:

```text
d17a4a66dbb79a5a913fa15f50619dd86851064361aa0c55b57955a0eef1f9e3  native-success.txt
4852ad7359b8b13a74cbff4e2b2205449293eff167cb7083430045813fad0d62  native-corruption.txt
```

The native integration test runs the same two bodies directly through
`cargo test --locked -p solana_kit_core --all-features --offline --test shared_errors`.

## Status and totals

The 365 rows are `complete` only for the name/integer contract above. The
canonical ledgers now contain 367 complete, 23 partial and 17,558 pending rows
across 17,948 declarations; phase 2 is 367 complete, 23 partial and 997 pending.
The package-disposition ledger is unchanged because the package is not complete.

## Reproduction

Use a clean checkout at the reference pin:

```sh
cargo test --locked -p solana_kit_core --all-features --offline --test shared_errors
cargo test --locked -p solana_kit_conformance --offline --lib
cargo run --locked --manifest-path tools/capability_inventory/Cargo.toml --offline -- check <PINNED_DART> .
cargo run --locked --manifest-path tools/capability_inventory/Cargo.toml --offline -- controls <PINNED_DART> .
```

The conformance integrity test binds the checked-in success report to the live
case registry, so `cargo test -p solana_kit_conformance --lib` fails if the case
name, report rows or suite identity drift.
