# Key grinding and transaction-message decompilation

This receipt covers the two additive public contracts implemented after the
Phase 2/3 gap audit. It does not change the canonical capability inventory,
shared conformance suite, suite version or any status threshold.

Pinned source: `0138bb956d2d81ccc0c56edc602401c0df2f2b46`.

## Key grinding and constant-time equality

Source references:

- `packages/solana_kit_keys/lib/src/key_pair.dart#grindKeyPairs`
- `packages/solana_kit_keys/lib/src/key_pair.dart#grindKeyPair`
- `packages/solana_kit_keys/lib/src/key_pair.dart#constantTimeEqual`
- `packages/solana_kit_signers/lib/src/keypair_signer.dart#grindKeyPairSigners`
- `packages/solana_kit_signers/lib/src/keypair_signer.dart#grindKeyPairSigner`

Added to `solana_kit_core::crypto`:

- `GrindPattern::{prefix, suffix}`, `GrindPattern::literal`, `GrindPattern::matches`
- `GrindPatternError::{InvalidCharacter, TooLong}`
- `GrindBudget::{new, amount, attempts}`
- `GrindError::{Allocation, Entropy, Exhausted}`
- `grind_keypairs`, `grind_keypairs_with`
- `constant_time_eq`

Added to `solana_kit_signers` under `crypto`:

- `grind_keypair_signers`, `grind_keypair_signers_with`

A successful call returns exactly `GrindBudget::amount()` keys. A zero amount
returns an empty vector without calling the entropy host. Literal patterns are
validated against the base58 alphabet and the 44-character address maximum before
generation; callers that need arbitrary matching, including the source's
`RegExp`, pass a predicate to `grind_keypairs_with`. The source concurrency knob
does not introduce parallel execution: generation is sequential, suspends inside
caller-owned `Entropy`, and yields cooperatively after 32 candidates without host
timers. Dropping the future cancels it. Rejected
candidates, retained candidates on entropy failure, and retained candidates on
budget exhaustion all drop zeroizing `Keypair` owners; no test claims to prove
physical erasure.

Entropy failure is returned immediately as `GrindError::Entropy` and stops the
grind. Budget exhaustion is `GrindError::Exhausted { attempts, found }`; partial
matches are never returned as success. Result capacity is bounded by the smaller
of the requested amount and the attempt budget. Unavailable or unrepresentable
result storage returns `GrindError::Allocation` before entropy is requested.

`constant_time_eq` compares contents through `subtle` (`default-features =
false`), already present in the crypto graph through `ed25519-dalek` and
`curve25519-dalek`. Only a differing length may short-circuit. The optional
direct dependency edge adds one dependency line to the root and affected
verification lockfiles and resolves no new package version.

Executed evidence: `crates/solana_kit_core/tests/keypair_grind.rs` (12 tests) and
`crates/solana_kit_signers/tests/keypair_grind.rs` (3 tests). Coverage includes
deterministic candidate sequences, exact counts, rejected candidates before a
match, prefix and suffix patterns, zero amounts, invalid and overlong literals,
exhaustion with partial matches present, entropy failure before and after a match,
a non-`Send` local host, batch yielding and cancellation with synchronous entropy,
allocation failure without entropy,
empty/equal/differing/unequal-length equality, and
signature verification for returned signers.

## Transaction-message decompilation

Source reference:

- `packages/solana_kit_transaction_messages/lib/src/decompile_message.dart`
  (`#decompileTransactionMessage`)

Added to `solana_kit_core::transaction`:

- `DecompiledMessage::{version, payer, lifetime, instructions, config}`
- `DecompiledMessage::with_lifetime`
- `DecompiledMessage::compile`
- `Message::decompile`

Callers own lookup snapshots as existing `LookupTable` values. Decompilation
preserves the version, fee payer, instruction order, account order and duplicates,
data, static signer/writable partitions, loaded writable-before-readonly order,
inline V1 configuration, and exact `AccountLookupMeta` table/index/non-signer role
provenance. Missing, duplicate, oversized (over 256 addresses), and
out-of-range snapshot data reject with `transaction::Error::InvalidLookup`;
out-of-list instruction indices reject with `Error::InvalidIndex` though a
validated message cannot supply them. Unreferenced snapshots are ignored.

The lifetime is copied exactly. Unlike the source's default maximum block height,
a decoded `Lifetime::Unresolved` stays unresolved; the caller supplies established
expiry or nonce metadata through `with_lifetime`. A structurally recognized nonce
advance is not promoted to trusted nonce metadata, and no account state is
authenticated. `DecompiledMessage::compile` calls the existing compiler
(`compile_v1` for version one, `compile_with_lookup_accounts` otherwise) and
therefore reports `Error::UnresolvedLifetime` until metadata is supplied.

Compilation merges privileges globally, so narrower per-instruction roles cannot
be restored. Received messages with noncanonical account ordering need not
recompile to identical bytes; canonical messages produced by the Kit compiler do.
The original signed message slice is never modified.

Executed evidence: `crates/solana_kit_core/tests/decompile.rs` (10 tests), covering
legacy, v0 and v1, inline V1 configuration, merged privileges, multiple tables
with mixed writable/readonly indices, duplicate instruction positions, the same
address with two different table origins, manual writable-before-readonly loaded
ordering, missing/out-of-range/conflicting/oversized snapshots, canonical
compile/decompile/recompile wire equality, received lifetime remaining
unresolved, caller-supplied blockhash and durable nonce metadata retained,
nonce-shape non-inference, and untouched signed bytes and signature slots.

## Limits

- Native behavior tests and all three Wasm target compile checks are claimed.
  Browser, WASI and interpreted-host execution of the new cases is not.
- No `capabilities.tsv` row, ID, status, checker, threshold, shared golden or
  suite version changed in this unit.
- Per-call signer configuration was implemented subsequently; its separate
  [receipt](signer-call-config.md) records the evidence.
