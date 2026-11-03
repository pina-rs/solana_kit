# Build a transaction

A Solana transaction holds a fee payer, a lifetime, and an ordered list of instructions. In Solana Kit you build each instruction, fetch a recent blockhash for the lifetime, and compile everything into an immutable `Message` in one call. This page builds a System Program SOL transfer.

You need these dependencies:

```toml
[dependencies]
solana_kit_core = { git = "https://github.com/pina-rs/solana_kit", features = ["crypto"] }
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_native = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_system = { git = "https://github.com/pina-rs/solana_kit" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

<Steps>

### Create the instruction

`solana_kit_system` has a typed builder for each of the fourteen System instructions. `TransferSol` takes the amount in lamports. `TransferSolAccounts` names the source and destination:

```rust
use solana_kit_core::{codecs::Limits as DecodeLimits, crypto::Keypair};
use solana_kit_system::{TransferSol, TransferSolAccounts};

let payer = Keypair::from_seed(&[7; 32]); // test seed; see "Addresses and keys"
let recipient = Keypair::from_seed(&[9; 32]).address();

let transfer = TransferSol { amount: 1_000_000 }.instruction(
    TransferSolAccounts {
        source: payer.address(),
        destination: recipient,
    },
    DecodeLimits::new(1232, 32),
)?;
```

The builder returns a `solana_kit_core::Instruction` with the System Program address, the right account roles (the source is a writable signer, the destination is writable), and the encoded data. The `Limits` argument caps the encoded instruction size. `TransferSol::parse(&instruction, limits)` reverses the process and checks the program, the accounts and the data strictly.

### Fetch a blockhash lifetime

Every transaction must expire. The usual way is a recent blockhash, valid until a known block height:

```rust
use solana_kit_client::{
    Client, Limits,
    methods::{Commitment, LatestBlockhash, ReadConfig},
};
use solana_kit_core::transaction::Lifetime;
use solana_kit_native::{HttpConfig, NativeHttp};

let transport = NativeHttp::new("https://api.devnet.solana.com", HttpConfig::default())?;
let client = Client::new(transport, Limits::default());

let read = ReadConfig {
    commitment: Some(Commitment::Confirmed),
    ..ReadConfig::default()
};
let latest = client.call(LatestBlockhash(Some(read))).await?.value;

let lifetime = Lifetime::Blockhash {
    blockhash: latest.blockhash,
    last_valid_block_height: latest.last_valid_block_height,
};
```

The blockhash is signed into the message. `last_valid_block_height` is never sent over the wire: it is metadata you keep so you can tell when the transaction has expired. [Sign and send](sign-and-send.md) uses it to stop waiting.

### Compile the message

```rust
use solana_kit_core::transaction::{Message, Transaction, Version};

let message = Message::compile(
    Version::V0,
    payer.address(), // fee payer
    lifetime,
    &[transfer],
    &[], // address lookup tables
)?;

println!("signers: {:?}", message.signers());
println!("{} bytes", message.bytes().len());

let transaction = Transaction::new(message);
```

`Message::compile` orders the accounts deterministically and merges repeated accounts so each keeps every privilege any instruction asked for. It puts the fee payer first and writes the exact wire bytes. It returns `transaction::Error` if the lifetime is unresolved, a program account has invalid privileges, there are too many accounts, or the message would not fit in a packet.

`Transaction::new` creates one empty signature slot for each required signer, in signer order. You fill them in the next step.

</Steps>

## Message versions

`Version` picks the wire format:

- `Version::Legacy`: the original format.
- `Version::V0`: adds address lookup tables. Pass table snapshots as `&[LookupTable]` in the last argument.
- `Version::V1`: uses an inline resource configuration and has no lookup tables. Use `Message::compile_v1(payer, lifetime, &instructions, config)` to set a `V1Config`.

Legacy and v0 packets are limited to 1232 bytes. Complete v1 envelopes are limited to 4096 bytes. Before you choose v1, check that your target cluster accepts it.

## Your own instructions

For programs without a typed client, build an `Instruction` directly from `solana_kit_core`:

```rust
use solana_kit_core::{AccountMeta, AccountRole, Address, Instruction};

let instruction = Instruction {
    program_address: Address::from_bytes([0; 32]),
    accounts: vec![AccountMeta {
        address: payer.address(),
        role: AccountRole::WritableSigner,
    }],
    data: vec![1, 2, 3, 4],
};
```

`AccountRole` has four variants: `Readonly`, `Writable`, `ReadonlySigner` and `WritableSigner`. Choose the narrowest role the program needs. Use the [codecs](../core/codecs.md) to encode `data` instead of writing bytes by hand.

## Durable nonce lifetimes

`Lifetime::DurableNonce` takes the place of a blockhash with a nonce account, so a transaction can be signed now and sent much later. Compilation checks that the first instruction is a matching `AdvanceNonceAccount`. This path is still partial: the full durable nonce lifecycle and validator journeys are pending. See [Messages and transactions](../core/transactions.md) and [Parity and status](../reference/parity.md) before you rely on it.

## Next steps

<Cards>
<Card title="Sign and send" href="sign-and-send.md">Sign this transaction, submit it, and wait for confirmation.</Card>
<Card title="System" href="../programs/system.md">Every System instruction builder and parser.</Card>
<Card title="Messages and transactions" href="../core/transactions.md">Lookup tables, v1, decoding and inspection.</Card>
</Cards>
