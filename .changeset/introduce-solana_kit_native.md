---
solana_kit_native: major
---

# solana_kit_native: Tokio HTTP, TLS, and WebSocket transports

Native HTTP/1 and text WebSocket transports for off-chain Solana applications.
`NativeHttp` implements the public client transport with caller-owned Tokio
execution, explicit deadlines, cancellation, authentication, and verified TLS.

Optional key support adds explicit exportable-key generation and intentional
Solana-keygen JSON backup with atomic publication and typed cleanup failures.
The optional Mobile Wallet Adapter transport keeps association, encryption, and
cancellation caller-owned.
