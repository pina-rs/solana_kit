# Helius application client

Typed Helius requests and responses use the executor independent Kit RPC client.
REST operations use a separate caller-owned service adapter. The application owns
approved origins, credentials, deadlines, receive limits and cancellation. No
operation starts a runtime or automatically retries a submission.

This crate is exclusively for off-chain application code. For Solana on-chain
programs, use Pina or the official Solana program libraries.

The crate covers DAS, ZK compression, enhanced RPC and transactions, wallet
queries, webhooks, authentication, staking preparation and transaction submission.
Optional `crypto` enables caller-key authentication signing. Runtime libraries,
ambient credentials and automatic submission retries are excluded.

Enable `signup` for `auth::auth_signup`. It checks the first developer project,
reuses or creates a project key, or prepares hosted checkout for a new subscription.
Pre-authenticated callers need no cryptography. Combine `signup` with `crypto`
for encoded-key authentication, supplying Unix milliseconds explicitly. A caller
factory binds the actual returned JWT to its approved Developer service adapter.
Project keys remain separate from bearer tokens. Payment approval and submission
remain with the application.

The `signup` feature adds fundamental HTTP URI and date parsing dependencies;
ordinary protocol clients keep their original dependency graph. Billing windows
require timestamps with explicit RFC3339 offsets. Preview errors propagate, zero
totals are rejected, and no payment-host environment variable is read.

REST adapters implement the shared `solana_kit_client::rest::Transport<Service>`
contract, reexported as `ServiceTransport<Service>`.

Wallet v1 beta operations use the independently approved `Service::Wallet`
origin. Legacy Enhanced Transactions operations continue to use `Service::Api`.
Wallet display amounts preserve exact JSON decimals; historical queries require
one `BalanceAtPoint` selector. No returned balance is a cryptographic proof.

Webhook management uses `Service::Webhooks`, independently approved for the
provider's `api-mainnet.helius-rpc.com` origin. Wire keys retain `webhookID` and
`webhookURL`. Updates keep the route identity outside their partial JSON body.
Provider errors remain inspectable as bounded, redacted `Error::Provider` values.
Delete success returns `true` after a successful HTTP acknowledgment; malformed
nonempty JSON remains an error under the shared admission policy.

Staking preparation reuses Kit System and Stake builders. The application supplies
the new stake account and its signer, owns RPC and execution, and decides whether
to submit the prepared transaction. Stake-account discovery checks the withdrawal
authority and preserves the evaluation context and account metadata. Withdrawal
evaluation remains under review; instruction construction does not establish
withdrawable funds.

```rust
use solana_kit_helius::{
    service::{Error, Service, ServiceClient, ServiceTransport},
    webhooks::GetAllWebhooks,
};

async fn load<T: ServiceTransport<Service>>(services: &ServiceClient<T>) -> Result<(), Error> {
    let webhooks = services.call(GetAllWebhooks {}).await?;
    // The application decides how to display or store the response.
    let _ = webhooks;
    Ok(())
}
```

This draft remains unpublished. Source coverage and the runtime evidence
are recorded in `docs/parity/helius.md` in the repository. The pinned Dart API
defines the porting scope; live provider compatibility needs separate evidence.
