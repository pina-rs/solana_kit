# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_anchor [0.2.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_anchor/v0.2.1) (2026-11-09)

### Changed

- **No package-specific changes were recorded; `solana_kit_anchor` was updated to 0.2.1.**

## solana_kit_anchor [0.2.0](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_anchor/v0.2.0) (2026-11-08)

### Breaking changes

#### solana_kit_anchor: runtime Anchor IDL coding

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #183](https://github.com/pina-rs/solana_kit/pull/183)

Encode and decode Anchor IDL 0.30 instructions, accounts, and events at runtime.
Exact-width dynamic values, explicit IDL discriminators, field-attributed encode
failures, and standard or program-defined error resolution are supported.

The only dependency graph is `no_std + alloc` and it starts no executor or
network work. Every operation takes explicit byte, item, and depth budgets, and
an unsupported generic field fails only when used rather than making the whole
IDL unreadable.

## solana_kit_anchor [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_anchor/v0.1.1) (2026-11-05)

### Features

#### Add a portable, bounded Anchor runtime with typed IDL 0.30 documents, all pinned

_Owner:_ Ifiok Jr. · _Introduced in:_ [785028d](https://github.com/pina-rs/solana_kit/commit/785028d7d67763b73a13392104eba1061ad5b786)

Borsh scalars and containers, exact discriminator prefixes, complete-log event
provenance checks and standard/custom error resolution. Keep application-owned
transport, account metadata and transaction composition in existing Kit APIs.

### Fixes

- **Clarify client validation and verification flows while preserving their wire contracts.**
  _Owner:_ Ifiok Jr. · _Introduced in:_ [af19680](https://github.com/pina-rs/solana_kit/commit/af196807568fa0fc5a34a89f074f39f0453dc475)
