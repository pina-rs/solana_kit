# Mobile Wallet Adapter runtime receipt

The bounded public Mobile Wallet Adapter protocol executes in the maintained
118-case portable suite. Its seven appended groups run all 15 original public
MWA tests, both URI regression groups and all nine independent protocol groups.
The prior 111 case bodies and ordered report entries are unchanged. The 37
original default-wallet tests and their five portable groups remain unchanged.
All 179 canonical MWA declaration rows remain partial.

The SDK is exclusively for off-chain application code. Solana on-chain programs
should use Pina or official Solana program libraries.

## Source and contracts

The corrected production wallet is byte-identical to accepted commit
`cc27902c0487e45326eefa9b48e4388a0e373a2e`. The portable frontier starts at
`36091fb0c2b56c5a6e6df0cb849431bf3666fa07`. Signed import
`b3fb79de052e40b668c5611690c754baa79b5bbb` preserves exact wallet source, public
tests, fixtures, license and declaration receipt. Complete import and executable
trees are independently reconstructed, including both aggregate lockfiles.
Executable `e39bb78ebb07a22fcc4e6bbeba549fb6a6d9faf1` has tree
`64883b7154ec8011a4cfac620d4ef9a13e7a6b40`.

The mechanical adapter retains all 26 complete public contract bodies, every
literal, 108 ordered assertion macro sites, and operation order within each
contract. Assertion failures become report errors. Fixture unwrap failures
remain hard failures. The native oracle diagnostic becomes an unused value
without removing its construction. Fixed groups change test scheduling only.

The public vectors exercise canonical P-256 points and scalars, authenticated
HELLO, ECDH/HKDF, RFC6979 ES256/P1363 signatures, AES-GCM bytes, counters, replay,
every-byte tampering, URI policy, JSON models/RPC and SIWS. Public ciphertext and
authentication vectors exercise the derived key portably. The private raw ECDH
and session-key byte unit remains separate native evidence. Its private access
is not claimed as a guest execution.

## Executed hosts

| Host                             | Canonical result          | Corruption result          |
| -------------------------------- | ------------------------- | -------------------------- |
| Native Rust 1.99.1               | 118/118                   | 117/118, sole corrupt hash |
| Actual Chrome 155.0.8059.85      | Full ordered suite passes | Sole corruption observed   |
| Wasmtime 50.0.0, WASI Preview 1  | 118/118                   | 117/118, sole corrupt hash |
| Wasmtime 50.0.0, WASI Preview 2  | 118/118                   | 117/118, sole corrupt hash |
| Actual pinned LeptosKit WasmHost | 118/118                   | 117/118, sole corrupt hash |

The plain guest imports exactly `leptoskit::commit`, `node_create` and
`node_set_props`. It imports no entropy, clock, JavaScript or WASI capability.
One fuel traps before any report commit. The maintained complete command runs
twice, with the second run following the fixed-count naming correction required
by the unchanged style floor. Actual Cargo-selected browser input and all eight
runtime artifacts are copied and hashed before compiler or feature changes.
The generated guest's own producer section identifies Rust 1.99.1
`58cf8f9c35b355199727627d07ae34a4e2846d67`. No MSRV guest execution is inferred.

## Fixed host budget

Before adapter, registry, count or fuel edits, a source-bound native run measured
16 named cryptographic backend work categories in the 26 original public tests. There are 573
entries. Imports/derivations, point parsing, signing, verification, ECDH, HKDF,
AES initialization/authentication and Kit Ed25519 calls are distinct categories.
The root reviewed the full inventory before fuel edits. Cached association-point copies and uncompressed point encoding use the ordinary group allowance. The immutable
[workload table](../../verification/conformance/src/mwa_cases/metering.rs)
retains every category. These are counted call-site entries, including rejected
constructor attempts, not equal-cost curve rounds or a performance measurement.

Existing rates remain 15,000,000 per ordinary group and 5,000,000 per crypto work
unit. Existing 257 transaction and 772 message units remain unchanged. Checked
host arithmetic grants `6,765,000,000 + 7*15,000,000 + 573*5,000,000`, or
9,735,000,000 fixture fuel. Category, count and overflow negatives reject.
This is a fixed conformance allowance, not a production budget or measured
minimum. Guest messages cannot grant or scale it.

## Native checks and limits

Current Rust and actual Rust 1.89 each pass all 55 original wallet tests, all 37
default-wallet tests, the ordinary seven-assertion consumer and standalone
panic-handler consumers with default and MWA features. The core/client suites
pass 304 tests on each compiler. Full workspace all/default runs pass 610/588
tests. Warning-denied workspace/private Clippy and docs, MSRV checks, three Wasm
checks, fuel tests, format, dprint and Monochange check/validate/preview pass.

The 179-ID, defining-hash, complete source inventory and license audits retain
missing/duplicate/hash/license rejection controls. The immutable original Dart
203-test and deterministic oracle evidence is carried by exact source/fixture
hashes. No fresh Dart execution is claimed here. Secret Debug/Display rejection
and consuming ownership are separate native compilation proofs.

The accepted P-256/AES backend and zeroization assessment carries unchanged. It
does not establish full cryptographic auditing or erasure of every temporary.
Callers own entropy, unpredictable globally unique IVs across both directions,
nonce issuance, clocks, expiry, persistence, replay storage and platform handover.
The independent reflection and repeated-IV controls preserve these demonstrated
limits. There is no installed-wallet, network, validator, desktop, Android or iOS
journey claim and no full package parity claim.
