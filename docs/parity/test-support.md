# Deterministic client test support

This additive receipt covers the unpublished `solana_kit_test_support` crate. It
is a developer-facing dev-dependency for exercising public
`solana_kit_client::Client` behavior without a node. It does not complete the
72-symbol pinned Dart `solana_kit_test_matchers` package, does not change any
canonical capability row, and does not claim whole-SDK completion.

## Source

The pinned reference is the local Dart repository at revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`, read through `git show`/`git ls-tree`.
The useful source capability is concentrated in
`packages/solana_kit_test_matchers/lib/src/{fake_rpc,fake_accounts,fake_signers,canned_rpc_responses}.dart`:
scripted RPC replies, account fixtures, signer mocks and canned results. The Dart
assertion matchers are intentionally not ported as an assertion DSL; downstream
Rust tests use ordinary `assert_eq!` against typed client values and exact
captured bytes.

## Public surface

| Item                              | Purpose                                                                               |
| --------------------------------- | ------------------------------------------------------------------------------------- |
| `ScriptedTransport`               | Finite ordered transport; clones share the script and request log.                    |
| `ScriptStep`                      | Success, correlated remote error, exact raw reply, typed failure or pending exchange. |
| `CapturedRequest`                 | Parsed ID and method plus exact body bytes and receive budget.                        |
| `PendingExchange`/`PendingStatus` | Manual wake, completion, delivery and cancellation control/observation.               |
| `canned`                          | Exact raw results for balance, latest blockhash and present/missing account reads.    |
| `signer` (feature `crypto`)       | Deterministic `Keypair` fixture using existing core crypto.                           |

`ScriptStep::success` and the remote-error constructors frame the envelope with
the request's actual ID. `ScriptStep::raw` leaves status and body untouched for
malformed, wrong-ID and non-success tests. Every exchange consumes exactly one
step and owns its reply bytes. An exhausted finite script returns
`TransportFailureKind::Unavailable` with an `UnexpectedRequest` source naming the
method, ID and script length; it never returns an empty successful-looking reply.
The real client still performs status, JSON, duplicate-key, budget, correlation
and schema validation.

A pending exchange records the request ID when the exchange starts. First poll
registers the caller's waker. `wake` notifies it without completing;
`complete`, `complete_success` and `complete_remote_error` accept one outcome and
notify the waiter; a second completion is rejected. Dropping the client call
future before delivery marks the exchange cancelled, clears the waker and drops
any supplied reply. The control handle retains no exchange future, runtime, task
or timer.

Waker cloning, waking and destruction, and retained error-source destruction,
run outside the fixture mutex. Three public-consumer regressions exercise
destructors that reenter `PendingExchange::status` during cancellation, waker
replacement and cancellation after error completion. All three first timed out
against the original locked destructors and pass after the ownership fix.

`ScriptedTransport` contains no global state, network access, clock, hidden retry
or background work. It is `Send + Sync` on native targets where the client error
type keeps those bounds, so a client can move across threads. Its exchange future
does not require `Send`; a local wrapper capturing `Rc` works with the same
public `Exchange` type.

## Canned fixtures and signer boundary

The canned helpers return raw result JSON and are used with a correlated
`ScriptStep::success`. `balance` preserves exact `u64` values including
`u64::MAX`; `latest_blockhash` uses a core `Hash`; `account` reuses the core
`EncodedAccount` model, validates data length against declared space, encodes
canonical base64 and maps an absent rent epoch to `u64::MAX`;
`missing_account` emits an explicit JSON null. These are three read fixtures, not
a second RPC response catalog.

The optional `crypto` feature is off by default and enables core crypto only.
`signer::test_keypair` uses a fixed public seed and must never hold real funds;
`signer::keypair` accepts a caller-owned seed. The returned core `Keypair` works
with the existing `solana_kit_signers` implementations when that crate is in the
test graph. No test key is present in default features or SDK runtime code.

## Deliberate omissions

- No address, byte, transaction or error assertion wrapper catalog.
- No subscription transport fixture or notification stream model.
- No typed `jsonParsed` account catalog.
- No responder closure, dynamic method registry or automatic retry.
- No node, HTTP adapter, timer, runtime, global state or production dependency
  edge from any SDK crate.

## Local evidence

All commands ran offline in this worktree with `CARGO_INCREMENTAL=0
CARGO_BUILD_JOBS=4` unless noted. No network, secret, validator, device, registry
or publication operation ran.

| Check                                                                                                            | Result                                                                                                            |
| ---------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `cargo +1.99.1 test -p solana_kit_test_support --locked --offline`                                               | 16 integration tests and 1 README doctest passed.                                                                 |
| `cargo +1.99.1 test -p solana_kit_test_support --all-features --locked --offline`                                | 17 integration tests and 1 README doctest passed.                                                                 |
| `cargo +1.99.1 test -p solana_kit_test_support --all-features --locked --offline` on `1.89.0`                    | 17 integration tests and 1 doctest passed at the declared MSRV.                                                   |
| `cargo +1.99.1 clippy -p solana_kit_test_support --all-targets --all-features --locked --offline -- -D warnings` | Passed with no warnings.                                                                                          |
| `RUSTDOCFLAGS="-D warnings" cargo +1.99.1 doc -p solana_kit_test_support --no-deps --all-features --offline`     | Passed.                                                                                                           |
| `cargo +1.99.1 fmt --all -- --check`                                                                             | Passed.                                                                                                           |
| `dprint check`                                                                                                   | Passed.                                                                                                           |
| `monochange check`                                                                                               | Passed; 0 issues in cargo, changeset, Dart and npm lint suites.                                                   |
| `cargo +1.99.1 xtask style`                                                                                      | Passed; repository readability 97.18, complexity 98.51; every file at least 90. New crate files were inventoried. |
| `cargo +1.99.1 xtask dependency-graphs`                                                                          | Passed all core graph selections and negative controls.                                                           |
| `cargo tree --workspace --locked -e normal,build -i solana_kit_test_support`                                     | Only the crate itself; no workspace package depends on it.                                                        |
| `cargo tree -p solana_kit_client --locked -e normal,build --no-default-features`                                 | No `solana_kit_test_support` entry.                                                                               |

A separate downstream consumer with path-only SDK dependencies lived at
an owned temporary `test-support-consumer` directory
and is not committed. It used the actual `Client` to assert exact `getBalance`
request bytes and ID, preserve `u64::MAX`, receive distinct remote and transport
failures, observe pending poll/wake/manual completion and drop cancellation,
exercise the deterministic signer through `MessageSigner`, move a native client
across a thread, and run an `Rc`-capturing local exchange. Seven tests passed.

## Limits

The crate is unpublished and `publish = false`. The external consumer is local
execution evidence, not a committed verification suite. No canonical
`capabilities.tsv` row, status, checker, threshold, shared golden or suite version
changed. Browser, WASI, LeptosKit, validator, device and registry-package journeys
were not exercised by this receipt.
