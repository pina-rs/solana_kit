# Per-call signer configuration

This receipt covers the signer configuration contract implemented after the
Phase 2/3 gap audit. It does not change the canonical capability inventory,
shared conformance suite, suite version or any status threshold.

Pinned source: `0138bb956d2d81ccc0c56edc602401c0df2f2b46`.

Source references:

- `packages/solana_kit_signers/lib/src/types.dart#SignerConfig`
- `packages/solana_kit_signers/lib/src/types.dart#TransactionSignerConfig`
- `packages/solana_kit_signers/lib/src/transaction_partial_signer.dart#signTransactions`
- `packages/solana_kit_signers/lib/src/transaction_modifying_signer.dart#modifyAndSignTransactions`
- `packages/solana_kit_signers/lib/src/transaction_sending_signer.dart#signAndSendTransactions`
- `packages/solana_kit_signers/lib/src/message_partial_signer.dart#signMessages`
- `packages/solana_kit_signers/lib/src/message_modifying_signer.dart#modifyAndSignMessages`
- `packages/solana_kit_signers/lib/src/sign_transaction.dart`
- `packages/solana_kit_wallet_adapter/lib/src/signer.dart#WalletAccountSigner`

## Public contract change

Added to `solana_kit_signers`:

- `SignerConfig { aborted: bool }`
- `TransactionSignerConfig { aborted: bool, min_context_slot: Option<u64> }`

Both are `Clone + Copy + Debug + Default + Eq + PartialEq` value types. There is
no inherited configuration class, constructor-owned configuration, parallel
`*_with_config` method, compatibility wrapper, or second pipeline.
`Default::default()` preserves the previous call behavior.

The narrow canonical rows exercised by this receipt are:
`SignerConfig`, `SignerConfig.aborted`, `SignerConfig.new`,
`TransactionSignerConfig`, `TransactionSignerConfig.minContextSlot` and
`TransactionSignerConfig.new`. This receipt does not change their canonical or
scoped-ledger status; it supplies the missing behavioral evidence without
altering IDs, checkers or thresholds.

Every low-level capability method now requires the value explicitly:

```rust
fn sign_message<'a>(
    &'a self,
    bytes: &'a [u8],
    config: SignerConfig,
) -> Self::Sign<'a>;

fn sign_messages<'a>(
    &'a self,
    messages: &'a [SignableMessage],
    config: SignerConfig,
) -> Self::Sign<'a>;

fn modify_messages<'a>(
    &'a self,
    messages: &'a [SignableMessage],
    config: SignerConfig,
) -> Self::Modify<'a>;

fn sign_transaction<'a>(
    &'a self,
    transaction: &'a Transaction,
    config: TransactionSignerConfig,
) -> Self::Sign<'a>;

fn modify_transaction<'a>(
    &'a self,
    transaction: &'a Transaction,
    config: TransactionSignerConfig,
) -> Self::Modify<'a>;

fn send_transaction<'a>(
    &'a self,
    transaction: &'a Transaction,
    config: TransactionSignerConfig,
) -> Self::Send<'a>;
```

Typed `TransactionSigners`, local/native transaction compositions, local/native
message compositions, the singular-to-bulk adapter, keypair backends and the
browser `WalletSigner` all accept the same per-call value. Their public pipeline
methods append the configuration argument:

- `TransactionSigners::{partially_sign, sign}`
- `TransactionSigners::modify_and_partially_sign` (`transaction, modifiers, config`)
- `TransactionSigners::sign_and_send` (`transaction, senders, config`)
- `local::Composition` and `native::Composition` transaction methods, including
  `sign_batch`
- `message::local::Composition::sign` and `message::native::Composition::sign`

This is a coordinated pre-1.0 API change. Repository callers, tests, fixtures and
documentation were migrated; no compatibility overload was retained.

## Semantics

`aborted: true` returns the existing `Error::Cancelled` or
`MessageError::Cancelled` before signing, prompting, simulation or network
effects. Pipeline entry points check it before sender cardinality, capability
conflicts, input count, snapshot validation and backend invocation, including
empty or otherwise invalid message batches. This differs deliberately from the
pinned implementation's repeated post-resolution checks: the Rust value is only a
per-call initial signal. Dropping an in-flight future remains the cancellation
mechanism after an operation has started.

`min_context_slot` is an exact optional `u64` simulation hint, not authenticated
freshness. Every transaction backend receives it unchanged: modifiers, partials,
senders, local heterogeneous compositions, native heterogeneous compositions and
the browser wallet. A local keypair does not simulate and explicitly ignores the
hint. No composition or adapter silently drops configuration.

The browser `WalletSigner` forwards `Some(slot)` as Wallet Standard
`options.minContextSlot` for `solana:signTransaction` and
`solana:signAndSendTransaction`, including `Some(0)`. `None` omits the field.
A slot above the JavaScript safe integer limit (`2^53 - 1`) rejects with
`WalletErrorCode::InvalidRequest` before an operation guard, prompt or wallet
invocation; a `u64` is never rounded through a JavaScript number. Aborted direct
calls use the existing cancellation mapping: `WalletFailure` with
`WalletErrorCode::Disconnected` and `solana_kit_signers::Error::Cancelled` as its
typed cause, without creating an operation guard.

Existing identities, role selection, modifier/partial/sender order, snapshot and
slot validation, cardinality checks, associated future types, local `Rc`
non-`Send` guarantees, native `Send` guarantees, generic failure preservation and
submission-identifier semantics are unchanged. No retry or additional network
effect was introduced.

## Executed evidence

`crates/solana_kit_signers/tests/config.rs` adds 10 native tests (9 without
`crypto`). They cover:

- Two consecutive calls observing different configurations with no retained
  constructor/default state.
- One call configuration applied to every transaction in a bounded batch.
- Identical configuration delivery to every modifier, partial and sender through
  typed `TransactionSigners`, heterogeneous local composition and a native
  composition spawned on Tokio. `None`, `Some(0)` and `Some(u64::MAX)` are
  compared exactly.
- A real `Rc` host suspended across `Pending`/wake retaining the per-call value.
- Aborted transaction calls preceding sender cardinality, conflict checks and all
  backend effects.
- Message modifiers and partials receiving one `SignerConfig`; aborted empty, 257
  message and invalid-signature inputs rejecting before effects.
- The singular adapter forwarding one config per item and rejecting aborted
  directly.
- Direct deterministic keypair message, bulk-message and transaction calls
  returning `Cancelled` before signing.

`crates/solana_kit_browser/tests/wallet/config.rs` adds 5 tests to the existing
real Chrome Wallet Standard fixture (20 wallet tests total). The fixture records
the actual outgoing inputs. The tests verify exact `Some(0)` forwarding, exact
`2^53 - 1` forwarding, omitted `None`, exact forwarding for partial, modifying and
sending features, rejection of `2^53` and `u64::MAX` with no wallet request or
guard, and aborted message/partial/modifying/sending calls with no wallet request.

`cargo +1.99.1 test -p solana_kit_signers --all-features` and
`cargo +1.89.0 test -p solana_kit_signers --all-features` execute the native
suite. The browser cases execute through the existing headless Chrome and
chromedriver fixture. The WASM compile check covers `solana_kit_browser` on
`wasm32-unknown-unknown`.

## Limits

- This unit adds no live RPC, validator, installed-device, WASI-host or
  interpreted-host execution evidence. Browser evidence is limited to the real
  Wallet Standard fixture.
- The scoped signer receipt continues to report partial/pending declarations and
  unchanged canonical IDs, statuses and thresholds. Its limit text no longer
  describes signer configuration as pending; this receipt supplies the narrowed
  evidence.
- A returned submission identifier remains a submission identifier, not proof of
  execution or confirmation.
