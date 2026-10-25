# Account Compression client

Build and parse all seven pinned SPL Account Compression instructions with Kit-owned
addresses, ordered account metadata and explicit caller limits. The crate also
provides all ten named header/event data codecs, nine program errors and checked
concurrent-tree allocation/canopy arithmetic. It uses `no_std + alloc` and depends
only on `solana_kit_core`.

Each instruction value exposes `instruction`, `instruction_with_program`, `parse`
and `parse_with_program`. Supply the ordered accounts explicitly; the
`modification_accounts` helper supplies the canonical Noop address for the three-role
modification family. Proof nodes are readonly non-signers appended after the fixed
roles for ReplaceLeaf, VerifyLeaf and InsertOrAppend. Builders reject privilege
escalation and wrong account counts. Addresses remain caller-owned; constructing an
instruction does not authenticate an existing tree or execute it.

```rust
use solana_kit_account_compression::{Append, modification_accounts};
use solana_kit_core::{Address, codecs::Limits};
let accounts = modification_accounts(Address::from_bytes([1; 32]), Address::from_bytes([2; 32]));
let instruction = Append { leaf: vec![7; 32] }.instruction(&accounts, Limits::new(256, 64))?;
# Ok::<(), solana_kit_core::codecs::Error>(())
```

This is exclusively an off-chain application client. For on-chain programs use Pina
or official Solana program libraries. No RPC, deployed-program transition or device
support is inferred from instruction construction. Source notices and intentional
wire/privilege dispositions are in `THIRD_PARTY_NOTICES.md`.
