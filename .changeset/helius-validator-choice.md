---
solana_kit_helius: minor
---

# Choose the validator for local staking preparation

Add `StakeRequest::validator_vote` for caller-selected validator vote accounts.
Use `None` in existing request literals to retain the Helius validator default.
Instruction and transaction preparation preserve the selection with the same
RPC reads and required owner and new stake-account signatures.
