# Durable nonce construction receipt

Solana Kit is exclusively an off-chain client SDK. Use Pina or official Solana
program libraries for on-chain programs.

`DurableNonce` records a strict 32-byte `Hash`, nonce account and authority.
`prepare_instructions` retains a matching first advance, replaces another
recognized advance, or prepends the canonical System instruction. It preserves a
matching writable authority signer and is idempotent by value.
`Lifetime::DurableNonce` compilation checks the first instruction and both
identities. A missing, malformed or misplaced advance returns
`InvalidNonceAdvance`. This is deliberately stricter than Dart's raw compiler,
which can compile nonce metadata without an advance.

`Message::nonce_advance` inspects the first instruction using merged privileges.
Decoded lifetime stays `Unresolved`. This operation does not authenticate nonce
state, balance, ownership, initialization, authority or validator eligibility.
A loaded v0 account requires lookup resolution and returns `InvalidLookup`.
A nonce that also signs after role merging fails the exact writable-nonsigner
recognizer, matching the pinned decompiler. The source authority may be writable.
No fetched state, RPC, confirmation, polling or expiry inference is added.

The existing immutable compiler and signature owners remain in use. Constructing
a changed nonce message creates fresh unsigned signature slots. Existing signed
bytes and signatures have no mutation API. Blockhash expiry metadata remains
explicit and is absent from received wire.

## Source and literal evidence

The reference is Dart commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46`.
`oracle.dart` calls its actual setter, compiler, encoder and decompiler. It emits
`dart-literals.tsv` for all three versions, unchanged and different token/account/
authority, matching advance, empty instructions, prefixed instructions,
payer-authority equality, duplicate nonce-authority roles, and zero and 255 hash
bytes. Every ordered instruction meta matches. Compilation matches all 30
messages byte-for-byte. All 30 Dart messages structurally decode with their exact
bytes retained. Actual validator nonce acceptance has not been tested.

The compiler uses the pinned Dart comparator's per-character ordering for all
static, loaded, and table groups. This places the prefix program `[9; 32]`, which
begins with `c`, before the RecentBlockhashes sysvar beginning with `S`.
The three original collation failures, their source probe, and unchanged literals
remain in the local Unit158 and Unit166 evidence. The oracle now requires exact
equality for every row, including when `EXACT_NONCE_PARITY=1` is set.

The fixture and oracle retain the source MIT notice in `DART-LICENSE`.
`receipt` validates all 746 canonical transaction IDs, immutable source contents,
actual SHA256 defining hashes and provenance. Its optional `--write` updates
only the explicit nonce symbol allowlist and preserves every earlier partial
mapping. Display, hash, equality, confirmation and general introspection remain
unpromoted. No declaration is complete.

## Reproduction

From the repository root, provide an existing explicit Dart package configuration
whose package roots point to the pinned checkout. No pub resolution is needed.

```sh
dart --packages="$PACKAGE_CONFIG" verification/durable-nonce/oracle.dart
cargo +1.89.0 test --manifest-path verification/durable-nonce/Cargo.toml --locked --offline
cargo +1.89.0 test --manifest-path verification/durable-nonce/Cargo.toml --no-default-features --locked --offline
cargo run --manifest-path verification/durable-nonce/Cargo.toml --bin receipt --locked --offline -- . "$DART_REFERENCE"
```

The isolated `no_std + alloc` library exports four pure verification groups.
Native execution covers preparation, wire boundaries, invalid prerequisites,
strict textual tokens and cryptographic signatures. Three Wasm target checks
compile those groups. Shared suite registration and browser, WASI and interpreted
host execution belong to later independent integration.
