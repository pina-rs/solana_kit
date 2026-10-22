# solana_kit_system

Portable System Program instruction data, typed account builders, strict parsers,
custom errors and nonce account snapshots. This crate is exclusively for off-chain
client applications. Do not use it in on-chain programs; use Pina or official
Solana program libraries there. `no_std + alloc` supports portable clients.

All fourteen data codecs share one validated schema algorithm. Each data type's
`instruction(accounts, limits)` constructs a fixed System Program invocation using
its generated typed accounts. Fixed Rent and RecentBlockhashes sysvars are
supplied automatically. Ordered duplicate accounts stay intact. Optional payer
and signing-base accounts are explicit. Seeded builders check UTF-8 byte length
and base signatures; callers supply the derived destination address. Address
hashing is available separately through optional Kit core cryptography.

```rust
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_system::{TransferSol, TransferSolAccounts};
# fn main() -> Result<(), solana_kit_system::ClientError> {
let instruction = TransferSol { amount: 500 }.instruction(
    TransferSolAccounts {
        source: Address::from_bytes([1; 32]),
        destination: Address::from_bytes([2; 32]),
    },
    Limits::new(1232, 32),
)?;
let (data, accounts) = TransferSol::parse(&instruction, Limits::new(1232, 32))?;
assert_eq!(data.amount, 500);
# Ok(())
# }
```

Parsers require canonical pre-compilation account roles, exact account count,
System identity, fixed sysvars, valid tag, bounded data and no trailing bytes.
Transaction-wide privilege escalation may require reconstruction before using a
canonical parser. `identify_instruction` only identifies the four-byte tag;
`parse_instruction` validates the complete instruction.

`NonceCodec` decodes exactly80-byte allocated account data. Legacy and Current
versions remain distinct. Initialized state has authority, stored nonce hash and
an exact u64 lamports-per-signature fee. Uninitialized state preserves the72
unused storage bytes; it has no authority or usable hash. This improves on the
flat Dart schema, whose uninitialized fields imply meaning that official state
serialization does not assign. Official bincode serializes an uninitialized
prefix as8bytes; allocated snapshots still occupy80. Only initialized Current
snapshots expose `durable_data`. Decoding never upgrades a historical hash.
`decode_nonce` checks owner/allocation metadata through Kit core account decoding.

`SystemError` maps all nine pinned custom error codes and preserves unknown u32
values. No RPC, wallet signing, seed derivation or validator execution is provided
by this crate. Explicit fields replace mutable Dart discriminators/defaults;
no implicit payer or funding amount is invented.

Generated from System IDL commit `e9cb05138ea802b01f1ddf79ea140dcd765c9db6`;
input SHA-256 `bb1a83194457a7fc735f9e5a806ecaced94a17276fc7b45f356171810f15b63a`.
The upstream Apache-2.0 notice is retained in `SYSTEM-LICENSE`. Private official
constructor and nonce oracles remain outside application dependencies.
Regenerate using `system`, `system-builders` and `system-errors` commands in
`verification/codegen-design`. Full host-registry integration of the two new
client cases and validator execution remain separate evidence work.
