# Address adjudication pilot

This note records the first reviewed use of
[`adjudications.tsv`](adjudications.tsv) for the twelve `solana_kit_address`
declarations. It does not complete the package, the phase, or the SDK.

## Scope and identity

The manifest is keyed by complete canonical ID and the defining source hash from
[`source-files.tsv`](source-files.tsv). The pilot spans four pinned files:

| Source                            | SHA256                                                             |
| --------------------------------- | ------------------------------------------------------------------ |
| `lib/src/address.dart`            | `d177e9e7d459923b1b816d76122424b3dae2c04d4f94b32cb545cd1946480221` |
| `lib/src/address_codec.dart`      | `c178cea779a85cd35a143d44e3f45dcf3c0e983ef9a5eb79bd530eec8660a002` |
| `lib/src/address_comparator.dart` | `d096642ea02001eeffdc754c4ef79be5fe2c1c69b45ca55de374a1a580b21079` |
| `lib/src/public_key.dart`         | `f9a565f9d0d6f8e96f1c2041544355b093c6c432198d16e614443daba1c0609d` |

Regeneration leaves `id`, `package`, `path`, `symbol`, `kind`, `line`, `owner`,
`phase`, `unit`, `sha256` and `provenance` byte-identical. Only `status`,
`evidence` and `notes` for these twelve rows differ from the previous inventory.

## Exact mapping

| Canonical ID suffix        | Rust equivalent                   | Status   | Executed case                       |
| -------------------------- | --------------------------------- | -------- | ----------------------------------- |
| `#Address`                 | `Address` byte newtype            | partial  | `address-base58-golden`             |
| `#Address.new`             | `Address::from_bytes`             | partial  | `public-address-collation-display`  |
| `#Address.value`           | `to_bytes`, `as_bytes`, `Display` | partial  | `address-base58-golden`             |
| `#address`                 | `str::parse::<Address>`           | partial  | `address-base58-golden` and rejects |
| `#assertIsAddress`         | `str::parse::<Address>`           | partial  | `address-base58-golden` and rejects |
| `#isAddress`               | `str::parse::<Address>().is_ok()` | partial  | `address-base58-golden` and rejects |
| `#getAddressCodec`         | `to_bytes` plus `TryFrom<&[u8]>`  | partial  | `address-base58-golden`             |
| `#getAddressDecoder`       | `TryFrom<&[u8]>`                  | partial  | `fixed-value-byte-lengths`          |
| `#getAddressEncoder`       | `to_bytes`                        | partial  | `address-base58-golden`             |
| `#getAddressComparator`    | `compare_addresses`               | complete | `public-address-collation-display`  |
| `#getAddressFromPublicKey` | `Address::try_from(&[u8])`        | partial  | `fixed-value-byte-lengths`          |
| `#getPublicKeyFromAddress` | `Address::to_bytes`               | complete | `address-base58-golden`             |

The rejection cases are `address-invalid-alphabet`, `address-invalid-length` and
`address-bounded-input`. Full IDs and the reviewed notes are in the manifest.

## Why only two rows are complete

`getAddressComparator` is complete because `public-address-collation-display`
sorts the full base58 rank alphabet, checks equality for identical values, and
checks that a prefix sorts before its extension. That is the exact comparator
contract, and the case is part of the shared `solana-kit-core-v56` suite.

`getPublicKeyFromAddress` is complete because `address-base58-golden` asserts the
exact 32 bytes for a base58 address and reproduces the canonical text. The Rust
type already guarantees that every constructible `Address` holds 32 bytes, so
`to_bytes` is the whole promised output contract.

The other ten stay partial for concrete reasons:

- The Dart `Address` is an extension type over `String`; Rust stores `[u8; 32]`.
  The wrapper identity, the unvalidated const string constructor and the
  zero-allocation `.value` accessor are not reproduced.
- `str::parse` matches success and rejection classes but returns `ParseError`
  variants instead of `SolanaError` codes and context.
- `getAddressDecoder` consumes a 32-byte prefix and leaves a cursor offset;
  `TryFrom<&[u8]>` rejects trailing bytes.
- `getAddressCodec` and `getAddressEncoder` return fixed-size codec objects;
  Rust exposes typed conversions without cursor or codec metadata.
- No checked-in report entry asserts a positive 32-byte to canonical-address
  conversion, so `getAddressFromPublicKey` remains partial.

## Evidence binding

The extractor accepts a cited case only when the evidence file contains the exact
line `<case>: Ok(())`. A prose file, a substring of source, an absent path, a
`..` path, an absolute path and a symlink escaping the repository are rejected.
A complete claim without evidence or without a case is also rejected, as are
unknown IDs, duplicate IDs, changed hashes, blank case names, invented case
names, malformed headers/field counts and unknown status tokens.

The checked-in report is the native execution. The
[parity-completion runtime receipt](../verification/parity-completion-runtime.md)
records the same suite passing in Chrome, both Wasmtime 50 previews and the
pinned LeptosKit host. Those runs are not separately checked-in case reports, so
this note claims only the receipt, not a per-host artifact it does not contain.

## Residual work

The remaining 17,936 canonical rows keep their previous status. No production
SDK source, root manifest, lockfile, workflow, shared suite, fixture, golden or
other family receipt changed. Completing the codec-object, error-code and
unvalidated-string contracts requires deliberate new API decisions or executable
equivalence proofs; they are not promoted by association with these two rows.
