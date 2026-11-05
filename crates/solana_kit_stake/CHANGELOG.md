# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_stake [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_stake/v0.1.1) (2026-11-05)

### Features

- **Add integer stake activation status.** Calculate effective, activating and deactivating stake with bounded borrowed history and integer V2 rates. Verify the public client against the published native interface without adding any Solana dependency to SDK application graphs.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [5c4b6f7](https://github.com/pina-rs/solana_kit/commit/5c4b6f79b93950c518e6ce7ba9f5208ba7fc5bcb)
- **Assess stake withdrawal from coherent account snapshots.** Add disjoint partial and full-close limits with history-based stake exposure, lockups and proposed authorities. Read Stake, Clock, Rent and History together through the standard RPC client with exact integer rent and cancellation propagation; do not use an unverified provider REST route.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [fd5c8c1](https://github.com/pina-rs/solana_kit/commit/fd5c8c1637bc08691bc707ba6329dfd93348d2e6)

#### Add portable off-chain Address Lookup Table, Config, Stake and Loader program

_Owner:_ Ifiok Jr. · _Introduced in:_ [846ed7f](https://github.com/pina-rs/solana_kit/commit/846ed7f0a23f4a01675a07c54919efc65b2a04ca)

builders and account codecs. Keep normal publication disabled while the SDK's
complete runtime and downstream-consumer requirements are verified.
