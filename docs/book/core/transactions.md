# Messages and transactions

`solana_kit_core::transaction` compiles instructions into an immutable message and wraps it in a transaction with one signature slot per required signer. It covers legacy, v0 and v1 messages. It also decodes received wire strictly and keeps the exact signed bytes. All of this is deterministic: no RPC, no clock and no randomness. Signing with a local key needs the core `crypto` feature.

## Instructions

An `Instruction` has a program address, ordered account metas and opaque data. Program client crates build these for you. You can also write one by hand:

```rust
use solana_kit_core::{AccountMeta, AccountRole, Address, Instruction};

fn transfer(from: Address, to: Address) -> Instruction {
    Instruction {
        program_address: Address::from_bytes([0; 32]), // System program
        accounts: vec![
            AccountMeta { address: from, role: AccountRole::WritableSigner },
            AccountMeta { address: to, role: AccountRole::Writable },
        ],
        // Transfer tag 2, then 1_000_000 lamports as little-endian u64.
        data: vec![2, 0, 0, 0, 64, 66, 15, 0, 0, 0, 0, 0],
    }
}
```

`AccountRole` is one of `Readonly`, `Writable`, `ReadonlySigner` or `WritableSigner`. When the same address appears more than once, the compiler merges its roles.

## Lifetimes

You must give every locally compiled message an explicit `Lifetime`:

| Variant                                                      | Use                                                                                 |
| ------------------------------------------------------------ | ----------------------------------------------------------------------------------- |
| `Lifetime::Blockhash { blockhash, last_valid_block_height }` | A recent blockhash. The expiry height is local metadata and is not part of the wire |
| `Lifetime::DurableNonce(DurableNonce)`                       | A durable nonce token plus its account and authority                                |
| `Lifetime::Unresolved(Hash)`                                 | Produced only by decoding. The wire carries the token but not its meaning           |

Compiling with `Unresolved` fails with `Error::UnresolvedLifetime`. Get a blockhash and its last valid height from `getLatestBlockhash`; see [RPC](rpc.md).

## Compile, sign and encode

`Message::compile(version, payer, lifetime, &instructions, &lookup_tables)` returns a frozen `Message`. The payer goes first. Accounts follow the deterministic Kit ordering. Invoked programs stay read-only and static.

```rust
use solana_kit_core::{
    Address, Hash,
    crypto::Keypair,
    transaction::{Lifetime, Message, PACKET_SIZE, Transaction, Version},
};

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let payer = Keypair::from_seed(&[7; 32]);
    let recipient: Address = "SysvarC1ock11111111111111111111111111111111".parse()?;
    let lifetime = Lifetime::Blockhash {
        blockhash: Hash::from_bytes([2; 32]),
        last_valid_block_height: 1_000,
    };

    let message = Message::compile(
        Version::V0,
        payer.address(),
        lifetime,
        &[transfer(payer.address(), recipient)],
        &[],
    )?;
    assert_eq!(message.signers(), [payer.address()]);

    let unsigned = Transaction::new(message);
    assert!(unsigned.encode(true).is_err()); // a slot is still empty

    let signed = unsigned.sign(&payer)?; // needs the `crypto` feature
    signed.verify()?;
    let wire = signed.encode(true)?;
    assert!(wire.len() <= PACKET_SIZE);
    Ok(())
}
```

A `Transaction` never changes in place. `sign` and `with_signature` return a new copy with the slot filled in.

- `Transaction::sign(&keypair)` signs the exact message bytes with a local `crypto::Keypair`.
- `Transaction::with_signature(signer, signed_bytes, signature)` installs a signature produced elsewhere, such as by a wallet. It checks that `signed_bytes` equal the message bytes and that the signer owns a slot. It does not check the signature cryptographically. Call `verify()` (with `crypto`) before you trust it.
- `encode(require_all_signatures)` writes the wire packet. Empty slots encode as 64 zero bytes when you pass `false`.

For signers that are not local keypairs, such as wallets, hardware or remote services, see [Signers](signers.md).

The shorter snippets below continue this example. They reuse `payer`, `recipient`, `lifetime`, `transfer` and, later, `wire` and `signed`.

## Message accessors

A compiled or decoded `Message` exposes read-only views: `bytes()` (the exact bytes every signer signs), `version()`, `header()`, `static_accounts()`, `signers()`, `instructions()` (each a `CompiledInstruction` with `program_index()`, `accounts()` and `data()`), `lookups()`, `lifetime()` and, for v1, `config()`.

## Address lookup tables

For v0, pass caller-owned `LookupTable { address, addresses }` snapshots. The compiler moves eligible non-signer, non-program accounts into table lookups:

```rust
use solana_kit_core::transaction::{LookupTable, Message, Version};

let table = LookupTable { address: table_address, addresses: vec![recipient] };
let message = Message::compile(
    Version::V0,
    payer.address(),
    lifetime,
    &[transfer(payer.address(), recipient)],
    &[table],
)?;
assert_eq!(message.lookups().len(), 1);
```

When an account appears in several snapshots, tables are ordered by base58 address and the first match wins. You must authenticate the snapshots yourself, for example by fetching the table account. The `solana_kit_address_lookup_table` program client decodes table accounts; see [Stake, Loader, Config, and lookup tables](../programs/runtime-programs.md).

To pin an account to one specific table and index, build instructions with `InstructionAccount` values that carry an `AccountLookupMeta`. Then call `Message::compile_with_lookup_accounts`. An explicit origin that does not match a snapshot fails with `Error::InvalidLookup`. It never falls back silently to static storage.

## Durable nonces

`DurableNonce::new(token, nonce_account, nonce_authority)` records your nonce values. `prepare_instructions` makes sure the first instruction is the matching System `AdvanceNonceAccount`. It keeps a matching advance, replaces a different one, or prepends one:

```rust
use solana_kit_core::{Hash, transaction::{DurableNonce, Lifetime, Message, Version}};

let nonce = DurableNonce::new(nonce_token, nonce_account, payer.address());
let instructions = nonce.prepare_instructions(&[transfer(payer.address(), recipient)]);
let message = Message::compile(
    Version::Legacy,
    payer.address(),
    Lifetime::DurableNonce(nonce),
    &instructions,
    &[],
)?;
assert_eq!(message.instructions().len(), 2);
```

Compiling checks the shape of the advance instruction. It never fetches or authenticates the nonce account's state. Those values are your assertions.

## Version one messages

`Message::compile_v1(payer, lifetime, &instructions, config)` builds a v1 message. V1 carries resource settings inline in a `V1Config` instead of Compute Budget instructions, and it has no lookup tables.

```rust
use solana_kit_core::transaction::{HeapSize, Message, V1Config, Version};

let config = V1Config::default()
    .with_compute_unit_limit(200_000)
    .with_priority_fee(5_000)
    .with_heap_size(HeapSize::new(262_144)?);
let message = Message::compile_v1(payer.address(), lifetime, &[], config)?;
assert_eq!(message.version(), Version::V1);
```

V1 signs the message prefix and then appends fixed-width signatures. The complete envelope can be up to `V1_PACKET_SIZE` (4096) bytes. Legacy and v0 stay at `PACKET_SIZE` (1232) bytes. A node's version number alone does not tell you whether v1 is active on a cluster.

## Decoding received wire

`Transaction::decode(&bytes)` and `Message::decode(&bytes)` validate the whole input. They reject unsupported versions, non-canonical lengths, bad partitions or indices, truncation, trailing bytes and oversized packets. The decoded transaction keeps the exact signed bytes and every slot. An unsigned slot decodes as `None`.

```rust
let received = Transaction::decode(&wire)?;
assert_eq!(received.message().bytes(), signed.message().bytes());
assert_eq!(received.message().lifetime(), Lifetime::Unresolved(Hash::from_bytes([2; 32])));
```

> [!WARNING]
> Decoding checks structure only. A decoded transaction is not authenticated until you call `verify()` with the `crypto` feature. Received rules are separate from compiler rules, so a valid message from elsewhere is not rejected just because the local compiler would have built it differently.

To turn a fetched RPC transaction into ordered account roles and outer and inner instructions, use `solana_kit_client::introspection::decode_rpc_transaction`.

## Off-chain messages

`solana_kit_core::offchain` signs application messages that are not transactions. `offchain::Message::v0` and `offchain::Message::v1` build validated messages with a caller-chosen byte budget. `Envelope` holds the ordered optional signatures.

```rust
use solana_kit_core::{
    codecs::Limits,
    crypto::Keypair,
    offchain::{Envelope, Message},
};

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let keypair = Keypair::from_seed(&[1; 32]);
    let message = Message::v1("Authorize this application", &[keypair.address()], 1024)?;
    let envelope = Envelope::unsigned(message)?.sign(&keypair)?;
    envelope.verify()?;

    let wire = envelope.encode(true, 2048)?;
    let received = Envelope::decode(&wire, Limits::new(2048, 4096))?;
    assert!(received.is_complete());
    Ok(())
}
```

A signature being present does not prove it is authentic. Call `verify()`.

## Errors

Every operation in this module returns `transaction::Error`. The common cases are:

| Variant                                                                                              | Cause                                                                    |
| ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `UnresolvedLifetime`                                                                                 | You compiled with `Lifetime::Unresolved`                                 |
| `PacketTooLarge`                                                                                     | The packet exceeds 1232 bytes (legacy/v0) or 4096 bytes (v1)             |
| `WritableProgram`                                                                                    | An invoked program is writable or is also the payer                      |
| `InvalidLookup`, `InvalidNonceAdvance`                                                               | A bad lookup origin or snapshot, or a missing or different nonce advance |
| `InvalidConfig`, `InvalidV1Limits`                                                                   | Bad v1 configuration or limits                                           |
| `UnknownSigner`, `ChangedMessage`, `MissingSignature`, `InvalidSignature`                            | Signature slot and verification failures                                 |
| `Truncated`, `TrailingBytes`, `InvalidLength`, `UnsupportedVersion`, `InvalidHeader`, `InvalidIndex` | Malformed wire                                                           |

## Status

> [!NOTE]
> The [message and transaction ledger](https://github.com/pina-rs/solana_kit/tree/main/docs/parity/message-transactions.md) still lists many reference capabilities as partial or pending, and Phase 3 is not complete. In particular, durable nonce lifecycle tooling and full introspection are unfinished. The [off-chain message receipt](https://github.com/pina-rs/solana_kit/tree/main/docs/parity/offchain-messages.md) lists its scope as partial.
