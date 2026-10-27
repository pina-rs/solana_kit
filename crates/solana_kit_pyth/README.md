# Solana Kit Pyth

Portable off-chain client code for Pyth wire messages, price accounts and Hermes
response models. The default crate uses `no_std + alloc` and the existing
`solana_kit_core` bounded codecs. Browser Wasm, WASI and native clients use the
same parsing code without an executor dependency. Optional Hermes JSON support
uses the portable Kit client admission layer and exact integer support.
On-chain programs should use Pina or official Solana program libraries.

The current implementation parses Wormhole v1 envelopes, official `AUWV` Merkle
commitments, variant-zero price feed messages and `PNAU` accumulator updates.
Payloads and variable byte fields borrow the original input. Allocated signature,
update and proof entries share the caller's item limit within each parse operation. Signature trimming borrows
unchanged inputs and allocates only when reducing the encoded signature count.
For a copied reduction, `max_items` counts output bytes, including the preserved
body and payload. Borrowed reductions allocate nothing.

Parsing validates structure and budgets. It does not authenticate guardians,
emitters, signatures, Merkle paths or prices. Header inspection and trimming have
the pinned Dart helpers' narrower contracts; use the complete envelope parser
when those helpers are applied to unvalidated input.

Legacy price accounts decode exact integer samples, rational EMA state and
ordered publisher components with a caller-selected allocation limit. An optional
read-context slot derives the effective status without rewriting the stored
aggregate. Receiver `PriceUpdateV2` accounts reuse the price-message fields with
Borsh little-endian decoding: full verification has a 133-byte prefix, partial
verification has a 134-byte prefix, and remaining account bytes are borrowed.
Account parsing establishes no owner, guardian authentication or price freshness.

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

The reference Dart Wormhole-message helper rejects the upstream fixture's actual
`AUWV` payload. This Rust implementation follows the official magic, variant,
slot, ring size and 20-byte root. It also rejects unknown proof variants and
preserves upstream-compatible minor versions and append-only extensions.

Enable `hermes` for bounded JSON price updates and feed metadata. Price and
confidence components retain arbitrary precision, including values beyond
128 bits. The shared JSON admission layer checks bytes, nesting, items, strings
and duplicate keys before typed decoding. Each integer component is limited to
8192 text bytes before conversion. `Price::approximate_f64` explicitly permits
floating-point loss; the stored integer remains exact.

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# #[cfg(feature = "hermes")]
# {
use solana_kit_client::rest::Budgets;
use solana_kit_pyth::hermes::{Encoding, decode_price_update};

let update = decode_price_update(
    br#"{"binary":{"encoding":"hex","data":[]}}"#,
    Budgets::default(),
)?;
assert_eq!(update.binary.encoding, Encoding::Hex);
assert!(update.parsed.is_none());
# }
# Ok(())
# }
```

Metadata equality includes all attribute entries. Malformed attribute values are
errors. Missing or null attributes and binary chunk lists become empty; absent
parsed results remain distinct from an explicitly empty result list.

The same feature provides a runtime-neutral Hermes `Client` for feed listings,
latest updates and updates at a signed Unix timestamp. It uses an existing
`rest::Transport<hermes::Service>` and `subscriptions::Clock` supplied by the
application. Native adapters can retain Send futures; browser and interpreted
guests can use local futures. Construction starts no I/O or task.

```rust
# #[cfg(feature = "hermes")]
# {
use solana_kit_client::{rest, subscriptions::Clock};
use solana_kit_pyth::hermes::{Client, Policy, Service};

fn client<T: rest::Transport<Service>, C: Clock>(transport: T, clock: C) -> Client<T, C> {
    Client::new(transport, clock, Policy::default(), rest::Budgets::default())
}
# }
```

`Policy::default()` grants 5000 ms per attempt and three retries, with delays of
100, 200 and 400 ms. Only I/O failures, elapsed exchange deadlines, HTTP 408/429
and statuses at least 500 retry. Policy denial, cancellation, resource limits,
clock failures and malformed success bodies stop immediately. Timing policy
validation rejects zero timeout, delay overflow and more than 32 retries.
Every losing exchange is dropped before backoff; dropping the whole operation
also drops its exchange and timer. HTTP errors retain their bounded original
body, while ordinary diagnostics hide response and adapter values.

Provider roots, custom headers and bearer authentication remain explicit
application adapter policy. Bind the Hermes `Service` slot to an approved
`NativeRest`, `BrowserRest`, `WasiRest` or LeptosKit REST route. The default
provider root is `hermes::DEFAULT_BASE_URL`; self-hosted path prefixes are
preserved by the shared request-target encoder. No ambient secrets are resolved.
Actual host-network composition evidence for this client remains follow-up work.
Pyth instruction builders and PDA helpers also remain open. This slice does not
establish full Pyth or ecosystem parity.
