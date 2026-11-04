---
solana_kit_native: minor
solana_kit_wallet: patch
---

# Connect to an explicit remote MWA reflector

Add a caller-owned two-stage remote association: connect once to an explicit
WSS reflector authority with selected TLS roots, validate its LEB128 reflector
ID, return a pending owner with the real remote association URI, then await the
wallet join before HELLO and the existing encrypted `NativeMobileWallet` path.
Offer only the pinned binary and base64 subprotocols, enforce one selected
protocol with no extensions, apply the encoded and decoded frame budgets, and
keep connection, join, operation and approval deadlines finite. No default
reflector, redirect, retry, background task or fallback transport is added.

Expose the existing remote reflector authority validation so a caller can reject
malformed authorities before opening a socket.
