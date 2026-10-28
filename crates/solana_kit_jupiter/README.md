# Solana Kit Jupiter

Jupiter Swap API v2, Price API v3 and Token API v2 clients for **off-chain application code**.
Use Pina or the Solana program libraries for on-chain programs.

The client reuses Kit's bounded REST transport. It owns no executor, origin,
credentials, retry loop or background task. Supply a transport that approves the
Jupiter service root, adds an optional `x-api-key`, denies redirects, enforces
receiving limits and deadlines, and cancels owned work when its future is dropped.
The same methods accept local Wasm futures and native Send-capable adapters.

```rust
use solana_kit_client::rest::{Budgets, Transport};
use solana_kit_jupiter::{Client, Error, Service, TokenItem};

async fn search<T: Transport<Service>>(transport: T) -> Result<Vec<TokenItem>, Error> {
    let client = Client::new(transport, Budgets::default());
    client.search_tokens("SOL", Some(10)).await
}
```

`get_prices` accepts at most fifty Kit addresses. `search_tokens`, `tagged_tokens`,
`category_tokens` and `recent_tokens` preserve the pinned Dart routes and query
defaults. Category and interval enums prevent unknown routes; `TokenLimit`
represents the category API's inclusive 1–100 range. Search retains its separately
specified optional integer limit. Query values remain data and cannot select an
origin or path.

Prices remain exact `serde_json::Number` values. Token price fields also accept the
reference numeric-string variants. Decimal counts fit `u8`, holder counts fit
`u64`, and invalid or overflowing known fields fail rather than becoming absent.
Block identifiers retain their string or integer text. JSON admission bounds
bytes, nesting, items and strings and rejects duplicate keys before decoding.
The standalone `decode_prices` and `decode_tokens` functions use the same boundary.

Missing and null optional fields are unavailable. Unknown fields are ignored.
Record and error diagnostics omit response data; bounded HTTP bodies remain
available explicitly. No price freshness, token verification, investment value,
swap safety or successful execution is inferred from an API response.

Normal publication remains disabled.

`get_order` requests a quote and an optional assembled transaction. Inspect
`encoded_transaction` with `decode_base64_swap_transaction`, review its accounts,
instructions and signed message, and sign it with the application's signers.
`execute_order` posts the caller's signed base64 transaction once and requires a
nonblank `request_id` from the accepted order. It preserves the identifier and
sends `lastValidBlockHeight` as a string when available. Execution responses keep
status, code, signature and error independently; a signature can accompany failure.
The application decides whether an execution succeeded and confirms landing.

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

`OrderRequest` preserves optional parameters and provider defaults. Extra query
parameters override built-in values, matching Dart; arbitrary values remain
encoded query data. Economic limits and taker requirements are enforced by the
provider. `Integer` retains signed decimal integers of arbitrary width without
floating point. It accepts number and decimal-string response variants, normalizes
leading zeroes, and rejects fractional, exponent and hexadecimal representations.
Expiry timestamps, basis points and execution codes use checked `i64` values.

`build_swap` returns nullable raw instructions for self-landing. Its ordered lookup
map preserves response key order and each table's on-chain address index order.
Missing or null lookup-address lists fall back to map keys; an explicit empty list
stays empty. `to_instruction`, `to_account_meta`, `to_lookup_table` and `to_lifetime`
perform explicit conversion into Kit-owned values. They reject missing privileges,
invalid addresses, malformed base64, oversized lookup tables and invalid lifetimes.
No instruction ordering, signing, RPC submission or table authenticity is inferred.

Use `decode_order`, `decode_build` and `decode_execution` for standalone response
decoding. These apply shared admission before typed allocation, require JSON objects
and retain the exact bounded response bytes in `raw_response`. Missing and null
optional fields stay absent. An unknown string order mode maps to `None`; nonstring
execution errors remain available in raw JSON, matching the reference. Other
malformed known fields reject rather than silently disappearing. Serde derives
alone do not provide the shared admission or raw-response retention boundary.

The conformance suite registers ten swap groups for native Rust, browser Wasm,
WASI and the existing LeptosKit guest harness. The fixtures exercise request wire
forms, exact precision, nullable models, malformed fields, Kit conversion,
HTTP/adapter errors, budgets and dropped-future cancellation. They establish
client contracts without making live Jupiter requests.
