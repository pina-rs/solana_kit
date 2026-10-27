# Pyth wire compatibility

This is a partial Pyth port. Hermes, JSON feeds, legacy price accounts,
PriceUpdateV2, instruction builders and PDA helpers remain pending. No wire test
establishes complete Pyth or ecosystem parity.

The Dart reference is commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46` of
<https://github.com/openbudgetfun/solana_kit>. Current protocol structure and the
golden transfer are pinned to `918664828173a60a2cde1f8431474efb02a6b75f` of
<https://github.com/pyth-network/pyth-crosschain>.

| Reference owner                                        | Rust behavior                                                                                                                                                     |
| ------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| VAA envelope, guardian headers and signature reduction | Same field widths, byte order, encoded signature order and narrower header/trim contracts. Borrow unchanged reductions.                                           |
| Price message                                          | Same exact integer fields, known-prefix decoding and lowercase feed identifier text.                                                                              |
| Wormhole verification message                          | Parse official `AUWV`, variant zero, slot, ring size and 20-byte root. Reject the synthetic Dart version/kind format.                                             |
| Accumulator transfer                                   | Borrow variable byte fields; bound allocated update and proof entries. Validate the actual proof variant. Preserve compatible minor versions and extension bytes. |

The verification-message difference is intentional. Executing the pinned Dart
helper against the upstream fixture produces `Unsupported Pyth wormhole message
version 65`. The primary protocol defines `AUWV`; its first byte is decimal 65.
The Rust fixture test must parse that actual message successfully.

The Dart accumulator parser skips the proof-variant byte, restricts minor versions
to zero and rejects trailing bytes. The primary serde format has a closed proof
enum and explicitly permits append-only data. Rust rejects unknown proof variants
and preserves the compatible extension data instead of discarding it.

Native cases live in `crates/solana_kit_pyth/tests/wire.rs`. The three registered
groups in `verification/conformance/src/pyth.rs` use the same public client APIs
and original fixture in the native, browser, WASI and production wasmi harnesses.
Actual CI execution remains required for each platform.
