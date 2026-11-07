# Surfpool

`solana_kit_surfpool` gives you typed requests for [Surfpool](https://github.com/solana-foundation/surfpool)'s `surfnet_*` cheatcodes. Use them to set up local account state, control the test clock, query confidential balances, profile transactions and upload program bytes. The requests run through the shared `solana_kit_client::Client`, so you can use any transport: native, browser, WASI or interpreted Wasm. The crate starts no executor and never retries.

The research target is Surfpool `v1.7.0` (commit `9be726da6456b58a3b39c099885c6c5904da4543`), pinned in [`docs/planning/references.toml`](https://github.com/pina-rs/solana_kit/blob/main/docs/planning/references.toml). That version is independent of this crate's own version.

## Add the crate

```toml
[dependencies]
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_core = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_surfpool = { git = "https://github.com/pina-rs/solana_kit" }
```

The optional `crypto` feature adds `get_ata`, which derives associated token addresses through Kit cryptography. `DEFAULT_SURFNET_ENDPOINT` is `http://127.0.0.1:8899`, the endpoint the Surfpool CLI uses by default.

## Run a cheatcode

Wrap an existing `Client` in `Surfnet` and pass requests to `execute`:

<!-- {=surfpoolFundExample|trim|codeBlock:"rust"} -->
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
<!-- {/surfpoolFundExample} -->

`Surfnet::rpc()` borrows the same client for ordinary Solana RPC calls, and `into_rpc()` hands it back to you. `Surfnet` also has small helpers that run cheatcodes in order and stop at the first failure: `fund_sol`, `fund_sol_many`, `fund_token`, `fund_token_many`, `set_token_balance`, `set_token_account` and `minimum_balance`.

When a test also needs subscriptions and a payer, `SurfpoolClient::new(rpc, subscriptions, payer)` holds all three. You still own each of them and decide when to drop it.

## The cheatcodes

All 28 methods from the pinned Dart Surfpool package have a public request type in `cheatcodes`. The builders in `builders` cover accounts, tokens, resets and streams.

| Area | Wire methods |
| --- | --- |
| Accounts and tokens | `surfnet_setAccount`, `surfnet_setTokenAccount`, `surfnet_resetAccount`, `surfnet_offlineAccount`, `surfnet_cloneProgramAccount`, `surfnet_streamAccount`, `surfnet_streamAccounts`, `surfnet_getStreamedAccounts` |
| Programs and IDLs | `surfnet_writeProgram`, `surfnet_setProgramAuthority`, `surfnet_registerIdl`, `surfnet_getActiveIdl` |
| Clock | `surfnet_timeTravel`, `surfnet_pauseClock`, `surfnet_resumeClock` |
| Network | `surfnet_setSupply`, `surfnet_getSurfnetInfo`, `surfnet_resetNetwork`, `surfnet_exportSnapshot`, `surfnet_registerScenario`, `surfnet_getLocalSignatures` |
| Cheatcode control | `surfnet_enableCheatcode`, `surfnet_disableCheatcode` |
| Profiling | `surfnet_profileTransaction`, `surfnet_getTransactionProfile`, `surfnet_getProfileResultsByTag` |
| Confidential transfers | `surfnet_getConfidentialBalance`, `surfnet_deriveConfidentialKeys` |

Numeric requests use `u64`. Typed response integers use `i128`, which covers the full signed and unsigned 64-bit range on every host. Optional arguments are left out of the request. To clear an authority, the request sends the provider's literal string `"null"`.

`Response<T>` holds the decoded value and keeps the envelope's context. Call `into_value()` if you only want the value. `SurfpoolRpcError` keeps the method name and the original client error in its `cause` field. `Debug` and `Display` show only the method name, because the cause can contain provider values.

## Control the clock

```rust
use solana_kit_client::Transport;
use solana_kit_surfpool::{
    Surfnet,
    cheatcodes::{PauseClock, ResumeClock, TimeTravel},
    errors::SurfpoolRpcError,
    types::TimeTarget,
};

async fn jump<T: Transport>(surfnet: &Surfnet<T>, slot: u64) -> Result<(), SurfpoolRpcError> {
    surfnet.execute(PauseClock).await?;
    surfnet.execute(TimeTravel(TimeTarget::AbsoluteSlot(slot))).await?;
    surfnet.execute(ResumeClock).await?;
    Ok(())
}
```

`TimeTarget` also has `AbsoluteEpoch` and `AbsoluteTimestamp` (Unix milliseconds). While the clock is paused, a default finalized read can lag behind. Read with `Commitment::Processed` if you need to see the slot you travelled to.

## Deploy program bytes

`Surfnet::deploy(DeployOptions::new(program_id, &bytes))` writes the program in order, one chunk per request, and then registers the IDL if you set one. The default chunk is 3 KiB of raw bytes. That fits the client's default 8 KiB wire budget, and also Surfpool 1.7.0's 15 MiB request-body cap, because hex encoding doubles the bytes. If you choose a larger `chunk_bytes`, it must fit both limits. A failed deployment leaves earlier writes in place and never replays them.

On native hosts, `solana_kit_native`'s `surfpool` feature adds file-based deployment (`surfpool::deploy`), artifact discovery and `OwnedSurfpool` process ownership on top of this crate. See [Native](../runtimes/native.md).

## Prepare CLI arguments

`SurfnetConfig` keeps the upstream defaults. `SurfnetConfig::start_args(ports, payer)` produces the exact `surfpool start` arguments for three distinct, caller-chosen ports (`config::Ports::new(rpc, websocket, studio)`). It never creates a process or reads the environment.

## Testing against a real Surfpool

The portable crate has no CLI launcher, port probe, random keypair generation, log capture or process shutdown. A separate host layer has to provide those.

The repository's private `verification/surfpool-native` binary runs stock Surfpool 1.7.0 through `NativeHttp`, `Client` and `Surfnet`. It needs a verified executable, a fresh private runtime directory, a fresh genesis hash, three ports and a macOS sandbox profile that blocks outbound network. Its journey checks startup funding, exact account bytes after `SetAccount`, and pause, time travel, processed read and resume. An optional `--body-cap` probe expects HTTP 413 for an oversized `WriteProgram`.

```text
solana_kit_surfpool_native_verification BIN PROFILE NEW_RUNTIME GENESIS NONCE RPC WS STUDIO [--body-cap]
```

> [!WARNING]
> Surfpool 1.7.0 makes a cloud version request even with `--offline` and `NO_DNA`. Clearing the environment is not a sandbox. If you run Surfpool in tests, contain it at the OS level.

The status of this testing, plainly:

- Two local runs against stock 1.7.0 passed the journey on Darwin arm64. A later strict replay of the oversized-request probe failed, and its cause has not been diagnosed.
- No CI workflow runs the real server journey yet.
- WebSocket delivery, signed System transfers, automatic ports and a public native lifecycle API remain next-phase work.

The details are in [`verification/surfpool-native/README.md`](https://github.com/pina-rs/solana_kit/blob/main/verification/surfpool-native/README.md).
