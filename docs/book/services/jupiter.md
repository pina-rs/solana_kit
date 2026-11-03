# Jupiter

`solana_kit_jupiter` is a client for Jupiter's Swap API v2, Price API v3 and Token API v2. It reuses the SDK's bounded REST transport and owns nothing else: no executor, origin, credentials, retry loop or background task. You supply a `rest::Transport<Service>`. It approves the Jupiter service root, adds an optional `x-api-key`, denies redirects, and enforces receive limits and deadlines.

> [!NOTE]
> The crate is unpublished (`publish = false`). Its conformance groups exercise request wire forms, precision, decoding and cancellation against local fixtures. They make no live Jupiter requests.

## Add the crate

```toml
[dependencies]
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_core = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_jupiter = { git = "https://github.com/pina-rs/solana_kit" }
```

The crate has no optional features. For a REST adapter (`NativeRest`, `BrowserRest`, `WasiRest` or the LeptosKit route), see [Choosing a runtime](../runtimes/index.md). `DEFAULT_BASE_URL` is `https://api.jup.ag`. Your adapter decides whether to use it.

## Prices and tokens

```rust
use solana_kit_client::rest::{Budgets, Transport};
use solana_kit_jupiter::{Client, Error, Service, TokenItem};

async fn search<T: Transport<Service>>(transport: T) -> Result<Vec<TokenItem>, Error> {
    let client = Client::new(transport, Budgets::default());
    client.search_tokens("SOL", Some(10)).await
}
```

| Method | Route | Notes |
| --- | --- | --- |
| `get_prices(&[Address])` | `GET /price/v3` | At most `MAX_PRICE_MINTS` (50) mints. Missing prices are omitted. |
| `search_tokens(query, limit)` | `GET /tokens/v2/search` | Optional integer limit |
| `tagged_tokens(tag)` | `GET /tokens/v2/tag` | Any provider tag, such as `verified` or `lfg` |
| `category_tokens(category, interval, limit)` | `GET /tokens/v2/{category}/{interval}` | `Category` and `Interval` enums. `TokenLimit` is 1 to 100. |
| `recent_tokens()` | `GET /tokens/v2/recent` | Provider order is preserved |

Prices stay exact `serde_json::Number` values and never pass through a float. If a known field is malformed or overflows, decoding fails rather than leaving it absent. You can also decode stored responses yourself with `decode_prices` and `decode_tokens`. They apply the same JSON admission budgets.

## Swaps

A swap has three steps: get an order, sign it yourself, then execute it.

```rust
use solana_kit_client::rest::{Budgets, Transport};
use solana_kit_core::Address;
use solana_kit_jupiter::{Client, Error, Integer, OrderRequest, OrderResponse, Service};

async fn quote<T: Transport<Service>>(
    transport: T,
    input_mint: Address,
    output_mint: Address,
) -> Result<OrderResponse, Error> {
    let request = OrderRequest::new(input_mint, output_mint, Integer::from(1_000_000));
    Client::new(transport, Budgets::default()).get_order(&request).await
}
```

<Steps>

### Request an order

`Client::get_order(&OrderRequest)` calls `GET /swap/v2/order`. The response has a quote and, optionally, an assembled transaction. `OrderRequest` has public optional fields such as `slippage_bps`, `swap_mode`, `taker` and `payer`. Use `extra_query_parameters` for anything else. Extra parameters override the built-in ones.

### Inspect and sign

Decode `encoded_transaction` with `decode_base64_swap_transaction`. Review its accounts, instructions and message, then sign it with your own signers. See [Signers](../core/signers.md).

### Execute once

`Client::execute_order(&order, signed_base64)` posts to `POST /swap/v2/execute` exactly once. It requires the order's non-blank `request_id`. A signature in the response can come with a failure, so check `status` and `code` yourself and confirm that the transaction landed.

</Steps>

To land the transaction yourself, `Client::build_swap` (`GET /swap/v2/build`) returns raw instructions and lookup tables instead. Convert them explicitly with `to_instruction`, `to_account_meta`, `to_lookup_table` and `to_lifetime`. These reject missing privilege flags, bad addresses, malformed base64 and oversized tables. The client does not infer instruction ordering or verify that a lookup table is authentic.

`Integer` holds arbitrary-width signed decimal integers. It accepts both number and string forms and rejects fractions, exponents and hex. `decode_order`, `decode_build` and `decode_execution` decode standalone responses and keep the exact bytes in `raw_response`.

## What a response does not tell you

The client infers nothing about price freshness, token verification, swap safety or successful execution from an API response. Economic limits and taker requirements are enforced by the provider, not the client.

The contract mapping to the Dart source is in [`docs/verification/jupiter-swap.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/jupiter-swap.md).
