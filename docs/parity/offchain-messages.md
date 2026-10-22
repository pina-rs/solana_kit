# Off-chain application messages

`solana_kit_core::offchain` provides bounded V0 and V1 application-message codecs
and immutable optional signing envelopes. These APIs are for off-chain clients.
They do not target Solana on-chain programs.

## Source and wire contract

The source package is `solana_kit_offchain_messages` at Dart commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. The checked source SHA-256 values are
in [source-hashes.txt](../../verification/offchain/source-hashes.txt). The
[Dart license](../../crates/solana_kit_core/tests/fixtures/offchain/DART-LICENSE)
accompanies the derived golden fixtures. The immutable reference checkout stayed
clean during extraction.

Every message begins with the 16-byte constant `ff736f6c616e61206f6666636861696e`
and a version byte. `codecs/signing_domain.dart`, `preamble_v0.dart`,
`preamble_v1.dart`, and `message_v0.dart` own these facts.

| Field              | V0                                                                   | V1                                                                 |
| ------------------ | -------------------------------------------------------------------- | ------------------------------------------------------------------ |
| Application domain | Exactly 32 bytes, validated by the existing Kit `Address` type       | Absent                                                             |
| Content format     | u8, 0 printable ASCII, 1 UTF-8, 2 long UTF-8                         | Absent, always UTF-8                                               |
| Signatories        | Nonzero u8 count, raw 32-byte addresses, order preserved             | Nonzero u8 count, strictly increasing unique raw 32-byte addresses |
| Content length     | u16 little-endian UTF-8 byte count                                   | Remaining frame bytes                                              |
| Content bounds     | Nonempty, at most 1232 bytes for formats 0 and 1, 65535 for format 2 | Nonempty, caller-selected budget                                   |

Printable ASCII means bytes 0x20 through 0x7e. These limits apply to message
content, independently of transaction packet limits. V1 has no source protocol
content cap. Appending valid UTF-8 to V1 changes its content. V0 rejects bytes
after its declared content length. Both decoders reject malformed UTF-8,
unsupported version or content discriminators, invalid signing domain,
truncation, empty content, and exhausted caller budgets.

`codecs/envelope.dart` and `codecs/signatures.dart` define the envelope as a
nonzero u8 signature count, ordered 64-byte signature slots, then the exact message
bytes. All-zero slots represent missing signatures. The count must match the
message. Signature presence and strict cryptographic verification are separate.

## Representation and scope

Two local sketches compared an owned text enum with a validated immutable byte
owner. The selected `Message` owns exact signing bytes and parsed immutable
metadata. The retained text copy gives callers an infallible borrowed `str`.
Decoding allocation budgets count every retained wire byte, text byte, and
signatory slot. An `Envelope` additionally counts signature slots. Length checks
and Reader claims precede allocation. V0 and V1 constructors keep domain and
format combinations separate. `Format` makes a V1 application domain impossible.

`Envelope::unsigned` owns a distinct set of absent signature slots.
`with_signature` and optional `sign` return a new envelope. Repeated deterministic
signing is stable. An unrelated signer is an error. `verify` uses the existing
strict Kit Ed25519 implementation over `Message::bytes()`. No host APIs, entropy,
transaction algorithms, or dependencies were added.

The [exact declaration receipt](offchain-messages.tsv) retains all 83 canonical
package declaration IDs and original source hashes. All 83 remain partial and
none are marked complete. The bounded immutable API combines source encoder,
decoder, compilation and signing operations. It does not expose every standalone
Dart factory or error-code wrapper. The canonical inventory was not changed.

Two deliberate differences remain explicit:

- The source V0 message codec permits duplicate addresses. Rust preserves this
  message behavior, but rejects duplicate V0 envelope slots. The source's
  address-keyed envelope map loses those duplicate slots and cannot round-trip
  them reliably.
- The pinned Dart default UTF-8 decoder strips a leading BOM. Its V0 encoder
  emits that BOM and its decoder then rejects the changed byte length. Rust
  preserves every valid UTF-8 byte, including BOM and null characters. The
  original-source divergence is an executed negative control, not a parity claim.

## Portable runtime integration

Signed executable source `1fd7a56f94b0589517c7586c012b23fedff0756b` integrates the
exact independently accepted offchain source and appends its three unchanged
portable functions after the corrected history suite's 72 groups. Suite
`solana-kit-core-v16` passes 75/75 groups natively, in actual Chrome, in Wasmtime
50.0.0 Preview 1 and 2, and in the actual pinned LeptosKit WasmHost. Each host
observes the sole hash corruption at 74/75. The first 72 case IDs, order and report
bytes remain exact. See the [runtime receipt](../verification/runtimes.md).

The unchanged independent Rust 1.89 consumer passes with default features and
crypto after only a dependency manifest path adaptation. It retains every source,
lock and original Dart oracle byte, and matches all four message, unsigned
envelope, signed envelope and deterministic signature fixtures. All 83 IDs remain
partial and none complete. Both documented V0 differences above remain explicit.
Offchain production algorithms, golden fixtures, dependencies and source pins
remain unchanged. The default core normal/build closure remains core plus bs58.

The full host command also passes graph/import controls and all five packaged-core
consumer groups. This proves synchronous offchain-client behavior. It establishes
no complete phase, validator, installed app, device or registry release readiness.
No publish dry run or release record was created during this integration.

## Historical Unit101 evidence

The original pinned Dart package's complete suite executed locally with Dart
3.13.3 and passed 1122 tests. The existing local package configuration needed
local cache selections for analyzer 14.4.0 and node_preamble language version
2.12. No source file changed and no package download occurred.

[oracle.dart](../../verification/offchain/oracle.dart) executed the original
message codecs, envelope compiler, deterministic signing and verification. Four
[golden rows](../../crates/solana_kit_core/tests/fixtures/offchain/golden.tsv) cover
all three V0 formats and V1, reversed input signatory order, multilingual UTF-8,
and null bytes. Rust matches every message, unsigned envelope, signed envelope,
and signature byte. The tests require exactly four complete fixture rows.

[boundaries.dart](../../verification/offchain/boundaries.dart) executed 18 original
source controls. Their [results](../../crates/solana_kit_core/tests/fixtures/offchain/dart-controls.tsv)
include 1232 and 65535 byte boundaries, overflow, empty content, invalid version,
format, count, length, UTF-8 and the BOM divergence.

Rust public integration tests additionally execute every public operation,
truncation at every V0 message and envelope byte, exact trailing policy,
allocation budgets, 255 and 256 signatory counts, V1 sorting and duplicates,
partial signing, unrelated signers, zero slots, immutable inputs, repeated
signing, missing signatures, and message/signature tampering. A clean ordinary
Rust 1.89 consumer signed, decoded and verified the exact public envelope bytes.
The isolated [portable functions](../../verification/offchain/src/lib.rs) execute
valid, malformed, and tamper cases natively without changing shared registration
or counts.

Full workspace all-feature and default tests, Rust 1.89 core tests, Clippy with
warnings denied, rustdoc with warnings denied, rustfmt, dprint, and Monochange
configuration and changeset checks pass. The improved Git-inventoried style gate
scores every authored file and repository category at least 90. No scoring rules
or thresholds changed. The default normal/build graph remains core plus bs58.
Selected optional graphs retain their exact dependency policy.

Core and isolated portable functions compile for `wasm32-unknown-unknown`,
`wasm32-wasip1`, and `wasm32-wasip2`. These are compile-only results for this
unit. No new browser, WASI, LeptosKit, validator, emulator or device runtime
execution is claimed. Isolated CI repeats native execution and the three-target
compile checks.

Monochange preview succeeds with publication disabled. The separate publish
packages dry run rejects the absence of a release record. It performs no registry
publication. No component placeholder or release was created.

## Reproduction

Run Cargo commands from the checkout. `$PACKAGE_CONFIG` points to the existing
local Dart package configuration. `$REFERENCE` is the clean pinned Dart checkout.
The original full test runner additionally uses the two cache selections noted
above. All these commands are local.

```sh
cargo test --workspace --all-features --offline
cargo test --workspace --offline
cargo +1.89.0 test -p solana_kit_core --all-features --offline
cargo test --manifest-path verification/offchain/Cargo.toml --offline
cargo clippy --workspace --all-features --all-targets --offline -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps --offline
cargo fmt --all --check
cargo fmt --manifest-path verification/offchain/Cargo.toml --check
dprint check
monochange check
monochange step validate
monochange preview --format json
dart --packages="$PACKAGE_CONFIG" verification/offchain/oracle.dart
dart --packages="$PACKAGE_CONFIG" verification/offchain/boundaries.dart
```

Compile core with `--all-features --target <target>` and the isolated crate with
`--manifest-path verification/offchain/Cargo.toml --target <target>` for each
Wasm target above. Run the improved `solana_kit_xtask style <checkout>` executable
specified by the standing orders. Raw local logs stay in the primary ignored
Unit 101 evidence store. Published receipts contain no personal filesystem paths.
