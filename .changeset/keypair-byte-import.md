---
"solana_kit_core": minor
---

Import 64-byte Ed25519 keypairs through the existing optional cryptographic
owner, rejecting incorrect lengths and public keys that do not match the seed.
Keep caller buffers unchanged and retain zeroizing ownership without secret
export. Component publication remains disabled in monochange.toml.
