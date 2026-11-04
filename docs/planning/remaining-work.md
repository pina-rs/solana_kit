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
| 2     |        1,387 |      61 |     924 |      402 |
| 3     |          746 |       0 |     746 |        0 |
| 4     |        1,403 |       0 |   1,403 |        0 |
| 5     |          262 |       0 |     262 |        0 |
| 6     |        9,991 |       0 |   9,991 |        0 |
| 7     |          751 |       0 |     751 |        0 |
| 8     |        3,408 |       0 |   3,408 |        0 |
| Total |       17,948 |      61 |  17,485 |      402 |

These totals include the address pilot, the twelve numeric adjudications, the
365 `SolanaErrorCode` name/integer claims and the 73 remaining
`solana_kit_errors` contracts (35 complete, 38 partial).
All 71 package-disposition rows are `inventoried`, with no `implemented`,
`verified`, or `blocked` claim. `rustc tools/check_plan.rs` reports
`verified 71 unique package records and their planning states`.

The original audit had one partial row (`mergeRoles`) and 17,947 pending. The
[address adjudication pilot](../parity/address-adjudication.md) adds a checked
manifest and reviews twelve address declarations: two complete, ten partial.
The subsequent [numeric adjudication](../parity/numeric-adjudication.md) records
twelve more partial declarations, and the
[error-code adjudication](../parity/error-code-adjudication.md) records 365
complete `SolanaErrorCode` name/integer claims, and the
[error-contract adjudication](../parity/error-contracts-adjudication.md) reviews
the other 73 declarations. All 17,948 IDs and source hashes are preserved. RW-01a and this bounded RW-10/RW-11 batch are implemented;
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

The original backlog audit added no production SDK change, root manifest/lockfile change,
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
| Portable conformance | `solana-kit-core-v59` passes 476/476 natively, in Chrome, both Wasmtime 50 previews and the pinned LeptosKit interpreter; corruption is 475/476. The first 461 v58 cases are preserved. [Receipt](../verification/error-contracts-runtime.md)                                                                                                         |
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

**Historical audit (superseded).** At the original audit snapshot the canonical
ledger had zero complete rows and the extractor built every row status through
`pilot(package, symbol)`:

- `tools/capability_inventory/src/ledger.rs:46` built every row status through
  `pilot(package, symbol)`.
- `tools/capability_inventory/src/ledger.rs:259-273` returned `partial` only for
  `solana_kit_instructions#mergeRoles` and `pending` for everything else.
- `Tables::check` at `:86-103` requires an exact regeneration match, so editing
  `capabilities.tsv` without changing the extractor was rejected.
- The existing controls at `:114-129` correctly reject missing, duplicate,
  invented-verified and unsupported-evidence rows.
- Scoped receipt generators hard-code non-completion. Examples:
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

Those guards still prevent silent mass promotion in their scoped receipts. They
are not the canonical adjudication mechanism.

**Current mechanism.** `docs/parity/adjudications.tsv` now supplies reviewed
rows keyed by complete canonical ID and defining source hash. `Manifest::parse`
rejects unknown, duplicate or hash-drifted IDs, absent or unsafe evidence, prose
evidence, blank or invented case names, malformed headers/field counts and
unknown statuses. `Tables::build` replaces only `status`, `evidence` and `notes`
for a matching ID; every other column is regenerated from the pinned source.
`check` requires an exact regeneration match and `controls` rejects every
manifest corruption in addition to missing, duplicate and invented rows.

The committed reviews are the twelve address declarations (two complete, ten
partial), twelve numeric/fixed-point declarations (all partial), 365
`SolanaErrorCode` declarations (all complete for exact name and integer only),
and the 73 remaining `solana_kit_errors` declarations (35 complete, 38 partial).
The canonical ledger therefore has 402 complete, 61 partial and 17,485 pending
rows. No package disposition is promoted: every package remains `inventoried`.
The error reviews do not claim the enum class as a whole, later-phase
integration or the package itself.

**Owning code/source.** `tools/capability_inventory` (extractor and controls),
all family receipt generators under `verification/` and `tools/codama-renderer`,
`docs/parity/capabilities.tsv`, and `docs/planning/parity.tsv`.

**Verification.** The package ledger check is available now:

```sh
rustc --edition=2024 -D warnings tools/check_plan.rs -o /tmp/solana-kit-check-plan
/tmp/solana-kit-check-plan
```

The full extractor check uses a clean checkout at the immutable Dart pin:

```sh
cargo run --locked --manifest-path tools/capability_inventory/Cargo.toml --offline -- check "$PINNED_DART" .
cargo run --locked --manifest-path tools/capability_inventory/Cargo.toml --offline -- controls "$PINNED_DART" .
cargo test --locked -p solana_kit_conformance --offline --lib
```

The current pin checkout is clean and the cached `tree-sitter` dependency is
available; the commands above executed in this unit. The earlier setup blocker
is historical.

**Next minimal unit (RW-01b).** Continue one bounded family at a time. Use the
existing machine-checked adjudication source; do not change IDs, hashes, source
extraction, thresholds, or the existing failure controls.

1. Pick one bounded family and one exact contract. Add or reuse an executable
   test/conformance body that proves every mapped row; do not infer neighboring
   declarations from a family name.
2. Append one manifest row per complete canonical ID with the defining source
   hash, exact successful case name and a bounded decision. Keep existing rows
   byte-identical and re-run the extractor `write`.
3. Prove only `status`, `evidence` and `notes` changed; `source-files.tsv`,
   `delivery-units.md` and every identity/hash/provenance column must stay
   byte-identical.
4. Run the native, Chrome, Wasmtime 50 Preview 1/2 and pinned LeptosKit hosts
   before the report is cited as complete evidence.
5. Promote the 71 package rows separately in `docs/planning/parity.tsv`, with
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

**Executed follow-up.** RW-02 is now closed by the
[publication runtime receipt](../verification/publication-runtime.md). All 32
existing projection bodies execute in four appended groups on native Rust,
Chrome, both Wasmtime previews and the pinned LeptosKit host: they passed in the
v58/461 report and still pass in the current v59/476 report, with only the
intended hash failure in the corruption control. The first 461 cases are
preserved. Findings below describe the earlier audit snapshot.

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

**Current finding.** The acceptance criterion is met on the owned emulator. The
[external Android wallet execution
receipt](../verification/installed-external-wallet-android-current.md) records
the 2026-11-04 final invocation based on `da4aaef2` with the cancellation repair
committed as `75c31455`: the installed `android-mwa` controller
drove a fresh ordinary generated application through authorization, funding,
message signature, rejection and approval of one exact 1,000,000-lamport System
transfer, the subscription race, cancellation of an actual pending wallet
prompt, fresh authorization, awaited disconnect and a genuine process relaunch
against the independently built pinned
`com.solana.mobilewalletadapter.fakewallet` at revision `100e0230`, exit 0, 18
and 6 named checkpoints. The implementation is materially ahead of the current
parity wording:

- `crates/solana_kit_native/src/mwa/mod.rs` implements a caller-owned local
  WebSocket association, HELLO negotiation, encrypted requests, deadlines and
  close, with the wallet sequence fix from `008f7285`.
- `crates/solana_kit_leptoskit/src/wallet_platform.rs` implements the Android
  availability/Intent/entropy guest calls.
- `crates/solana_kit_leptoskit/src/wallet_guest.rs` connects the association,
  launches the wallet and exposes supported MWA methods.
- `crates/solana_kit_leptoskit/android/MwaPlatform.kt` performs the foreground
  Activity handover.
- `verification/leptoskit-app-journey/wallet/guest/flow.rs` selects the Android
  mobile wallet path, and `flow/mobile.rs` now ends a cancelled pending request
  on its already-unloaded lease while an explicit idle-panel release still
  closes the owner.

Twenty-six focused native tests pass:

```sh
cargo test -p solana_kit_native --features mwa --lib mwa --offline
```

The earlier installed iOS, Android and macOS receipts exercise the **local**
wallet controller; the external-wallet acceptance above is the separate MWA
evidence and does not claim production wallets or physical devices.

**Owning source.** Dart
`packages/solana_kit_mobile_wallet_adapter/lib/src/local_association_scenario.dart`,
`intent_launcher_io.dart`, `platform_check_io.dart`; current Rust paths above.

**Verification.** The [installed external-wallet controller](../verification/leptoskit-app-journey/installed/README.md) is the
executable acceptance (`android-mwa` platform, explicit device, adb binary and
wallet package). The owned validator and an independent observer verify the
ledger bytes, signature, fee and balances; the wallet package/version is
recorded before any UI action and a missing wallet stays an environment
prerequisite. The retained final run and its hashes are listed in the receipt.

**Next minimal unit.** None for this emulator/reference-wallet acceptance.
Physical-device and production-wallet compatibility are separate extensions,
not prerequisites added to this criterion. Their absence remains an explicit
limit of the receipt.

**Blocker/substitution.** External wallet availability is an environment
prerequisite. MWA on iOS is not offered by the platform and should be recorded
as unavailable, not implemented as a fake transport.

### RW-04 Remote reflector association and transport

**Kind.** Native source behavior implemented with bounded substitutions;
public-reflector and installed-wallet acceptance remain separate.

**Implemented follow-up.** The
[native reflector receipt](../parity/mwa-remote-reflector.md) records the
caller-owned two-stage WSS association, binary/base64 negotiation, real
server-assigned ID/URI, wallet join, HELLO and the shared encrypted wallet path.
Twenty-six focused tests pass on current Rust and Rust 1.89, using owned TLS
peers. No default reflector, retry or SDK-owned background task is added.
The missing-connector finding below describes the original audit snapshot.

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

**Next minimal unit.** Native implementation and loopback acceptance are
recorded above. Public-reflector interoperability requires a named endpoint and
an explicit privacy decision; installed external-wallet proof stays under RW-03.
Neither is established by the native fixture.

**Substitution note.** Browser association via a hidden iframe
(`intent_launcher_web.dart`) is also unimplemented. Treat it as a separate
browser scope decision; it is not needed for the Android installed-wallet
acceptance in RW-03.

### RW-05 Public WASI socket and clock named-host receipt

**Kind.** Evidence. The implementation and a CI harness exist.

**Executed follow-up.** RW-05 is now closed for its named host by the
[public socket/clock receipt](../verification/wasi-text-sockets-current.md):
18 CLI modes and 17 embedded executions passed under Wasmtime 50 on macOS
ARM64, including actual Store destruction and replacement. The audit findings
and reproduction plan below describe the earlier snapshot. Broader WASI hosts
and Linux/Windows evidence remain separate work.

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

**Kind.** Deliberate accepted caller contract; normal CLI registry install
remains an external release prerequisite under RW-07.

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

**Accepted decision.** The SDK keeps the caller-explicit
`cleanup_failed` -> `CleanupOutcome::PreservedSubmitted` -> `resume` sequence as
the replacement for the source's automatic catch/cleanup/recovery. It is a
deliberate caller contract, not byte-for-byte automatic behavior: no destructive
cleanup runs from `Drop` or a broad error handler, and no ambiguous mutation is
replayed. `engine.rs:159-203` requires a known persisted `release_id`, refuses a
pending `Effect::Cleanup`, refuses an already acknowledged deletion or preserved
submission, persists `Effect::Cleanup` before the single call, and records
`deleted`/`preserved_submitted` only after an acknowledged outcome. `resume`
(`:130-157`) and `Snapshot::admit` reject pending effects before network work.
`hydrate.rs:80-124,373-417` binds the submitted session to the persisted release,
ingestion and publication identities; a preserved submission whose correlated
stage is not `Submitted` is invalid. On the preserved-submitted path, metadata
comes from the acknowledged cache, session or bundle (`:157-179`), so no upload
or store submit is issued. The exact path is
`tests/workflow/failures.rs::preserved_submitted_cleanup_reconciles_without_resubmitting_to_store`;
it is included in `publisher-workflow-durable-effects-and-cancellation` and now
executes on all five hosts in the v58/461 receipt. The decision is recorded in
[the workflow record](../parity/dapp-publisher-workflow.md).

**Normal CLI enablement.** The native binary is implemented and usable from a
checkout or archive (`cargo install --git`, prebuilt `publisher-cli-v*`). The
crate has `publish = false`; the book says it is not on crates.io because its SDK
dependencies are unpublished. That is a registry/release prerequisite, not a
missing CLI command. It is covered by RW-07.

**Verification.** The existing behavior is executable:

```sh
cargo test -p solana_kit_dapp_publisher --features workflow --test workflow --offline
```

Result at `79444fb8` and on this unit's rerun: 41 passed, 0 failed. The
preserved-submission test also asserts no `publication.submitToStore` procedure
and no upload observation. The source's automatic catch/cleanup/recovery path
remains intentionally unported and therefore has no Rust test; the accepted
caller path does.

**Next minimal unit.** None for the recovery decision; it is recorded above and
in the workflow record. CLI registry enablement stays under RW-07. Do not add
automatic destructive cleanup, a broad catch/replay, or a redundant production
wrapper.

### RW-07 Whole-workspace registry and package release rehearsal

**Kind.** Executed local rehearsal, with external publication prerequisites
still open.

**Acceptance criterion.** Phase 1 requires repeatable release preparation.
Phase 8 requires clean-consumer packaging and real release readiness; only
implemented crates may be published.

**Executed follow-up.** The [package rehearsal receipt](../verification/sdk-package-rehearsal-current.md)
records a successful disposable Monochange apply and lock refresh from the fresh
`8b1b342a` snapshot, 34 dependency-ordered archives, 34 fresh archive reviews over
1,287 members, and a 207-package offline registry. The downstream consumer passes
9 default and 12 selected-feature tests on current Rust and actual Rust 1.89,
plus a portable Wasm compile check. Default and selected PackageID/member audits
are separate; missing-archive and checksum controls use fresh namespaces. Wallet's
interior-space filenames and Token-2022's bounded GNU long-name records are now
ordinary fresh reviews, not form exceptions.

The rehearsal exposed a stale target-specific Browser-to-Wallet requirement
during Wallet's planned 0.2.0 bump. Browser now inherits the shared requirement,
and Wallet's typed release writer updates it before lock refresh. Repository
versions and publication settings stay unchanged; all version writes occur in
the disposable preparation.

The same local-registry installation exposed a hard-coded CLI version display.
The executable now reports its own Cargo package version, while the retained
library constant follows library metadata. A staged CLI 9.8.7/library 1.2.3 copy
passed the black-box version tests, and corrected 0.1.1 archives installed from a
fresh 207-package registry report `solana-kit-publisher 0.1.1`. In that
correction run only those two archives were rebuilt and reviewed; the fresh
34-archive run above requalifies all of them.

**Remaining limits.** LeptosKit's Git upstream is not a registry package. Twenty
included crates retain `publish = false`; the others stop at an offline
registry-API dry run. Actual registry versions, publication decisions and
authorization are still required. The corrected archives still exclude their
lockfiles, so installation resolves a fresh dependency graph instead of proving
an immutable published lock. This remains local macOS package evidence, not
Linux/Windows execution, publication or whole-SDK completion.

**Owning code.** `monochange.toml`, the shared Wallet dependency,
`xtask/src/package_review/archive.rs`, `xtask/src/package_rehearsal/`,
`xtask/src/registry_fixture.rs`, and `xtask/PACKAGE_REVIEW.md`.

**Verification.**

```sh
monochange check
monochange preview --format json
# Prepare only in an owned disposable source copy.
monochange prepare
cargo xtask package-rehearsal /absolute/path/to/rehearsal.json
```

**Next minimal unit.** None for local archive-form support; Wallet and
Token-2022 now pass the maintained gate as fresh reviews. Real registry API
acceptance and publication remain explicitly authorized release work; this
local gate does not enable them.

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

**Executed follow-up.** The [Subscriptions lifecycle receipt](../verification/subscriptions-program-current.md)
adds real subscribe/cancel/resume, self-CPI events, exact token delegation and
four confirmed unchanged-state rejections. The reviewed suite passes 19
integration cases: 18 program journeys and a two-validator isolation regression.
The gossip port is reserved explicitly. The original authority/plan-only finding
below is historical; broader delegation, transfer and other-program operations
remain open.

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

**Next minimal unit.** Extend a remaining pinned-program flow, such as
Subscriptions transfer/delegation or Squads vault proposal/execution. The
subscribe/cancel/resume unit is complete. Treat the missing Pyth receiver
artifact as an external blocker, not a code failure.

### RW-10 Fixed-point residual reconciliation

**Kind.** Reviewed partial adjudications plus deliberate presentation substitutions.
The implementation and executable evidence
already exist.

**Acceptance criterion.** Phase 2 exact numeric behavior and malformed/overflow
controls.

**Current finding.** `docs/parity/fixed-points-remaining.tsv` names five IDs;
their reason text now points at the executable evidence and
[the composition note](../parity/fixed-point-composition.md):

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

The old remaining TSV is no longer current gap evidence. This unit refreshed only
the five reason/notes texts in that file and the 126-entry receipt; the IDs
remain `remaining`, and counts, source hashes and status are unchanged. The
126-entry receipt keeps its historical statuses. The five canonical rows now
have checked partial [adjudications](../parity/numeric-adjudication.md); the
package/Phase 2 are not claimed complete.

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

Result at `79444fb8` and on this unit's rerun: 6 + 6 passed, 0 failed.

**Next minimal unit.** The five residual IDs are adjudicated as partial. Continue
family-by-family review under RW-01; no production codec or floating-point change
is required by this batch.

### RW-11 Numeric width and alphabet residuals

**Kind.** Reviewed partial adjudications with explicit bounds and range checks.
This item was corrected during this audit:
it is not a missing generic BigInt codec.

**Acceptance criterion.** Phase 2 exact integer/string codecs.

**Current finding.** `docs/parity/numeric-string-codecs-remaining.tsv` and the
88-entry scoped receipt previously said four `readBigInt*`/`writeBigInt*`
helpers lack arbitrary caller-selected widths. That reason is now stale: the
current implementation provides arbitrary nonzero widths, with stricter range
and cursor rules than the raw Dart helpers.
`crates/solana_kit_core/src/codecs/big_number.rs` exposes
`BigNumber<BigUint>` and `BigNumber<BigInt>` with an explicit nonzero byte width
and endian, including range/truncation/trailing checks. The
`core_parity/bigints.rs` tests exercise 2-, 3- and 33-byte widths and rejection
paths. The four canonical rows now have checked partial adjudications for
truncation, zero-width and offset differences, rather than a missing codec.

The three `getBaseXCodec`/`getBaseXEncoder`/`getBaseXDecoder` rows remain partial
for bounded execution and the explicit code-unit API. `Utf16BaseX` already
represents non-BMP alphabets and lone surrogates as `&[u16]`/`Vec<u16>`, alongside
the scalar-string `BaseX`. Encoding is capped at 4096 symbols and alphabets at
65536 entries; decoding uses the shared reader budget. The reference has no
equivalent bounds. All three canonical rows now have checked partial
[adjudications](../parity/numeric-adjudication.md).

**Owning code/source.** `crates/solana_kit_core/src/codecs/big_number.rs`,
`crates/solana_kit_core/tests/core_parity/bigints.rs`, the text-codec module,
pinned Dart `packages/solana_kit_codecs_numbers/lib/src/utils.dart:154-219` and
`packages/solana_kit_codecs_strings/lib/src/base_x.dart`.

**Verification.** The generic-width implementation executes with:

```sh
cargo test -p solana_kit_core --features bigint-codecs --test core_parity --offline
```

Result on this unit's rerun: 13 passed, 0 failed. The external consumer in
`numeric-string-codecs.md` already implements a checked three-byte numeric codec;
extend that pattern rather than adding another integer implementation.

**Next minimal unit.** These seven residual IDs are adjudicated as partial.
Continue family-by-family review under RW-01; no production codec change is
required unless a missing width/endian case is found.

### RW-12 Current documentation drift

**Kind.** Documentation/evidence. Phase 8 requires API and docs to agree.

**Executed follow-up.** This unit corrected the named current-looking claims in
one docs-only change:

- `leptoskit-http.md` now records installed local-wallet `Client<GuestHttp>`
  journeys and keeps external MWA/public-reflector interoperability separate.
- `design.md` no longer calls those installed journeys future work.
- `mobile-wallet-protocol.md` records local transport, Android handover and the
  wallet control surface as implemented, and links the later native reflector
  TLS receipt. External-wallet and public-reflector interoperability stay open.
- `dapp-publisher.md` links the v58/461 five-host receipt and keeps the
  cross-language signature oracle unexecuted.
- `subscriptions.md` links the four appended portable groups.
- `numeric-string-codecs.md` and both numeric scoped receipts describe the
  existing `BigNumber` widths (RW-11).
- `fixed-points-remaining.tsv` and `fixed-points.tsv` reasons point at the
  composition/presentation tests and note (RW-10).
- `docs/book/runtimes/wasi.md` was already corrected with the RW-05 receipt.

`docs/planning/tasks.md` remains an old checklist and is not a current status
authority. Historical counts and receipts stay labeled as historical.

**Current finding.** No named stale claim remains in this scoped set. Broader
documentation drift outside the list is separate work.

**Next minimal unit.** None for these files; future docs work should name the
specific stale claim and its current evidence.

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

| Finding                                               | Classification                                   | Action                                                            |
| ----------------------------------------------------- | ------------------------------------------------ | ----------------------------------------------------------------- |
| Automatic failed-release deletion                     | Deliberate safe substitution                     | Do not port as default; require an explicit caller action         |
| Automatic preserved-submission recovery               | Accepted caller-explicit contract                | Recorded decision; use cleanup/resume, no automatic recovery      |
| MWA local transport / Android handover / wallet UI    | Implemented; local-wallet evidence exists        | Keep local paths; RW-03 remains for external-wallet proof         |
| MWA remote reflector                                  | Native transport and TLS proof implemented       | Preserve RW-04 receipt; public interoperability stays separate    |
| Publication projections                               | Implemented; five-host proof recorded            | RW-02 closed; preserve the receipt                                |
| WASI public socket/clock                              | Implemented; named-host receipt recorded         | RW-05 closed for the named host; broader hosts remain open        |
| Linux/Windows                                         | CI jobs exist; no executed evidence              | RW-08                                                             |
| Registry/release                                      | Release prerequisite                             | RW-07; CLI code is not missing                                    |
| Fixed-point conversions and callback constructors     | Already implemented/evidenced; reasons refreshed | Adjudicate canonical rows; do not restore the Dart numerical bugs |
| Arbitrary-width BigInt codecs                         | Already implemented; reasons refreshed           | Adjudicate canonical rows; do not add another codec               |
| Non-BMP BaseX alphabets                               | Deliberate UTF-16 boundary                       | Keep the safe Rust behavior; record the difference                |
| Subscription pooling, coalescing, autoping, reconnect | Not required by the stated acceptance            | Keep caller-owned transport; do not add speculative machinery     |
| Mac2/XCTest                                           | Admin prerequisite                               | RW-13; do not retry sudo                                          |
| Historic `*-remaining.tsv` lists                      | Historical slice evidence                        | Do not count as current gaps; use canonical IDs for RW-01         |

## Achievable order

1. **RW-01a**: adjudication manifest plus the twelve-address pilot are complete.
   The error reviews add 365 complete name/integer rows and 35 complete plus 38
   partial contract rows. Continue with reviewed family-by-family adjudication
   under RW-01b.
2. **RW-02**: publication projections in the shared suite. Closed by the
   v58/461 five-host receipt.
3. **RW-05**: WASI socket/clock named-host receipt. Closed for its named host;
   broader WASI hosts remain open.
4. **RW-03**: external Android wallet acceptance. This is the largest genuine
   Phase 7 behavior proof still absent; it may reveal small implementation fixes.
5. **RW-04**: native reflector transport and owned TLS evidence are complete;
   public-reflector interoperability remains a separate acceptance boundary.
6. **RW-06**: publisher reconciliation decision. The caller-explicit
   cleanup/resume contract is accepted; CLI registry enablement stays in RW-07.
7. **RW-08**: Linux/Windows CI evidence; can run in parallel with the above.
8. **RW-07**: the local prepared-package rehearsal, all 34 fresh archive reviews
   and CLI version display are executed; address real-registry release
   prerequisites next.
9. **RW-09**: remaining program breadth. The twelve RW-10/RW-11 residual IDs
   have reviewed partial adjudications; broader numeric-family closure stays
   under RW-01.
10. **RW-12/RW-13**: the named RW-12 documentation claims are reconciled; the
    Mac2 admin prerequisite remains.
11. **RW-01b onward**: phase-ordered ledger closure, starting with Phase 2 then
    Phase 3, 4/5, 6, 7 and 8. Do not start with generated-program rows; they are
    the largest tranche and need the adjudication mechanism proven first.

This order is achievable because each step has an independent executable result.
It is not a promise that every item is small: declaration closure and external
wallet acceptance are long-horizon work.

## Next declaration unit

**RW-01b: bounded Phase 2 family adjudication.**

- Start with the fixed-point and numeric residual mappings above, one family at
  a time. Review the source contract, executable cases and documented
  substitutions before choosing a status. A receipt link alone cannot complete
  a declaration.
- Use the checked manifest introduced by the address pilot. Preserve every
  canonical ID, source hash and provenance field, and all corruption controls.
- Do not touch production SDK crates, root manifests/locks, workflows, shared
  conformance or fixtures merely to promote a status. A real uncovered contract
  becomes its own implementation or execution unit.
- Require successful extractor regeneration, unchanged source identity columns,
  justified case mappings, and rejection of unknown IDs, hash drift, missing
  evidence and invented cases. Package completion stays separate from a small
  declaration pilot.

The whole SDK remains incomplete after these bounded units. The completion claim
requires the remaining backlog plus a full release rehearsal.
