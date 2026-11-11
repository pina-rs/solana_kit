# Crates

This page lists every crate under [`crates/`](https://github.com/pina-rs/solana_kit/tree/main/crates). All of them are off-chain client code. For on-chain programs, use Pina or the official Solana program libraries. Each crate's README has the exact API surface, and the [parity ledgers](parity.md) record how much of the reference each one covers.

## Release status at a glance

- No SDK implementation crate has been released to crates.io. Every crate below except the umbrella is at workspace version `0.1.0` and is consumed as a git dependency (see [Installation](../getting-started/installation.md)).
- Most crate names are reserved on crates.io with `0.0.0` placeholders that contain no SDK APIs. `solana_kit_jupiter`, `solana_kit_sns`, `solana_kit_dapp_publisher` and `solana_kit_dapp_publisher_cli` have no crates.io entry.
- The `solana_kit` umbrella crate (`0.0.0`) is a published name reservation only. It has no SDK APIs and no dependencies. Do not add it expecting functionality.
- The only released artifact is the `solana-kit-publisher` CLI binary, `publisher-cli-v0.1.0` on GitHub Releases (see [dApp Store publisher](../services/dapp-publisher.md)).

The `no_std` column shows whether the crate root is `#![no_std]` (with `alloc`). `no_std + alloc` is what makes a crate portable across hosts. It does not mean the crate supports on-chain use.

## Foundation

| Crate                                                                                             | Purpose                                                                                                                                        | `no_std` | Notes                                                                        |
| ------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ---------------------------------------------------------------------------- |
| [`solana_kit`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit)                 | Umbrella name reservation                                                                                                                      | yes      | `0.0.0`, no SDK APIs                                                         |
| [`solana_kit_core`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_core)       | Addresses, hashes, signatures, instructions, bounded codecs, legacy/v0/v1 transactions, account snapshots, sysvars, deterministic cryptography | yes      | Features: `crypto`, `fixed-points`, `bigint-codecs`, `keccak`, `text-codecs` |
| [`solana_kit_client`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_client)   | Executor-neutral typed RPC, REST transport contract, subscriptions protocol, confirmation                                                      | no       | Features: `confirmation-factories`, `reactive`, `resource-estimation`        |
| [`solana_kit_signers`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_signers) | Asynchronous signer capabilities                                                                                                               | yes      | Feature: `crypto`                                                            |
| [`solana_kit_wallet`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_wallet)   | Wallet Standard application contracts                                                                                                          | yes      | Feature: `mwa`                                                               |

## Runtimes

| Crate                                                                                                 | Purpose                                                              | `no_std` | Host                                                                   |
| ----------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | -------- | ---------------------------------------------------------------------- |
| [`solana_kit_native`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_native)       | Bounded Tokio HTTP/TLS and WebSocket transports, plus a REST adapter | no       | Native OS. Features: `keys`, `mwa`, `surfpool`                         |
| [`solana_kit_browser`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_browser)     | Window Fetch and WebSocket transports                                | no       | `wasm32-unknown-unknown`. Features: `keys`, `wallet`                   |
| [`solana_kit_wasi`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_wasi)           | Caller-owned WASI Preview 2 transports                               | no       | `wasm32-wasip2`. Feature: `websocket`                                  |
| [`solana_kit_leptoskit`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_leptoskit) | LeptosKit transports and optional wallet controls                    | no       | Interpreted Wasm guest and host. Features: `guest`, `host`, `wallet-*` |

See [Choosing a runtime](../runtimes/index.md).

## Program clients

| Crate                                                                                                                               | Purpose                                                                 | `no_std` | Notes                                               |
| ----------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- | -------- | --------------------------------------------------- |
| [`solana_kit_system`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_system)                                     | System builders, parsers and nonce codecs                               | yes      | [System](../programs/system.md)                     |
| [`solana_kit_token`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_token)                                       | SPL Token codecs and instruction builders                               | yes      | [Token](../programs/token.md)                       |
| [`solana_kit_token_2022`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_token_2022)                             | Token-2022 codecs and instruction clients                               | yes      | Feature: `client`                                   |
| [`solana_kit_compute_budget`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_compute_budget)                     | Compute Budget instruction client                                       | yes      | [Small programs](../programs/small-programs.md)     |
| [`solana_kit_memo`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_memo)                                         | Memo instruction client                                                 | yes      | Handwritten                                         |
| [`solana_kit_associated_token_account`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_associated_token_account) | Associated Token Account instructions                                   | yes      | Feature: `crypto` (PDA derivation)                  |
| [`solana_kit_stake`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_stake)                                       | Stake program codecs and builders                                       | yes      | [Runtime programs](../programs/runtime-programs.md) |
| [`solana_kit_loader`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_loader)                                     | Loader v3 and v4 codecs and builders                                    | yes      |                                                     |
| [`solana_kit_config`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_config)                                     | Config program clients                                                  | yes      |                                                     |
| [`solana_kit_address_lookup_table`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_address_lookup_table)         | Address lookup table program clients                                    | yes      | Feature: `pda`                                      |
| [`solana_kit_account_compression`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_account_compression)           | Account Compression instructions and events                             | yes      |                                                     |
| [`solana_kit_mpl_token_metadata`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_mpl_token_metadata)             | MPL Token Metadata client                                               | yes      | Feature: `pda`. [Metaplex](../programs/metaplex.md) |
| [`solana_kit_mpl_core`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_mpl_core)                                 | MPL Core instructions, accounts and plugins                             | yes      | Feature: `pda`                                      |
| [`solana_kit_mpl_bubblegum`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_mpl_bubblegum)                       | Bubblegum compressed NFT client                                         | yes      | Features: `pda`, `tree`, `hashing`, `das`           |
| [`solana_kit_attestation_service`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_attestation_service)           | Solana Attestation Service client                                       | yes      | Feature: `pda`                                      |
| [`solana_kit_squads`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_squads)                                     | Squads V4 multisig clients                                              | yes      | Feature: `pda`                                      |
| [`solana_kit_subscriptions`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_subscriptions)                       | Recurring-payments Subscriptions program client (not RPC subscriptions) | yes      | Feature: `pda`                                      |
| [`solana_kit_sns`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_sns)                                           | Offline Solana Name Service primitives                                  | yes      |                                                     |
| [`solana_kit_anchor`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_anchor)                                     | Bounded Anchor IDL, Borsh, events and errors                            | yes      | [Anchor](../programs/anchor.md)                     |

## Services and tools

| Crate                                                                                                                   | Purpose                                                                  | `no_std` | Notes                                                                                     |
| ----------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ | -------- | ----------------------------------------------------------------------------------------- |
| [`solana_kit_helius`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_helius)                         | Typed Helius APIs                                                        | no       | Features: `crypto`, `checkout`, `signup`. [Helius](../services/helius.md)                 |
| [`solana_kit_privacy_rings`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_privacy_rings)           | Privacy Rings instructions, viewing-key decrypt, indexer RPC, and a remote prover | no       | Feature: `crypto`. [Privacy Rings](../services/privacy-rings.md)                          |
| [`solana_kit_jupiter`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_jupiter)                       | Jupiter swap, price and token clients                                    | no       | [Jupiter](../services/jupiter.md)                                                         |
| [`solana_kit_pyth`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_pyth)                             | Pyth wire, accounts, Hermes and receiver instructions                    | yes      | Features: `crypto`, `hermes`. [Pyth](../services/pyth.md)                                 |
| [`solana_kit_surfpool`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_surfpool)                     | Typed Surfpool cheatcodes                                                | no       | Feature: `crypto`. [Surfpool](../services/surfpool.md)                                    |
| [`solana_kit_dapp_publisher`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_dapp_publisher)         | dApp Store attestation, transaction policy, portal protocol and workflow | no       | Features: `crypto`, `cli`, `portal`, `workflow`                                           |
| [`solana_kit_dapp_publisher_cli`](https://github.com/pina-rs/solana_kit/tree/main/crates/solana_kit_dapp_publisher_cli) | Native `solana-kit-publisher` binary                                     | no       | Released as `publisher-cli-v0.1.0`. [dApp Store publisher](../services/dapp-publisher.md) |

## Outside `crates/`

The workspace also holds non-SDK members. These are covered in [Contributing](../contributing/index.md) and [Verification](../contributing/verification.md):

- `xtask`: the `cargo xtask` verification commands.
- `verification/*`: private conformance, runtime and boundary harnesses.
- `tools/`: the capability inventory, the Codama renderer and plan checkers.
- `packages/codama-renderers-rust-solana-kit`: an NPM Codama renderer that generates Kit-based Rust program clients.
- `docs/site`: this documentation site.
