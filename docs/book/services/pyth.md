# Pyth

`solana_kit_pyth` is off-chain client code for Pyth price data. It has four parts: Wormhole and accumulator wire parsing, legacy and receiver price accounts, an optional Hermes HTTP client, and builders for the receiver program's posting instructions. The default crate is `no_std + alloc` and uses the `solana_kit_core` bounded codecs, so the same parsing code runs natively, in the browser and under WASI.

> [!IMPORTANT]
> Parsing validates structure and resource budgets. It does **not** authenticate guardians, emitters, signatures, Merkle paths, account owners or price freshness. Do those checks yourself, or rely on the on-chain receiver program to do them.

## Add the crate

```toml
[dependencies]
solana_kit_core = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_pyth = { git = "https://github.com/pina-rs/solana_kit" }
```

| Feature | Adds |
| --- | --- |
| `hermes` | `hermes` module: bounded JSON models and the runtime-neutral Hermes `Client` (pulls in `solana_kit_client`, `serde`, `serde_json`) |
| `crypto` | PDA helpers: `config_address`, `treasury_address`, `guardian_set_address` and `ReceiverAccounts::derive` |

The crate is unpublished (`publish = false`).

## Wire messages

Every parser takes a `solana_kit_core::codecs::Limits` value that caps the bytes and items it allocates.

```rust
use solana_kit_core::codecs::{Error, Limits};
use solana_kit_pyth::{
    PythPriceFeedMessage, parse_accumulator_update_data, parse_pyth_price_feed_message,
};

fn first_price(data: &[u8], limits: Limits) -> Result<PythPriceFeedMessage, Error> {
    let update = parse_accumulator_update_data(data, limits)?;
    let message = update.updates.first().ok_or(Error::InvalidValue)?.message;
    parse_pyth_price_feed_message(message, limits)
}
```

| Function | Parses |
| --- | --- |
| `parse_wormhole_vaa` | Wormhole v1 VAA envelopes |
| `get_guardian_set_index`, `get_vaa_signature_count` | Header fields only (narrower checks, as in the Dart helpers) |
| `trim_vaa_signatures` | Reduces the signature count. Unchanged input is borrowed, not copied. |
| `parse_pyth_wormhole_message` | The `AUWV` Merkle-root commitment |
| `is_accumulator_update_data`, `parse_accumulator_update_data` | `PNAU` accumulator updates |
| `parse_pyth_price_feed_message` | Variant-zero price feed messages |

Payloads and variable-length byte fields borrow from your input. On one point this crate deliberately differs from the Dart reference. It follows the official `AUWV` format, which the pinned Dart helper rejects. The reasoning is in [`docs/parity/pyth-wire.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/pyth-wire.md).

## Price accounts

`decode_pyth_price_account` decodes legacy price accounts into `PythPriceAccount`: exact integer samples, EMA state and an ordered list of publisher components, within your allocation limit. If you pass the slot your read was made at, it derives the effective status. More than `PYTH_MAX_SLOT_DIFFERENCE` (25) slots demotes the trading status, and the stored aggregate is left untouched.

`decode_price_update_v2_account` decodes receiver `PriceUpdateV2` accounts. A full verification has a 133-byte prefix (`PRICE_UPDATE_V2_FULL_SIZE`) and a partial one has 134 bytes (`PRICE_UPDATE_V2_PARTIAL_SIZE`). The Dart decoder always requires 134. `feed_id_hex` returns the 64-digit lowercase feed ID.

## Hermes

Enable `hermes` to get the models and a client for Hermes' three GET routes. The client takes your REST transport, a `subscriptions::Clock`, a retry `Policy` and JSON `Budgets`:

```rust
use solana_kit_client::{rest, subscriptions::Clock};
use solana_kit_pyth::hermes::{Client, Policy, Service};

fn client<T: rest::Transport<Service>, C: Clock>(transport: T, clock: C) -> Client<T, C> {
    Client::new(transport, clock, Policy::default(), rest::Budgets::default())
}
```

| Method | Route |
| --- | --- |
| `get_price_feeds(query, asset)` | `/v2/price_feeds`, with an optional symbol query and `AssetType` filter |
| `get_latest_price_updates(ids, options)` | Latest updates for the given feed IDs |
| `get_price_updates_at_timestamp(publish_time, ids, options)` | Updates at a signed Unix timestamp in seconds |

`UpdateOptions` selects the `Encoding` and the `parsed` and `ignore_invalid_price_ids` flags. `Policy::default()` allows 5000 ms per attempt and three retries, with delays of 100, 200 and 400 ms. Only I/O failures, elapsed deadlines, HTTP 408 or 429, and statuses of 500 or above are retried. `Policy::new` rejects a zero timeout and more than `MAX_HTTP_RETRIES` (32) retries. Each losing exchange is dropped before the backoff starts.

Prices and confidence values are arbitrary-precision integers, capped at 8192 text bytes each. `Price::approximate_f64` is the explicit lossy conversion. `decode_price_update` and `decode_price_feeds` decode stored responses with the same admission checks.

`hermes::DEFAULT_BASE_URL` is `https://hermes.pyth.network`. Bind the Hermes `Service` slot to an approved adapter such as `NativeRest`. The provider root, headers and bearer authentication all belong to that adapter.

> [!NOTE]
> Native TCP tests compose the client with `NativeRest` and `NativeHttp` against local fixture peers. Network composition on browser, WASI and the interpreted host still needs its own evidence.

## Receiver instructions

`ReceiverInstruction` builds `post_update` and `post_update_atomic`. Each has an exact Anchor discriminator, Borsh parameters and seven positional accounts:

```rust
use solana_kit_core::{Address, Instruction, codecs::{Error, Limits}};
use solana_kit_pyth::{
    MerklePriceUpdate, PostUpdateParams, ReceiverAccounts, ReceiverInstruction,
};

fn posted(
    accounts: ReceiverAccounts,
    encoded_vaa: Address,
    update: &MerklePriceUpdate<'_>,
) -> Result<Instruction, Error> {
    ReceiverInstruction::PostUpdate {
        accounts,
        encoded_vaa,
        params: PostUpdateParams::new(update),
    }
    .instruction(Limits::new(4096, 64))
}
```

`ReceiverAccounts::new` uses the standard receiver program, with the payer as write authority. Its fields are public, so you can override them. `PYTH_SOLANA_RECEIVER_PROGRAM`, `PYTH_WORMHOLE_PROGRAM` and `PYTH_PUSH_ORACLE_PROGRAM` match the pinned reference. `random_treasury_id` takes exactly one byte from an entropy callback you provide. The crate has no ambient RNG.

The builders do no I/O and no submission, and they authenticate nothing.

## Status

These slices cover the wire, accounts, Hermes and receiver instructions. They do not complete Pyth parity. The records are:

- [`docs/parity/pyth-wire.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/pyth-wire.md)
- [`docs/parity/pyth-accounts.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/pyth-accounts.md)
- [`docs/parity/pyth-hermes.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/pyth-hermes.md)
- [`docs/parity/pyth-receiver-instructions.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/pyth-receiver-instructions.md)
