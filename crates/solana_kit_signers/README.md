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
let signed = signers.sign(&transaction).await?;
```

`TransactionSigners` polls partial signers concurrently and merges results in
input order. Its futures support local state. Native generic callers can require
`for<'a> P::Sign<'a>: Send`, together with `P: Sync`, before spawning.

The `crypto` feature implements message and partial transaction signing for
core's deterministic `Keypair`. It also verifies accepted backend signatures.
Without `crypto`, acceptance checks ownership, exact bytes, and nonzero slots.
Applications must verify external signatures before trusting authenticity.

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
let identifier = signers.sign_and_send(&transaction).await?;
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
GAT contracts remain unchanged.

Composition accepts at most 256 registrations. `sign_batch` accepts at most 256
transactions, signs in input order, and stops at the first failure. Earlier
backend effects cannot be rolled back. Message batches and message modification,
attachment/configuration, key generation, and persistence remain pending.
