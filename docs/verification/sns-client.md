# Offline SNS client contracts

The public `solana_kit_sns` crate ports the offline exports of the pinned Dart
SNS package at `0138bb956d2d81ccc0c56edc602401c0df2f2b46`. It uses Kit's
cryptography and has no RPC or on-chain program implementation. Its MIT source
notice is retained with the crate. Publication remains disabled.

The source contracts cover all 28 record labels and ten program addresses,
SHA-256 and domain PDA derivation, subdomains, V1 and V2 records, and reverse
lookups. Registry data has a 96-byte header; name values use a little-endian
UTF-8 byte length, remove an initial BOM on decoding, and may have trailing
account allocation. Reverse values retain or remove the leading NUL marker as
requested. V2 records require the 104-byte account and record headers before
exposing lazy, bounded identifier and content slices. Identifier bytes do not
establish staleness or association validation.

The `solana-kit-core-v46` registry preserves the preceding 388 cases and appends
four SNS groups, for 392 canonical cases. The corruption control is expected to
pass 391 of 392. These are source expectations, not host execution receipts.
Native, browser, WASI, wasmi, and generated desktop and mobile artifacts require
fresh execution. No RPC resolution, network access, or full ecosystem parity is
claimed by this phase.
