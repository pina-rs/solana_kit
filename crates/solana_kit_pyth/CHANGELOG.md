# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_pyth [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_pyth/v0.1.1) (2026-11-05)

### Features

- **Add portable Pyth wire parsing.** Parse bounded borrowed Wormhole, AUWV, price and accumulator messages. Preserve upstream-compatible extensions and explicit signature-reduction contracts. Hermes and remaining Pyth clients remain separate implementation work.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [bbfc983](https://github.com/pina-rs/solana_kit/commit/bbfc983c6aa5c9b31f410c3ea6fb97a171c1c8f6)
- **Decode legacy Pyth and receiver price accounts.**
  _Owner:_ Ifiok Jr. · _Introduced in:_ [c739680](https://github.com/pina-rs/solana_kit/commit/c739680ce4a1f7a59ce2054fa5f6fbbb44f59cf5)
- **Decode exact Hermes JSON response models.** Optional Hermes models preserve arbitrary-precision components and publication metadata. Bounded decoders enforce shared JSON budgets and reject duplicate keys without adding a runtime to the default wire client.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [9168b31](https://github.com/pina-rs/solana_kit/commit/9168b31dbcc5a0a53534b48cc0f38f52a06834b7)
- **Execute portable Hermes GET requests with bounded retries.** Add feed listing, latest updates and signed-timestamp updates over existing caller-owned REST adapters and clocks. Preserve ordered parameters, bounded response bodies, finite exponential backoff, per-attempt timeout and cancellation ownership.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [ff0cb84](https://github.com/pina-rs/solana_kit/commit/ff0cb841db41b9844e69d661908c485537ca00ec)
- **Build receiver posting instructions and derive Pyth PDAs.** Encode bounded Borsh parameters and exact seven-account posting instructions using Kit-owned types. Reuse optional Core crypto for config, treasury and guardian-set derivation, retain custom program/authority support, and select treasury bytes through caller-owned entropy.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [f4bdbef](https://github.com/pina-rs/solana_kit/commit/f4bdbef95a920cc72032e9543284eaeb05e0da4f)
