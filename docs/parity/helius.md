# Helius application client

`solana_kit_helius` ports the Helius package from Dart source commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. Its MIT notice is retained alongside
the Unlicense for original Rust code. All APIs are for off-chain application
clients. Use Pina or the official Solana program libraries for on-chain programs.

The crate includes DAS, ZK compression, priority fees, enhanced RPC and
transactions, wallet queries, webhooks, authentication and developer services,
staking preparation, Sender submission and binary preconfirmation decoding.
These are typed protocol operations. Applications supply their adapters,
approved destinations, authentication, entropy, storage and scheduling.

## Runtime ownership

RPC operations use `solana_kit_client::Client`. REST operations use the shared
`rest::Transport<Service>`, request body and JSON admission contracts. This lets
the maintained native, browser and WASI REST adapters serve Helius without a
second transport implementation. Normal dependencies exclude Tokio, browser
bindings, Wasm runtimes and upstream Solana clients. Cryptographic authentication
signing is optional through `crypto`; deterministic protocol models need no
ambient randomness.

One REST call performs one exchange. Transport implementations enforce receiving
limits, approved origins, deadlines and cancellation. The client checks status,
JSON budgets and duplicate keys before typed success. Sensitive fields and raw
HTTP error bodies require explicit access and stay out of ordinary diagnostics.

Declarative response models require JSON objects even when nested inside other
objects or arrays. Their map decoder preserves field constraints, missing/null
distinctions and exact numeric values without an intermediate JSON value tree.

Pagination has finite page and retained-record budgets and detects repeated
cursors. Checkout observation uses caller-owned monotonic time and sleep futures.
Its overall window is checked between exchanges; each exchange needs its own
transport deadline. Polling observes status and does not resubmit payments.
Preconfirmation is an untrusted early signal, not transaction confirmation.

## Verification scope

The maintained runner imports the original public contract bodies. The base
registry retains 35 groups, or 37 with cryptography. Checkout expands these to
40 or 42 groups. The historical signup reports contain 44 or 47 groups; the
[completion source unit](helius-signup-completion.md) expands the expected signup
registry to 49 or 52, pending execution and report refresh. Six fixed report
manifests guard names and counts. Seven existing native multi-thread tests require concrete REST,
staking preparation, stake-discovery, unified-signup, checkout, renewal and
compute-estimation futures to retain `Send`. Orchestration tests are feature-gated.
Local `Rc` exchanges exercise local futures without a universal `Send` requirement.
The new signup-completion native witness and its five portable groups remain
unexecuted at this source handoff.

The base suites passed native Rust, Chrome, both WASI previews and the production
LeptosKit interpreter in [the foundation PR](https://github.com/pina-rs/solana_kit/pull/76).
All 47 checks and 22 workflow runs passed, including intended corruption, fuel
exhaustion and unrelated-panic controls. The signup suites also passed these hosts
in [the signup PR](https://github.com/pina-rs/solana_kit/pull/78). The expanded
payment suites pass local native execution; their exact-source browser, WASI and
interpreter execution remains required on this change's CI. Generated installed
desktop, Android and iOS application journeys remain part of the wider goal.

The canonical [capability ledger](capabilities.tsv) retains all 1,790 Helius
declarations. They remain pending. The provenance audit checks all 52 distinct
defining source files against the immutable hashes. Neither those hashes nor
the contract groups establish declaration-by-declaration behavioral parity.
Paid-service credentials are not required for deterministic tests; no live
provider verification or normal registry release is claimed.

## Compute unit estimation

`transactions::get_compute_units` accepts caller-owned transaction wire and
explicit shared `ReadConfig` constraints. One bounded `simulateTransaction`
exchange encodes the exact bytes as Base64, sets `replaceRecentBlockhash: true`
and `sigVerify: false`, and preserves the node's evaluation context with exact
`u64` units. A reported zero is valid. A typed execution failure takes precedence
over units; absent or null units and contexts below the requested minimum slot
reject through distinct errors. Debug and Display redact response details while
typed variants retain context and the original failure for explicit inspection.

The pinned Dart `get_compute_units.dart:17-56` supplies the estimator behavior;
its source hash remains in `source-files.tsv`. The Rust API uses Solana
transaction wire supplied by the caller. This unit adds no smart-transaction
construction or submission behavior.

The shared `compute-units-estimation` group covers actual RPC framing, exact
Base64 bytes and flags, one exchange, zero and large integers, execution-error
precedence, missing/null/malformed units, bounded admission, redaction, minimum
slots and cancellation of a suspended exchange. It is registered in the six
feature streams shared by native, browser, both WASI previews and the production
interpreter. The all-feature native runner and concrete Tokio Send case pass
locally, alongside strict Clippy and API documentation. Current-source browser,
both WASI previews and production interpreter execution remain required in CI.

## Pure priority fees and compute buffers

`transactions::resolve_priority_fee` reuses exact `Decimal` inputs for the
estimate, optional rate cap and optional total-lamport cap. Negative values clamp
to zero. Rates floor to whole micro-lamports, and total caps floor before reducing
the rate. The total is the checked integer ceiling of rate times compute units
divided by one million, preserving a consistent fee for every transaction version.
Zero units yields zero total and ignores the total cap. Caps apply before final
wire-range checks, so even a cap of `u64::MAX + 1` can rescue a huge estimate at
`u64::MAX` units into rate 1000000 and total `u64::MAX`.

The private decimal conversion handles fractions and exponents without binary
floating point or a new arbitrary-precision dependency. It visits the existing
token and at most an integer's digit width of exponent padding. Comparison
ceilings retain every cap that can constrain a supported rate. Final rate and
total overflow have distinct typed errors.

`transactions::buffer_compute_units` follows the pinned helper's binary ceiling,
then applies the caller minimum, positive floor and maximum of 1400000. A zero
minimum is allowed; even a minimum above the maximum clamps to that maximum.
The default `100000 * 1.1` ceiling remains 110001, so rate 2000 costs 221 lamports.
Finite negative percentages retain the source floor behavior. Non-finite config
or multiplication rejects before the result narrows to `NonZeroU32`.

The source is `priority_fee.dart` plus `bufferComputeUnits` in
`create_smart_transaction.dart`; their immutable hashes remain in `source-files.tsv`.
Rust's JSON-decimal inputs exclude the Dart helper's NaN, infinity and non-numeric
caps. Unsigned types exclude negative units and minimums. Exact large fee rates
and caps retain decimal precision that Dart doubles can lose. Both output fields
must fit wire `u64`; the Dart total is an unbounded `BigInt`.

The shared `priority-fee-resolution` group retains the source pure-fee and buffer
fixtures, with exact large/exponent inputs, simultaneous caps, fractional ceilings,
cap rescue, zero cases, overflow and invalid buffer controls. Native public
contracts and all six portable feature streams register the same group for
browser, both WASI previews and the production interpreter. This unit adds no
RPC, signing, submission or Send-suite cases. Native contracts, strict all-feature
Clippy and API documentation pass. Current-source CI remains required for the
six portable feature streams and generated installed app journeys.

## Complete DAS records

The pinned provider schema at `8560889fffe1af14d69e99d001cec29196756b87`
allows partial NFT editions, token accounts and token-account pages. The existing
wire types retain that behavior, including missing and null fields. The Dart Kit
requires mint and edition for an NFT edition; address, mint, owner and amount for
a token account; and total, limit and token accounts for its page.

`CompleteNftEdition`, `CompleteTokenAccount` and `CompleteTokenAccountList`
provide those complete application contracts through direct deserialization or
fallible conversion from the provider types. Conversion moves typed fields and
optional metadata without JSON reencoding or implicit defaults. A partial nested
account rejects the entire complete page. `IncompleteDasRecord` retains the wire
field, distinguishes missing from null and identifies the nested account position.

The shared `das-vectors` group exercises all nine required fields, both absence
forms, exact integers, opaque extension metadata and array/type rejection.
It also converts real typed `getTokenAccounts` Method results. Its registration
is shared by native, browser, both WASI previews and the production LeptosKit
runner. Registration alone does not prove execution of this change; its current
source must pass CI. These checks do not authenticate provider records, establish
live service availability or complete the Helius declaration ledger.

## Unified signup

The opt-in `signup` feature authenticates a caller or accepts a pre-authenticated
identity. A fallible caller factory binds the resulting developer JWT exactly
once to an approved service adapter. An API key is never reused as a bearer JWT.
The encoded-key path accepts standard, URL-safe, unpadded and percent-escaped
Base64, validates both key halves and erases temporary key buffers before I/O.
It requires `crypto`; pre-authenticated signup does not.

The first existing project controls the result. Matching subscriptions reuse
the first key or create exactly one key when the successful key list is empty.
Other plans return an explicit upgrade outcome. New subscriptions require contact
data, freeze one configured price, preview it and initialize one hosted checkout.
The result supplies unsigned payment terms; it never signs or submits funds.

Twelve boundary fixtures execute the pinned Dart billing helper independently.
Rust preserves its whole-hour monthly and yearly comparisons for explicit-offset
RFC3339 timestamps. Agent plans bypass duration matching, as the reference does.
Offset-free dates and normalized invalid calendar values are not accepted through
host-local timezone behavior. They fail the existing-plan match instead.

The Rust flow deliberately propagates preview transport, HTTP and schema errors
instead of adopting the reference's broad catch. It rejects zero preview and
initialized totals, validates explicit payment bases before checkout, encodes
opaque intent IDs and query keys, and never reads payment-host environment values.
One resolved price is reused instead of repeating configuration discovery.
These are explicit hardening and ownership choices.

HTTP URI and percent encoding dependencies are optional with `checkout`.
`signup` composes it with parsing-only date dependencies. The original default and
crypto graphs remain separate. CI checks all six combinations on every target, including
injected-Tokio rejection. Submitted payments can now resume through checkout and
project/key provisioning using the [completion helper](helius-signup-completion.md).
Approval and transaction submission remain application-owned. Completion runtime
verification and full declaration-level coverage remain unfinished. No live
provider call is exercised.

## Checkout creation, upgrades and renewals

The `checkout` feature exposes `auth::create_payment`, which accepts
`CreatePaymentRequest` with one `PaymentPrice`.
A named plan resolves one configured price; an explicit identifier bypasses
configuration discovery. Preview and initialization receive that same identifier,
reference, optional coupon and positive quantity. Contact fields are redacted and
optional for these flows. New-account signup retains its required contact checks
before using this shared implementation.

`auth::upgrade_plan` is the same operation with an existing project ID as `ref_id`.
It leaves upgrade eligibility with the provider. `auth::pay_renewal` reads exactly
one existing intent and requires its returned identity to match and its status to
be `pending`. It initializes nothing. Both return unsigned `PaymentLink` values;
approval, transaction construction, signing and submission remain caller-owned.

`PaymentError` distinguishes input, zero-total, intent-state and service failures.
Signup wraps payment-input failures in `SignupError::Payment`; original service
errors retain `SignupError::Service`. Payment URL construction uses `PaymentError`
as well. These APIs remain part of the unpublished implementation.

The pinned `createPayment`, `upgradePlan` and `payRenewal` sources define this
scope. Rust represents price alternatives with an enum and positive quantities
with `NonZeroU64`. It deliberately rejects mismatched renewal identities and zero
initialized totals, propagates preview failures, and reuses one resolved price.
Empty and dot-segment intent IDs cannot navigate outside the intended payment path.
It does not repeat the reference's potentially inconsistent price discovery or
broad error catch. Low-level checkout operations remain available for explicit
application policies.

Five shared groups exercise all plan/period pairs, explicit-price forwarding,
exact amounts above the JavaScript safe-integer boundary, renewal rejection,
provider error custody and actual suspended-exchange cancellation. Each runtime
executes those original bodies. Cancellation during initialization releases the
local exchange; it cannot undo a request already accepted by the provider. The
caller must reconcile an uncertain intent before choosing another initialization.
Checkout-only configurations execute without the signup date parser, with and
without cryptography, through the same browser, WASI and production interpreter paths.
These current-source portable executions remain required on this change's CI.

## Provider contract review

The independent provider reference is Helius SDK commit
`8560889fffe1af14d69e99d001cec29196756b87`. Its enhanced transaction schema
identifies `feePayer` as an address string, correcting the Dart source's integer
field. A shared executable regression rejects the old implementation before the
correction and retains the address through serialization afterward.

The Dart source exposes separate legacy Auth and Developer API operations. Their
service slots and credentials remain distinct. Legacy Auth routes retain the
Dart wire contracts; the current primary SDK documents the Developer routes.
Those references do not establish live acceptance of the legacy endpoints.

Enhanced optional fields, numeric and string token amounts, recursive
instructions, raw token balance deltas and REST history query names now follow
the provider schemas. Priority-fee lookback uses a validated slot count from
1 through 150. V2 account and transaction history requests use positional
arguments. Account pages preserve context and require a nullable `paginationKey`;
an empty intermediate page still follows its cursor. History retains full or
signature records and checks the requested transaction encoding.

Shared regressions reproduced the old optional-field, V2 tuple and boolean
lookback failures before these corrections. Native contracts and compilation
checks exercise the corrected schemas; paid-service behavior remains unverified.

The 26 ZK method fixtures now use the Photon server schema at commit
`ca866a609a355077c4590b2e512e58b879087233`. Context, nullable account lookups,
direct-array batch parameters and nested token data follow that schema. Batch
counts, returned identities and tree associations are checked before exposing
results. The shared DAS/ZK asset-signature method has one request and decoder,
with signature/operation tuples in its result.

The server's compressed Groth16 components are JSON byte arrays. Both Helius SDK
references incorrectly describe them as strings; the Rust client follows the
[server schema](https://github.com/helius-labs/photon/blob/ca866a609a355077c4590b2e512e58b879087233/src/openapi/specs/api.yaml).
Decoding these fields does not authenticate a proof or verify its curve points.

The twelve DAS request fixtures now cover nested sorting and display option
objects, search grouping pairs and token-account cursors. NFT editions decode
as a page object with master-edition facts. Edition and token-account methods
reject sequence results even when every response field is optional. Asset,
proof and page models retain indexed slots, native balances and optional
metadata. Prices and large integers round-trip without floating-point coercion.

The edition-page regression failed against the old sequence decoder. Additional
controls reject invalid sorting, option values, grouping lengths and numeric
fields. Optional fields retain missing and explicit null values. These fixtures
do not establish live service acceptance, cryptographic proof verification or
complete validation of every nested extension schema.

The wider review still needs to adjudicate DAS required fields and extension
schemas, remaining provider services and live withdrawal-program compatibility. The Dart fixtures
alone do not establish current provider compatibility.

Wallet operations now target the provider's Wallet v1 beta API. Applications
approve its `Service::Wallet` origin independently from the legacy Enhanced
Transactions API. Identity and batch identity, paged balances, history,
transfers, historical balances and initial funding follow the provider's seven
endpoint implementations. Wallet history remains distinct from the enhanced
transaction operation on the v0 service.

Historical requests select one time, datetime or slot through `BalanceAtPoint`.
Decoding checks the echoed wallet, mint and selector; identity lookups check the
returned address. Required nullable metadata remains distinct from a missing
field. Display balances and signed deltas retain exact JSON numbers. Raw amounts
remain strings. These checks do not authenticate a balance or prove a provider's
economic calculations. The Wallet API is beta; deterministic fixtures do not
establish live service availability.

Staking instruction and transaction preparation now runs locally through the Kit
System and Stake builders. It uses exact lamport inputs, checked rent addition,
the caller's new account identity and caller-owned aggregate signer. Preparation
reads rent and a recent blockhash through the existing RPC client. It returns a
signed v0 transaction without submitting it. Changed messages and missing
signatures reject; `crypto` additionally authenticates every signature.

`StakeRequest::validator_vote` preserves the optional validator choice in the
pinned Dart `CreateStakeTransactionRequest` and its custom-validator fixture.
`Some(address)` selects that validator vote account; `None` retains
`HELIUS_VALIDATOR`. Both instruction-only and signed transaction preparation
carry that address into the readonly vote account of the delegation instruction.

The unsupported draft staking REST routes and service slot are removed. The
shared `staking-preparation` group checks instruction bytes, roles, request
counts, default and custom validator selection, overflow, signer failures,
changed snapshots and cancellation. Validator cases resolve the compiled vote
account, retain owner and new-account signatures, and require one rent read or
two RPC reads for instruction-only or transaction preparation, respectively.

`staking::get_withdrawable_amount` replaces the draft provider route with one
standard contextual RPC read of the account, Clock, Rent and Stake History.
Its rules follow the pinned Stake processor at
`b37e4f76d4fbd08ad28c291f77620541489089c7`: before deactivation the full delegated
amount is reserved; cooldown uses the integer history calculation; lockup needs
both epoch and timestamp boundaries or the matching proposed custodian signer.
The result separates partial amounts retaining rent from complete inactive closure.
It does not approve intermediate amounts that would leave an underfunded account.

The current program derives reserve from live Rent and actual allocation. The
RPC adapter uses exact integer arithmetic for threshold bits representing 1 or 2,
including a literal odd reserve above the JavaScript safe-integer range. Other
rent settings, incomplete or reordered history, mismatched clock/context slots,
missing or foreign sysvars, malformed data and authority failures reject. The
caller selects the deployed rate-change epoch explicitly. Older program versions
may have different reserve rules; their compatibility is not inferred.

Seven public Stake withdrawal bodies passed native execution, alongside the
original 37 Stake tests. The maintained generator includes all 88 program bodies,
and its actual native runner passed 88/88. Helius native contracts exercise the
real shared RPC frame, exact amount sets, failures and drop cancellation of the
single suspended exchange. Strict SDK Clippy passed. Current-source browser,
both WASI previews and production LeptosKit wasmi still require this change's CI.
These fixtures do not authenticate a node, identify a deployed program version,
prove live provider availability, supply signatures or guarantee a later withdrawal.

Stake-account discovery uses one contextual base64 program-account scan, filtering
the requested withdrawal authority at byte offset 44. The shared RPC client and
Stake decoder validate account representations, owner, allocation and state bytes.
The operation rechecks withdrawal authority, retains Helius delegations and
preserves account metadata and the returned slot. The caller controls minimum
context, account-count and codec budgets. A decoded discovery result does not
authenticate node state or establish withdrawable funds.

Webhook contracts now follow the provider's six endpoint implementations and
shared webhook types at the pinned SDK commit. `webhookID` and `webhookURL`
retain their exact casing. Creation permits omitted delivery policy and supports
`encoding`; updates carry a separate partial configuration without a body ID.
Response formats remain open strings and retain optional active-state metadata.
The Webhooks origin is approved independently through `Service::Webhooks`.

The primary response fixture failed against the old `webhookId` decoder. Shared
contracts exercise all six methods, opaque path identities, omitted fields,
empty deletion acknowledgments and exact provider-error custody. Non-null error
envelopes reject before typed success decoding, including deletion. The shared
JSON admission policy rejects malformed nonempty success bodies; it does not
adopt the TypeScript deletion helper's permissive parse-error fallback. No live
webhook creation, delivery or deletion is exercised by these fixtures.
