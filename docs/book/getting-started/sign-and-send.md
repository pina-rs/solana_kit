# Sign and send

This page takes the transaction from [Build a transaction](build-a-transaction.md), signs it, submits it, and waits until it reaches the commitment you want. Each step is a separate call. The SDK never retries a submission and never decides on its own that a transaction is confirmed.

The examples need `solana_kit_core` and `solana_kit_signers` with `crypto`, plus `tokio` with the `time` feature for the polling delay:

```toml
[dependencies]
solana_kit_core = { git = "https://github.com/pina-rs/solana_kit", features = ["crypto"] }
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_native = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_system = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_signers = { git = "https://github.com/pina-rs/solana_kit", features = ["crypto"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time"] }
```

## Sign, send, and confirm

<Steps>

### Sign

With `crypto`, sign directly with a `Keypair`:

```rust
let signed = transaction.sign(&payer)?;
signed.verify()?; // optional: checks every signature against the message bytes
```

`sign` returns a new `Transaction` with the payer's slot filled in. It fails with `transaction::Error::UnknownSigner` if the keypair is not one of the message's signers. If several parties sign, call `sign` once per keypair, or add an external signature with `with_signature(signer, signed_bytes, signature)`. That method checks that the bytes match the message exactly.

If you want signers to be swappable (local keys, hardware, wallets), use `solana_kit_signers`. With its `crypto` feature, `Keypair` implements the signer traits:

```rust
use solana_kit_signers::{SignerCapability, TransactionSigners};

let capability = SignerCapability::new(&payer);
let signed = TransactionSigners::new(&[&capability])?
    .sign(&transaction)
    .await?;
```

`TransactionSigners` runs the partial signers concurrently and merges their results in input order. [Signers](../core/signers.md) covers modifying and sending signers, and composition.

### Send

Encode the signed transaction and submit the exact bytes:

```rust
use solana_kit_client::methods::{
    Commitment,
    submission::{SendConfig, SendTransaction, SignedWire, WireEncoding},
};

let wire = signed.encode(true)?; // true: every signature slot must be filled
let signature = client
    .call(SendTransaction {
        wire: SignedWire::new(&wire)?,
        encoding: WireEncoding::Base64,
        config: SendConfig {
            preflight_commitment: Some(Commitment::Confirmed),
            ..SendConfig::default()
        },
    })
    .await?;
println!("sent {signature}");
```

`SignedWire::new` checks the bytes before anything goes over the network. It decodes them, makes sure every signature slot is filled, and saves the primary signature. When the node replies, the client checks that the signature it reported matches that saved signature. A mismatch returns `Error::SubmissionSignatureMismatch`.

Set `preflight_commitment` to match the commitment your inputs were read at. If you leave it out, the node runs its preflight check at its own default commitment. Then an account funded moments ago at `Confirmed` can look empty, and the send is rejected with "Attempt to debit an account but found no record of a prior credit".

<Callout kind="warning" title="A send result is not a confirmation">

A successful `SendTransaction` only means one node accepted the bytes. It does not prove the transaction was broadcast or executed. Do not resend blindly when a send fails or times out: the node may already have accepted it.

</Callout>

### Confirm

`solana_kit_client::confirmation::Confirmation` is a small state machine. You feed it observations and it tells you when the outcome is settled. With HTTP only, poll signature statuses for success or failure, and poll epoch info for blockhash expiry:

```rust
use std::time::Duration;

use solana_kit_client::{
    Client, Transport,
    confirmation::{Confirmation, Event, Policy},
    methods::{
        Commitment, Contextual, ReadConfig,
        node::GetEpochInfo,
        submission::SignatureStatuses,
    },
};
use solana_kit_core::{Signature, transaction::Lifetime};

async fn confirm<T: Transport>(
    client: &Client<T>,
    signature: Signature,
    lifetime: Lifetime,
) -> Result<(), Box<dyn std::error::Error>> {
    let policy = Policy {
        commitment: Commitment::Confirmed,
        min_context_slot: 0,
    };
    let read = ReadConfig {
        commitment: Some(policy.commitment),
        ..ReadConfig::default()
    };
    let mut confirmation = Confirmation::new(signature, lifetime, policy)?;

    loop {
        let Contextual { context, mut value } =
            client.call(SignatureStatuses(&[signature], None)).await?;
        let snapshot = Contextual { context, value: value.pop().flatten() };

        if let Some(outcome) = confirmation.observe(Event::Status { signature, snapshot }) {
            return Ok(outcome?);
        }

        let epoch = client.call(GetEpochInfo(Some(read))).await?;

        if let Some(outcome) = confirmation.observe(Event::Epoch(epoch)) {
            return Ok(outcome?);
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}
```

Call it with the lifetime you compiled with:

```rust
confirm(&client, signature, signed.message().lifetime()).await?;
println!("confirmed");
```

`observe` returns `None` while the outcome is still open. It returns `Some(Ok(()))` once the status reaches the policy's commitment, and `Some(Err(failure))` in two cases:

- The transaction failed on chain: `Failure::Transaction`, with the node's typed error.
- The block height passed `last_valid_block_height`: `Failure::BlockHeightExceeded`. Once that happens, the blockhash has expired and the transaction can never land, so you can safely build and sign a new one.

`Confirmation::new` rejects a `Lifetime::Unresolved` lifetime. That is the kind a decoded transaction carries, because the expiry height is not in the wire bytes. So keep the `Lifetime` you compiled with.

</Steps>

## Funding a test account

On a fresh keypair, request lamports before sending:

```rust
use solana_kit_client::methods::{
    history::ConfirmationStatus,
    submission::RequestAirdrop,
};

let airdrop = client.call(RequestAirdrop(payer.address(), 1_000_000_000, None)).await?;
wait_for_airdrop(&client, airdrop).await?;
```

Do not pass the airdrop to `confirm`. The node picks the airdrop transaction's blockhash, so you do not know its lifetime, and a guessed lifetime can make `confirm` report a false `BlockHeightExceeded` while the airdrop is still valid. Poll its status alone, with a timeout of your own:

```rust
/// Polls the airdrop's status until it is confirmed, for at most 60 seconds.
async fn wait_for_airdrop<T: Transport>(
    client: &Client<T>,
    signature: Signature,
) -> Result<(), Box<dyn std::error::Error>> {
    tokio::time::timeout(Duration::from_secs(60), async {
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;

            let Contextual { mut value, .. } =
                client.call(SignatureStatuses(&[signature], None)).await?;
            let Some(status) = value.pop().flatten() else {
                continue;
            };

            if let Some(failure) = status.err {
                return Err(failure.into());
            }

            if matches!(
                status.confirmation_status,
                Some(ConfirmationStatus::Confirmed | ConfirmationStatus::Finalized)
            ) {
                return Ok(());
            }
        }
    })
    .await?
}
```

Public devnet airdrops are rate limited and often fail. A local validator (`solana-test-validator`, at `http://127.0.0.1:8899`) or [Surfpool](../services/surfpool.md) is more reliable for development. `NativeHttp` accepts plain `http://` endpoints, so point it at the local URL.

## Other confirmation strategies

- `confirmation::send_and_confirm(client, transaction, config, policy, observe, cancellation)` sends a signed `Transaction` and checks the reported signature. It then waits on an `Observer` that you implement, so you control the subscriptions, timers and cleanup. It also sets `preflight_commitment` to the policy's commitment for you.
- The `confirmation-factories` feature of `solana_kit_client` adds `confirm_signature` and `submit_and_confirm`. They combine a WebSocket signature subscription with HTTP reconciliation, an injected clock, a deadline and cancellation. See [Subscriptions](../core/subscriptions.md).

## Status

The steps on this page (build a System transfer, sign, send with `SendTransaction`, and confirm by polling) were run end to end against a local `solana-test-validator` (Agave 4.2.2) while this page was written. The project's own published verification has not yet completed: the System client receipt still lists validator execution and durable nonce transaction journeys as unfinished, and some composed confirmation-factory paths still await verification. See [Parity and status](../reference/parity.md) for the current state.

## Next steps

<Cards>
<Card title="Messages and transactions" href="../core/transactions.md">Signature slots, decoding and inspection.</Card>
<Card title="Signers" href="../core/signers.md">Compose local keys, wallets and sending signers.</Card>
<Card title="Errors" href="../core/errors.md">Handle transport, RPC and transaction failures.</Card>
</Cards>
