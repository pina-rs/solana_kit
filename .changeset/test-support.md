---
solana_kit_test_support: minor
---

# Add deterministic client test support

Add an unpublished dev-dependency crate with finite scripted transports, exact
ordered request capture, correlated success and remote-error replies, typed
transport failures, observable pending wake/completion/cancellation, and small
balance/account/blockhash result fixtures. An optional deterministic keypair
fixture reuses core crypto and stays off by default. No SDK runtime graph,
canonical ledger, production client, CI or other documentation changes.
