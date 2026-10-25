---
solana_kit_wallet: major
solana_kit_browser: minor
solana_kit_signers: minor
---

Add owned browser Wallet Standard discovery, account authorization and Kit signing capabilities.
Wallet metadata methods now return owned snapshots so changing backend state never requires an invalid borrowed reference.
Signer capabilities and local/native transaction compositions can preserve a caller-selected failure type while existing default errors retain their native Send behavior.
Browser failures retain their original JavaScript causes behind redacted diagnostics.
