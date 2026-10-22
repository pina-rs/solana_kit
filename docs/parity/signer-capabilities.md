# Signer capabilities

This unit adds `solana_kit_signers` for off-chain application clients. It remains
`no_std + alloc`, with optional core deterministic crypto and no host adapter.
On-chain programs should use Pina or official Solana program libraries.

The [scoped receipt](signer-capabilities.tsv) retains all 85 exact declarations
from `solana_kit_signers` at Dart commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. There are 26 partial declarations,
59 pending declarations, and zero complete declarations. The canonical inventory
is unchanged. Each row retains its exact ID, source hash, and provenance.

`SignatureOutput` freezes the address, authorized bytes, and signature. Core owns
the immutable transaction message and address ordered signature slots. Partial
results cannot change bytes or claim another signer's address. Changed modifier
output cannot reuse old signatures or introduce signatures for foreign owners.
The `crypto` feature verifies accepted signatures with existing core Ed25519.
Without that feature, signature acceptance is structural rather than proof of
authenticity.

`SignerCapability::new(&backend)` creates one explicit logical capability.
Repeated references to one handle share identity. Two handles have distinct
identities even when they borrow the same backend. The handle contains a backend
reference, so it has nonzero size even for zero-sized backends. Its storage
identifies that capability while borrowed. This does not promise arbitrary Rust
backend object identity from memory addresses. It needs no extra allocation,
global registry, unsafe code, or entropy.

`TransactionSigners` deduplicates repeated handles and rejects distinct handles
sharing one Solana address, applies modifiers in input order, polls partial signers
concurrently on the same final snapshot, and merges in input order. Sending
requires one explicit sender and populated non-sender slots. With `crypto`,
existing signatures are verified before submission. There are no retries,
confirmation, detached tasks, entropy sources, or noop signature producers.

A caller-owned nonzero identity token plus a backend reference also distinguishes
zero-sized resources in an executed comparison. The borrowed typed handle is
selected because it binds identity to one backend and avoids a separate token
that callers could associate with different backends accidentally. Reuse the
same handle when callers intentionally share one resource. Creating a second
handle represents a separate resource. Partial, modifying, and sending stages
all require handles. The backend traits and concrete associated futures retain
their existing contracts.

| Ownership choice            | Executed evidence                                                                                                                            | Decision                                                                                    |
| --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| Concrete associated futures | Local `Rc` survives `Pending` and wake. An ordinary Rust 1.89 generic consumer spawns on two Tokio threads with `for<'a> P::Sign<'a>: Send`. | Selected. Preserves each backend's actual auto traits.                                      |
| Local boxed erased future   | The same local operation suspends and resumes. A compile-fail doctest rejects requiring `Send` of an erased local future.                    | Supported explicitly for local composition. Native composition separately preserves `Send`. |

The native tests cover deterministic Dart golden signatures, invalid crypto,
foreign identities, changed bytes, missing signatures, cancellation, rejection,
drop cleanup, concurrent partials, modifier ordering, stale signatures, and
missing or ambiguous senders. The isolated consumer also executes portable
valid, malformed, and tampered cases. The shared suite and its counts are
unchanged. Browser, WASI, interpreted-host, validator, and device execution of
these new cases are not claimed. All three Wasm targets compile.

Original pinned Dart test files execute directly through the local package
configuration. Deduplication, direct transaction signing, sender resolution,
modifying and partial interfaces, and keypair suites pass 51 tests in total.
The original `KeyPairSigner` also produces the committed deterministic golden
fixture. The generic cached Dart test runner is incompatible with cached
analyzer and Node preamble versions, so direct file execution supplies the
original test evidence without modifying reference files.

Automatic heterogeneous transaction roles and bounded ordered transaction batches
are implemented through explicit `local` and `native` registrations. Message
batches, message modification, instruction signer attachment, runtime type
assertions, key grinding and generation, file persistence, and signer configuration
remain uncovered. Those gaps explain the
partial and pending statuses. They are not placeholder APIs.

Regenerate the receipt from the repository root with these commands.

```sh
rustc --edition 2024 -D warnings verification/signer-capabilities/receipt.rs -o target/signer-receipt
target/signer-receipt docs/parity/capabilities.tsv docs/parity/signer-capabilities.tsv
cargo test -p solana_kit_signers --all-features --locked
cargo +1.89.0 test --manifest-path verification/signer-capabilities/Cargo.toml --offline --locked
```

The reference license is retained in the signer crate's `DART-LICENSE`. No Dart
implementation or cryptographic algorithm was copied. Registry publication stays
disabled in Monochange. A successful dry-run preview is release planning evidence
and does not authorize publication.

Unit 145 adds automatic transaction role selection with all seven explicit
capability combinations. Native and local registration constructors support
different backend types in one composition without absent-interface stubs.
Native adapters retain `Send` in their boxed future types and require the actual
associated futures to satisfy it. The local form imposes no `Send` bound. A typed
heterogeneous GAT chain also compiled, but actual Tokio spawning exposed a stable
higher-rank borrowed-registration limitation. The selected adapters avoid that
limitation; original backend GAT contracts and logical identities remain intact.

The original Dart algorithm produces 112 literal role oracles covering every
single interface combination and every ordered pair in signing and sending modes.
Rust checks the committed oracle directly. Sending rejects missing senders and
more than one sender-only registration, including repeated registration of one
sender-only handle before deduplication. Composite senders may be demoted.
The selected sender does not participate in modifying or partial signing.

Core validation deliberately exceeds Dart: conflicting logical handles reject
before effects; changed bytes cannot reuse authorization; foreign signatures,
missing non-sender slots, invalid crypto and zero submission identifiers reject.
A composition validates at most 256 registrations. Its ordered `sign_batch`
accepts at most 256 transactions and stops on the first failure. Message batches,
backend-native bulk calls and the other pending declaration contracts remain open.
