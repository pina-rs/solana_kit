# Solana Kit Pyth

Portable off-chain client code for Pyth wire messages. This crate uses
`no_std + alloc` and the existing `solana_kit_core` bounded codecs. Browser Wasm,
WASI and native clients use the same parsing code without a runtime dependency.
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

Hermes requests, JSON feeds, legacy price accounts, PriceUpdateV2 accounts,
instruction builders and PDA helpers remain implementation work. Wire support
alone does not establish full Pyth or ecosystem parity.
