# Capability worklist

This is a source inventory and implementation worklist for the immutable Dart
Solana Kit revision `0138bb956d2d81ccc0c56edc602401c0df2f2b46`. An inventoried
symbol is not implemented SDK behavior. The source-derived default is `pending`
except for one explicitly partial native role-merge mapping; reviewed rows in
[`adjudications.tsv`](adjudications.tsv) may replace only `status`, `evidence`
and `notes`. The reviewed rows adjudicate the twelve `solana_kit_address`
declarations in [`address-adjudication.md`](address-adjudication.md), twelve
numeric/fixed-point residual declarations in
[`numeric-adjudication.md`](numeric-adjudication.md), and the 365
`SolanaErrorCode` enum constants in
[`error-code-adjudication.md`](error-code-adjudication.md). None of those
reviews completes its package or the SDK.

The target is an off-chain client SDK for application code. Program clients build
and decode interactions with deployed programs. `no_std + alloc` supports portable
clients; it is not an on-chain support promise. Use Pina or official Solana program
libraries for on-chain programs.

`capabilities.tsv` records every extracted public declaration/member reached from
public Dart `lib/*.dart` and CLI `bin/*.dart` entrypoints, public lint configuration,
and the Codama renderer's `src/index.ts`. Members use `Type.member`; unnamed
constructors use `Type.new`. Getter/setter declarations share one stable ID and
retain both signature kinds. Definitions reexported through several barrels
appear once, at their defining package/path. IDs exclude source lines and kinds:

```text
dart:<package>:<package-relative-path>#<qualified-public-symbol>
ts:<package>:<package-relative-path>#<qualified-public-symbol>
```

`source-files.tsv` hashes all Dart, TypeScript and lint YAML under each top-level
package's `lib`, `bin`, or `src`, including nonexported implementation files and
generated files. Entrypoints show export provenance; defining line, SHA-256 and
immutable revision remain auditable. Compiler `target/` output is excluded at
all depths. Tests, examples, renderer template assets and private local variables
are not independent exported declarations. This inventory does not claim that
public method bodies or their behavioral cases have been exhaustively analyzed.

## Reproduce and check

Use a clean Git checkout of the exact reference pin. No secrets are required.
From the Rust repository root:

```sh
cargo run --manifest-path tools/capability_inventory/Cargo.toml --locked -- write /path/to/pinned-dart
cargo run --manifest-path tools/capability_inventory/Cargo.toml --locked -- check /path/to/pinned-dart
cargo run --manifest-path tools/capability_inventory/Cargo.toml --locked -- controls /path/to/pinned-dart
cargo test --manifest-path tools/capability_inventory/Cargo.toml --locked
```

`write` replaces only the three generated worklists. `check` regenerates them in
memory and rejects missing, extra, duplicate or altered rows, source hashes,
status/evidence claims, and delivery membership. Reviewed claims are loaded from
`adjudications.tsv` after extraction and may replace only `status`, `evidence`
and `notes` for the complete canonical ID they key. A claim must match the
defining source hash; a `complete` claim needs at least one exact case that is a
successful `<case>: Ok(())` entry in the maintained conformance success report.
The checker validates its complete case block, unique case names and full-success
footer. The conformance integrity test binds that report to the live case registry.
Other report formats require an explicit validator before they qualify. Unknown and
duplicate IDs, hash drift, absent or unsafe evidence paths, prose evidence,
blank or invented cases, malformed headers/field counts and unknown statuses all
fail. `controls` deletes a real extracted capability row and source-file row,
duplicates both, inserts an invented verified/evidence claim, and corrupts the
reviewed manifest in every way above; each must be rejected. Tests also reject
absent pilot evidence and a missing reference checkout. Adding real
implementation evidence requires an intentional reviewable manifest row.

The dedicated parity CI checks the actual pinned source, runs syntax and negative
fixtures, repeats the check deterministically, and enforces monostyle 0.3.27 with
generated source included. The native pilot artifact is retained execution
history, not a fresh execution by this extractor and not cross-runtime evidence.

## Source reconciliation

The manifests contain exactly **70 Dart packages plus one TypeScript renderer**.
Manifest package names and the exact top-level manifest set are checked against
`docs/planning/parity.tsv`, including packages without runtime libraries.

The RPC source `packages/solana_kit_rpc_api/lib/src/solana_rpc_api.dart` defines
53 literal all-cluster methods plus test-cluster `requestAirdrop`, for **54**.
These are catalog entries, while the separate RPC convenience extension provides
only 14 distinct methods and four parsed helpers. Their public list constants
are ledger declarations; the 54 catalog values below are required typed RPC
implementation work, not invented Dart methods:

```text
getAccountInfo getAgGenesisCert getBalance getBlock getBlockCommitment
getBlockHeight getBlockProduction getBlocks getBlocksWithLimit getBlockTime
getClusterNodes getEpochInfo getEpochSchedule getFeeForMessage getFirstAvailableBlock
getGenesisHash getHealth getHighestSnapshotSlot getIdentity getInflationGovernor
getInflationRate getInflationReward getLargestAccounts getLatestBlockhash
getLeaderSchedule getMaxRetransmitSlot getMaxShredInsertSlot
getMinimumBalanceForRentExemption getMultipleAccounts getProgramAccounts
getRecentPerformanceSamples getRecentPrioritizationFees getSignaturesForAddress
getSignatureStatuses getSlot getSlotLeader getSlotLeaders getStakeMinimumDelegation
getSupply getTokenAccountBalance getTokenAccountsByDelegate getTokenAccountsByOwner
getTokenLargestAccounts getTokenSupply getTransaction getTransactionCount
getTransactionsForAddress getVersion getVoteAccounts isBlockhashValid
minimumLedgerSlot requestAirdrop sendTransaction simulateTransaction
```

`packages/solana_kit_rpc_subscriptions_api/lib/src/solana_rpc_subscriptions_api.dart`
defines **nine** subscription families: stable `accountSubscribe`, `logsSubscribe`,
`programSubscribe`, `rootSubscribe`, `signatureSubscribe`, `slotSubscribe`; unstable
`blockSubscribe`, `slotsUpdatesSubscribe`, `voteSubscribe`. Notification and
unsubscribe name transformations belong to the same protocol families. They
need typed implementation and lifecycle evidence; catalog membership proves
neither client completeness nor runtime transport support.

The **17** source program clients are `solana_kit_address_lookup_table`,
`solana_kit_associated_token_account`, `solana_kit_attestation_service`,
`solana_kit_compute_budget`, `solana_kit_config`, `solana_kit_loader`,
`solana_kit_memo`, `solana_kit_mpl_bubblegum`, `solana_kit_mpl_core`,
`solana_kit_mpl_token_metadata`, `solana_kit_spl_account_compression`,
`solana_kit_squads`, `solana_kit_stake`, `solana_kit_subscriptions`,
`solana_kit_system`, `solana_kit_token`, `solana_kit_token_2022`. All have source
and exported declarations in the ledger. `solana_kit_subscriptions` is the
recurring-payments program. The generation script's current PROGRAMS list has
12 entries, so it must not be mistaken for the complete 17-client set. ATA is
explicitly handwritten; other generation inputs are audited in program-inputs.tsv.

## Coverage and limits

The parser uses pinned tree-sitter Dart and TypeScript grammars, not a line lexer.
Every syntax error, missing token, unfamiliar declaration node, unresolved
export, part directive and conditional export is recorded in source-files.tsv.
`parsed-no-unhandled-nodes` means the visited declaration syntax was recognized;
it is deliberately not named complete or behaviorally covered. Public fields,
methods, accessors, constructors, enum values, extension types, functions,
aliases, operators, and exported TypeScript interfaces are represented.

Two source diagnostics remain: mobile wallet adapter `intent_launcher.dart:1`
and `platform_check.dart:1` select IO/web implementations conditionally. Both
branches are inventoried as a union; selecting and proving the correct Rust host
behavior remains unresolved. Inherited members remain identified at their
exported defining declaration rather than being duplicated per subtype. Dart
implicit constructors, mixin resolution, extension applicability and externally
inherited APIs require semantic analysis beyond syntax traversal. The renderer's
default export aliases its named `renderVisitor` capability. Export alias/default
renaming outside that known source shape is not a promised general TS analyzer.

No source fixture or reference implementation was copied into this repository;
syntax tests are newly authored. Source hashes and locations are provenance,
not a license grant. Preserve applicable upstream licenses when later copying
fixtures or generator assets.

`delivery-units.md` gives bounded package tranches with exact ledger membership,
source size and dependencies. These refine implementation scope without claiming
that source symbol count predicts effort. Codec composition and generator reuse
should reduce repeated program work; host/device wallet work is the largest
unresolved integration boundary.
