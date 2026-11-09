# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_associated_token_account [0.2.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_associated_token_account/v0.2.1) (2026-11-09)

### Changed

- **No package-specific changes were recorded; `solana_kit_associated_token_account` was updated to 0.2.1.**

## solana_kit_associated_token_account [0.2.0](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_associated_token_account/v0.2.0) (2026-11-08)

### Breaking changes

#### solana_kit_associated_token_account: ATA constructors

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #183](https://github.com/pina-rs/solana_kit/pull/183)

A handwritten `no_std + alloc` client for create, idempotent create, and nested
recovery instructions. Builders preserve every account position and duplicate;
canonical parsing checks the ATA identity, exact length, tags, account count, and
roles.

The optional `crypto` feature derives associated addresses through the Kit core
PDA implementation. System and token account identities remain caller-owned; the
crate does not perform account-existence checks, signing, or validator work.

## solana_kit_associated_token_account [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_associated_token_account/v0.1.1) (2026-11-05)

### Features

#### Add handwritten portable off-chain Memo and Associated Token Account clients.

_Owner:_ Ifiok Jr. · _Introduced in:_ [c2363aa](https://github.com/pina-rs/solana_kit/commit/c2363aa5c0085c31bbe059d5ffbb3f7effb2c81a)

Preserve raw UTF-8 payloads, historical memo extraction, ordered signer accounts,
all three ATA constructors and strict canonical parsing. Optional existing core
crypto derives associated addresses for caller-selected token identities.
Publication remains disabled. These clients do not expand the finite generator.
