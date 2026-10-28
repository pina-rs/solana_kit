# solana_kit_surfpool

Typed Surfpool cheatcodes for off-chain Solana applications. Use this library to
prepare local account state, control the test clock, query confidential balances,
profile transactions and upload caller-provided program bytes.

All 28 methods from the pinned Dart Surfpool package are represented by public
request types. Requests run through `solana_kit_client::Client` and its existing
transport, correlation IDs, JSON admission, byte limits and depth limits. Supply
the native, browser, WASI or interpreted Wasm transport appropriate to your host.
Futures may retain local `Rc` values. The library starts no executor or retries.

```rust
use solana_kit_client::{Client, Transport};
use solana_kit_core::Address;
use solana_kit_surfpool::{Surfnet, builders::SetAccount};
use solana_kit_surfpool::errors::SurfpoolRpcError;

async fn fund<T: Transport>(rpc: Client<T>, address: Address)
    -> Result<(), SurfpoolRpcError>
{
    let surfnet = Surfnet::new(rpc);
    surfnet.execute(SetAccount::new(address).with_lamports(10_000_000_000)).await?;
    Ok(())
}
```

`cheatcodes` contains the complete method family. `builders` provides account,
token, reset and stream builders. Numeric requests use `u64`; typed response
integers use `i128` to retain the entire signed and unsigned 64-bit domain on
every host. Open provider objects use exact `serde_json` numbers. Optional
arguments are omitted; explicit `false` and the profiling null placeholder are
retained. Authority clearing sends the provider's literal string `"null"`.

`Response<T>` exposes the decoded value and retains contextual envelope metadata.
Use `into_value()` for the pinned Dart cheatcode convenience behavior. RPC errors
retain the method and the original shared error, including HTTP status, remote
code, original message, raw data and transport source. Error `Debug` and `Display`
include only approved method names, and the implicit error source chain stops at
`SurfpoolRpcError`. Inspect its `cause` field explicitly for original details;
those details can contain sensitive provider values or adapter text.

`SurfnetConfig` preserves upstream defaults and prepares exact CLI arguments
with three explicit, distinct ports. `DeployOptions` uploads borrowed bytes in
sequence, then registers an optional parsed IDL. Its default chunk size is the
pinned 15 MiB; configure the client's JSON budget accordingly, or choose a smaller
nonzero chunk size. A failed deployment leaves prior writes visible and never
replays them. The optional `crypto` feature derives associated token addresses
through existing Kit cryptography.

`SurfpoolClient` composes the RPC owner with caller-provided subscriptions and
payer. Host adapters own endpoints, authentication, deadlines and cancellation.
Drop or recover these owners explicitly when their work is finished.

This package contains no CLI launcher, port probe, random keypair generation,
process shutdown, log capture, filesystem artifact discovery or automatic signer
disposal. Those native Dart conveniences still need a separate host layer.
There is no on-chain program implementation: use Pina or the Solana program
libraries for on-chain work. The shared Kit client accepts all HTTP 2xx statuses;
the pinned Dart internal client accepted only 200.

The `tests/support` public API fixtures are also registered in the integrated
conformance suite for native, browser, WASI and interpreted Wasm execution. Their
presence does not establish that any host has executed them.

Source provenance and retained license notices appear in
`THIRD_PARTY_NOTICES.md` and `MIT-DART.txt`. Original Rust code uses the Unlicense.
