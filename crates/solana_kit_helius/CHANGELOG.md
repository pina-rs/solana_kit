# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## solana_kit_helius [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/solana_kit_helius/v0.1.1) (2026-11-05)

### Features

- **Assess stake withdrawal from coherent account snapshots.** Add disjoint partial and full-close limits with history-based stake exposure, lockups and proposed authorities. Read Stake, Clock, Rent and History together through the standard RPC client with exact integer rent and cancellation propagation; do not use an unverified provider REST route.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [fd5c8c1](https://github.com/pina-rs/solana_kit/commit/fd5c8c1637bc08691bc707ba6329dfd93348d2e6)
#### Add complete DAS application records

_Owner:_ Ifiok Jr. · _Introduced in:_ [6907c4d](https://github.com/pina-rs/solana_kit/commit/6907c4dc3bb4d674ec009b26aa313052df90fcc3)

Require the Dart Kit's edition, token-account and page fields through explicit
complete types and fallible conversions. Preserve partial provider wire models,
exact integer amounts, optional metadata and indexed missing/null errors.

#### Add executor-neutral Helius RPC and REST operations, typed wire models,

_Owner:_ Ifiok Jr. · _Introduced in:_ [0406bf9](https://github.com/pina-rs/solana_kit/commit/0406bf965d7daa7d59bf63f7993dee8de38cb1ba)

bounded pagination, subscription bridges and caller-timed checkout observation.
Reuse Kit REST adapters and JSON admission. Keep normal publication disabled.
Align provider DAS option objects, edition pages, search filters and indexed metadata.
Use the Wallet v1 service with typed pages, exact decimals and one historical selector.
Build staking instructions locally and prepare transactions with caller-owned signing.
Discover typed Helius stake accounts with contextual RPC reads and bounded decoding.
Align all six webhook endpoints with primary wire casing, partial updates and provider errors.
Require object representations for declarative response models at every nesting level.

#### Estimate exact compute units from transaction wire

_Owner:_ Ifiok Jr. · _Introduced in:_ [6cdff7c](https://github.com/pina-rs/solana_kit/commit/6cdff7c8821bf44966f8d2af171764e01258f85e)

Add `transactions::get_compute_units` for one bounded simulation of caller-owned
transaction bytes with explicit read constraints. Preserve exact `u64` units and
evaluation context, including a valid zero report. Surface execution failure,
missing or null units, stale context and shared RPC errors through typed,
redacted errors.

#### Add unsigned checkout creation, project plan changes and pending-intent renewals

_Owner:_ Ifiok Jr. · _Introduced in:_ [a4fc982](https://github.com/pina-rs/solana_kit/commit/a4fc982567b181fdd3704b73030410d7bd4bc000)

behind a smaller checkout feature. Signup composes it with date parsing.
Support named plans or explicit prices, optional contact
data and positive quantities. Share checkout preparation with unified signup.
Keep payment input failures in PaymentError and SignupError::Payment, retaining
original service errors. Reject mismatched renewal identities, dot-segment IDs and zero totals;
propagate preview failures without retrying initialization or submitting funds.

#### Resolve exact priority fees and buffered compute units

_Owner:_ Ifiok Jr. · _Introduced in:_ [fb8ffdf](https://github.com/pina-rs/solana_kit/commit/fb8ffdf31ce8b62919eb91104e1d4336c212e0f8)

Add pure `transactions::resolve_priority_fee` with exact decimal flooring,
consistent rate and total caps, checked rounded-up lamports and typed wire-range
errors. Add `transactions::buffer_compute_units` with the source binary ceiling,
positive floor and runtime maximum, rejecting non-finite buffer arithmetic.

#### Add signup payment completion behind the existing signup feature.

Resume a

_Owner:_ Ifiok Jr. · _Introduced in:_ [94c00ac](https://github.com/pina-rs/solana_kit/commit/94c00ac97b23ef2511ad4353d7a12b2a0d1d354e)

caller-submitted payment receipt through checkout observation, bounded first-project
discovery, and existing API-key provisioning. Preserve pending receipts on timeout,
errors and cancellation. Keep approval, signing, submission, JWT binding, time and
transport deadlines with the application.

#### Add optional unified signup with caller-owned JWT binding, time and transport.

_Owner:_ Ifiok Jr. · _Introduced in:_ [31cb66c](https://github.com/pina-rs/solana_kit/commit/31cb66c60aafa802f214e7c9135579f855e915c7)

Reuse existing project keys or create one key, and prepare hosted checkout for new
subscriptions without signing or sending payments. Validate encoded signing keys,
billing windows and payment links. Propagate preview failures and reject zero totals.
Keep HTTP and date parsing dependencies behind the signup feature.

#### Prepare unsigned Helius smart transactions

_Owner:_ Ifiok Jr. · _Introduced in:_ [fdbc492](https://github.com/pina-rs/solana_kit/commit/fdbc492e2d6633d3b77fa0ac62cbee4a11bc90da)

Add `transactions::create_smart_transaction` with caller-owned instructions,
payer identities, simulation lifetime and lookup snapshots. Simulate before
recommended account-key pricing and final blockhash refresh; reuse exact buffer
and fee caps, then compile unsigned v0 price/limit instructions with the real
refreshed expiry. Preserve typed failures and cancellation without signing or
submitting the transaction.

Reconcile the retained producer against current client/signup/TLS interfaces;
preserve its shared behavioral predicates through fallible source bodies and
append one group without changing the accepted prefix. Native-only capture emits
actual executed outcomes for root-owned golden regeneration; normal verification
continues to require exact goldens. All new execution remains pending Root/CI.

#### Choose the validator for local staking preparation

_Owner:_ Ifiok Jr. · _Introduced in:_ [e2e8532](https://github.com/pina-rs/solana_kit/commit/e2e8532e191825dccd72bb7f1866303a8ef366e3)

Add `StakeRequest::validator_vote` for caller-selected validator vote accounts.
Use `None` in existing request literals to retain the Helius validator default.
Instruction and transaction preparation preserve the selection with the same
RPC reads and required owner and new stake-account signatures.
