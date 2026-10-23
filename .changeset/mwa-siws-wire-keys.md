---
"solana_kit_wallet": patch
---

Correct SIWS JSON interoperability with MWA wallets by encoding and decoding
`chainId`, `issuedAt`, `expirationTime`, `notBefore`, and `requestId` in camelCase.
Rust field names and the surrounding MWA envelope keep their existing names.
