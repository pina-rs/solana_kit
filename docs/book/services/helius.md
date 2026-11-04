# Helius

`solana_kit_helius` is a typed client for the Helius APIs: DAS, ZK compression, priority fees, enhanced RPC and transactions, wallet queries, webhooks, authentication and developer services, staking preparation, Sender submission and preconfirmation frames. It does not bring its own networking. JSON-RPC methods run through the shared `solana_kit_client::Client`, and REST operations run through a caller-owned `rest::Transport`, so the same code works on every runtime the SDK supports.

> [!WARNING]
> The crate is unpublished (`publish = false`) and its 1,790 capability-ledger declarations are still marked pending. The fixtures are deterministic and need no paid credentials. No live provider compatibility is claimed. See [Parity and status](../reference/parity.md).

## Add the crate

```toml
[dependencies]
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_helius = { git = "https://github.com/pina-rs/solana_kit" }
```

Pair it with a transport from [Choosing a runtime](../runtimes/index.md). Optional features:

| Feature    | Adds                                                                                                                           |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `crypto`   | Caller-key authentication signing (`auth::sign_auth_message`); with staking, signature authentication of prepared transactions |
| `checkout` | `auth::create_payment`, `auth::upgrade_plan` and `auth::pay_renewal`, which prepare unsigned payment links                     |
| `signup`   | `auth::auth_signup` and `auth::resume_signup_and_pay`; implies `checkout` and adds RFC3339 date parsing                        |

## How the client is shaped

There are two independent owners, and `Helius<R, S>` simply holds both:

| Owner | Type                           | Used for                                                                               |
| ----- | ------------------------------ | -------------------------------------------------------------------------------------- |
| RPC   | `solana_kit_client::Client<R>` | DAS, ZK compression, priority fees, enhanced RPC V2, compute estimation, staking reads |
| REST  | `service::ServiceClient<S>`    | Webhooks, wallet, enhanced transactions, auth, admin, developer, Sender                |

`Helius::new(rpc, services)` composes owners you have already configured. It creates no credentials or tasks.

Every REST operation names a `service::Service` slot (`Api`, `Webhooks`, `Wallet`, `Auth`, `Admin`, `Sender`, `Developer`). Your transport maps each slot to an approved origin and adds authentication. A server response can never pick the destination. `Config::new(api_key)` validates a non-empty key and exposes the documented origins (`rpc_origin`, `rest_origin`, `wallet_origin`, `websocket_origin`, `sender_origin`) for you to wire into your adapters. It does no I/O itself.

## Call a JSON-RPC method

Request types implement the client's `Method` trait, so you pass them to `Client::call`:

```rust
use solana_kit_client::{Client, Error, Transport};
use solana_kit_helius::das::{GetAssetRequest, HeliusAsset};

async fn asset<T: Transport>(rpc: &Client<T>, id: &str) -> Result<HeliusAsset, Error> {
    rpc.call(GetAssetRequest { id: id.to_owned(), options: None }).await
}
```

Request structs have public fields, and optional parameters are `Option`s. `None` leaves the parameter out entirely. The covered method families include:

| Module         | Methods                                                                                                                                                                                                                                 |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `das`          | `getAsset`, `getAssetBatch`, `getAssetProof`, `getAssetProofBatch`, `getAssetsByAuthority`, `getAssetsByCreator`, `getAssetsByGroup`, `getAssetsByOwner`, `getNftEditions`, `getTokenAccounts`, `getSignaturesForAsset`, `searchAssets` |
| `zk`           | The Photon compression methods, such as `getCompressedAccount`, `getValidityProof` and `getIndexerHealth`                                                                                                                               |
| `priority_fee` | `getPriorityFeeEstimate`                                                                                                                                                                                                                |
| `rpc_v2`       | `getProgramAccountsV2`, `getTokenAccountsByOwnerV2`, `getTransactionsForAddress`, `getTransfersByAddress`, plus bounded cursor collection (`all_program_accounts`, `all_token_accounts_by_owner`)                                       |

DAS records follow the provider's partial-response contract. If you need complete records, convert with `TryFrom` into `CompleteNftEdition`, `CompleteTokenAccount` or `CompleteTokenAccountList`. `IncompleteDasRecord` tells you which field was missing.

## Call a REST operation

REST operations implement `service::Operation` and go through `ServiceClient::call`. Each call is exactly one exchange:

```rust
use solana_kit_helius::{
    service::{Error, Service, ServiceClient, ServiceTransport},
    webhooks::GetAllWebhooks,
};

async fn load<T: ServiceTransport<Service>>(services: &ServiceClient<T>) -> Result<(), Error> {
    let webhooks = services.call(GetAllWebhooks {}).await?;
    let _ = webhooks;
    Ok(())
}
```

`ServiceClient::new(transport, budgets)` takes explicit `Budgets`. The client checks the HTTP status, the byte and JSON budgets, and duplicate keys before it decodes a typed response. Provider errors come back as bounded, redacted `Error::Provider` or `Error::Http` values.

## Transactions and fees

The `transactions` module has helpers for preparing transactions. None of them sign or submit.

| Function                   | What it does                                                                                         |
| -------------------------- | ---------------------------------------------------------------------------------------------------- |
| `get_compute_units`        | Simulates your transaction wire once and returns the exact unit count plus the node's context        |
| `resolve_priority_fee`     | Floors a decimal fee rate and applies rate and total-lamport caps                                    |
| `buffer_compute_units`     | Adds a percentage buffer to a unit count, with the source's floor and maximum                        |
| `create_smart_transaction` | Builds an **unsigned** v0 transaction with a compute price and limit placed before your instructions |

`create_smart_transaction`'s portable and native witnesses had not been executed when the crate was last updated. Its parity record is `docs/parity/helius-smart-preparation.md`.

## Sender

`sender::Submission` posts caller-signed wire bytes to the Sender origin once. It never signs or retries. Build the wire with `SignedWire::new(&bytes)?` from `solana_kit_client`. A successful response is a typed `Acknowledgement(Signature)`, and it must equal the transaction's own primary signature. An acknowledgement does not prove broadcast, confirmation or finality.

All `SenderRegion` helpers point at `https://sender.helius-rpc.com`, which Helius routes to the nearest region. Native REST composition rejects remote plain-HTTP roots.

## Staking

`staking` prepares stake transactions locally with the Kit System and Stake builders. You supply the new stake account and its signer, and you decide whether to submit. `StakeRequest::validator_vote` picks a vote account, and `None` delegates to `HELIUS_VALIDATOR`. `get_helius_stake_accounts` finds stake accounts by withdrawal authority. `get_withdrawable_amount` reads the stake account, Clock, Rent and Stake History in one `getMultipleAccounts` call and returns separate partial-withdrawal and full-close limits.

## Auth, signup and checkout

With `signup`, `auth::auth_signup` checks your first developer project. It then reuses or creates a project key, or prepares hosted checkout for a new subscription. After you approve and submit the payment yourself, `auth::resume_signup_and_pay` watches the intent and finishes provisioning. It uses the monotonic clock and sleep futures you pass in. With `checkout`, `create_payment`, `upgrade_plan` and `pay_renewal` return unsigned `PaymentLink` values. Payment approval, signing and submission always stay with your application.

## What is not included

- No runtime, executor, ambient credentials or environment lookup.
- No automatic retries for submissions or payments.
- No on-chain code. For Solana programs, use Pina or the official Solana program libraries.

The detailed status lives in [`docs/parity/helius.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/helius.md) and [`docs/parity/helius-signup-completion.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/helius-signup-completion.md).
