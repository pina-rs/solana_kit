# solana_kit_surfpool

Typed Surfpool cheatcodes for off-chain Solana applications. Use this library to
prepare local account state, control the test clock, query confidential balances,
profile transactions and upload caller-provided program bytes.

The external CLI research target is Surfpool `v1.7.0`, commit
`9be726da6456b58a3b39c099885c6c5904da4543`. Release asset digests are pinned in
[`docs/planning/references.toml`](../../docs/planning/references.toml). A verified
Darwin arm64 binary reports `surfpool 1.7.0`; an actual server compatibility
journey remains pending. This target is independent of this SDK crate's version.

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
pinned Dart 15 MiB. Surfpool 1.7.0 caps each HTTP request body at 15 MiB: raw
program bytes expand to exactly twice their length in hex, plus JSON framing.
The default raw chunk therefore exceeds that server cap. Choose a smaller
nonzero chunk that fits both the server cap and the client's complete wire budget.

Current preparation also copies a configured chunk and allocates its full hex
string before wire rejection; native allocation regressions await execution and
repair. A failed deployment leaves prior writes visible and never
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
