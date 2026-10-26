---
solana_kit_helius: minor
---

Add optional unified signup with caller-owned JWT binding, time and transport.
Reuse existing project keys or create one key, and prepare hosted checkout for new
subscriptions without signing or sending payments. Validate encoded signing keys,
billing windows and payment links. Propagate preview failures and reject zero totals.
Keep HTTP and date parsing dependencies behind the signup feature.
