# Solana Kit Jupiter

Jupiter Price API v3 and Token API v2 clients for **off-chain application code**.
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

This phase implements the price and token surface. Swap order, build, execution
and transaction decoding remain separate delivery work. Normal publication is
disabled; a future name reservation is version `0.0.0` only.
