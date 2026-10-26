# Squads V4 application client

Construct Squads V4 multisig/config/vault/proposal instructions, decode already
fetched account snapshots and derive the eight pinned PDA recipes using Kit-owned
addresses, instruction metadata and bounded codecs. The default graph is core
plus base58; the optional `pda` feature enables core's deterministic cryptography.
There are no HTTP, executor, JavaScript or Solana validator/client dependencies.

This is exclusively an off-chain SDK for application code. Use Pina or official
Solana program libraries for on-chain programs. Portable `no_std + alloc` support
does not imply on-chain support.

Each generated instruction value provides `instruction`, `instruction_with_program`,
`parse` and `parse_with_program`. Account inputs retain source order and duplicates.
Optional roles use the explicit program-ID readonly sentinel. No signer is invented,
no transaction is submitted and no RPC or clock is invoked by a builder. Execution
instructions preserve individually privileged remaining accounts supplied by the
caller; their runtime validity still depends on the stored transaction/actions.

For example, construct and parse a program-authority change with explicit signers:

```rust
use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::Limits};
use solana_kit_squads::{PROGRAM, ProgramConfigSetAuthority};

let value = ProgramConfigSetAuthority { new_authority: Address::from_bytes([7; 32]) };
let roles = [
    AccountMeta { address: Address::from_bytes([8; 32]), role: AccountRole::Writable },
    AccountMeta { address: Address::from_bytes([9; 32]), role: AccountRole::ReadonlySigner },
];
let limits = Limits::new(4096, 256);
let instruction = value.instruction(&roles, limits)?;
assert_eq!(instruction.program_address, PROGRAM);
assert_eq!(ProgramConfigSetAuthority::parse(&instruction, limits)?, value);
# Ok::<(), solana_kit_core::codecs::Error>(())
```

Typed snapshot decoders preserve owner, allocation, lamports, rent epoch and other
core account metadata. They validate the complete eight-byte account discriminator
and payload, including strict trailing-byte policy. `identify_account` only identifies
a header. `validate_vault_message` checks header partitions, indexes and explicit
size/item bounds; it does not authenticate lookup-table or bank state.

Enable `pda` for `find_program_config`, `find_multisig`, `find_vault`,
`find_transaction`, `find_proposal`, `find_batch_transaction`,
`find_ephemeral_signer` and `find_spending_limit`. Every helper has a matching
`_with_program` variant. Vault/signer indexes are `u8`, transaction/batch indexes
are full-width little-endian `u64`, and the transaction index inside a batch is
little-endian `u32`.

The source tables are original Rust normalization of the pinned MIT Dart client,
retaining its notice. Generation uses the existing shared typed renderer through
`solana_kit_codegen squads-generate OUTPUT`. See
`THIRD_PARTY_NOTICES.md` for pins and exact source differences.

The native suite executes 48 original behavior owners and the public example.
The same bodies are imported by the shared portable registry, including all 36
literal instruction cases. A read-only external oracle checks the complete pinned
primary model without distributing its source. Declaration-level parity,
validator transitions and installed-device journeys require separate evidence.
