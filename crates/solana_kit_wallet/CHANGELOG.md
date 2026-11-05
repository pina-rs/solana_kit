# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_wallet [0.2.0](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_wallet/v0.2.0) (2026-11-05)

### Breaking changes

#### Add owned browser Wallet Standard discovery, account authorization and Kit signing capabilities.

_Owner:_ Ifiok Jr. · _Introduced in:_ [eb1e6ac](https://github.com/pina-rs/solana_kit/commit/eb1e6ac886f2693c19894268efb6cf88d9d8c97a)

Wallet metadata methods now return owned snapshots so changing backend state never requires an invalid borrowed reference.
Signer capabilities and local/native transaction compositions can preserve a caller-selected failure type while existing default errors retain their native Send behavior.
Browser failures retain their original JavaScript causes behind redacted diagnostics.

### Features

#### Add optional portable Mobile Wallet Adapter association, cryptography, handshake,

_Owner:_ Ifiok Jr. · _Introduced in:_ [2796281](https://github.com/pina-rs/solana_kit/commit/2796281284959ebbf88ca4810563f751f0cd8412)

ordered encryption, JSON-RPC wallet contracts, and SIWS messages with independent
Dart compatibility vectors. Applications supply private scalars, fresh IVs,
transports, and platform handover. The default Wallet Standard graph is unchanged.

#### Connect to an explicit remote MWA reflector

_Owner:_ Ifiok Jr. · _Introduced in:_ [028b8b8](https://github.com/pina-rs/solana_kit/commit/028b8b862b4a41c5c955dff19a00c21b218fe5ed)

Add a caller-owned two-stage remote association: connect once to an explicit
WSS reflector authority with selected TLS roots, validate its LEB128 reflector
ID, return a pending owner with the real remote association URI, then await the
wallet join before HELLO and the existing encrypted `NativeMobileWallet` path.
Offer only the pinned binary and base64 subprotocols, enforce one selected
protocol with no extensions, apply the encoded and decoded frame budgets, and
keep connection, join, operation and approval deadlines finite. No default
reflector, redirect, retry, background task or fallback transport is added.

Expose the existing remote reflector authority validation so a caller can reject
malformed authorities before opening a socket.

- **Add optional wallet view models, browser controller projections, Android Intent.** and entropy assets, and a caller-owned native binary Mobile Wallet Adapter.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [eb1e6ac](https://github.com/pina-rs/solana_kit/commit/eb1e6ac886f2693c19894268efb6cf88d9d8c97a)
#### Add portable off-chain Wallet Standard account and icon metadata, concrete-future

_Owner:_ Ifiok Jr. · _Introduced in:_ [35979c6](https://github.com/pina-rs/solana_kit/commit/35979c616ef1a88b319f04ca9e2d0c138e06a444)

capability traits, and a caller-owned passive registry. Platform discovery,
authorization UI, and mobile wallet bridges remain pending.

Revalidate registration name decisions after reentrant membership or property
changes, deduplicate Rc identity independently, and return a typed error if all
32 observation passes are invalidated. Getters, callbacks, and capture destruction
remain outside registry borrows.

### Fixes

- **Restore the optional MWA feature's no_std + alloc support with dependency-free.** URI validation, preserving strict association URI and reflector authority rules.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [2796281](https://github.com/pina-rs/solana_kit/commit/2796281284959ebbf88ca4810563f751f0cd8412)
#### Correct SIWS JSON interoperability with MWA wallets by encoding and decoding

_Owner:_ Ifiok Jr. · _Introduced in:_ [2796281](https://github.com/pina-rs/solana_kit/commit/2796281284959ebbf88ca4810563f751f0cd8412)

`chainId`, `issuedAt`, `expirationTime`, `notBefore`, and `requestId` in camelCase.
Rust field names and the surrounding MWA envelope keep their existing names.

- **Execute the portable wallet contracts and corrected reentrant registry through.** the shared native, browser, WASI and interpreted guest conformance suite.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [35979c6](https://github.com/pina-rs/solana_kit/commit/35979c616ef1a88b319f04ca9e2d0c138e06a444)
