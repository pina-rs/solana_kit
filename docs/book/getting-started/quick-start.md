# Quick start

This page builds the smallest useful program: a native binary that connects to devnet, reads an account balance, and fetches the latest blockhash. It shows the pattern every Solana Kit program follows: you pick a transport, wrap it in a `Client`, and call typed methods.

## Set up the project

```sh
cargo new kit-quick-start
cd kit-quick-start
```

Add the dependencies to `Cargo.toml`:

```toml
[dependencies]
solana_kit_core = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_native = { git = "https://github.com/pina-rs/solana_kit" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

See [Installation](installation.md) for pinning a revision and for the optional features.

## Write the program

Replace `src/main.rs` with:

```rust
use solana_kit_client::{
    Client, Limits,
    methods::{Balance, LatestBlockhash},
};
use solana_kit_core::Address;
use solana_kit_native::{HttpConfig, NativeHttp};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. A transport owns the endpoint, TLS, headers and deadlines.
    let transport = NativeHttp::new("https://api.devnet.solana.com", HttpConfig::default())?;

    // 2. The client owns JSON-RPC framing, request IDs and parsing budgets.
    let client = Client::new(transport, Limits::default());

    // 3. Typed methods return typed values.
    let address: Address = "11111111111111111111111111111111".parse()?;
    let balance = client.call(Balance(address, None)).await?;
    println!("slot {}: {} lamports", balance.context.slot, balance.value);

    let latest = client.call(LatestBlockhash(None)).await?;
    println!(
        "blockhash {} is valid through block height {}",
        latest.value.blockhash, latest.value.last_valid_block_height
    );

    Ok(())
}
```

Run it:

```sh
cargo run
```

The output looks like this (your numbers will differ):

```sh
slot 518379988: 1 lamports
blockhash mLYTfpBhtRA7KxGQZje6j4SPo4AdjL35ibpLC39GRcR is valid through block height 505541747
```

## What just happened

**`NativeHttp::new`** checks the endpoint and builds a Tokio HTTP/1 transport. It does not touch the network. HTTPS always verifies the certificate chain and host name, and you cannot turn that off. Use `HttpConfig` to set headers (for example an API key), deadlines and trust roots.

**`Client::new`** wraps any `Transport`. The second argument, `solana_kit_client::Limits`, caps the request and response body size and JSON nesting depth. `Limits::default()` allows 8192 bytes and depth 32. That is enough for small reads; raise `bytes` before fetching large accounts:

```rust
let client = Client::new(transport, Limits { bytes: 64 * 1024, depth: 32 });
```

**`client.call(method)`** sends one typed JSON-RPC request. Each method is a plain struct in `solana_kit_client::methods`: `Balance(address, config)` maps to `getBalance`, `LatestBlockhash(config)` maps to `getLatestBlockhash`. Methods that report a context return `Contextual<T>`, which holds the node's `context.slot` next to the `value`.

**`None`** in `Balance(address, None)` means "use the node's default commitment". Pass a `ReadConfig` to choose one:

```rust
use solana_kit_client::methods::{Commitment, ReadConfig};

let config = ReadConfig {
    commitment: Some(Commitment::Confirmed),
    ..ReadConfig::default()
};
let balance = client.call(Balance(address, Some(config))).await?;
```

> [!NOTE]
> The client never retries, never reads credentials from the environment, and never starts a runtime. If a call fails, the returned `solana_kit_client::Error` tells you whether the failure came from the transport, the HTTP status, the JSON framing, the response schema, or the node's own JSON-RPC error.

All 54 RPC catalog methods have typed implementations. They are grouped under `methods::node`, `methods::economics`, `methods::accounts`, `methods::tokens`, `methods::history`, `methods::submission`, `methods::transaction_results`, `methods::address_history` and `methods::simulation`. [RPC](../core/rpc.md) covers them.

## Other runtimes

The `Client` code above is the same everywhere. Only the transport changes: `BrowserHttp` from `solana_kit_browser` in the browser, `WasiHttp` from `solana_kit_wasi` on WASI Preview 2, and `GuestHttp` from `solana_kit_leptoskit` in a LeptosKit guest. Each has its own setup; see [Choosing a runtime](../runtimes/index.md).

## Next steps

<Cards>
<Card title="Addresses and keys" href="addresses-and-keys.md">Parse addresses, derive PDAs and create keypairs.</Card>
<Card title="Fetch an account" href="fetch-an-account.md">Read and decode account data.</Card>
<Card title="Build a transaction" href="build-a-transaction.md">Compile a System transfer.</Card>
</Cards>
