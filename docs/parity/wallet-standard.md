# Wallet Standard application contracts

This unit implements portable off-chain application contracts. Browser discovery,
real wallet authorization UI, mobile platform bridges, and device journeys remain
pending. On-chain programs use Pina or official Solana program libraries.

## Ownership decision

Two small designs were compared before implementation. A closed enum containing
all feature implementations makes dynamic discovery straightforward, but requires
boxed futures or a universal backend type and excludes independent extensions.
A typed feature map keeps each concrete backend and its associated futures. The
caller discovers a concrete capability by identifier and type, then invokes its
trait. This design preserves local Rc futures and native Send futures without an
executor or universal Send bound. The typed map is the chosen design.

The passive registry owns Rc references supplied by the caller. Registration and
listener handles use weak references and explicit idempotent removal. Handle Drop
removes the entry. Registry events snapshot callbacks before invocation so callback
reentry can register, unregister, or dispose without holding mutable borrows.

A plain mutable Vec registry with returned event values was also considered. It
avoids interior mutability but cannot represent the pinned synchronous callback
contract. A caller-owned Rc registry with borrow-free dispatch preserves that
observable behavior and has no singleton or platform discovery claim.

Registration compares an incoming name with current metadata read outside registry
borrows. A revision token changes on insertion, removal, disposal, and an explicit
`changed` notification. The commit step checks that token before accepting either
a name match or an insertion, and independently deduplicates the supplied Rc
identity. Removed snapshot entries cannot reject an available incoming wallet,
and a nested insertion cannot make an outer insertion use an obsolete snapshot.
No registration-time name is frozen.

A reserved-entry or queued-registration design was also considered. It adds
pending-entry visibility and nested guard cancellation rules to the synchronous
API. Revision validation preserves ordinary nested operations with less state.
Each registration permits at most 32 observation passes. If every pass is
invalidated, `RegistryError::UnstableRegistration` reports the failure without
inserting the outer wallet or emitting its registration event; nested operations
and their events remain effective. Tests execute both exhaustion after 32
invalidations and success after 31. This bound limits registry retries, not
arbitrary work or recursion performed inside a user callback.

Metadata can change without notifying the registry. Names then describe the
values observed during the accepted pass, rather than a simultaneous snapshot
of arbitrary mutable getters. Call `changed` after updating registered metadata
when an in-progress observation must be invalidated. Rc identity is always
checked independently of those name observations. Callback, wallet, and listener
capture destruction occurs outside registry borrows, including invalidated
snapshot cleanup and disposal.

## Verification scope

The source is pinned at Dart commit 0138bb956d2d81ccc0c56edc602401c0df2f2b46.
The per-declaration receipt records canonical IDs and source hashes. Declaration
coverage remains partial until independent review and platform execution complete.
Raw returned signature bytes do not establish cryptographic verification.

## Implemented contracts and limits

The new crate implements immutable account and icon metadata, exact namespaced
identifier checks, typed extensible feature lookup, all eight wallet error
categories with concrete causes, and passive registry lifetime operations.
Connect, disconnect, property events, and all six Solana signing and submission
capabilities have concrete associated futures and deterministic backend tests.
The contracts preserve all sign-in fields, submission options, batch modes,
settled partial failures, and offchain message version advertisements.

Accounts accept independent display strings and 32-byte keys. Signing responses
preserve arbitrary raw signature bytes. These contracts do not validate a signed
transaction, verify Ed25519, authorize a user, or manufacture wallet success.
Known transaction versions parse only the exact strings `legacy`, `0`, and `1`.
Unknown versions remain unsupported. No transaction downgrade is attempted.
The executed Dart icon probe requires canonical base64 padding and trailing bits.
Rust uses the same strict decoder policy.

The registry uses Rc, and the dynamic feature map retains local concrete backends.
A concrete native signing backend can retain Send futures independently. The ordinary external
Rust 1.89 consumer executes an actual Tokio task, including a signing operation,
and a local Rc future across Pending and wake. Registry handles deliberately
remove their registration or listener on Drop, replacing Dart's explicit callback
ownership with Rust RAII. Existing callback snapshots finish dispatch even when
a callback removes a listener. Mutations affect subsequent dispatches.

All 36 fallback logos and the generic glyph preserve exact pinned data URIs.
The package license and the source's MIT and CC0 attributions are retained.
Independent upstream asset provenance review remains pending. The extraction
binary regenerates assets from the immutable source.

The declaration receipt contains all 203 canonical IDs with verified source
SHA256 values. Every row remains partial. Portable behavior is implemented and
executed with deterministic backends. The [wallet runtime receipt](../verification/wallet-runtime.md)
records five shared assertion groups executed on native Rust, Chrome, both Wasmtime
previews and pinned LeptosKit. Independent integration review, browser discovery,
wallet UI, MWA and installed-host integration remain pending. Deterministic
execution does not establish deployed wallet or device support.

## Reproduction

Run the workspace test suite and the wallet crate's Rust 1.89 tests. The private
`verification/wallet-consumer` workspace executes the ordinary consumer with
`cargo +1.89.0 run --offline --bin wallet_consumer`. Its `graphs` binary checks the
actual default normal and build dependency closure and rejects five injected
host, RNG, and Solana dependency controls. Its `receipt` binary verifies every
canonical declaration's source hash before writing the per-declaration receipt.
The coordinator's ignored primary report retains exact commands and raw logs.
