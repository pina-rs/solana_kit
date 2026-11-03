# Signers

`solana_kit_signers` models signing as explicit asynchronous capabilities. A local key, a hardware device, a browser wallet and a remote service all implement the same small traits. The crate composes them in a fixed order and checks every result against the exact bytes that were meant to be signed. It is `no_std + alloc`, with no runtime, entropy, filesystem or network dependency.

## Capability traits

Each trait is a separate permission. A backend implements only the ones it actually supports.

| Trait | Operation | Returns |
| --- | --- | --- |
| `Signer` | `address()` | The address whose signature slot it owns |
| `MessageSigner` | `sign_message(bytes)` | A `SignatureOutput` over exact bare bytes |
| `TransactionPartialSigner` | `sign_transaction(&transaction)` | A `SignatureOutput` for one immutable transaction snapshot |
| `TransactionModifyingSigner` | `modify_transaction(&transaction)` | A replacement `Transaction`, applied before partial signers run |
| `TransactionSendingSigner` | `send_transaction(&transaction)` | A submission `Signature`. This is not proof that the transaction executed |

Each trait has an associated future type, such as `type Sign<'a>`. This lets a backend keep local state such as `Rc` across an `.await` while a native backend stays `Send`. The traits also take an optional failure type parameter (`TransactionPartialSigner<Failure = Error>`), so a host backend can return its own error type without losing the original cause.

`SignatureOutput` freezes the address, the authorized bytes and the signature together. A result that names a different address or covers different bytes is rejected.

## Local keypairs

With the `crypto` feature, the core `crypto::Keypair` implements `Signer`, `MessageSigner` and `TransactionPartialSigner`:

```rust
use solana_kit_core::crypto::Keypair;
use solana_kit_signers::MessageSigner;

async fn example() -> Result<(), solana_kit_signers::Error> {
    let keypair = Keypair::from_seed(&[7; 32]);
    let output = keypair.sign_message(b"exact bare bytes").await?;
    assert_eq!(output.signature(), keypair.sign(b"exact bare bytes"));
    Ok(())
}
```

Ways to create a keypair:

- `Keypair::from_seed(&[u8; 32])` derives a key deterministically from a seed you supply.
- `Keypair::from_bytes(&bytes)` imports exactly 64 bytes: the seed, then the matching public key. Any other length, or a mismatched public half, fails with `KeypairImportError`.
- `ExportableKeypair::generate(&mut entropy).await` creates a new key from your `crypto::Entropy` source. Choose `ExportableKeypair` only if you need backups. `backup()` returns zeroizing `KeypairBackup` storage, and `into_signer()` gives up the export ability.

A plain `Keypair` cannot export its secret and has no `Debug` output. The core selects no random source. Host adapters provide entropy, for example `solana_kit_native`'s `keys::NativeEntropy` behind its `keys` feature. See [Addresses and keys](../getting-started/addresses-and-keys.md) and [Native](../runtimes/native.md).

## Signing a transaction

Wrap each logical backend in a `SignerCapability`. Then give the handles to `TransactionSigners`:

```rust
use solana_kit_core::transaction::Transaction;
use solana_kit_signers::{SignerCapability, TransactionSigners};

async fn sign(
    keypair: &solana_kit_core::crypto::Keypair,
    transaction: &Transaction,
) -> Result<Transaction, solana_kit_signers::Error> {
    let capability = SignerCapability::new(keypair);
    let signers = TransactionSigners::new(&[&capability])?;
    signers.sign(transaction).await
}
```

`TransactionSigners`:

- removes repeated references to the same handle. Two distinct handles that claim one address fail with `Error::ConflictingSigner`.
- polls the partial signers concurrently on one final snapshot and merges their results in input order.
- has `partially_sign`, which allows empty slots, and `sign`, which requires all of them. It also has `modify_and_partially_sign` and `sign_and_send`.

The handle carries the identity, not the backend. A new `SignerCapability::new(&backend)` is a separate resource even when it borrows the same backend. Reuse one handle when several callers share a resource.

## Composing different backends

When you combine signers of different kinds, register each one with the roles it supports. Use `local::Composition` with `local::Registration` for single-threaded or browser code. Use `native::Composition` with `native::Registration` when you need `Send` futures on a multi-thread executor.

```rust
use solana_kit_signers::{
    SignerCapability,
    native::{Composition, Registration},
};

// `hardware` and `wallet` are your own backends.
let first = SignerCapability::new(&hardware);
let second = SignerCapability::new(&wallet);
let signers = Composition::new(vec![
    Registration::partial(&first),
    Registration::composite(&second),
])?;
let signature = signers.sign_and_send(&transaction).await?;
```

The registration constructors are `partial`, `modifying`, `sending`, `partial_modifying`, `partial_sending`, `modifying_sending` and `composite`. Each one requires the matching backend traits at compile time. Nothing inspects traits at runtime. The composition then follows fixed rules:

1. Modifying signers run first, in order. A changed transaction cannot keep old signatures.
2. Partial signers sign the final snapshot.
3. A sending signer submits once, if you call `sign_and_send`. It prefers a sender-only registration over a composite one. Having no sender fails with `MissingSender`. Having more than one sender-only registration fails with `MultipleSenders`.

A composition accepts at most 256 registrations. `sign_batch` accepts at most 256 transactions and stops at the first failure. Effects that a backend has already performed cannot be rolled back.

## Implementing a signer

A backend implements `Signer` plus the traits it supports. This one returns a ready future:

```rust
use solana_kit_core::{Address, crypto::Keypair, transaction::Transaction};
use solana_kit_signers::{Error, SignatureOutput, Signer, TransactionPartialSigner};
use std::future::{Ready, ready};

struct Remote {
    key: Keypair,
}

impl Signer for Remote {
    fn address(&self) -> Address {
        self.key.address()
    }
}

impl TransactionPartialSigner for Remote {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;

    fn sign_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Sign<'a> {
        let bytes = transaction.message().bytes();
        ready(Ok(SignatureOutput::new(self.address(), bytes, self.key.sign(bytes))))
    }
}
```

A real wallet or hardware backend returns its own future, which can stay pending while the user approves.

> [!IMPORTANT]
> With the `crypto` feature, the crate verifies every accepted signature with Ed25519. Without it, acceptance only checks the address, the exact bytes and that the signature is nonzero. In that case, verify external signatures yourself before you trust them.

## Message signers

`solana_kit_signers::message` handles signing many bare messages at once. `SignableMessage::new(bytes)` or `SignableMessage::from_text(text)` holds the exact bytes plus a signature dictionary. `from_text` does not normalize the text or wrap it in an off-chain envelope. For the standard off-chain message format, see [Messages and transactions](transactions.md).

- `MessagePartialSigner` signs a whole slice of messages in one call.
- `MessageModifyingSigner` returns one replacement per input.
- `SingularMessageSigner(backend)` adapts a plain `MessageSigner` by calling it once per message.
- `message::local` and `message::native` provide their own `Registration` and `Composition` types.

## Errors

`solana_kit_signers::Error` covers `Rejected`, `Cancelled`, `TooManySigners`, `ConflictingSigner`, `ForeignSigner`, `ChangedSnapshot`, `MissingSender`, `MultipleSenders` and `Transaction(transaction::Error)`.

## Status

The [signer capability receipt](https://github.com/pina-rs/solana_kit/tree/main/docs/parity/signer-capabilities.md) lists the reference declarations as partial or pending, and none as complete. Signer attachment and configuration, key generation inside this crate, and persistence are still pending. Wallet-backed signers are covered in [Wallets](../runtimes/wallets.md).
