# Independent Token-2022 data controls

These public controls compile the complete output from the existing immutable-IDL
generator. Generate that output with `token-2022-data` or `token-2022-proof`, then
select its absolute `generated.rs` path through `TOKEN_LAYOUTS_GENERATED`.

```sh
TOKEN_LAYOUTS_GENERATED=/absolute/generated.rs cargo test --offline --locked --manifest-path verification/token-layouts-independent/Cargo.toml --lib
```

The terminator controls accept either an encoder error or an exact value round
trip. They fail when encoding succeeds and decoding silently removes extensions.
The literal TLV, transaction, malformed-frame, duplicate-entry, and capacity
controls exercise the same public emitted codecs and core Reader APIs.

The 355-byte observation records a structural codec boundary. It does not claim
that named codecs implement size-based account dispatch or all program-state and
extension-affinity checks. These are off-chain client data controls.

Account TLV collection encoding rejects any zero-tag entry with `Error::InvalidValue`.
The new integration tests put that entry before, between and after real entries in
both Mint and Token. `encoded_len`, `write` and `encode` all reject it. A failed
account write may retain its base fields, as the public Encoder contract permits.
The collection preflight writes no extension bytes. Empty and valid collections
still round-trip. Standalone `Extension::Uninitialized` retains its two zero bytes.
