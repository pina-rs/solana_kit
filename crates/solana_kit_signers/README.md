<!-- {=brandHeader|trim} -->
<p align="center">
  <img src="https://raw.githubusercontent.com/pina-rs/solana_kit/main/assets/brand/solana-kit-mark.png" alt="Solana Kit" width="112" height="112">
</p>
<!-- {/brandHeader} -->

# Solana Kit signers

Portable asynchronous signing capabilities for off-chain application clients.
This crate uses `no_std + alloc`. It has no runtime, entropy, filesystem, or
network dependency. Solana on-chain programs should use Pina or official program
libraries instead.

`MessageSigner` signs exact bare bytes. `TransactionPartialSigner` authorizes an
immutable transaction snapshot. `TransactionModifyingSigner` runs before partial
signers. `TransactionSendingSigner` represents explicit submission permission.
Sending returns a submission identifier and does not confirm execution.

Create a `SignerCapability::new(&backend)` handle for each logical resource.
`TransactionSigners` deduplicates repeated references to that handle and rejects
distinct handles claiming one Solana address. A new handle has a distinct
identity even when it borrows the same backend. Reuse one handle intentionally
when several callers share a resource. Handle identity remains unique while
borrowed, including for zero-sized backends, without allocation or entropy.

```rust,ignore
let capability = SignerCapability::new(&backend);
let signers = TransactionSigners::new(&[&capability, &capability])?;
let signed = signers.sign(&transaction, TransactionSignerConfig::default()).await?;
```

Every signing call takes a per-call `SignerConfig` or `TransactionSignerConfig`.
`TransactionSignerConfig` carries `aborted` and `min_context_slot`. The same value
is delivered unchanged to every backend: typed pipelines, local and native
heterogeneous compositions, modifiers, partials, senders, and message adapters
never store it. `Default::default()` preserves the previous behavior. An aborted
configuration fails with the existing `Error::Cancelled` or
`MessageError::Cancelled` before signing, prompting, simulation, or network
effects. Dropping an in-flight future remains the cancellation mechanism after an
operation starts.

`min_context_slot` is an exact optional simulation hint, not authenticated
freshness. Every transaction backend receives it, including modifiers and
partials. A local keypair does not simulate and explicitly ignores the hint. A
backend that converts the slot to a narrower or rounded representation must reject
values it cannot represent exactly.

`TransactionSigners` polls partial signers concurrently and merges results in
input order. Its futures support local state. Native generic callers can require
`for<'a> P::Sign<'a>: Send`, together with `P: Sync`, before spawning.

The `crypto` feature implements message and partial transaction signing for
core's deterministic `Keypair`. It also verifies accepted backend signatures.
Without `crypto`, acceptance checks ownership, exact bytes, and nonzero slots.
Applications must verify external signatures before trusting authenticity.

`grind_keypair_signers` and `grind_keypair_signers_with` apply the core finite
`GrindBudget` and caller-owned entropy, then convert every matched exportable key
with `into_signer`. Returned keypair signers keep the existing message and
partial transaction implementations without any backup/export capability.

See the scoped parity receipt in `docs/parity/signer-capabilities.md`.

Use `local::Registration` for heterogeneous local backends, or
`native::Registration` when spawning on a multi-thread executor. Declare exactly
which supported interfaces participate with `partial`, `modifying`, `sending`,
`partial_modifying`, `partial_sending`, `modifying_sending`, or `composite`.
Each constructor requires those backend traits. There is no trait introspection
or implementation for an absent backend interface.

```rust,ignore
use solana_kit_signers::native::{Composition, Registration};
let first = SignerCapability::new(&hardware);
let second = SignerCapability::new(&wallet);
let signers = Composition::new(vec![
    Registration::partial(&first),
    Registration::composite(&second),
])?;
let identifier = signers.sign_and_send(&transaction, TransactionSignerConfig::default()).await?;
```

Automatic composition selects a sender-only registration when present, otherwise
the first sending composite. It excludes that sender from signing. All remaining
modifiers without partial signing run; otherwise the first composite modifies.
Other composites become partial signers. Sending requires at least one sender and
at most one sender-only registration, checked before deduplication as in Dart.
Distinct same-address handles and inconsistent declarations reject before effects.

Native registrations require `Sync` backends and `Send` associated futures for
every declared interface, preserving `Send` in their erased futures. Local
registrations support `Rc` state across suspension. The explicit choice prevents
a local future from silently losing native scheduling capability. The backend
associated-future contracts remain unchanged; every backend call now receives the
caller's per-call configuration explicitly.

Composition accepts at most 256 registrations. `sign_batch` accepts at most 256
transactions, signs in input order, and stops at the first failure. Earlier
backend effects cannot be rolled back. Instruction attachment, key generation and
persistence remain pending.

Use `message::SignableMessage` for exact bare bytes and an immutable signature
dictionary. `from_text` copies UTF-8 without normalization or an automatic off-chain
envelope. Snapshot construction imposes no content-length limit.

`message::MessagePartialSigner` receives the whole bounded slice once and returns
one exact-byte signature dictionary per input. `message::MessageModifyingSigner`
returns one replacement per input. Both receive the same per-call `SignerConfig`.
Choose `message::local` or `message::native` registrations explicitly. A composite
backend is registered in one role.
Modifiers run sequentially before partials share one immutable final slice.
Results merge in registration order. At most 256 messages and registrations
are accepted before effects. Changed bytes cannot retain stale signatures,
partial outputs cannot claim foreign owners, and cardinality is checked.

`message::SingularMessageSigner(backend)` explicitly adapts the original
`MessageSigner` by invoking it once per input in order. Implement the bulk trait
directly for a backend-native bulk operation. The adapter preserves the singular
future's actual auto traits and stops at its first failure.

Low-level capability traits require a per-call configuration argument and accept a
defaulted failure parameter. Existing implementations keep `Error`; a local host
backend can instead return its concrete error without discarding an original
platform cause. Transaction `local` and
`native` registrations and compositions preserve that parameter. Construct a typed
composition with `Composition::<Failure>::with_failure`, where `Failure:
From<Error>` retains protocol-validation failures. `Composition::new` keeps the
existing default-error behavior.

`local::Registration::partial_with` admits a backend with another concrete error
through an explicit caller-selected mapper. This allows a local key and a browser
wallet to share the normal ordered/concurrent composition without mapping every
backend failure to rejection. The default native error and native future bounds
remain unchanged; browser-local causes do not enter portable default graphs.
