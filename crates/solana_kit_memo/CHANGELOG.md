# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_memo [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_memo/v0.1.1) (2026-11-05)

### Features

#### Add handwritten portable off-chain Memo and Associated Token Account clients.

_Owner:_ Ifiok Jr. · _Introduced in:_ [c2363aa](https://github.com/pina-rs/solana_kit/commit/c2363aa5c0085c31bbe059d5ffbb3f7effb2c81a)

Preserve raw UTF-8 payloads, historical memo extraction, ordered signer accounts,
all three ATA constructors and strict canonical parsing. Optional existing core
crypto derives associated addresses for caller-selected token identities.
Publication remains disabled. These clients do not expand the finite generator.
