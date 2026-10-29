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

`transactions::get_compute_units` simulates caller-owned transaction wire once
through the bounded RPC client, using Base64, blockhash replacement and disabled
signature verification. Supply explicit `ReadConfig` constraints. The result
retains exact `u64` units and the node's evaluation context; zero is valid.
Execution failures, unavailable units and stale contexts have distinct typed
errors with redacted formatting. The all-feature native contracts and concrete
Tokio Send case pass locally, alongside strict Clippy and API documentation.
The compute-estimation phase passed all 49 CI checks, including browser Wasm,
both WASI previews, the production interpreter and installed app journeys.

`transactions::resolve_priority_fee` floors exact decimal rates and applies rate
and total-lamport caps to the same resolved rate. Its checked integer total rounds
up, and both outputs must fit `u64`. Zero units and negative-to-zero fee clamping
retain the source behavior. `transactions::buffer_compute_units` returns a positive
compute limit with the source floor and maximum. Its binary ceiling intentionally
maps 100000 units to 110001 at the default ten percent buffer; a rate of 2000
then costs 221 lamports. Non-finite buffer arithmetic rejects before narrowing.
These pure helpers add no RPC or transaction composition. Their native contract
fixtures, strict Clippy and API documentation pass. Current-source browser, WASI
and production interpreter execution remain required in CI.

DAS wire records follow the provider's partial-response contract. Applications
that require complete records can convert `NftEdition`, `TokenAccount` and
`TokenAccountList` through `TryFrom` into `CompleteNftEdition`,
`CompleteTokenAccount` and `CompleteTokenAccountList`. The complete types require
the Dart Kit's mint/edition, account identity/amount and page counts/account list.
`IncompleteDasRecord` distinguishes missing fields from explicit nulls and reports
the position of an incomplete account in a page. Optional metadata retains its
missing/null distinction, and raw amounts remain exact `u64` values.

```rust
use solana_kit_helius::das::{CompleteTokenAccountList, TokenAccountList};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let wire: TokenAccountList =
    serde_json::from_str(r#"{"total":0,"limit":0,"token_accounts":[]}"#)?;
let page = CompleteTokenAccountList::try_from(wire)?;
assert!(page.token_accounts.is_empty());
# Ok(())
# }
```

Complete types also deserialize directly, rejecting omitted or null required
fields and incomplete nested accounts. Completeness checks field presence and
types; it does not authenticate provider data or validate on-chain account state.

Enable `signup` for `auth::auth_signup`. It checks the first developer project,
reuses or creates a project key, or prepares hosted checkout for a new subscription.
Pre-authenticated callers need no cryptography. Combine `signup` with `crypto`
for encoded-key authentication, supplying Unix milliseconds explicitly. A caller
factory binds the actual returned JWT to its approved Developer service adapter.
Project keys remain separate from bearer tokens. Payment approval and submission
remain with the application.

After submission, borrow a `SignupAndPayResult::Pending` receipt with
`auth::resume_signup_and_pay`. Supply separate checkout and project-discovery
policies, monotonic time and sleep futures. Completion observes the existing
intent, discovers its first project and reuses or creates its API key. Checkout
timeout preserves the pending receipt; borrowing retains it on errors and drop.
See the [completion contract](../../docs/parity/helius-signup-completion.md) for
ownership and verification scope.

The `signup` feature composes `checkout` with date parsing; ordinary protocol
clients keep their original dependency graph. Billing windows
require timestamps with explicit RFC3339 offsets. Preview errors propagate, zero
totals are rejected, and no payment-host environment variable is read.

Enable `checkout` for `auth::create_payment`, `auth::upgrade_plan` and
`auth::pay_renewal`. Select a named plan or one exact price through `PaymentPrice`.
An upgrade uses the existing project ID as `CreatePaymentRequest::ref_id`.
A renewal reads one matching pending intent and returns its existing terms.
All three prepare unsigned links; none signs, sends or automatically retries a payment.
Preview failures propagate through `PaymentError`. Signup retains these failures
through `SignupError::Payment` or its original typed service cause.
Checkout adds HTTP URI and percent encoding dependencies without a date parser.

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
to submit the prepared transaction. Set `StakeRequest::validator_vote` to choose a
validator vote account; `None` delegates to `HELIUS_VALIDATOR`. Stake-account
discovery checks the withdrawal authority and preserves the evaluation context
and account metadata.

`staking::get_withdrawable_amount` reads the stake account, Clock, Rent and Stake
History in one standard `getMultipleAccounts` evaluation. It returns distinct
partial-withdrawal and full-close limits after checking the proposed authority,
lockup and history-based activation. The current program uses live rent and
exact integer thresholds of 1 or 2; other rent configurations are rejected.
Supply the deployed program's rate-change epoch explicitly (`Some(0)` for the
pinned current integer rules). Older deployments may use different rent rules.

The bundled read requires a matching clock/context slot and complete recent
history. These checks detect inconsistency rather than authenticating a node.
The full 512-entry history alone occupies 16,392 bytes before base64 encoding.
Configure the transport and shared client's receive budgets for the bundled
response, such as 64 KiB for an ordinary 200-byte stake allocation. The query's
per-account decode limits do not raise those separate receive budgets.
Signer identities remain proposals; destination checks, actual signatures,
submission-time state and execution still belong to the application. No
unverified provider withdrawal REST route or hidden extra bank read is used.

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
