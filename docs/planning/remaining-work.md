# Remaining work to the whole-SDK definition of done

This is a finite, acceptance-keyed backlog for the off-chain Solana Kit Rust SDK.
It is not a fresh declaration-by-declaration audit and it does not promote any
status. The whole SDK is not complete.

## Scope and current totals

The reference is Dart commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46`. The
implementation snapshot for this audit is `79444fb8`. The [plan definition of done](plan.md) requires every package capability to have implemented, behaviorally
evidenced Rust behavior, every promised runtime journey to execute, and registry
release preparation to succeed. A compiled placeholder, a skipped test, or a
documented design alone does not satisfy an entry.

The canonical [capability ledger](../parity/capabilities.tsv) contains **17,948**
declarations across **70** packages with extracted declarations. The
[package disposition ledger](parity.tsv) contains **71** top-level packages.
`source-files.tsv` covers **1,581** defining source files.

| Phase | Declarations | Partial | Pending | Complete |
| ----- | -----------: | ------: | ------: | -------: |
| 2     |        1,387 |       1 |   1,386 |        0 |
| 3     |          746 |       0 |     746 |        0 |
| 4     |        1,403 |       0 |   1,403 |        0 |
| 5     |          262 |       0 |     262 |        0 |
| 6     |        9,991 |       0 |   9,991 |        0 |
| 7     |          751 |       0 |     751 |        0 |
| 8     |        3,408 |       0 |   3,408 |        0 |
| Total |       17,948 |       1 |  17,947 |        0 |

The single partial row is `mergeRoles`; the remaining 17,947 rows are pending.
All 71 package-disposition rows are `inventoried`, with no `implemented`,
`verified`, or `blocked` claim. `rustc tools/check_plan.rs` reports
`verified 71 unique package records and their planning states`.

These are the audit snapshot totals. The subsequent
[address adjudication pilot](../parity/address-adjudication.md) adds a checked
manifest and reviews twelve address declarations: two complete, ten partial.
The canonical totals after that pilot are 2 complete, 11 partial and 17,935
pending, with the same 17,948 IDs and source hashes. RW-01a is implemented;
family-by-family adjudication and the other backlog items remain open.

Scoped receipts retain their own exact counts and are not canonical totals:
`message-transactions.tsv` has 746 rows (46 partial, 700 pending);
`signer-capabilities.tsv` has 85 (39 partial, 46 pending);
`transaction-confirmation.tsv` has 59 (31 partial, 28 pending);
`subscriptions.tsv` has 173 (68 partial, 105 pending);
`transaction-introspection.tsv` has 42 partial;
`offchain-messages.tsv` has 83 partial;
`mobile-wallet-protocol.tsv` has 179 partial;
`verification/instruction-plans/canonical.tsv` has 212 partial.

This unit added no production SDK change, root manifest/lockfile change,
workflow change, shared-suite change, fixture/golden change, or canonical
status/ID/hash/checker/threshold change.

## Status authority and historical evidence

- `docs/parity/capabilities.tsv` is the canonical declaration ledger.
- `docs/planning/parity.tsv` is the package-disposition ledger, not a behavioral
  completion ledger.
- `docs/parity/*-remaining.tsv` files are historical slice receipts with their
  own original unit labels and counts. They are not current gap inventories.
- `docs/planning/tasks.md` is an old delivery checklist. Several lines are stale
  and do not override the receipts linked below.
- A missing Dart wrapper name is not proof of missing Rust behavior. The
  [Phase 2/3 reconciliation](../parity/phase-two-three-current.md) records the
  typed Rust equivalents and the evidence that replaces several named wrappers.

## Already established behavior

These results are inputs to the backlog, not completion claims.

| Area                 | Current executed evidence                                                                                                                                                                                                                                                                                                                             |
| -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Portable conformance | `solana-kit-core-v56` passes 455/455 natively, in Chrome, both Wasmtime 50 previews and the pinned LeptosKit interpreter; corruption is 454/455. [Receipt](../verification/parity-completion-runtime.md)                                                                                                                                              |
| Program clients      | 17 owned-validator tests pass for builtins and pinned external programs. [Receipt](../verification/program-integration.md)                                                                                                                                                                                                                            |
| Codama package       | Packed NPM consumer, 9 NPM tests, 116 generator tests and the exact 9 groups on native/Chrome/WASI/LeptosKit. [Receipt](../verification/codama-package-current.md)                                                                                                                                                                                    |
| Native wallet        | iOS controller twice, Android controller twice on a fresh API 35 arm64 emulator, and a three-process macOS computer-use journey with independent ledger/state oracles. [iOS](../verification/installed-wallet-current.md), [Android](../verification/installed-wallet-android-current.md), [macOS](../verification/installed-wallet-macos-current.md) |
| Added core behavior  | Grinding/equality, message decompilation, explicit signer configuration, Token plans, typed parsed accounts, `test_support`, fixed-point composition and migration examples are merged and evidenced.                                                                                                                                                 |

## Backlog

Each item states its kind, acceptance criterion, current finding, owning
code/source, verification, and the smallest next unit. The order in this section
is the recommended execution order.

### RW-01 Canonical declaration closure

**Kind.** Implementation/tooling plus evidence. This is the largest remaining
acceptance item and the one that blocks every phase from an honest completion
claim.

**Acceptance criterion.** Phase 8: "the capability ledger is complete, runtime
evidence is linked." The plan's definition of done also requires behavioral
evidence for every package capability.

**Current finding.** The canonical ledger has zero complete rows. The extractor
does not read scoped family receipts and cannot be incrementally adjudicated:

- `tools/capability_inventory/src/ledger.rs:46` builds every row status through
  `pilot(package, symbol)`.
- `tools/capability_inventory/src/ledger.rs:259-273` returns `partial` only for
  `solana_kit_instructions#mergeRoles` and `pending` for everything else.
- `Tables::check` at `:86-103` requires an exact regeneration match, so editing
  `capabilities.tsv` without changing the extractor is rejected.
- The existing controls at `:114-129` correctly reject missing, duplicate,
  invented-verified and unsupported-evidence rows.
- Scoped receipt generators also hard-code non-completion. Examples:
  `verification/signer-capabilities/audit.rs:41` panics on any completion claim
  and `:44` fixes the total at 85/39/46; `verification/durable-nonce/src/bin/receipt.rs:95`
  accepts only `pending|partial`; `verification/instruction-plans/coverage.rs:38`
  always writes `partial`; `verification/transaction-confirmation/src/bin/receipt.rs:95`
  prints `complete=0`; `tools/codama-renderer/src/bin/program_clients_audit.rs:186`
  treats a `pending` to `complete` change as corruption;
  `tools/codama-renderer/src/alt_config_receipt.rs:53` requires `partial`;
  `verification/collection-layouts/receipt.rs:89` starts every row at `partial`;
  and `verification/wallet-consumer/src/bin/receipt.rs:30` reports all rows as
  partial.

These guards are useful: they prevent silent mass promotion. They are not an
incremental adjudication mechanism.

**Owning code/source.** `tools/capability_inventory` (extractor and controls),
all family receipt generators under `verification/` and `tools/codama-renderer`,
`docs/parity/capabilities.tsv`, and `docs/planning/parity.tsv`.

**Verification.** The package ledger check is available now:

```sh
rustc --edition=2024 -D warnings tools/check_plan.rs -o /tmp/solana-kit-check-plan
/tmp/solana-kit-check-plan
```

The full extractor check needs a clean checkout at the immutable Dart pin and a
cached `tree-sitter` dependency:

```sh
cargo run --manifest-path tools/capability_inventory/Cargo.toml --locked -- check "$PINNED_DART"
```

At this snapshot the available local reference checkout contains the pin object
but its working HEAD is not the pin, and the offline dependency cache lacks
`tree-sitter`; the extractor test command therefore could not be executed in
this unit. This is an environment/setup blocker, not evidence that the ledger is
wrong.

**Next minimal unit (RW-01a).** Add one machine-checked adjudication source and
pilot exactly one small family. Do not change IDs, hashes, source extraction,
thresholds, or the existing failure controls.

1. Add `docs/parity/adjudications.tsv` with columns
   `id`, `sha256`, `status`, `evidence`, `cases`, `notes`, `decision`.
   Every row is keyed by the complete canonical ID, not package/symbol.
2. Extend the extractor to load that file after the inventory. Validate:
   every ID exists once; the hash equals the canonical source hash; status is
   `pending|partial|complete`; every evidence path exists; `complete` requires
   nonempty exact case names; `partial` requires a bounded note; unknown,
   duplicate, hash-drifted, or evidence-free rows fail.
3. Regenerate only `status`, `evidence` and `notes` from that source. Leave
   `id`, `package`, `path`, `symbol`, `kind`, `line`, `owner`, `phase`, `unit`,
   `sha256` and `provenance` byte-identical by construction.
4. Add negative controls for an unknown ID, a duplicate ID, a changed hash, a
   missing evidence file, a `complete` row with no case, and an invented
   verification claim. Keep the existing controls.
5. Pilot the twelve `solana_kit_address` declarations in
   `docs/parity/capabilities.tsv` using existing address/collation/PDA executable
   tests. One complete row is enough to prove the mechanism; do not promote the
   rest of the family by association.
6. Only after the pilot: roll out one family at a time, in phase order. Use a
   single test or conformance case to cover a group only when the case proves
   every mapped row's exact contract, and record that mapping in `cases`.
7. Promote the 71 package rows separately in `docs/planning/parity.tsv`, with
   evidence, only after the corresponding capability families are honestly
   complete. `check_plan` already rejects implementation claims without evidence.

**Completion rule for a row.** `complete` means the exact public behavior is
exercised by a committed executable test or conformance case on the promised
runtime, with no ignored/skipped/filtered substitute. A compiled API, a type
alias, a documentation link, or a generator's ability to emit code is not
sufficient. Deliberate substitutions stay `partial` unless an executable
equivalence proof and a recorded decision justify `complete`.

**Explicit non-goals.** No mass status edit. No ID or hash rewrite. No weakening
of `Tables::check` or the family auditors. No claim that a scoped receipt's
partial count completes canonical rows.

### RW-02 Publication projections in shared conformance

**Kind.** Evidence. The implementation already exists.

**Acceptance criterion.** Phase 8 runtime evidence is linked; the publisher
package acceptance includes portable execution, not only native tests.

**Current finding.** The normalized publication projection has 32 independent
native tests in `crates/solana_kit_dapp_publisher/tests/publication.rs` and its
four modules (`bundle` 12, `diagnostics` 3, `ingestion` 8, `session` 9). They
pass:

```sh
cargo test -p solana_kit_dapp_publisher --features portal --test publication --offline
```

Result at `79444fb8`: 32 passed, 0 failed. They are not in the v56 shared suite.
`verification/conformance/src/lib.rs:205-207` appends only the publisher
attestation, portal and workflow groups. The 32 projection bodies therefore have
no Chrome, WASI or LeptosKit execution evidence.

**Owning code/source.** `crates/solana_kit_dapp_publisher/tests/publication.rs`,
`verification/conformance/src/publisher.rs`,
`verification/conformance/src/publisher_portal.rs`,
`verification/conformance/src/publisher_workflow.rs`,
`crates/solana_kit_dapp_publisher/src/publication/`.

**Verification.** After appending, run the full five-host gate with the pinned
host executables:

```sh
CHROME=/path/to/chrome CHROMEDRIVER=/path/to/chromedriver WASMTIME=/path/to/wasmtime \
  CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 CARGO_NET_OFFLINE=true cargo xtask all
```

The suite identity and expected case count must be updated only with the new
groups, and the canonical report/corruption reports must be regenerated from the
actual run. These are pure deterministic projections with no crypto, so no fuel
rate increase is expected; verify that against the actual report rather than
assuming it.

**Next minimal unit.** Add
`verification/conformance/src/publisher_publication.rs` containing the four
existing modules and four appended groups (or 32 cases if the parent wants
one-to-one labels). Update the registry/identity and committed reports, then run
`cargo xtask all`. No production code should change.

### RW-03 External-wallet MWA acceptance

**Kind.** Evidence. The local transport and Android handover already exist.

**Acceptance criterion.** Phase 7: run a real generated Android application
through balance fetch, signing, send/confirm, subscribe/unsubscribe and
pending-I/O unload; record wallet/platform availability instead of assuming
Android MWA exists on iOS.

**Current finding.** The implementation is materially ahead of the current
parity wording:

- `crates/solana_kit_native/src/mwa/mod.rs` implements a caller-owned local
  WebSocket association, HELLO negotiation, encrypted requests, deadlines and
  close.
- `crates/solana_kit_leptoskit/src/wallet_platform.rs` implements the Android
  availability/Intent/entropy guest calls.
- `crates/solana_kit_leptoskit/src/wallet_guest.rs` connects the association,
  launches the wallet and exposes supported MWA methods.
- `crates/solana_kit_leptoskit/android/MwaPlatform.kt` performs the foreground
  Activity handover.
- `verification/leptoskit-app-journey/wallet/guest/flow.rs` selects the Android
  mobile wallet path.

Five native loopback/transport tests pass:

```sh
cargo test -p solana_kit_native --features mwa --lib mwa --offline
```

Result at `79444fb8`: 5 passed, 0 failed. That proves protocol transport against
an independent peer, not an installed external wallet. The installed iOS,
Android and macOS receipts exercise the **local** wallet controller, not MWA.

**Owning source.** Dart
`packages/solana_kit_mobile_wallet_adapter/lib/src/local_association_scenario.dart`,
`intent_launcher_io.dart`, `platform_check_io.dart`; current Rust paths above.

**Verification.** The existing loopback command is above. The missing acceptance
is a real emulator/application run with a named installed wallet: launch the
association Intent, approve authorization, sign a message and a transaction,
reject one request, disconnect, and verify the wallet/ledger effects
independently. There is no single executable command for that yet.

**Next minimal unit.** Add one explicitly gated installed-wallet scenario to
`verification/leptoskit-app-journey/installed` that selects Android MWA and
records the wallet package/version and availability. Run it once on the existing
API 35 arm64 emulator against a named external wallet, with the owned validator
and an independent observer. Do not treat a missing wallet installation as a
code failure; record the external prerequisite.

**Blocker/substitution.** External wallet availability is an environment
prerequisite. MWA on iOS is not offered by the platform and should be recorded
as unavailable, not implemented as a fake transport.

### RW-04 Remote reflector association and transport

**Kind.** Missing source behavior, unless the parent explicitly accepts a
local-Android-only scope.

**Acceptance criterion.** Phase 7 Mobile Wallet Adapter protocol coverage and
the plan's package-capability definition. The source package exposes both local
and remote association scenarios.

**Current finding.** `solana_kit_wallet::mwa::Association::Remote` and
`association_uri` can parse/build a reflector URI, but there is no remote
connector. `NativeAssociation::connect` binds only a private local port;
`crates/solana_kit_native/src/mwa/mod.rs` has no reflector/remote transport.
Dart `remote_association_scenario.dart` connects to a reflector WebSocket,
accepts binary or base64 subprotocol, receives a reflector ID, waits for the
wallet join, sends HELLO and then uses the same encrypted session.

**Owning source.** Dart
`packages/solana_kit_mobile_wallet_adapter/lib/src/remote_association_scenario.dart:69-158,515-528`;
current Rust `crates/solana_kit_wallet/src/mwa/association.rs` and
`crates/solana_kit_native/src/mwa/`.

**Verification.** Add a loopback TLS reflector fixture and an executable test
covering: exact remote association URI, binary and base64 subprotocol selection,
reflector ID, reflection-established signal, HELLO/HELLO_RSP, encrypted
request/response, close and a deadline/cancellation path. Run:

```sh
cargo test -p solana_kit_native --features mwa --offline
```

Then repeat against a named public reflector only when an explicit endpoint and
privacy review exist. There is no current command because the code is absent.

**Next minimal unit.** Add a caller-owned `ReflectorTransport`/`connect_remote`
that takes an explicit endpoint, TLS roots and budgets; do not add an ambient
default host, discovery, retry loop or background task. Keep the local transport
unchanged and reuse the existing session/crypto path. If the parent accepts
local-only MWA scope, record that decision and keep this item open rather than
claiming remote parity.

**Substitution note.** Browser association via a hidden iframe
(`intent_launcher_web.dart`) is also unimplemented. Treat it as a separate
browser scope decision; it is not needed for the Android installed-wallet
acceptance in RW-03.

### RW-05 Public WASI socket and clock named-host receipt

**Kind.** Evidence. The implementation and a CI harness exist.

**Acceptance criterion.** Phase 4/5: WASI execution plus host unload, late
replies and quiet-stream cancellation across transports.

**Current finding.** `crates/solana_kit_wasi` exposes `WasiWebSocket`,
`WasiClock` and `Reactor` under the `websocket` feature. A private harness at
`verification/wasi-text-sockets` checks real WS/WSS peers, fragmented text,
frame bounds, quiet cancellation, process destruction and embedded Store
replacement. `.github/workflows/client-sessions.yml` runs it, but the repository
has no committed execution receipt. The current book/runtime text still says the
socket path awaits full execution.

**Owning code/source.** `crates/solana_kit_wasi/src/`, `verification/wasi-text-sockets/`,
`.github/workflows/client-sessions.yml`.

**Verification.** Set `WASMTIME` to the pinned Wasmtime 50 executable and run the
harness from the workflow:

```sh
cargo +1.99.1 fmt --manifest-path verification/wasi-text-sockets/Cargo.toml --all --check
cargo +1.99.1 test --locked --manifest-path verification/wasi-text-sockets/Cargo.toml --bin runner embedded::tests
cargo +1.99.1 build --locked --manifest-path verification/wasi-text-sockets/Cargo.toml \
  --bin guest --target wasm32-wasip2 --message-format=json > /tmp/wasi-text-sockets.json
# select the single guest artifact from /tmp/wasi-text-sockets.json, then:
cargo +1.99.1 run --locked --manifest-path verification/wasi-text-sockets/Cargo.toml \
  --bin runner -- "$WASMTIME" /path/to/guest.wasm
cargo +1.99.1 run --locked --manifest-path verification/wasi-text-sockets/Cargo.toml \
  --bin runner -- --embedded /path/to/guest.wasm
```

**Next minimal unit.** Execute the harness once, retain the output and artifact
hashes, add a committed receipt, and correct the current status lines in
`crates/solana_kit_wasi/README.md`, `docs/book/runtimes/wasi.md` and the relevant
parity link. No production code change unless the run exposes a defect.

### RW-06 Publisher start reconciliation and normal CLI enablement

**Kind.** Deliberate safe caller contract with one bounded source-parity gap;
normal CLI registry install is an external release prerequisite.

**Acceptance criterion.** Phase 8 publisher package behavior and release
readiness; the package-capability definition of done.

**Current finding.** Dart `PublicationWorkflow.startPublication` has an automatic
failure path:

- `publication_workflow.dart:185-235` catches a failure, tries to recover the
  release identity from the ingestion session, calls `cleanupFailedRelease`, and
  if the cleanup reports `preservedSubmitted`, calls `recoverPublicationResult`.
- `publication_workflow.dart:540-578` rehydrates the submitted session/bundle
  and returns a successful workflow result.
- `publication_workflow.dart:581-606` performs the single cleanup call.
- The pinned source tests cover cleanup, preserved-submission recovery and
  cleanup failure at `test/publication_workflow_test.dart:919,947,995`.

Rust deliberately exposes the primitives instead of automatic destructive
behavior: `Workflow::cleanup_failed` at
`crates/solana_kit_dapp_publisher/src/workflow/engine.rs:159-203`, persisted
effect intent and `Error::Uncertain` at `workflow/error.rs`, and `resume` by
snapshot/session/release at `engine.rs:125-157`. The workflow receipt says it
does not invent automatic uncertain-effect reconciliation, and the book says the
CLI never replays an ambiguous step.

Judgment: automatic deletion of a failed release is destructive and should not be
ported as a default. Automatic recovery of a **preserved submitted** release is
safe and is not behaviorally equivalent to requiring the caller to resume
manually. The smallest honest closure is either an explicit, typed
`reconcile_start` helper that performs the source's fallback lookup, one cleanup,
and preserved-submission hydration, or a recorded decision that the
caller-explicit `resume` contract is the accepted substitution.

**Normal CLI enablement.** The native binary is implemented and usable from a
checkout or archive (`cargo install --git`, prebuilt `publisher-cli-v*`). The
crate has `publish = false`; the book says it is not on crates.io because its SDK
dependencies are unpublished. That is a registry/release prerequisite, not a
missing CLI command. It is covered by RW-07.

**Verification.** The existing behavior is executable:

```sh
cargo test -p solana_kit_dapp_publisher --features workflow --test workflow --offline
```

Result at `79444fb8`: 41 passed, 0 failed. There is no Rust test for the source's
automatic catch/cleanup/recovery path because that path is intentionally absent.

**Next minimal unit.** Add a decision entry (or a typed `reconcile_start` helper
with simulated cleanup/preserved-submission fixtures) and keep the destructive
auto-cleanup out of `Drop`/broad error handling. Do not add retries or automatic
replay of an ambiguous mutation.

### RW-07 Whole-workspace registry and package release rehearsal

**Kind.** Evidence plus external release prerequisite.

**Acceptance criterion.** Phase 1: repeatable release preview for a real initial
package. Phase 8: clean-consumer packaging succeeds, release readiness succeeds,
and only implemented crates are published.

**Current finding.** `monochange check` passes with 0 issues. A read-only preview
at `79444fb8` reports 35 affected packages, 29 release targets and 110
changesets, but every target has `release: false`, `package_publications` is
empty, and all 29 targets plan `0.1.1`. It computes versions/changelogs but does
not execute the configured lockfile refresh or a publish rehearsal. Twenty crates
explicitly set `publish = false`; none sets `publish = true`.

Only `solana_kit_core 0.1.0` has a local-registry and clean-consumer receipt
([core package delivery](../verification/core-package-delivery.md)). The Codama
NPM tarball has a separate packed consumer. There is no full workspace
dependency-ordered package set, no clean downstream consumer of
`solana_kit_client`/`native`/`wallet` from that set, and no `cargo publish
--dry-run` result for the SDK closure.

**Owning code/source.** `monochange.toml`, `crates/*/Cargo.toml`,
`xtask/src/registry_fixture.rs`, `.github/workflows/publish.yml`,
`docs/verification/core-package-delivery.md`.

**Verification.**

```sh
monochange check
monochange preview --format json
monochange preview --diff
cargo package -p solana_kit_core --all-features --locked
cargo package -p solana_kit --locked
```

For the missing rehearsal, package every implemented crate in dependency order,
materialize a fresh local registry from the complete closure, and build a clean
consumer outside the repository. `cargo publish --dry-run` is the final check;
do not publish. The exact commands depend on the release metadata unit and must
be recorded, not inferred.

**Next minimal unit.** On a dedicated release-rehearsal branch (not in this docs
unit), enable publish metadata only for implemented crates, run Monochange's
planned apply/lock refresh in a disposable copy, inspect the complete diff, then
exercise the full local registry and downstream consumer. Keep the CLI registry
install blocked until this succeeds. Record external authorization before any
real publication.

### RW-08 Native Linux and Windows execution evidence

**Kind.** Evidence/external CI. No code substitute exists.

**Acceptance criterion.** Runtime evidence policy: native Tokio requires
Linux/macOS/Windows checks. Phase 1 also requires repeatable execution logs.

**Current finding.** The local gates are macOS. `.github/workflows/plan.yml`
defines a `native-core` matrix on `ubuntu-24.04`, `macos-15` and `windows-2025`;
`native-http.yml` and `native-websocket.yml` define their own OS matrices. The
repository contains no committed result from those jobs. The existing receipts
explicitly say configured CI is not executed Linux/Windows evidence.

**Owning code/source.** `.github/workflows/plan.yml`,
`.github/workflows/native-http.yml`,
`.github/workflows/native-websocket.yml`, and the existing local receipts.

**Verification.** Execute the matrix jobs and retain their full logs, toolchain
versions and artifact hashes. The core commands are:

```sh
cargo +1.99.1 fmt --all --check
cargo +1.99.1 test --workspace --all-features --locked
cargo +1.99.1 clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo +1.99.1 doc --workspace --all-features --no-deps --locked
cargo +1.99.1 run --quiet -p solana_kit_conformance --locked
```

The actual HTTP/WebSocket jobs add their loopback/TLS fixtures. A local macOS
run cannot satisfy this item.

**Next minimal unit.** Run the existing workflows on a branch containing this
work, attach the run URLs and exact logs, and update the runtime receipts. This
is a verification unit, not a code change.

### RW-09 Program-client operation breadth and fixture blockers

**Kind.** Evidence, with one external fixture blocker.

**Acceptance criterion.** Phase 6 pinned validator integration; the plan's
"every public operation has behavioral evidence" rule.

**Current finding.** The 17-test owned-validator suite passes and is strong
family evidence, but its own receipt records uncovered operations:

- Pyth receiver has no `.so` in the pinned fixture set, so no positive receiver
  case exists.
- Subscriptions covers authority and plan only; subscribe/cancel/resume, fixed
  and recurring delegations, and event decoding are not exercised.
- Squads covers config, multisig creation and member expansion; vault
  transaction, proposal and execution flows are not exercised.
- Bubblegum covers one V1 tree and mint/transfer/burn; V2, collections, DAS and
  vouchers are not exercised.
- The Anchor imposter is hash-recorded but not exercised.

**Owning code/source.** `verification/program-integration/` and the program
crates; pinned fixture artifacts described in
[program integration](../verification/program-integration.md).

**Verification.** The current suite is:

```sh
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 cargo +1.99.1 test --locked --offline \
  --manifest-path verification/program-integration/Cargo.toml -- --test-threads=1
```

Add one real instruction family per unit and assert independent post-state, not
just the client's own bytes.

**Next minimal unit.** Add Subscriptions subscribe/cancel/resume using the
already-pinned `subscriptions-v0.5.0.so` fixture. Treat the missing Pyth receiver
artifact as an external blocker, not a code failure.

### RW-10 Fixed-point residual reconciliation

**Kind.** Stale scoped evidence plus deliberate presentation substitutions. The
implementation and executable evidence already exist.

**Acceptance criterion.** Phase 2 exact numeric behavior and malformed/overflow
controls.

**Current finding.** `docs/parity/fixed-points-remaining.tsv` still names five
IDs, but [the composition note](../parity/fixed-point-composition.md) records
current evidence for all five:

- `binaryFixedPointToNumber` and `decimalFixedPointToNumber` have literal Dart
  oracle comparisons for finite, extreme, cancellation, subnormal, underflow and
  rounding values. The differences are deliberate: Rust returns typed
  `Error::Overflow` for nonfinite values and preserves finite subnormals instead
  of reproducing Dart's premature `Infinity`/`NaN`/zero. The note calls these
  deviations out explicitly.
- `FixedPointCodec.new`, `FixedPointDecoder.new` and `FixedPointEncoder.new` are
  covered by test-local public `Encoder`/`Decoder` implementations and
  `Paired`/`Mapped` composition. The tests execute callback errors, offsets,
  width mismatch, truncation, trailing bytes and budgets.

The old remaining TSV is not current gap evidence. The 126-entry receipt and
canonical rows still need adjudication, and the package/Phase 2 are not claimed
complete.

**Owning code/source.** `crates/solana_kit_core/src/fixed_points/`,
`crates/solana_kit_core/tests/fixed_point_numbers.rs`,
`crates/solana_kit_core/tests/fixed_point_composition.rs`,
`docs/parity/fixed-point-composition.md`; pinned Dart
`packages/solana_kit_fixed_points/lib/src/binary_formatting.dart`,
`decimal_formatting.dart`, `fixed_point_codec.dart`.

**Verification.** Both evidence targets execute:

```sh
cargo test -p solana_kit_core --features fixed-points \
  --test fixed_point_numbers --test fixed_point_composition --offline
```

Result at `79444fb8`: 6 + 6 passed, 0 failed.

**Next minimal unit.** Regenerate or supersede `fixed-points-remaining.tsv` from
the composition note, record the accepted presentation deviations, and map the
five IDs in the adjudication source. No production codec or floating-point
change is required.

### RW-11 Numeric width and alphabet residuals

**Kind.** Stale scoped evidence plus deliberate UTF-16 substitution. This item
was corrected during this audit: it is not a missing generic BigInt codec.

**Acceptance criterion.** Phase 2 exact integer/string codecs.

**Current finding.** `docs/parity/numeric-string-codecs-remaining.tsv` says four
`readBigInt*`/`writeBigInt*` helpers lack arbitrary caller-selected widths. The
current implementation already provides exactly that contract:
`crates/solana_kit_core/src/codecs/big_number.rs` exposes
`BigNumber<BigUint>` and `BigNumber<BigInt>` with an explicit nonzero byte width
and endian, including range/truncation/trailing checks. The
`core_parity/bigints.rs` tests exercise 2-, 3- and 33-byte widths and rejection
paths. That receipt is therefore stale for those four IDs; the rows still need
canonical evidence adjudication, not a new codec.

The three `getBaseXCodec`/`getBaseXEncoder`/`getBaseXDecoder` rows remain partial
for a different reason: non-BMP alphabets index Dart UTF-16 code units and can
produce lone surrogates, which Rust `str` cannot represent. This is a deliberate
language-boundary substitution. Keep the safe Rust behavior and do not copy the
Dart lone-surrogate result.

**Owning code/source.** `crates/solana_kit_core/src/codecs/big_number.rs`,
`crates/solana_kit_core/tests/core_parity/bigints.rs`, the text-codec module,
pinned Dart `packages/solana_kit_codecs_numbers/lib/src/utils.dart:154-219` and
`packages/solana_kit_codecs_strings/lib/src/base_x.dart`.

**Verification.** The generic-width implementation executes with:

```sh
cargo test -p solana_kit_core --features bigint-codecs --test core_parity --offline
```

The external consumer in `numeric-string-codecs.md` already implements a checked
three-byte numeric codec; extend that pattern rather than adding another integer
implementation.

**Next minimal unit.** Update the scoped numeric receipt and adjudication mapping
to point the four BigInt IDs at `BigNumber` and its exact tests, then record the
BaseX non-BMP substitution. No production codec change is required unless a
missing width/endian case is found.

### RW-12 Current documentation drift

**Kind.** Documentation/evidence. Phase 8 requires API and docs to agree.

**Current finding.** These current-looking files still describe earlier states:

- `docs/verification/leptoskit-http.md:110-111` says generated desktop, Android
  and iOS HTTP journeys are future work. The installed wallet receipts execute
  `Client<GuestHttp>` in generated apps.
- `docs/parity/mobile-wallet-protocol.md` says platform handover, transports,
  Android Intents and UI remain pending. Local transport, Android handover and
  the wallet control surface are implemented; external-wallet and remote
  reflector evidence remain the real gaps.
- `docs/parity/dapp-publisher.md:92` says portable-host execution remains
  unexecuted. The workflow groups are now in the shared conformance suite.
- `docs/parity/subscriptions.md:36` says its isolated cases were not added to
  the shared suite. `subscription-runtime.md` records four appended groups.
- `docs/parity/numeric-string-codecs.md` still says arbitrary multiword BigInt
  widths are not public, although `BigNumber` now provides them (RW-11).
- `docs/parity/fixed-points-remaining.tsv` still gives "unevidenced" reasons for
  the five IDs that `fixed-point-composition.md` now covers (RW-10).
- `docs/book/runtimes/wasi.md:130` still points at an awaiting-execution status
  for the public socket/clock path; RW-05 must supply the receipt first.
- `docs/architecture/design.md:290` still says generated mobile HTTP journeys
  remain future work.
- `docs/planning/tasks.md` is an old checklist and is not a current status
  authority.

**Next minimal unit.** After RW-03/RW-05/RW-06 close their evidence, correct
these files in one docs-only change, preserving each historical count and
explicitly labeling old evidence. Do not use a stale file as proof that current
code is absent.

### RW-13 Mac2 automation admin prerequisite

**Kind.** Blocker, admin prerequisite; the functional macOS proof is separate.

**Acceptance criterion.** Phase 7 automated installed macOS journey; the
maintained controller path, not the computer-use fallback.

**Current finding.** The macOS computer-use journey passed with independent
ledger/state checks. The Mac2/XCTest controller route still times out while
enabling automation; `DevToolsSecurity` is disabled, and the user-approved
`sudo -n DevToolsSecurity -enable` failed because a password is required. No
sudo retry, secret collection or repeated permission request was attempted.

**Owning code/source.** `verification/leptoskit-app-journey/installed/`,
[macOS receipt](../verification/installed-wallet-macos-current.md).

**Verification.** After an administrator enables developer-tool authorization,
run the maintained installed controller against the generated macOS wallet and
retain the XCTest output. This unit cannot perform that action.

**Next minimal unit.** An administrator prerequisite, not code: enable
`DevToolsSecurity` (or the equivalent authorization) and rerun once. Keep the
functional computer-use proof distinct from the automated driver route.

## Classification before coding

| Finding                                               | Classification                                   | Action                                                                  |
| ----------------------------------------------------- | ------------------------------------------------ | ----------------------------------------------------------------------- |
| Automatic failed-release deletion                     | Deliberate safe substitution                     | Do not port as default; require an explicit caller action               |
| Automatic preserved-submission recovery               | Bounded source-parity gap                        | Add typed reconcile helper or record the caller-explicit decision       |
| MWA local transport / Android handover / wallet UI    | Implemented; external-wallet proof absent        | Run RW-03; do not rewrite the transport                                 |
| MWA remote reflector                                  | Missing source behavior                          | Implement RW-04 or accept local-only scope explicitly                   |
| Publication projections                               | Implemented; shared-host proof absent            | RW-02                                                                   |
| WASI public socket/clock                              | Implemented; named-host receipt absent           | RW-05                                                                   |
| Linux/Windows                                         | CI jobs exist; no executed evidence              | RW-08                                                                   |
| Registry/release                                      | Release prerequisite                             | RW-07; CLI code is not missing                                          |
| Fixed-point conversions and callback constructors     | Already implemented/evidenced; scoped list stale | Update the receipt/adjudication; do not restore the Dart numerical bugs |
| Arbitrary-width BigInt codecs                         | Already implemented; scoped receipt stale        | Update the receipt/adjudication; do not add another codec               |
| Non-BMP BaseX alphabets                               | Deliberate UTF-8 boundary                        | Keep the safe Rust behavior; record the difference                      |
| Subscription pooling, coalescing, autoping, reconnect | Not required by the stated acceptance            | Keep caller-owned transport; do not add speculative machinery           |
| Mac2/XCTest                                           | Admin prerequisite                               | RW-13; do not retry sudo                                                |
| Historic `*-remaining.tsv` lists                      | Historical slice evidence                        | Do not count as current gaps; use canonical IDs for RW-01               |

## Achievable order

1. **RW-01a**: adjudication manifest plus the twelve-address pilot. This unlocks
   honest incremental closure and prevents a later mass-promotion pass.
2. **RW-02**: publication projections in the shared suite. Small, independent,
   and closes a clear evidence hole.
3. **RW-05**: WASI socket/clock named-host receipt. The harness already exists.
4. **RW-03**: external Android wallet acceptance. This is the largest genuine
   Phase 7 behavior proof still absent; it may reveal small implementation fixes.
5. **RW-04**: remote reflector transport, unless a local-only decision is made.
6. **RW-06**: publisher reconciliation decision or typed helper.
7. **RW-08**: Linux/Windows CI evidence; can run in parallel with the above.
8. **RW-07**: full release rehearsal, after the release metadata and evidence
   decisions are stable.
9. **RW-09/RW-10/RW-11**: program breadth and the two stale numeric receipts.
   RW-10 and RW-11 are evidence reconciliation, not codec implementation.
10. **RW-12/RW-13**: documentation alignment and the Mac2 admin prerequisite.
11. **RW-01b onward**: phase-ordered ledger closure, starting with Phase 2 then
    Phase 3, 4/5, 6, 7 and 8. Do not start with generated-program rows; they are
    the largest tranche and need the adjudication mechanism proven first.

This order is achievable because each step has an independent executable result.
It is not a promise that every item is small: declaration closure and external
wallet acceptance are long-horizon work.

## Next unit to brief

**RW-01a: canonical adjudication manifest and one-family pilot.**

- Goal: prove honest incremental adjudication without changing any canonical ID,
  hash, source extraction, threshold, or existing failure control.
- Files to add/change: `docs/parity/adjudications.tsv`,
  `tools/capability_inventory/src/ledger.rs`, its tests and README, and the
  twelve `solana_kit_address` rows in `docs/parity/capabilities.tsv` after
  successful regeneration.
- Do not touch production SDK crates, root manifests/locks, workflows, shared
  conformance, fixtures/goldens, or the other family receipts.
- Acceptance: the extractor check passes with the same 17,948 IDs and hashes;
  exactly the piloted rows change status/evidence/notes; unknown, duplicate,
  hash-drift, missing-evidence and complete-without-case controls all fail;
  `rustc tools/check_plan.rs` still reports 71 package records.
- Parallel quick win if the parent wants a smaller first change: RW-02, adding
  the 32 existing publication projection bodies to shared conformance.

The whole SDK remains incomplete after either unit. The honest completion claim
requires the remaining backlog plus a full release rehearsal.
