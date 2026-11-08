---
solana_kit_associated_token_account: major
---

# solana_kit_associated_token_account: ATA constructors

A handwritten `no_std + alloc` client for create, idempotent create, and nested
recovery instructions. Builders preserve every account position and duplicate;
canonical parsing checks the ATA identity, exact length, tags, account count, and
roles.

The optional `crypto` feature derives associated addresses through the Kit core
PDA implementation. System and token account identities remain caller-owned; the
crate does not perform account-existence checks, signing, or validator work.
