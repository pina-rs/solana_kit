# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_surfpool [0.2.0](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_surfpool/v0.2.0) (2026-11-08)

### Breaking changes

#### solana_kit_surfpool: typed Surfpool cheatcodes

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #183](https://github.com/pina-rs/solana_kit/pull/183)

All 28 methods from the pinned Surfpool package are represented by public request
types and run through `solana_kit_client::Client`. Prepare local account state,
control the test clock, query confidential balances, profile transactions, and
upload caller-provided program bytes.

The client uses the transport, correlation IDs, admission limits, and byte and
depth budgets of the chosen host. It starts no executor, adds no retry loop, and
remains independent of the Surfpool server version.

## solana_kit_surfpool [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_surfpool/v0.1.1) (2026-11-05)

### Features

#### Add all 28 pinned Surfpool cheatcode request types through Kit's shared RPC

_Owner:_ Ifiok Jr. · _Introduced in:_ [8db3aef](https://github.com/pina-rs/solana_kit/commit/8db3aef7c7d55ea611ba16386247954d5681e7f4) · _Last updated in:_ [3c94d14](https://github.com/pina-rs/solana_kit/commit/3c94d144425e754fbd8ad9363eef5f96bcc1e722)

client, with exact integers, contextual responses, account builders, typed
errors, startup configuration and sequential deployment from caller bytes.
Keep process lifecycle, endpoint adapters, randomness and file discovery explicit
host responsibilities. Disable normal publication pending runtime verification.

Stream hexadecimal JSON into the bounded shared writer and borrow deployment
chunks without changing their configured size, wire fields, offsets or typed
failures. Preserve explicit original errors while redacting ordinary diagnostics.
Default writes use 3 KiB raw chunks to fit the client's complete 8 KiB wire
budget. Two local Surfpool 1.7.0 runs observed the selected native journey;
a later strict oversized-request replay failed and its typed cause remains
undiagnosed. Current-source CI remains required independently. Public native
process ownership remains a separate host layer.

### Fixes

#### Add optional native Surfpool deployment from borrowed program bytes or an explicit

_Owner:_ Ifiok Jr. · _Introduced in:_ [ce133c6](https://github.com/pina-rs/solana_kit/commit/ce133c6e76908d8843bde2e16c337394ca68b009)

regular file path, with finite caller-selected read limits. Reuse sequential portable
program writes before opening the optional IDL file, preserve exact JSON fields, and
retain partial writes and typed causes on later failures or cancellation.
Preserve literal unknown JSON keys, including serde_json's internal token names,
without changing the parser's existing nesting ceiling.
Allow the permanent portable crate in SDK dependency manifests while keeping
normal implementation publishing disabled in Monochange configuration.

#### Use default deployment chunks that fit the client wire budget and Surfpool

_Owner:_ Ifiok Jr. · _Introduced in:_ [3c94d14](https://github.com/pina-rs/solana_kit/commit/3c94d144425e754fbd8ad9363eef5f96bcc1e722)

1.7.0's HTTP body cap. Preserve exact caller-selected chunks and typed budget
errors without silently resizing writes.
