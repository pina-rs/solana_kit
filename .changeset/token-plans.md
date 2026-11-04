---
solana_kit_token: minor
---

Add the pinned handwritten Token plan helpers behind an optional `plans`
feature: indivisible create-mint planning, idempotent ATA creation followed by
`MintToChecked` and `TransferChecked`, and synchronous associated-address
derivation variants. Preserve source field names, defaults and program override
precedence while keeping the default Token-to-core dependency closure unchanged.
