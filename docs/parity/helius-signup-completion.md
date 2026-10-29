# Signup payment completion

The existing `signup` feature now exposes `auth::resume_signup_and_pay`,
`SignupCompletionPolicy` and `SignupCompletionError`. This source unit completes
the orchestration after the application approves and submits the payment prepared
by `auth_signup`. It adds no dependencies or runtime.

## Caller contract

After `auth_signup` returns `SignupResult::PaymentRequired`, retain its identity
and payment link. Independently approve, sign and submit the payment using the
application's transaction flow. Construct `SignupAndPayResult::Pending` from those
values and the actual submitted `Signature`, then borrow it when calling
`resume_signup_and_pay`.

Supply a Developer `ServiceClient` bound to the same identity's JWT, two explicit
`checkout::PollPolicy` values, a monotonic millisecond clock and a fallible
sleep-until function. The checkout policy bounds intent observation. A fresh
provisioning window begins only after confirmed checkout and bounds first-project
discovery. An empty final allowed project response returns `ProvisioningTimeout`
without starting another timer. Both windows check time between exchanges;
individual status, project, details and key-creation requests need caller transport
deadlines.

The helper observes the existing checkout intent and never initializes or
resubmits payment. Completed checkout discovers the first project, then reuses
the existing private signup provisioning helper. That helper reads the project's
details and reuses its first key. Only a successful empty key list permits one
creation request. It then builds mainnet and devnet endpoints from that key.
Details or creation failures propagate without retries.

Checkout expiry and failure become `Expired` and `Failed`. Exhausting checkout
time or observations returns the same `Pending` identity, link and signature.
Exhausting project discovery returns `ProvisioningTimeout`. Checkout, discovery,
timer, policy and project failures retain their typed causes. Every non-pending
input returns unchanged with no requests or clock access.

The input receipt remains borrowed through the entire future. The caller retains
it on every error and after dropping a suspended exchange or timer. Resuming that
receipt observes the intent again; it never sends the transaction again. Concrete
local transports and closures may capture `Rc`; concrete native inputs may retain
`Send`. The helper owns no task, executor or cancellation controller.

## Reference behavior and intentional boundaries

The pinned Dart revision is `0138bb956d2d81ccc0c56edc602401c0df2f2b46`.
`lib/src/auth/plan_management.dart:93–147` discovers the first project, reuses or
creates its first API key, and derives endpoints. Its `signupAndPay` entry at line
151 returns existing-subscription and upgrade results before payment. Lines
199–260 observe a submitted signature, provision on completion, distinguish
expired/failed checkout, and retain pending payment data. Its SHA-256 is
`c714d2b5db2e692b3fb5a69b4db268dc4ad9c87206f39b4c3be1af4a121dd00a`.

`test/auth/signup_test.dart:444–575` supplies the corresponding completion cases.
Its SHA-256 is
`a960491e2b5d67a338454929304e54d0cfce3eaf88d4a491835709e2c46d836e`.

Rust splits at the application-owned payment submission boundary. It requires
explicit policies rather than installing the Dart provisioning interval and
timeout defaults. Checked deadlines, decreasing-clock rejection, finite observation
counts and typed service/timer errors retain the existing Rust checkout contract.
No live provider, payment submission or declaration-wide parity is claimed.

## Source verification and execution handoff

Five new groups contain the original portable assertion bodies:

- `signup-completion-existing`: all five non-pending outcomes bypass observation.
- `signup-completion-provisioning`: delayed first-project discovery, first-key
  reuse, empty-key creation, endpoints, exact wire and request counts.
- `signup-completion-outcomes`: pending receipts, zero observation window, expired
  and failed status, and HTTP 410.
- `signup-completion-failures`: finite project budgets, service/schema failures,
  policy validation, decreasing time, deadline overflow and caller timer failures.
- `signup-completion-cancellation`: dropping suspended status, project, details,
  key-creation and timer futures releases their actual owned work and preserves
  the borrowed receipt.

The maintained Helius registry includes these groups for native, browser, both
WASI previews and the production interpreter. Its expected signup counts become
49 without crypto and 52 with crypto. A separate native Tokio test owns the
concrete client and receipt across `spawn` and asserts completed provisioning.
All new bodies are **UNEXECUTED** in this source handoff.

Source review of commit `fcbde821` found that the final empty project response
still started a timer after exhausting `max_polls`. The correction returns timeout
before that timer. Its regression uses a one-read budget and a counted timer that
would fail if called; the existing timer-failure control still exercises a window
with another observation available. This finding and correction have no runtime
verification at this source handoff.

Execution-derived `report*.txt` manifests retain the historical 44/47 signup
streams. Consequently their fixed-report admission needs an actual refresh before
the portable signup runners can validate the expanded registry. From the repository
root, the first native body targets bypass those historical report comparisons:

```sh
cargo test --locked -p solana_kit_helius --features signup --test public_contracts -- --nocapture
cargo test --locked -p solana_kit_helius --features signup,crypto --test public_contracts -- --nocapture
cargo test --locked -p solana_kit_helius --features signup --test native_signup_completion
```

The test-only registry prints `PASS <group>` only after that group's assertions
return, then prints its observed total after all bodies return. The runtime owner
can capture those real 49/52 streams before refreshing their fixed manifests and
running the portable targets. Formatting and source analysis cannot substitute
for these executions, strict Clippy, API documentation, or target runtime checks.
