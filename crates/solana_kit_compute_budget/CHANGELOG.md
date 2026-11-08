# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_compute_budget [0.2.0](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_compute_budget/v0.2.0) (2026-11-08)

### Breaking changes

#### solana_kit_compute_budget: typed compute-budget instructions

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #183](https://github.com/pina-rs/solana_kit/pull/183)

Strict bounded codecs, owned Kit instruction builders, parsers, and instruction
list helpers for all five pinned Compute Budget layouts. Every builder returns a
normal `solana_kit_core::Instruction` and every parser enforces the exact wire
shape.

`RequestUnits` remains available as the deprecated historical layout, but
building its bytes does not imply validator acceptance. Heap alignment and
network limits remain validator policy.

## solana_kit_compute_budget [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_compute_budget/v0.1.1) (2026-11-05)

### Features

#### Add portable off-chain Compute Budget data codecs, canonical instruction builders

_Owner:_ Ifiok Jr. · _Introduced in:_ [3f4ad31](https://github.com/pina-rs/solana_kit/commit/3f4ad31f4feff024feac271c79bea9111fc99b48)

and parsers, plus instruction-list update helpers with exact unsigned prices.

Discovery rejects unknown qualifying Compute Budget tags before a selected match
and tolerates trailing limit and price bytes, matching the pinned Dart helpers.
The unreleased limit lookup now returns a Result so callers can handle malformed
tags. Canonical whole-instruction parsing remains strict.
