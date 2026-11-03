# Parity and status

The project tracks its progress against a pinned reference with machine-checked ledgers, not prose. This page covers what the reference is, how the ledgers are organized, what the status words mean, and where to look for the current state of an area. In short, a great deal is implemented and tested, and very little is marked complete. Status claims are kept conservative on purpose.

## The reference

The comparison baseline is the Dart Solana Kit at [`openbudgetfun/solana_kit`](https://github.com/openbudgetfun/solana_kit), pinned to commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46`. That project is itself modelled on the TypeScript `@solana/kit`, and its own upstream compatibility marker is `@solana/kit` 8.5.0. This Rust SDK does not compare itself to `@solana/kit` directly, and it makes no compatibility claim against it.

The pinned reference has 70 Dart packages plus one TypeScript Codama renderer: the core SDK, 17 program clients, six external integrations (Helius, Jupiter, Pyth, SNS, Surfpool and the dApp Store publisher), the wallet protocols, Flutter adapters and developer tooling. Every reference pin lives in [`docs/planning/references.toml`](https://github.com/pina-rs/solana_kit/blob/main/docs/planning/references.toml), including Wasm Solana, LeptosKit and Surfpool `v1.7.0`. Area-specific records also pin primary upstream sources where they matter, such as the Pyth crosschain, the Helius SDK and the Photon schema.

## The ledgers

| File | What it records |
| --- | --- |
| [`docs/planning/parity.tsv`](https://github.com/pina-rs/solana_kit/blob/main/docs/planning/parity.tsv) | One row per top-level reference package (71), with its owning Rust crate and phase. Every row is currently `inventoried`. |
| [`docs/parity/capabilities.tsv`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/capabilities.tsv) | Every public declaration extracted from the reference (17,948 rows), with a stable ID, owner, phase, status, evidence and delivery unit |
| [`docs/parity/source-files.tsv`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/source-files.tsv) | SHA-256 and parser coverage for every reference source file |
| [`docs/parity/delivery-units.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/delivery-units.md) | Source-order planning tranches of at most 200 IDs each |
| `docs/parity/<area>.md` and `.tsv` | Per-area records: what is mapped, how it was verified and what remains |
| [`docs/verification/`](https://github.com/pina-rs/solana_kit/tree/main/docs/verification) | Execution receipts: what ran, on which hosts, with which negative controls |
| [`docs/planning/decisions.tsv`](https://github.com/pina-rs/solana_kit/blob/main/docs/planning/decisions.tsv) | Timestamped decision trail |

Capability IDs have this form:

```text
dart:<package>:<package-relative-path>#<qualified-public-symbol>
ts:<package>:<package-relative-path>#<qualified-public-symbol>
```

The [capability worklist README](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/README.md) explains the extraction rules: tree-sitter parsing, how members and constructors are named, and what is deliberately excluded.

## What the status words mean

The central `capabilities.tsv` is a worklist. An inventoried symbol is not implemented behavior. Every row there is `pending` except one explicitly `partial` native role-merge mapping, and no row is marked verified or complete. The checker refuses any other status unless matching evidence comes with it.

The per-area ledgers use a richer vocabulary. The repository has no single glossary for these words. The meanings below summarize how the ledgers and their receipts use them:

| Status | Meaning |
| --- | --- |
| `pending` | In scope, with no accepted Rust equivalent yet. Source mapping alone does not count. |
| `partial` | A Rust equivalent exists for part of the behavior, or it exists but some required evidence (a host, a journey, an independent oracle) is still missing |
| `implemented` | The Rust equivalent exists, with behavioral tests recorded in the area's receipt. It is still not full verification across every promised runtime. |
| `implemented-bounded` | Implemented, with an explicit caller-supplied bound that the reference does not have (for example, fixed-point and numeric codec limits) |
| `implemented-native; runtime parity pending` | Executed natively, with other hosts still to be shown (Anchor) |
| `inventoried` | Package-level row only. Not a capability claim. |

The project's bar for "complete" is in [`docs/planning/plan.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/planning/plan.md). Every capability needs an implemented, behaviorally evidenced Rust equivalent, every public operation needs behavioral evidence, every promised runtime journey must run, and registry release preparation must succeed. A compiled placeholder, a skipped test or documentation alone cannot complete an entry.

> [!NOTE]
> When a crate README or receipt says something "remains required on CI" or is "unexecuted", treat that capability as unverified on those hosts. This book follows the same rule.

## Where things stand

A few area ledgers, as checked in:

| Ledger | Status counts |
| --- | --- |
| `rpc-node.tsv` | 98 implemented, 12 pending |
| `rpc-economics.tsv` | 39 implemented, 2 partial, 1 pending |
| `rpc-history-submission.tsv` | 130 implemented, 16 pending |
| `message-transactions.tsv` | 46 partial, 700 pending |
| `fixed-points.tsv` | 121 implemented-bounded, 5 remaining |
| `subscriptions.tsv` | 68 partial, 105 pending |
| `wallet-standard.tsv` | 203 partial |
| `anchor.tsv` | 120 implemented-native, runtime parity pending |
| `token.tsv`, `token-2022.tsv` | All pending in the source ledger. The runtime receipts record what executes. |

These counts change with every unit of work. Read the files themselves for the current numbers. The service pages ([Helius](../services/helius.md), [Jupiter](../services/jupiter.md), [Pyth](../services/pyth.md), [Surfpool](../services/surfpool.md), [dApp Store publisher](../services/dapp-publisher.md)) each list their own parity records.

## Reproduce the checks

The capability inventory tool lives at `tools/capability_inventory`. Run it against a clean checkout of the pinned Dart commit:

```sh
cargo run --manifest-path tools/capability_inventory/Cargo.toml --locked -- write /path/to/pinned-dart
cargo run --manifest-path tools/capability_inventory/Cargo.toml --locked -- check /path/to/pinned-dart
cargo run --manifest-path tools/capability_inventory/Cargo.toml --locked -- controls /path/to/pinned-dart
cargo test --manifest-path tools/capability_inventory/Cargo.toml --locked
```

`write` regenerates the three generated worklists. `check` regenerates them in memory and rejects missing, extra, duplicate or altered rows, as well as any status or evidence claim that has no support. `controls` deletes, duplicates and forges rows to prove that the checker catches each one. The `Capability inventory` workflow (`parity.yml`) runs these checks in CI against the real pinned source.

The package ledger has its own dependency-free checker, `tools/check_plan.rs`. Its instructions are in [`docs/planning/verification.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/planning/verification.md):

```sh
rustc +1.99.1 --edition=2024 -D warnings tools/check_plan.rs -o /tmp/solana-kit-check-plan
/tmp/solana-kit-check-plan /path/to/pinned/dart-reference
```

To learn how the receipts are produced, see [Verification](../contributing/verification.md).
