# Pyth account compatibility

This phase ports the legacy account types, price samples, EMA state, publisher
components, operational flags and receiver price-update accounts from the Dart
`price_account.dart` API at `0138bb956d2d81ccc0c56edc602401c0df2f2b46`.
It uses Kit-owned addresses and codecs without Solana Rust dependencies.

| Dart contract                                         | Rust contract                                                                                           |
| ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| Numeric account, price-type and status enums          | Explicit numeric variants and `from_value`; unknown values map to `Unknown`.                            |
| Price sample and EMA `fromBytes(data, offset)`        | `PythPriceInfo::from_bytes` and `PythEma::from_bytes` with checked offsets and caller limits.           |
| Legacy account fields and ordered publishers          | `PythPriceAccount` retains all fields, exact integers and bounded publisher storage.                    |
| Optional `currentSlot`                                | Caller-provided `Option<u64>`; more than 25 slots demotes only the effective trading status.            |
| Full or partial verification                          | `PythVerificationLevel`; full sorts above all partial counts, with an optional numeric signature count. |
| Receiver feed, price, confidence, time and EMA fields | `PriceUpdateV2Account::price_message` uses the existing `PythPriceFeedMessage` value type.              |
| Receiver feed identifier text                         | `PriceUpdateV2Account::feed_id_hex` returns 64 lowercase digits.                                        |

The primary receiver layout is pinned to Pyth crosschain revision
`918664828173a60a2cde1f8431474efb02a6b75f`. Its Borsh enum serializes `Full` as one
byte and `Partial` as a tag plus one signature-count byte. The serialized account
prefix is therefore 133 or 134 bytes. The program allocates the larger size.
The Dart decoder always demands 134 bytes, even for the compact full variant.
Executing it with a 133-byte full vector reproduces that rejection; Rust follows
the variant's actual field layout and accepts both compact and allocated forms.

The legacy C-layout reference is Pyth SDK revision
`4b309c9b6bf5229723470bee1c3ab581482c4421`. No unsafe casts or native struct-layout
assumptions are used. Unsigned slots and confidences retain all 64 bits. Zero
next-account keys become `None`; unknown corporate markers and account tail bytes
are preserved. Publisher bytes must exist before any vector allocation.

Hermes and JSON APIs, PDA helpers and instruction builders remain pending.
Native tests and three portable conformance groups exercise the public account
APIs. Browser, WASI and production wasmi execution must still pass for this phase.
