# Unsigned Helius smart preparation

This source-only stack starts at main checkpoint
`4191d2288293aa00ccace309a74837cd6fa26f73` and preserves the frozen reconciliation
`4c690a192d1ded2654a0a655023d75da89388dca`. That earlier F34-based source and its
ignored quality evidence remain on their original branch. Both restore the retained producer
`347bba41b952e2270629d0de13217f6e7c453486` (tree
`fdcf668e86ba7ef19a934fbd1ea89097ace508a1`). Smart-preparation behavior is
preserved without a compatibility shim. The pipeline reuses the existing
simulation, fee, buffering and RPC read interfaces. Fee request construction
enumerates every optional field explicitly against the current schemas.
Current TLS sender, signup completion and client resource/reactive code remain
intact. The only new SDK dependency is the existing portable Compute Budget crate;
Root owns authentic resolution of root and independent consumer lock graphs.

## Dependency resolution prerequisite

Root unit 1087 genuinely resolved both Cargo.lock and
verification/helius-portable/Cargo.lock. The root graph adds the Compute Budget
dependency edge; the portable graph adds its package and dependency edge. The
source worker preserved those generated lock changes without editing them.

The first native SDK compile of `ee572ce5` with those locks exited 101: two fee
request constructors incorrectly assumed `Default` implementations. The repair
sets all omitted request/options fields to `None` and only `recommended` to
`Some(true)`, preserving the exact account-key request JSON. No model gains a
Default implementation. The retained failure is evidence/1087/run-smart-native-b/sdk-tests.stderr.
Compilation and behavioral success require Root's rerun. Historical native,
portable and OS qualifications do not qualify this repaired producer.

## Client contract

`create_smart_transaction` accepts typed caller instructions, an explicit payer
or signer-address fallback, a real simulation BlockhashValue and caller-owned
lookup snapshots. All caller Compute Budget instructions, including malformed
variants, are removed before compilation. Missing payer and zero explicit compute
limit reject before RPC. The stripped draft and final price/limit layout are
Core-compiled with the supplied simulation lifetime before any exchange.

One unsigned simulation precedes recommended account-key pricing and a final
fresh-blockhash read. Fee identities retain payer first and unique instruction
account encounter order; program IDs are not added. An explicit positive limit
overrides buffering while simulation still executes. Otherwise the existing
binary buffer is used. Exact rate/total caps reuse the existing pure resolver.
Missing/null/invalid fee estimates, execution errors, phase-tagged RPC failures,
invalid final expiry and stale final contexts reject through typed redacted errors.

The final blockhash and real expiry come from the final RPC response. Simulation
replacement metadata cannot substitute for final expiry. Maintained Compute Budget
builders encode price then limit before the original user order. Returned signature
slots remain empty. Signer addresses are payer candidates, never signing capabilities.
Simulation and final-blockhash contexts remain separate observations without a
single-bank-snapshot claim. Only blockhash lifetimes are supported; no nonce,
lookup discovery, signer callback, submission, retry or spawned task is added.
Dropping the preparation future drops its currently owned exchange.

Pinned Dart anchors are create_smart_transaction.dart:92–172 and
create_tx_message.dart:79–96 at revision
0138bb956d2d81ccc0c56edc602401c0df2f2b46. Their source-files.tsv member hashes are:

| Member                        | SHA-256                                                            |
| ----------------------------- | ------------------------------------------------------------------ |
| create_smart_transaction.dart | `08fb1f4a3522d929a6abce7b74d74c9c11b9fe8be49ac1d30a025b562f2894c5` |
| create_tx_message.dart        | `75775a7049d5867eebc1b5ae6e3f882b8f73333ac6eb4c3e51c35e6ea5c3888d` |

Rust compiles actual unsigned wire, uses an explicit simulation lifetime and typed
lookup snapshots. Existing exact-decimal, buffer and wire-range qualifications
remain applicable. No canonical declaration ID or pending parity verdict changes.

## Preserved source predicates

The retained smart group still checks default `100000 -> 110001` units and
`2000 -> 221` lamports, explicit 75000 units, exact request order and input JSON,
payer fallback/override, account-key deduplication, caller lookups, stripped draft,
final price/limit bytes, user instruction order, unsigned wire and real expiry.
Boundary bodies preserve zero/missing payer, NaN/nonfinite buffers, invalid
construction, each RPC phase, simulation failure, missing/null/malformed/overflow
fee, malformed expiry and stale final context. Cancellation still polls each of
the three suspended phases and checks request ownership and one cancellation.
The retained native multi-thread witness still requires a Send future, exact
1000-unit/2-lamport default, two budget instructions, no signatures and three calls.

Only the new smart shared bodies/helpers change from assertions/expectations to
`Result<(), &'static str>`. SDK-derived indices and missing fields are checked;
transport fixture violations retain their first cause so a negative RPC case
cannot accidentally accept a malformed request as its intended provider error.
No unwind catcher, dummy success, no-op feature path or SDK implementation copy
is introduced. An additional wrong-order oracle executes the real preparation,
changes only the expected method order, and requires its concrete returned error.
A native capture control passes that same failing source case to the real capture
executor and requires failure. These authored controls are not executed locally.

The accepted CASES registry and its bodies remain unchanged. The single new
SMART_CASE is appended after that prefix, retaining the existing group name
smart-transaction-preparation. Native public-contract entry points propagate
its Result. Existing legacy group assertion semantics remain unchanged.

| Feature configuration           | Base declaration | Source proposal |
| ------------------------------- | ---------------: | --------------: |
| base, no crypto/checkout/signup |               35 |              36 |
| crypto only                     |               37 |              38 |
| checkout, no crypto/signup      |               40 |              41 |
| crypto + checkout, no signup    |               42 |              43 |
| signup, no crypto               |               49 |              50 |
| crypto + signup                 |               52 |              53 |

These counts register one new group in each configuration. They are not executed
results. Checked-in reports remain untouched and historical; default verification
must reject them until Root regenerates them from real output. All new compiler,
native, browser, both WASI and production wasmi verification is **UNEXECUTED** by
this source worker. Root owns execution after the explicit-constructor repair.

## Native capture and Root handoff

The existing binary had no documented flags. The added native-only `--capture`
mode executes every accepted body and then the new fallible smart case, printing
PASS only after a body actually returns. A smart failure emits FAIL with its
static cause, returns an error and produces a nonzero exit without an aggregate
PASS summary. No successful report row is synthesized from a checked-in report.
Capture skips only historical golden comparison. Normal mode still compares
every case name/order, final count summary and end of file exactly; stale or
trailing golden rows reject. Unknown arguments exit 2. Capture is unavailable to
the browser/WASI/wasmi guests. The existing corruption feature executes first
unchanged in both modes and cannot become a capture success.

After authentic root/consumer Cargo graph resolution, Root can first test the
SDK source and compile its native Send witness:

```sh
cargo +1.99.1 test --locked -p solana_kit_helius --features crypto,signup
```

Generate actual native stdout separately for each configuration. The report
targets are under verification/helius-portable/:

| Report target                 | Additional Cargo arguments before `--`           |
| ----------------------------- | ------------------------------------------------ |
| report.txt                    | defaults: crypto + signup                        |
| report-no-crypto.txt          | --no-default-features --features signup          |
| report-checkout.txt           | --no-default-features --features crypto,checkout |
| report-checkout-no-crypto.txt | --no-default-features --features checkout        |
| report-base-crypto.txt        | --no-default-features --features crypto          |
| report-base.txt               | --no-default-features                            |

The six exact capture commands, in that report-target order, are:

```sh
cargo +1.99.1 run --quiet --locked \
  --manifest-path verification/helius-portable/Cargo.toml \
  --bin solana_kit_helius_portable -- --capture

cargo +1.99.1 run --quiet --locked \
  --manifest-path verification/helius-portable/Cargo.toml \
  --bin solana_kit_helius_portable --no-default-features --features signup -- --capture

cargo +1.99.1 run --quiet --locked \
  --manifest-path verification/helius-portable/Cargo.toml \
  --bin solana_kit_helius_portable --no-default-features --features crypto,checkout -- --capture

cargo +1.99.1 run --quiet --locked \
  --manifest-path verification/helius-portable/Cargo.toml \
  --bin solana_kit_helius_portable --no-default-features --features checkout -- --capture

cargo +1.99.1 run --quiet --locked \
  --manifest-path verification/helius-portable/Cargo.toml \
  --bin solana_kit_helius_portable --no-default-features --features crypto -- --capture

cargo +1.99.1 run --quiet --locked \
  --manifest-path verification/helius-portable/Cargo.toml \
  --bin solana_kit_helius_portable --no-default-features -- --capture
```

Require exit 0, the exact unchanged prefix, the appended smart group and the
declared count before copying each captured stdout verbatim to its report target.
Never hand-author PASS rows. Verify intended corruption remains nonzero and
contains the unchanged corruption marker, including capture:

```sh
cargo +1.99.1 run --quiet --locked \
  --manifest-path verification/helius-portable/Cargo.toml --features corrupt -- --capture
```

After actual report promotion, rerun without `--capture`, then run native tests
and strict Clippy for the independent manifest with the six feature combinations.
Run its browser, both WASI and production host journeys using the existing CI
commands and negative controls. Do not use `--all-features` for ordinary positive
execution because `corrupt` intentionally fails. This source work performs no
Cargo/compiler/runtime, transaction signing, submission, network or credential I/O.
