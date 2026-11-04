# RPC

`solana_kit_client` provides `Client<T>`, a typed JSON-RPC client. It owns the protocol: request IDs, framing, correlation, response validation and size budgets. It does no I/O of its own. A `Transport` you choose moves the bytes, and every result comes back as a Kit value such as `Address`, `Hash`, `Signature` or `MaybeEncodedAccount`, never raw JSON. The crate uses `std`, but it has no runtime, network, JavaScript or randomness dependency.

## Create a client

`Client::new(transport, limits)` takes any `Transport` plus a `Limits` budget. `Limits::default()` allows 8192 bytes per request or reply and 32 levels of JSON nesting. Raise `bytes` for large responses such as blocks or many accounts.

On a native host, use the Tokio adapter:

```rust
use solana_kit_client::{Client, Limits};
use solana_kit_native::{HttpConfig, NativeHttp};

fn connect() -> Result<Client<NativeHttp>, Box<dyn std::error::Error>> {
    let transport = NativeHttp::new("https://api.devnet.solana.com/", HttpConfig::default())?;
    Ok(Client::new(transport, Limits::default()))
}
```

Creating the client sends nothing. The other hosts have their own adapters: `BrowserHttp` in `solana_kit_browser`, `WasiHttp` in `solana_kit_wasi`, and `GuestHttp` in `solana_kit_leptoskit`. See [Choosing a runtime](../runtimes/index.md).

## Call a method

Every RPC method is a typed request value that implements `methods::Method`. Pass it to `client.call(...)`:

```rust
use solana_kit_client::{
    Client, Error, Transport,
    methods::{Balance, Commitment, LatestBlockhash, ReadConfig},
};
use solana_kit_core::Address;

async fn balance<T: Transport>(client: &Client<T>, owner: Address) -> Result<u64, Error> {
    let config = ReadConfig { commitment: Some(Commitment::Confirmed), ..ReadConfig::default() };
    let balance = client.call(Balance(owner, Some(config))).await?;
    println!("slot {}: {} lamports", balance.context.slot, balance.value);

    let latest = client.call(LatestBlockhash(None)).await?;
    println!("valid until height {}", latest.value.last_valid_block_height);
    Ok(balance.value)
}
```

Methods that the node answers in the context of a slot return `Contextual<T>`, which holds `context.slot` and `context.api_version` next to the `value`. The client keeps a missing field distinct from an explicit JSON `null` through `Presence<T>`.

## Catalog coverage

All 54 methods in the pinned reference catalog have typed implementations.

| Module                                                                         | Methods                                                                                                                                                                                                                                                                                                                           |
| ------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `methods`                                                                      | `Balance` (getBalance), `LatestBlockhash`, `AccountInfo` (full base64 getAccountInfo)                                                                                                                                                                                                                                             |
| `methods::node` (19)                                                           | `Slot`, `BlockHeight`, `GetEpochInfo`, `GetEpochSchedule`, `GetHealth`, `Version`, `Identity`, `GenesisHash`, `ClusterNodes`, `SlotLeader`, `SlotLeaders`, `LeaderSchedule`, `BlockProduction`, `TransactionCount`, `RecentPerformanceSamples`, `HighestSnapshotSlot`, `MaxRetransmitSlot`, `MaxShredInsertSlot`, `AgGenesisCert` |
| `methods::economics` (8)                                                       | Inflation governor, rate and reward, `Supply`, largest accounts, `RecentPrioritizationFees`, vote accounts, stake minimum delegation                                                                                                                                                                                              |
| `methods::accounts` and `methods::tokens` (9)                                  | `MultipleAccounts`, `ProgramAccounts`, `MinimumBalanceForRentExemption`, `BlockhashValid` (isBlockhashValid), `TokenAccountBalance`, `TokenSupply`, `TokenLargestAccounts`, `TokenAccountsByOwner`, `TokenAccountsByDelegate`                                                                                                     |
| `methods::history`, `methods::transaction_results`, `methods::address_history` | `Blocks`, `BlocksWithLimit`, `BlockTime`, `BlockCommitment`, `FirstAvailableBlock`, `MinimumLedgerSlot`, `SignaturesForAddress`, `Transaction`, `Block`, `TransactionsForAddress`                                                                                                                                                 |
| `methods::submission`, `methods::simulation`                                   | `SendTransaction`, `SignatureStatuses`, `FeeForMessage`, `RequestAirdrop`, `SimulateTransaction`                                                                                                                                                                                                                                  |

Some details to know:

- Account reads keep the encoding, data slices and allocation metadata. Compressed bytes stay encoded. Parsed program payloads keep their exact JSON.
- Raw token amounts are parsed from decimal strings straight into `u64`, separately from the display values.
- Address history returns at most 1000 signatures per page by default.
- Missing transaction metadata in old blocks and transactions is preserved, not invented.

The fetch helpers on `Client`, such as `fetch_encoded_account`, are covered in [Accounts and sysvars](accounts.md).

## Sending a transaction

`SendTransaction` takes a `SignedWire`. `SignedWire::new(&bytes)?` validates a complete signed legacy, v0 or v1 packet before anything is sent, and it remembers the primary signature. The node's reply must name that same signature, or the call fails with `Error::SubmissionSignatureMismatch`.

```rust
use solana_kit_client::{
    Client, Transport,
    methods::submission::{SendConfig, SendTransaction, SignedWire, WireEncoding},
};
use solana_kit_core::{Signature, transaction::Transaction};

async fn send<T: Transport>(
    client: &Client<T>,
    transaction: &Transaction,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let wire = transaction.encode(true)?;
    let signature = client
        .call(SendTransaction {
            wire: SignedWire::new(&wire)?,
            encoding: WireEncoding::Base64,
            config: SendConfig::default(),
        })
        .await?;
    Ok(signature)
}
```

A matching acknowledgement does not prove that the transaction was broadcast or confirmed. The client never retries and never assumes confirmation. `SendConfig::max_retries` only passes a retry count to the node. To wait for a commitment level, use the confirmation strategies in [Subscriptions](subscriptions.md).

## Transports

`Transport` is a one-method trait with a concrete associated future. It receives an owned `Request`, which holds the JSON `body` and the `max_reply_bytes` budget. It returns a `Reply` with the HTTP `status` and `body`.

```rust
use solana_kit_client::{Reply, Request, Transport, TransportError};
use std::future::{Ready, ready};

struct Fixed(&'static str);

impl Transport for Fixed {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, _request: Request) -> Self::Exchange<'_> {
        ready(Ok(Reply { status: 200, body: self.0.as_bytes().to_vec() }))
    }
}
```

A fixed-reply transport like this is useful in tests. A real adapter has these responsibilities:

- It owns the endpoint, authentication headers, TLS policy, header limits and the deadline.
- It enforces `max_reply_bytes` while receiving. The client checks the size again before parsing.
- It cancels its owned work when the exchange future is dropped.
- It never retries on its own.
- It reports failures as `TransportError::new(kind)` or `TransportError::with_source(kind, cause)`, with a `TransportFailureKind` of `Unavailable`, `Denied`, `Deadline`, `Cancelled`, `Io`, `BodyLimit`, `HeaderLimit` or `InvalidResponse`.

Request IDs run from 1 to `u32::MAX - 1` and never wrap. If two clients share one multiplexed connection, give each its own correlation namespace.

## Provider methods

`Method` is an open trait. You can add a provider-specific RPC method and still get the client's envelope, correlation and budget checks:

```rust
use serde::{Deserialize, Serialize};
use solana_kit_client::{Error, methods::Method};

#[derive(Serialize)]
struct ProviderStatus(Vec<String>);

#[derive(Deserialize)]
struct Status {
    healthy: bool,
}

impl Method for ProviderStatus {
    type Response = Status;
    const NAME: &'static str = "providerStatus";

    fn decode(&self, result: &str) -> Result<Status, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}
```

`decode` receives only the validated `result` fragment. For REST-style provider APIs, see the `rest` module and the [Helius](../services/helius.md) integration.

## Errors

`client.call` returns `solana_kit_client::Error`:

| Variant                                                 | Meaning                                                                           |
| ------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `Transport(TransportError)`                             | The adapter failed. Its `TransportFailureKind` and original cause are kept        |
| `Status(u16)`                                           | The HTTP status was outside 200 to 299                                            |
| `BodyLimit`, `DepthLimit`                               | The byte or JSON nesting budget was exceeded                                      |
| `Json`, `Envelope`                                      | The JSON or the JSON-RPC envelope was malformed                                   |
| `Correlation`                                           | The reply belongs to another request                                              |
| `Remote(RemoteError)`                                   | The node returned a JSON-RPC error. `code`, `message` and `data` are kept exactly |
| `Schema`                                                | The result did not match the method's typed schema                                |
| `SubmissionSignatureMismatch`                           | `sendTransaction` acknowledged a different signature                              |
| `IdExhausted`                                           | This client used up its request IDs                                               |
| `AccountData`, `AccountNotDecoded`, `DiagnosticContext` | Account payload and diagnostic boundary failures                                  |

`RemoteError.data` keeps the difference between missing and explicit `null`. It also keeps the exact JSON text, including numbers too large for a float. To map errors onto stable numeric codes, see [Errors](errors.md).

On `wasm32-unknown-unknown`, `TransportError` can hold non-`Send` sources, such as a browser JavaScript value. Errors that contain one are therefore not `Send` or `Sync` on that target.

## Status

> [!NOTE]
> The protocol, including all 54 methods with literal requests and typed results, runs against in-memory transports on native Rust, Chrome, WASI Preview 1 and 2, and the LeptosKit interpreter. The [complete catalog receipt](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/rpc-history.md) has the details. Network evidence belongs to each adapter, and adapter maturity differs by host. Check the runtime pages before relying on one.
