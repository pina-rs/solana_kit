# Mobile Wallet Adapter protocol

The optional `solana_kit_wallet/mwa` feature implements the portable protocol
layer of pinned Dart package `solana_kit_mobile_wallet_adapter_protocol` at
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. The pinned wire contract negotiates
`v1` and `legacy`, uses `/v1/associate` paths, and uses the v1 WebSocket
subprotocol. This receipt does not invent a different protocol-v2 frame format.

This is an off-chain application SDK. Solana programs should use Pina or the
official Solana program libraries.

## Contracts and choices

An optional wallet feature preserves the existing default wallet and core
closures. A separate protocol crate would add package ownership without changing
this feature's transport boundary. RustCrypto supplies P-256 ECDH, RFC6979
ECDSA-SHA256, HKDF-SHA256, AES-128-GCM, and Ed25519 verification through Kit core.
No cryptographic algorithm is implemented locally.

The complete optional feature supports `no_std + alloc`. Borrowed `iri-string`
URI validation enables no dependency features and adds no transitive packages.
The association boundary preserves u16 authority ports, rejects unsupported
IPvFuture literals, and requires reflector ports and IPv6 addresses to round-trip
without normalization. It retains exact URI prefix, path, escaping and duplicate
parameter behavior. The ordinary
[`no_std` consumer](../../crates/solana_kit_wallet/tests/fixtures/mwa/no_std/src/lib.rs)
defines its own panic handler, so any transitive `std` dependency fails to build:

```sh
cargo build --locked --manifest-path crates/solana_kit_wallet/tests/fixtures/mwa/no_std/Cargo.toml
cargo build --locked --manifest-path crates/solana_kit_wallet/tests/fixtures/mwa/no_std/Cargo.toml --features mwa
```

Private scalars must be nonzero and below the P-256 order. Public points must be
exactly 65 canonical uncompressed X9.62 bytes and finite points on P-256.
Association signatures are 64-byte P1363 values. HELLO_REQ is exactly 129 bytes.
HELLO_RSP begins with a 65-byte peer point, followed by optional encrypted session
properties. The HKDF salt is the association point, with empty context and a
16-byte output key.

Frames contain a four-byte big-endian sequence, a 12-byte IV, and ciphertext with
a 16-byte GCM tag. The sequence is authenticated as AAD. The caller supplies fresh,
unpredictable IVs across both directions. `Session` owns independent directional
counters and its key. It advances receive state only after authentication and
exact sequence matching. Closing consumes the session. No host entropy, clocks,
sockets, globals, detached tasks, or default endpoints are installed.

SIWS rejects CR and LF in all scalar fields and resource entries. Result
verification checks the expected signer, claimed address, algorithm, exact
message, and strict Ed25519 signature. Nonce issuance, expiry, and application
replay storage remain caller responsibilities. Request mapping preserves the
pinned legacy behavior, including retaining `chain` while adding `cluster`.
Response parsing additionally requires the expected ID and a valid JSON-RPC 2.0
envelope. Compact JWS verification accepts only the pinned ES256 header.

`SignInPayload` uses the MWA SIWS camelCase wire keys `chainId`, `issuedAt`,
`expirationTime`, `notBefore`, and `requestId`, while Rust fields retain idiomatic
snake_case names. The surrounding MWA keys, including `sign_in_payload` and
`auth_token`, retain snake_case. This intentionally corrects the pinned Dart
`types.dart` lines 240–245, which emit snake_case for those five SIWS fields.
The primary Java [contract](https://github.com/solana-mobile/mobile-wallet-adapter/blob/main/android/common/src/main/java/com/solana/mobilewalletadapter/common/signin/SignInWithSolanaContract.java),
Java [JSON parser and serializer](https://github.com/solana-mobile/mobile-wallet-adapter/blob/main/android/common/src/main/java/com/solana/mobilewalletadapter/common/signin/SignInWithSolana.java),
and JS [types](https://github.com/solana-mobile/mobile-wallet-adapter/blob/main/js/packages/mobile-wallet-adapter-protocol/src/types.ts)
use camelCase (verified content blobs `f1ac0168b668cb2bf59a6a2ee03bf0984151cd98`,
`0dd8adba1836972462821c9d4c2c418e681f2e00`, and
`744c4728ed145180283a662fa8709326410c2392`, respectively).
Literal incoming/outgoing JSON, exact SIWS text, omission/null decoding and a
nested authorization envelope cover this interoperability contract.

## Source and declaration accounting

The [declaration table](mobile-wallet-protocol.tsv) retains all 179 unique
canonical IDs from 18 defining Dart files with their immutable SHA256 values.
The package has 18 `lib/src` files and one export file, 19 library files in total.
All rows remain partial. Several upstream APIs obtain ambient randomness or
expose PointyCastle objects; this adaptation accepts caller-owned inputs and
keeps cryptography implementation types private. Typed JSON models preserve
optional-field omission and null decoding. Solana address and signature checks
reuse Kit-owned domain types.

The package MIT license is retained in
[`MWA-LICENSE`](../../crates/solana_kit_wallet/MWA-LICENSE).
The native receipt tool verifies canonical uniqueness, count, actual defining
source hashes, and retained license bytes before emitting the table.

```sh
rustc --edition=2024 crates/solana_kit_wallet/tests/fixtures/mwa/receipt.rs -o target/mwa-receipt
target/mwa-receipt docs/parity/capabilities.tsv "$PINNED_DART" crates/solana_kit_wallet/MWA-LICENSE
```

## Verification and limits

All nine original Dart test families execute from an explicit owned package
configuration without dependency resolution. All 203 original tests pass.
The committed deterministic PointyCastle oracle reproduces exact P-256 points,
ECDH and HKDF inputs, RFC6979 HELLO and JWS signatures, AES-GCM frame bytes, and
SIWS text. Rust verifies and matches these values. Dart independently accepts
Rust HELLO signatures, ES256 JWS, and encrypted frames.

Rust tests reject malformed and off-curve points, invalid scalars, forged
signatures, wrong keys, every single-byte frame mutation, truncated frames,
sequence gaps, replays, exhausted counters, ambiguous URI parameters, wrong RPC
IDs, unsupported JWS algorithms, and SIWS line injection. An encrypted memory
session exercises handshake, authenticated properties, authorization, account
conversion, deauthorization, and explicit close.

An ordinary native consumer executes on current Rust and actual Rust 1.89 with
compiler flags and wrappers unset. All three Wasm targets compile on current
Rust. Rust 1.89 also compiles the plain guest and WASI Preview 2 consumer. Its
WASI Preview 1 target is not installed. Both plain guest artifacts have zero
imports; a valid module importing `host::rng` is rejected by the same checker.
These are compile and import checks, not browser, WASI, interpreter, or device
runtime execution. A Rust 1.89 release-profile attempt failed to load its host
serde-derive dylib on this machine. Development-profile proofs do not establish
release-profile support for that compiler and host combination.

Platform association handover, transport lifecycle, Android intents, reflector
networking, browser discovery, wallet UI, and installed desktop or mobile wallet
journeys remain pending. This unit does not claim full MWA platform parity.

## Portable protocol execution

The subsequent [runtime receipt](../verification/mwa-runtime.md) executes all
15 public MWA tests, both URI groups and nine independent protocol groups across
native Rust, Chrome, WASI Preview 1/2 and the actual pinned LeptosKit interpreter.
The existing SIWS group additionally checks the complete camelCase wire payload,
exact incoming message text, omission/null behavior and nested authorization JSON.
The earlier compiler-only milestone above remains historical evidence. The raw
private crypto unit remains native evidence; public exact vectors exercise the
same derivation through authenticated ciphertext in every portable host.
All 179 declaration rows remain partial. Entropy, IV uniqueness across both
directions, time, persistence, nonce/replay policy and platform handover stay
caller-owned. These portable protocol assertions do not complete platform parity.
