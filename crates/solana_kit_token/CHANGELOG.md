# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_token [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_token/v0.1.1) (2026-11-05)

### Features

- **Add generated off-chain Token instruction and account codecs, explicit ordered.** account builders, strict parsers, custom identities and lossless program errors.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [bd424a9](https://github.com/pina-rs/solana_kit/commit/bd424a9992c56f4232abd0cfe8f36e1d74fd387c)
#### Add the pinned handwritten Token plan helpers behind an optional `plans`

_Owner:_ Ifiok Jr. · _Introduced in:_ [ccf7c60](https://github.com/pina-rs/solana_kit/commit/ccf7c604466ad8ebaa2659301c25e7473440634c)

feature: indivisible create-mint planning, idempotent ATA creation followed by
`MintToChecked` and `TransferChecked`, and synchronous associated-address
derivation variants. Preserve source field names, defaults and program override
precedence while keeping the default Token-to-core dependency closure unchanged.

### Fixes

#### Preserve Token wire behavior while emitting enum codecs through a shared typed

_Owner:_ Ifiok Jr. · _Introduced in:_ [bd424a9](https://github.com/pina-rs/solana_kit/commit/bd424a9992c56f4232abd0cfe8f36e1d74fd387c)

variant table. Verify all generated corpus files, including full 256-tag domains,
with pinned monostyle in the private generator workflow.
