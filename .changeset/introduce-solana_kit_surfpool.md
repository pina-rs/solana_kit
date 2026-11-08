---
solana_kit_surfpool: major
---

# solana_kit_surfpool: typed Surfpool cheatcodes

All 28 methods from the pinned Surfpool package are represented by public request
types and run through `solana_kit_client::Client`. Prepare local account state,
control the test clock, query confidential balances, profile transactions, and
upload caller-provided program bytes.

The client uses the transport, correlation IDs, admission limits, and byte and
depth budgets of the chosen host. It starts no executor, adds no retry loop, and
remains independent of the Surfpool server version.
