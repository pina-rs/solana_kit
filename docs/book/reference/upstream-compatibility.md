# Upstream compatibility

This page records which upstream releases the Rust SDK is based on. The tables are rendered by [`mdt`](https://github.com/ifiokjr/mdt) from [`docs/planning/references.toml`](https://github.com/pina-rs/solana_kit/blob/main/docs/planning/references.toml) and [`docs/planning/upstream-versions.toml`](https://github.com/pina-rs/solana_kit/blob/main/docs/planning/upstream-versions.toml) through the providers in [`.templates/upstream.t.md`](https://github.com/pina-rs/solana_kit/blob/main/.templates/upstream.t.md). Do not edit the tables by hand: run `mdt update` after editing either input.

## `@solana/kit` marker

<!-- {=upstreamSummary|trim} -->

- Upstream `@solana/kit` version in the pinned reference: `8.5.0`
- Pinned Dart reference: [`openbudgetfun/solana_kit`](https://github.com/openbudgetfun/solana_kit) at [`0138bb956d2d`](https://github.com/openbudgetfun/solana_kit/tree/0138bb956d2d81ccc0c56edc602401c0df2f2b46)
- This Rust SDK ports that reference; it does not compare itself to `@solana/kit` directly and makes no direct compatibility claim.

<!-- {/upstreamSummary} -->

The Dart reference is the comparison baseline for every area ledger. The upstream marker travels with that reference; it is not a separate claim that this SDK matches `@solana/kit` method for method. The [parity and status](parity.md) page explains what the ledgers do and do not establish.

## Release parity

Each row pairs a `solana_kit` workspace version with the upstream `@solana/kit` version it is based on. The workspace version is the compatibility marker: component crates release independently, and the `solana_kit` umbrella crate remains a dependency-free `0.0.0` name reservation.

<!-- {=upstreamParity|trim} -->

| `solana_kit` workspace | `@solana/kit` | Pinned Dart commit | Released   |
| ---------------------- | ------------- | ------------------ | ---------- |
| `0.2.0`                | `8.5.0`       | `0138bb956d2d`     | 2026-11-08 |
| `0.1.1`                | `8.5.0`       | `0138bb956d2d`     | 2026-11-05 |

<!-- {/upstreamParity} -->

## Upstream references

The upstream repositories the workspace is built and verified against. Tag and commit pins are fixed; `used by` names the crate or crates that draw on the reference.

<!-- {=upstreamPins|trim} -->

| Upstream repository                                                           | Pin                       | Used by                                                        |
| ----------------------------------------------------------------------------- | ------------------------- | -------------------------------------------------------------- |
| [`anza-xyz/kit`](https://github.com/anza-xyz/kit)                             | `v8.5.0`                  | all crates (upstream marker)                                   |
| [`openbudgetfun/solana_kit`](https://github.com/openbudgetfun/solana_kit)     | `0138bb956d2d`            | all crates (comparison baseline)                               |
| [`pina-rs/wasm_solana`](https://github.com/pina-rs/wasm_solana)               | `6f43f957a069`            | `solana_kit_browser`, `solana_kit_native` (transport research) |
| [`leptoskit/leptoskit`](https://github.com/leptoskit/leptoskit)               | `2e575b1e7e81`            | `solana_kit_leptoskit` (runtime dependency)                    |
| [`solana-foundation/surfpool`](https://github.com/solana-foundation/surfpool) | `v1.7.0` (`9be726da6456`) | `solana_kit_surfpool` (external CLI research target)           |

<!-- {/upstreamPins} -->

## Reproduce the check

The templates read both TOML inputs through `[data]` in `mdt.toml`. From the repository root:

```sh
mdt update   # rewrite the tables in README.md and this page
mdt check    # fail when a consumer or formatter output is stale
```

The companion validator checks the invariants a template cannot express. It is dependency-free and compiles with the pinned toolchain:

```sh
rustfmt +1.99.1 --edition 2024 --check tools/check_upstream.rs
rustc +1.99.1 --edition=2024 -D warnings tools/check_upstream.rs -o /tmp/solana-kit-check-upstream
/tmp/solana-kit-check-upstream
```

It fails when the newest parity row disagrees with the pinned Dart commit or `@solana/kit` marker, when the recorded workspace version disagrees with `solana_kit_core`, when parity rows are out of order or carry invalid release labels, or when a pin in `references.toml` is missing from its `sources` list.
