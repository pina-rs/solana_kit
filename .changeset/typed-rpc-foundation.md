---
solana_kit_client: minor
---

# Add executor-neutral typed RPC foundation

Call getBalance, getLatestBlockhash and full-base64 getAccountInfo through one
bounded, correlated JSON-RPC client. Preserve exact integers, account metadata,
explicit absence, detailed failures and concrete native or local adapter futures.
Provider methods reuse the same envelope through an open typed extension trait.
This off-chain client slice leaves the remaining 51 catalog methods and network
adapters pending. Component publication remains disabled.
