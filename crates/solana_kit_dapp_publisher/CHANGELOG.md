# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_dapp_publisher [0.1.3](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_dapp_publisher/v0.1.3) (2026-11-09)

### Changed

- **No package-specific changes were recorded; `solana_kit_dapp_publisher` was updated to 0.1.3.**

## solana_kit_dapp_publisher [0.1.2](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_dapp_publisher/v0.1.2) (2026-11-08)

### Changed

- **No package-specific changes were recorded; `solana_kit_dapp_publisher` was updated to 0.1.2.**

## solana_kit_dapp_publisher [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_dapp_publisher/v0.1.1) (2026-11-05)

### Features

#### Add publisher attestation and prepared transaction policy

_Owner:_ Ifiok Jr. · _Introduced in:_ [4191d22](https://github.com/pina-rs/solana_kit/commit/4191d2288293aa00ccace309a74837cd6fa26f73)

Add portable publication attestation messages and sealed prepared-transaction
policy with explicit cryptographic verification. Keep signing, portal calls,
uploads and publication caller-owned. All runtime verification remains pending.
Reject conflicting bytes in an occupied caller signature slot while preserving
matching signatures and filling only empty slots.
Share all 22 original publisher assertion bodies with five portable conformance
groups without enabling host services or performing publication.

#### Add typed portal protocol and bounded ingestion retry

_Owner:_ Ifiok Jr. · _Introduced in:_ [733dbd2](https://github.com/pina-rs/solana_kit/commit/733dbd2e8ff315b549998b32c766a05d3397257c)

Add an optional runtime-neutral portal client with thirteen fixed typed
operations, raw backend DTOs and bounded duplicate-aware JSON admission. Keep
origin/authentication with the caller's transport, presigned binary uploads with
a separate caller-owned uploader, and signing/submission explicitly caller-owned.
Only explicit idempotent ingestion creation supports the bounded three-attempt
retry policy with caller clock/deadline. Normalized workflows, metadata/media and
native CLI remain future work. Preserve null envelope branch presence and let
the deadline win completion ties after polling and decoding. Twenty-three
synthetic protocol test bodies retain their predicates through four fallible
shared native/portable groups and a real failure-continuation unit control;
all new portal compiler/runtime verification is unexecuted by this source worker.

#### Add resumable ingestion and metadata hydration

_Owner:_ Ifiok Jr. · _Introduced in:_ [3c85abf](https://github.com/pina-rs/solana_kit/commit/3c85abf1ce4d593427df07135d8810f4a823d636)

Add an optional caller-driven ingestion/hydration coordinator with bounded
start/resume polling, atomic caller-owned checkpoints, pending-effect non-replay,
immutable local APK reads, typed 0.4.0 metadata and bounded media rehosting.
Reuse Kit hashing/codecs and one shared completion-time deadline boundary.
Preserve signer/policy separation and explicit single cleanup. This unit stops
before mint/save/verify/attest/store completion; native adapters and CLI remain
future work. Thirty-two synthetic test bodies are authored, with all new
compiler/runtime/portable-host verification unexecuted by this source worker.

#### Add deterministic normalized publication models, typed portal projections,

_Owner:_ Ifiok Jr. · _Introduced in:_ [4385f1b](https://github.com/pina-rs/solana_kit/commit/4385f1b11ecc67ca3b600339f46e80dc55e7264e)

checkpoint and status precedence, and progress messages that omit provider text.
Require caller-supplied timestamp fallbacks and retain unknown backend values for
explicit inspection. Fail publication identity generation when system entropy is
unavailable instead of using zero-filled identifiers.

### Fixes

#### Report the publisher CLI's own package version

_Owner:_ Ifiok Jr. · _Introduced in:_ [13e744f](https://github.com/pina-rs/solana_kit/commit/13e744f194dc31d85da9292b969b152dfd687fdb)

Make `--version` and the help heading use the executable's Cargo package version
instead of the publisher library's constant, and make the retained library
constant follow its own package metadata. The two packages can be released
independently, so the binary must not inherit a library version.
