# Migrating from Dart Solana Kit

This guide maps the pinned Dart Solana Kit packages to the current Rust crates.
It is not a package-by-package API clone. Rust ownership, explicit transports, and
caller-owned executors replace several Dart conveniences, so the useful migration
unit is the workflow rather than the original class.

The pinned reference is Dart commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. The Rust SDK is pre-1.0 and is
exclusively for off-chain applications. Do not use it in Solana on-chain programs;
use Pina or the official Solana program libraries there. `solana_kit` 0.0.0 is a
dependency-free name reservation, so depend on the component crates below instead
of the umbrella crate.

## Translate concepts before syntax

| Dart Solana Kit                          | Rust Solana Kit                                                                    | Migration note                                                                                       |
| ---------------------------------------- | ---------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `Address`                                | `solana_kit_core::Address`                                                         | A typed 32-byte value. Parse untrusted text; do not pass strings through the API.                    |
| `Blockhash`                              | `solana_kit_core::Hash`                                                            | Hashes and addresses are distinct types even though both are 32 bytes.                               |
| `SignatureBytes`                         | `solana_kit_core::Signature`                                                       | Verification is explicit and requires the `crypto` feature.                                          |
| `BigInt` amounts                         | `u64` for RPC and raw token quantities; optional exact widths for program data     | Node-facing lamports, slots and raw token amounts are bounded integers, never `f64`.                 |
| Nullable parameters and optional configs | `Option<T>` fields on request structs                                              | `None` generally means "omit and use the node default"; it does not recreate a Dart factory default. |
| `createSolanaRpc(url)` / `Rpc`           | `Client<T: Transport>`                                                             | The client owns a concrete transport, request IDs and parse limits.                                  |
| `createHttpTransportForSolanaRpc`        | `NativeHttp`, `BrowserHttp`, or `solana_kit_wasi::WasiHttp`                        | Each platform adapter is explicit; there is no ambient endpoint or retry policy.                     |
| `KeyPairSigner` in a transaction message | `SignerCapability` plus `TransactionSigners` or a local/native `Composition`       | Capabilities are borrowed, deduplicated logical handles.                                             |
| `TransactionSignerConfig`                | `solana_kit_signers::TransactionSignerConfig`                                      | Supplied on every call, not stored in a signer or pipeline.                                          |
| `getCreateMintInstructionPlan`           | `solana_kit_token::create_mint_plan`                                               | Requires the optional `plans` feature.                                                               |
| `getMintToAtaInstructionPlanAsync`       | `mint_to_ata_derived_plan`                                                         | Requires the `plans` feature; PDA derivation is synchronous in Rust.                                 |
| `rpcSubscriptions` channel               | `solana_kit_client::subscriptions::Session`                                        | The caller owns the socket, clock and reconnect policy.                                              |
| Wallet adapter discovery                 | `solana_kit_wallet` contracts; `solana_kit_browser::wallet` for Window discovery   | Selection and UI belong to the host application.                                                     |
| Mobile Wallet Adapter                    | `solana_kit_wallet::mwa` protocol; `solana_kit_native::mwa` for a native transport | The SDK never launches an Android intent or a wallet UI.                                             |

## Values, integers and configuration

An address or hash is parsed once. Display and serialization produce base58, but
application code keeps the typed value.

```rust
use solana_kit_core::{Address, Hash};

let program: Address = "11111111111111111111111111111111".parse()?;
let blockhash: Hash = "11111111111111111111111111111111".parse()?;
assert_eq!(program.to_bytes(), [0; 32]);
assert_eq!(blockhash.to_bytes(), [0; 32]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Rust does not reproduce Dart's `BigInt` for ordinary RPC values. Lamports, slots,
token base units and node counters use exactly sized Rust integers, and a value
that does not fit is an error rather than a rounded JavaScript number.

```rust
let lamports: u64 = "9007199254740993".parse()?;
assert_eq!(lamports, 9_007_199_254_740_993);
# Ok::<(), Box<dyn std::error::Error>>(())
```

The `solana_kit_client::methods::tokens::TokenBalance` type keeps the exact raw
`amount` separately from `ui_amount` and `ui_amount_string`; use the string or raw
amount for display and logic. For wider program data, enable core's
`bigint-codecs` feature and use bounded `BigInt`/`BigUint` codecs. For exact
decimal or binary fractions, enable `fixed-points`. Neither is a default
dependency.

Configuration is an ordinary value with explicit fields. `None` is preserved on
the wire; it is not replaced by Dart's factory commitment.

```rust
use solana_kit_client::methods::{
    Commitment, ReadConfig,
    accounts::AccountConfig,
    submission::SendConfig,
};

let read = ReadConfig {
    commitment: Some(Commitment::Confirmed),
    min_context_slot: None,
};
let account = AccountConfig {
    read,
    encoding: None,
    data_slice: None,
};
let send = SendConfig {
    preflight_commitment: Some(Commitment::Confirmed),
    ..SendConfig::default()
};
assert_eq!(account.read.commitment, Some(Commitment::Confirmed));
assert_eq!(send.preflight_commitment, Some(Commitment::Confirmed));
```

The distinction between absent JSON and explicit `null` is represented by
`Presence<T>` in received client values. Do not collapse that with `Option<T>`.
See [the typed RPC design](../architecture/rpc.md) for the boundary.

## Client, transport and cancellation

`Client` is an executor-neutral protocol owner. It starts no runtime, opens no
socket on construction, installs no credentials, and never retries. The adapter
owns the endpoint, TLS roots, caller headers, deadlines and response-body budget.
Each `Client` owns one request-ID namespace. If one multiplexed connection serves
several clients, their IDs must not collide; repeated calls on one client are
separate exchanges even when their parameters match.

Native applications select `NativeHttp` and use their own Tokio runtime:

```rust,no_run
use solana_kit_client::{Client, Limits};
use solana_kit_native::{HttpConfig, NativeHttp};

let transport = NativeHttp::new("https://rpc.example.invalid/", HttpConfig::default())?;
let client = Client::new(transport, Limits::default());
# let _ = client;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Browser, WASI and LeptosKit applications select their owning adapter rather than
inventing a native runtime. The [browser HTTP design](../architecture/browser-http.md)
and [WASI adapter](../../crates/solana_kit_wasi/README.md) record those capability
boundaries.

Cancellation is ownership. Hold the future returned by `Client::call` until it
completes; dropping it cancels the adapter's owned exchange. `NativeHttp` closes
its socket and TLS/HTTP work on drop, although an OS hostname lookup may finish
without being able to send. Dropping a spawned Tokio `JoinHandle` only detaches
the task, so native callers that spawn must abort the handle and await it. Browser
futures are deliberately local and must not be turned into detached work.

Commitment is explicit per request. An absent commitment uses the node default.
The Dart RPC factory injects `confirmed`, so pass it explicitly when you need that
behavior:

```rust,no_run
use solana_kit_client::{
    Client, Transport,
    methods::{Balance, Commitment, ReadConfig},
};
use solana_kit_core::Address;

async fn confirmed_balance<T: Transport>(
    client: &Client<T>,
    address: Address,
) -> Result<u64, solana_kit_client::Error> {
    let config = ReadConfig {
        commitment: Some(Commitment::Confirmed),
        min_context_slot: None,
    };
    let value = client.call(Balance(address, Some(config))).await?;

    Ok(value.value)
}
```

Submission has a separate `SendConfig::preflight_commitment`; do not reuse a read
config as a send policy.

## Fetch and decode with context

`fetch_encoded_account` returns `Contextual<MaybeEncodedAccount>`. Keep the node
context beside the value, and distinguish an absent account from binary, parsed or
compressed account data.

```rust,no_run
use solana_kit_client::{
    Client, Transport,
    methods::{Contextual, ReadConfig},
};
use solana_kit_core::{
    accounts::Account,
    codecs::Limits,
    sysvars::{CLOCK_ADDRESS, Clock, OWNER_ADDRESS, SysvarCodec},
};

async fn fetch_clock<T: Transport>(
    client: &Client<T>,
) -> Result<Contextual<Account<Clock>>, Box<dyn std::error::Error>> {
    let Contextual { context, value } = client
        .fetch_encoded_account(CLOCK_ADDRESS, ReadConfig::default())
        .await?;
    let value = value
        .require()?
        .decode(&SysvarCodec::<Clock>::NEW, OWNER_ADDRESS, Limits::new(40, 0))?;

    Ok(Contextual { value, context })
}
```

`AccountData` keeps the representation explicit. `Bytes` holds decoded wire
bytes; interpreting program state still requires a codec. `Parsed` is the node's
program-defined JSON; `Zstd` is still compressed.
Selecting `JsonParsed` does not make unknown programs parsed, and binary bytes are
not silently reinterpreted.

```rust,no_run
use solana_kit_client::{
    Client, Transport,
    Error,
    methods::{
        Contextual,
        accounts::{AccountConfig, AccountData, AccountEncoding, AccountInfoWithConfig},
    },
};
use solana_kit_core::{Address, accounts::MaybeAccount};

async fn account_representation<T: Transport>(
    client: &Client<T>,
    address: Address,
) -> Result<(), Error> {
    let Contextual { context, value } = client
        .call(AccountInfoWithConfig(
            address,
            Some(AccountConfig {
                encoding: Some(AccountEncoding::JsonParsed),
                ..AccountConfig::default()
            }),
        ))
        .await?;
    let _slot = context.slot;

    if let MaybeAccount::Present(account) = value {
        match account.data {
            AccountData::Parsed(_parsed) => (),
            AccountData::Bytes { .. } | AccountData::Zstd { .. } => (),
        }
    }

    Ok(())
}
```

`decode_account` and `decode_maybe_account` apply a caller-selected binary decoder
while preserving account metadata. `MaybeEncodedAccount::decode` additionally
checks the expected owner, reported size and trailing bytes. Missing accounts stay
`Missing`; they are never converted into empty data or zero balances.

Enable `solana_kit_client`'s `parsed-accounts` feature to project known node
payloads while retaining the original raw JSON. Parser names select a schema;
they do not authenticate account ownership. Unknown programs and discriminators
remain explicit variants rather than invented binary data.

```rust
use solana_kit_client::{
    Limits, methods::accounts::ParsedAccount,
    parsed::{ParsedAccountData, TokenProgramAccount},
};
use serde_json::value::RawValue;

let literal = r#"{"type":"mint","info":{"decimals":6,"isInitialized":true,"supply":"18446744073709551615"}}"#;
let account = ParsedAccount {
    program: "spl-token".into(),
    parsed: RawValue::from_string(literal.into())?,
    space: Some(82),
};
let ParsedAccountData::Token(TokenProgramAccount::Mint(mint)) =
    account.decode_typed(Limits::default())?
else {
    panic!("fixture must project to a mint");
};
assert_eq!(mint.supply, u64::MAX);
assert_eq!(account.parsed.get(), literal);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Testing without a node

Use `solana_kit_test_support` as a development dependency. Its finite scripts
exercise the actual client parser and correlation checks; request capture lets
the test inspect what application code sent. Pending controls also let tests
observe completion, wakeups and cancellation without a runtime or network.

```rust
use solana_kit_client::{Client, Limits, methods::Balance};
use solana_kit_core::Address;
use solana_kit_test_support::{ScriptStep, ScriptedTransport, canned};

let transport = ScriptedTransport::new([
    ScriptStep::success(canned::balance(7, u64::MAX)),
]);
let client = Client::new(transport.clone(), Limits::default());
let reply = futures::executor::block_on(
    client.call(Balance(Address::from_bytes([0; 32]), None)),
)?;
assert_eq!(reply.context.slot, 7);
assert_eq!(reply.value, u64::MAX);
assert_eq!(transport.requests()[0].method, "getBalance");
assert_eq!(transport.remaining(), 0);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Sign, send and confirm without hidden retries

Signing capabilities are explicit and a call receives its configuration directly.
Use one `SignerCapability` handle for one logical resource; repeated references to
that handle deduplicate, while distinct handles claiming the same address conflict.

```rust,no_run
use solana_kit_core::{
    Hash,
    crypto::Keypair,
    transaction::{Lifetime, Message, Transaction, Version},
};
use solana_kit_signers::{SignerCapability, TransactionSignerConfig, TransactionSigners};

async fn sign_with_local_key(
    keypair: &Keypair,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let message = Message::compile(
        Version::Legacy,
        keypair.address(),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([9; 32]),
            last_valid_block_height: 100,
        },
        &[],
        &[],
    )?;
    let unsigned = Transaction::new(message);
    let capability = SignerCapability::new(keypair);
    let signers = TransactionSigners::new(&[&capability])?;

    Ok(signers
        .sign(
            &unsigned,
            TransactionSignerConfig {
                aborted: false,
                min_context_slot: Some(0),
            },
        )
        .await?)
}
```

Message signers take `SignerConfig { aborted }`; transaction signers take
`TransactionSignerConfig { aborted, min_context_slot }`. `aborted: true` returns
the cancellation error before signing. Dropping an in-flight future is the
cancellation mechanism after a backend starts. A local keypair ignores
`min_context_slot` because it does not simulate; wallets receive the exact hint,
and the browser adapter rejects values that cannot be represented as a JavaScript
safe integer.

Submission takes exact signed wire and binds the returned identifier to its primary
signature. The SDK performs one exchange and does not retry or re-sign.

```rust,no_run
use solana_kit_client::{
    Client, Transport,
    methods::{
        Commitment,
        submission::{SendConfig, SendTransaction, SignedWire, WireEncoding},
    },
};
use solana_kit_core::{Signature, transaction::Transaction};

async fn submit_once<T: Transport>(
    client: &Client<T>,
    transaction: &Transaction,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let wire = transaction.encode(true)?;
    let signed_wire = SignedWire::new(&wire)?;
    let signature = client
        .call(SendTransaction {
            wire: signed_wire,
            encoding: WireEncoding::Base64,
            config: SendConfig {
                preflight_commitment: Some(Commitment::Confirmed),
                ..SendConfig::default()
            },
        })
        .await?;

    Ok(signature)
}
```

A returned signature is a submission identifier, not execution. Confirmation is a
separate policy with a caller-owned observer that registers subscriptions before
reconciliation and closes them explicitly. `send_and_confirm` does not add
submission retries; any node-side retry is the `SendConfig` you supplied.

```rust,no_run
use core::future::Future;
use solana_kit_client::{
    Client, Transport,
    confirmation::{Observer, Policy, WaitError, send_and_confirm},
    methods::{Commitment, submission::SendConfig},
};
use solana_kit_core::{Signature, transaction::Transaction};

async fn submit_and_confirm<T, O, F, C>(
    client: &Client<T>,
    transaction: &Transaction,
    observe: F,
    cancellation: C,
) -> Result<Signature, WaitError>
where
    T: Transport,
    O: Observer,
    F: FnOnce(Signature) -> O,
    C: Future<Output = ()>,
{
    send_and_confirm(
        client,
        transaction,
        SendConfig {
            preflight_commitment: Some(Commitment::Confirmed),
            ..SendConfig::default()
        },
        Policy {
            commitment: Commitment::Confirmed,
            min_context_slot: 0,
        },
        observe,
        cancellation,
    )
    .await
}
```

See the [confirmation receipt](../verification/transaction-confirmation.md) for
the event, expiry, nonce and cleanup contract. Observer implementations are
application adapters; the SDK supplies no hidden polling loop.

## Token plans

Enable `solana_kit_token/plans` for the pinned handwritten helpers. That feature
adds System and associated-token-account dependencies, including ATA PDA
derivation. The helpers construct instructions only: no signer, blockhash,
network request, executor or transaction is selected. The caller still compiles
and signs the plan.

`create_mint_plan` is indivisible: the System `CreateAccount` and Token
`InitializeMint2` must execute in one transaction or bundle. Its default funding
is the pinned source's `1_461_600` lamports, not a live rent lookup. Fetch the
cluster's current rent-exempt amount and pass it explicitly when it differs.

```rust,no_run
use solana_kit_client::{
    Client, Transport,
    methods::{
        Commitment,
        accounts::{CommitmentConfig, MinimumBalanceForRentExemption},
    },
};
use solana_kit_core::{Address, codecs::Limits, plans::Plan, Instruction};
use solana_kit_token::{CreateMintConfig, CreateMintInput, create_mint_plan};

async fn rent_aware_mint_plan<T: Transport>(
    client: &Client<T>,
    payer: Address,
    new_mint: Address,
    mint_authority: Address,
) -> Result<Plan<Instruction>, Box<dyn std::error::Error>> {
    let rent = client
        .call(MinimumBalanceForRentExemption(
            82,
            Some(CommitmentConfig {
                commitment: Some(Commitment::Confirmed),
            }),
        ))
        .await?;
    let plan = create_mint_plan(
        CreateMintInput {
            payer,
            new_mint,
            decimals: 6,
            mint_authority,
            freeze_authority: None,
            mint_account_lamports: Some(rent),
        },
        CreateMintConfig::default(),
        Limits::new(4096, 4096),
    )?;

    Ok(plan)
}
```

The mint-to-ATA and transfer-to-ATA plans create the associated account
idempotently, then use `MintToChecked` or `TransferChecked`. The derived variants
replace Dart's async PDA helper with a synchronous Kit derivation. Configured
program identities apply both to instruction program addresses and to account
metadata positions.

```rust
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_token::{
    MintToAtaConfig, MintToAtaDerivedInput, TransferToAtaConfig, TransferToAtaDerivedInput,
    mint_to_ata_derived_plan, transfer_to_ata_derived_plan,
};

let payer = Address::from_bytes([1; 32]);
let owner = Address::from_bytes([2; 32]);
let mint = Address::from_bytes([3; 32]);
let authority = Address::from_bytes([4; 32]);
let recipient = Address::from_bytes([5; 32]);
let token_program = Address::from_bytes([6; 32]);
let ata_program = Address::from_bytes([7; 32]);
let system_program = Address::from_bytes([8; 32]);

let config = MintToAtaConfig {
    token_program: Some(token_program),
    associated_token_program: Some(ata_program),
    system_program: Some(system_program),
};
let mint_plan = mint_to_ata_derived_plan(
    MintToAtaDerivedInput {
        payer,
        owner,
        mint,
        mint_authority: authority,
        amount: 1_000_000,
        decimals: 6,
    },
    config,
    Limits::new(4096, 4096),
)?;
assert_eq!(mint_plan.flatten()[0].program_address, ata_program);
assert_eq!(mint_plan.flatten()[1].program_address, token_program);

let transfer_plan = transfer_to_ata_derived_plan(
    TransferToAtaDerivedInput {
        payer,
        mint,
        authority,
        recipient,
        amount: 250_000,
        decimals: 6,
    },
    TransferToAtaConfig {
        token_program: Some(token_program),
        associated_token_program: Some(ata_program),
        system_program: Some(system_program),
    },
    Limits::new(4096, 4096),
)?;
assert_eq!(transfer_plan.flatten().len(), 2);
# Ok::<(), Box<dyn std::error::Error>>(())
```

These helpers perform no balance, owner, delegate or rent validation. The
idempotent ATA constructor is not a balance check, and `MintToChecked` or
`TransferChecked` can still fail on-chain. Low-level Token builders remain
available when explicit multisignature account assembly is required.

## Subscriptions

`Session` owns one typed protocol connection over a caller-supplied `TextSocket`
and `Clock`. Registration is acknowledged before the handle becomes active;
unsubscribe waits for a true acknowledgement, and false is an error. Shutdown
consumes the session and awaits actual socket teardown.

```rust,no_run
use solana_kit_client::subscriptions::{
    Account, Budget, Clock, Drain, Session, SessionError, TextSocket,
};
use solana_kit_core::Address;

async fn watch_account<S: TextSocket, C: Clock>(
    socket: S,
    clock: &C,
    address: Address,
    deadline: u64,
    cleanup_deadline: u64,
) -> Result<(), Vec<SessionError<S::Error>>> {
    let mut session = Session::new(socket, 1, Budget::default());
    let operation = async {
        let handle = session.subscribe(Account(address, None), deadline)?;
        session.flush(clock, deadline).await?;

        match session.next(&handle, clock, deadline).await? {
            Drain::Notification(_) | Drain::Closed | Drain::Pending => (),
        }

        session.unsubscribe(handle, clock, deadline).await
    }.await;
    let cleanup = session.shutdown(clock, cleanup_deadline).await;
    let errors: Vec<_> = [operation, cleanup].into_iter().filter_map(Result::err).collect();

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}
```

Give cleanup its own deadline. The example attempts shutdown even when an
operation fails and retains both errors if cleanup also fails.

The SDK does not reconnect, replay missed notifications or hide a generation gap.
A replacement reports `Gap`; create a new generation and state the gap to your
application. Dropping a handle cancels local delivery, but only the awaited
`unsubscribe` or `shutdown` establishes cleanup. The [subscription runtime receipt](../verification/subscription-runtime.md) records the portable protocol
evidence and its transport limits.

## Wallets and Mobile Wallet Adapter

Portable wallet contracts live in `solana_kit_wallet`: discovery and selection are
host application concerns, while concrete backends implement `Connect`, `Events`
and the Solana signing capabilities. A `WalletFeatures` snapshot yields a typed
capability; returned signatures remain raw bytes and must be verified before they
are trusted.

For a browser, `solana_kit_browser::wallet` supplies `WalletDiscovery`,
`WalletController`, account selection and a typed `WalletSigner`. Selection code
is browser-only because it requires a real `Window`. The
[compiled browser example](../../verification/migration-examples/examples/browser_wallet.rs)
is checked for `wasm32-unknown-unknown`; the maintained Chrome wallet tests in
[`crates/solana_kit_browser/tests/wallet.rs`](../../crates/solana_kit_browser/tests/wallet.rs)
show the executed discovery, connection, selection and signing journey.

Mobile Wallet Adapter is a protocol, not a launcher. The SDK generates association
material and an association URI; the host application performs the Android intent
or other platform handover, then returns control for the encrypted session. No
intent, wallet UI, entropy source or transport is started implicitly.

```rust,no_run
use serde_json::Map;
use solana_kit_native::{
    keys::NativeEntropy,
    mwa::{MwaConfig, NativeAssociation},
};
use solana_kit_wallet::mwa::Method;

async fn hand_over_to_wallet(
    entropy: NativeEntropy,
    wallet_base: Option<&str>,
    launch: impl FnOnce(&str) -> Result<(), Box<dyn std::error::Error>>,
) -> Result<(), Vec<Box<dyn std::error::Error>>> {
    let setup: Result<_, Box<dyn std::error::Error>> = async {
        let association = NativeAssociation::generate(entropy).await?;
        let uri = association.uri(wallet_base)?;
        launch(&uri)?;

        Ok(association.connect(MwaConfig::default()).await?)
    }.await;
    let mut wallet = setup.map_err(|error| vec![error])?;
    let operation = wallet.call(Method::GetCapabilities, Map::new()).await
        .map(|_| ()).map_err(Box::<dyn std::error::Error>::from);
    let cleanup = wallet.close().await.map_err(Box::<dyn std::error::Error>::from);
    let errors: Vec<_> = [operation, cleanup].into_iter().filter_map(Result::err).collect();

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}
```

The application supplies `launch` using its platform API. Once connected, the
example attempts close even after a failed request and retains cleanup errors.

The browser and mobile adapters own their platform cancellation and permission
semantics. This crate does not claim that a wallet exists on a device, that an
Android emulator supports MWA, or that a browser exposes a Wallet Standard
provider. Check the [Wallet Standard receipt](../parity/wallet-standard.md) and
[MWA protocol receipt](../parity/mobile-wallet-protocol.md) before relying on a
platform.

## What the host still owns

Keep these decisions in the application, adapter or host project:

- the executor, task spawning, clock, timers and cancellation/deadline policy;
- transport construction, endpoints, TLS roots, caller authentication and grants;
- wallet discovery, user gestures, UI, authorization prompts and account choice;
- Android intents, iOS handover, association launching and installed-app lifecycle;
- entropy, key storage, persistence, key backup and erasure;
- subscription reconnection, gap disclosure, event routing and application state;
- confirmation observers, RPC polling, nonce authentication and validator policy;
- any on-chain program logic, which belongs in Pina or official program libraries.

The SDK supplies bounded protocol and value owners. It does not grant itself host
capabilities, start a hidden runtime, or turn an off-chain client into a validator
or on-chain runtime.

## Compiled example inventory and limits

The file [`verification/migration-examples/src/lib.rs`](../../verification/migration-examples/src/lib.rs)
includes this guide as crate documentation. Every Rust block in this guide is
therefore a rustdoc test; the browser-only recipe is the separate example target
listed last.

| Guide block                   | Compilation                                   |
| ----------------------------- | --------------------------------------------- |
| Address and hash parsing      | doctest, executed                             |
| Exact integer parsing         | doctest, executed                             |
| Typed read/send configuration | doctest, executed                             |
| Native transport construction | doctest, compile-only (`no_run`)              |
| Confirmed balance call        | doctest, compile-only (`no_run`)              |
| Clock fetch and sysvar decode | doctest, compile-only (`no_run`)              |
| Binary/parsed account match   | doctest, compile-only (`no_run`)              |
| Typed parsed projection       | doctest, executed                             |
| Scripted client fixture       | doctest, executed                             |
| Per-call signer configuration | doctest, compile-only (`no_run`)              |
| Exact-wire submission         | doctest, compile-only (`no_run`)              |
| Caller-observer confirmation  | doctest, compile-only (`no_run`)              |
| Rent-aware create-mint plan   | doctest, compile-only (`no_run`)              |
| Idempotent ATA mint/transfer  | doctest, executed                             |
| Subscription session          | doctest, compile-only (`no_run`)              |
| Native MWA handover           | doctest, compile-only (`no_run`)              |
| Browser wallet selection      | example target, `wasm32-unknown-unknown` only |

Reproduce with cached, offline dependencies:

```sh
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
  cargo +1.99.1 test --manifest-path verification/migration-examples/Cargo.toml --locked --offline
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
  cargo +1.99.1 test --manifest-path verification/migration-examples/Cargo.toml --doc --locked --offline
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
  cargo +1.99.1 clippy --manifest-path verification/migration-examples/Cargo.toml \
  --all-targets --locked --offline -- -D warnings
RUSTDOCFLAGS="-D warnings" CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
  cargo +1.99.1 doc --manifest-path verification/migration-examples/Cargo.toml \
  --no-deps --locked --offline
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
  cargo +1.89.0 test --manifest-path verification/migration-examples/Cargo.toml \
  --doc --locked --offline
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
  cargo +1.99.1 check --manifest-path verification/migration-examples/Cargo.toml \
  --target wasm32-unknown-unknown --example browser_wallet --locked --offline
```

Guide doctests execute on native Rust 1.99.1 and 1.89.0. The browser example is
compile-only for `wasm32-unknown-unknown`, checked with both toolchains. No check
in this directory opens a network connection, signs a real transaction, launches a
wallet, contacts a device, publishes a package, or runs a validator. Compilation
does not establish runtime acceptance for any platform or wallet.
