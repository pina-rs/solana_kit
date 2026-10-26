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

The maintained runner imports the original public contract bodies. Its default
registry contains 35 named groups, including two optional signing groups; the
non-crypto registry contains 33. Fixed report manifests guard names and counts.
Three native multi-thread tests require concrete REST, staking preparation and
stake-discovery futures to retain `Send`. Local `Rc` exchanges exercise
cancellation without a universal `Send` requirement.

The CI recipe runs those bodies in native Rust, Chrome, WASI Preview 1 and
Preview 2, then in the production LeptosKit interpreter. It includes intended
corruption, fuel exhaustion and unrelated-panic controls. A recipe is not an
execution receipt: browser, WASI runtime and interpreter acceptance remains
pending until the exact source passes CI. Generated installed desktop, Android
and iOS application journeys remain part of the wider implementation goal.

The canonical [capability ledger](capabilities.tsv) retains all 1,790 Helius
declarations. They remain pending. The provenance audit checks all 52 distinct
defining source files against the immutable hashes. Neither those hashes nor
35 contract groups establish declaration-by-declaration behavioral parity.
Paid-service credentials are not required for deterministic tests; no live
provider verification or normal registry release is claimed.

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
schemas, remaining provider services and withdrawal evaluation. The Dart fixtures
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

The unsupported draft staking REST routes and service slot are removed. The
shared `staking-preparation` group checks instruction bytes, roles, request
counts, overflow, signer failures, changed snapshots and cancellation. State-dependent
withdrawal amounts remain unfinished. In particular, the
provider SDK's epoch comparison will not establish the withdrawal contract
without independent Stake state/history review.

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
