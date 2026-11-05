# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_system [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_system/v0.1.1) (2026-11-05)

### Features

#### Add generated typed System instruction builders, canonical account parsers,

_Owner:_ Ifiok Jr. · _Introduced in:_ [65c1de6](https://github.com/pina-rs/solana_kit/commit/65c1de6e98920cdd57e48c8d28c4b34d7cfbb470)

lossless custom errors and bounded initialized/uninitialized nonce snapshots for
off-chain applications.

#### Add System instruction data codecs

_Owner:_ Ifiok Jr. · _Introduced in:_ [b74b8f8](https://github.com/pina-rs/solana_kit/commit/b74b8f893ba38c10cb208dfeadebb49ed7473d5e)

Provide bounded encode and strict decode for all fourteen pinned System
instruction-data layouts, including account creation with prefunding. Use exact
u64 amounts and u64 UTF-8 byte-length prefixes. This initial off-chain client
slice provides data codecs; account builders, nonce account decoding, PDAs and
program errors remain separate work. Publication remains disabled.
