# Interpreted conformance fuel

This private harness executes the complete off-chain client suite in the pinned
upstream `WasmHost`. Every canonical and corruption report must match the native
case names, order, errors, pass count, and suite identity.

The host grants fuel separately to each guest entry. Starting the guest and
pressing its report each execute the whole suite. Wasmi 2 meters Wasm operations,
dynamic copying, and first-use lazy function translation. It does not meter
elapsed time. The upstream application default and production source are unchanged.

The harness grants the current 97-case suite **2,725,000,000 fuel per entry**.
The host computes `15_000_000 × (EXPECTED_CASE_COUNT - 1) +
5_000_000 × (MAX_BATCH + 1)` with checked arithmetic. These are pooled workload
allowances, not individual case limits. The inputs are compile-time fixture
metadata, never guest reports. Missing cases, invalid batch sizes, and arithmetic
overflow reject. A separate fresh guest with one fuel must trap with
`OutOfFuel` before committing anything. Production host defaults are unchanged.

The current total is 15,000,000 times 96 ordinary groups plus 5,000,000 times
257 authenticated transactions. All 97 canonical cases and the 96/97 sole-hash
corruption report execute through the actual pinned host. The 92-case minimum
measurements below are historical; no 97-case minimum is claimed.

## Accepted signer workload and historical 92-case measurements

The fixed batch fixture retains all 256 successful transactions and their
immutable signed-byte assertions. Its later good/bad/good control authenticates
one more transaction before rejecting the second. The 257-size rejection performs
no backend effects. Each successful transaction performs strict Ed25519 response
validation and completed-transaction verification, for 514 verifications in this
group. The fixture also derives its fixed key and signs its fixed message once.
`MAX_BATCH` is the same 256 constant used by these existing assertions.

Fresh actual pinned-host trials of the unchanged complete guest have these
minimum passing allowances. Every row has five exact-boundary successes and five
one-less fuel traps.

| Execution                                 | Minimum passing fuel |
| ----------------------------------------- | -------------------: |
| Complete canonical 92-case guest          |        2,343,080,723 |
| Canonical then complete corruption report |        2,343,080,723 |
| Isolated exact batch diagnostic           |        1,262,633,345 |

The isolated diagnostic completes the same `batches()` body, then rejects because
it deliberately has no suite report node. It is not a conformance pass. Its
threshold includes the wrapper and first-use translation. The thresholds are
not additive consumed-fuel counters. The full guest artifact has SHA256
`d68c3f1ea8f7458537f65a0073a4c3abdaa8bb52a151a4b1e653a71fc852c9e1`.
The preserved pre-accounting artifact has SHA256
`cd4c74d1c1a259c1b98b69967e3aa30c784a6b1fedcc386e24f5ed2492bd9bde`
and a 2,343,079,778 boundary for both complete report paths, also repeated five
times at the minimum and one less. Rebuilding after the constant visibility
change produces a distinct artifact and a boundary 945 fuel higher. No fixture
function body changes. Each artifact's repeated boundary range is zero.

The old uniform allowance of 1,380,000,000 traps on this finite combined workload.
Keeping 15 million for each of the other 91 ordinary groups gives 1,365,000,000.
The isolated batch threshold divided by its 257 authenticated transactions is
4,912,970.214 fuel. Rounding up to the next whole million gives the explicit
5-million rate and a 1,285,000,000 batch envelope. Four million would not cover
that observed work. The resulting total retains 13.10% above the full threshold.

A fixed 2.5-billion whole-suite cap would cover this artifact but would hide the
workload again. Raising every ordinary group to 26 million would also pass but
would replace the previously justified ordinary rate. Charging each batch
transaction the ordinary 15-million rate would grant 5,220,000,000 in total.
The selected separate transaction term retains the ordinary policy and uses the
smallest whole-million rate covering the measured fixed batch work.

Fuel measures metered interpreter work, not SDK speed or elapsed time. Fresh hosts
avoid translation or trap history across trials. Both complete-report paths have
the same deterministic boundary. Near-minimum one-less traps retain one report
commit, so exhaustion does not promise rollback. The one-fuel control separately
requires zero commits. Changed fixtures or compiler artifacts require new
measurement rather than an assumption that these bounds cover every workload.

## Original allowance evidence

The previous 81-case suite received `15_000_000 × EXPECTED_CASE_COUNT`, or
1,215,000,000 fuel per entry, with checked multiplication.

The original 81-case artifact has SHA256
`8a831c97c38ae699d6cf686a9b4257b3be01cc9aa4ca24ef1b3de13326ed9dbf`.
Fresh actual-host trials found a minimum start allowance of 1,019,776,430.
Five trials pass at that boundary and five trap with one fuel less.
The former 1,000,000,000 allowance traps without a report. A measured finite
1,200,000,000 allowance passes all 81 cases. The workload-scaled allowance retains
19.14% headroom above that original measured minimum. The 15-million rate rounds
above the observed 12.93-million average of the 77-case baseline.

The final accepted parent adds the literal LatestBlockhash request. Its rebuilt
guest has SHA256 `2e35bfc7f379c33f76eadc8adf85fb669b14c13a75e7fe66afbcbfc08aec0a44`.
Its minimum is 1,019,911,869, again with five passes and five one-less traps.
The allowance retains 19.13% headroom for this final artifact.

The diagnostic guest executes unchanged fixture bodies with separate selection
wrappers. These are minimum passing entry allowances, including each wrapper and
its translation state. They are not directly additive fuel-consumption counters.

| Diagnostic workload                      | Minimum passing fuel |
| ---------------------------------------- | -------------------: |
| Empty report                             |              225,068 |
| Original baseline, 77 cases              |          995,002,380 |
| Nine subscription families               |            3,761,219 |
| Typed subscription variants              |            8,526,770 |
| Subscription lifecycle, all 32 cycles    |           10,943,013 |
| Malformed subscription bounds            |            4,742,191 |
| Full diagnostic suite, 81 cases          |        1,019,567,233 |
| Lifecycle invoked twice, 64 total cycles |           21,276,866 |
| Address and simulation baseline          |          891,083,713 |
| Primitive and crypto baseline, 12 cases  |           19,483,114 |
| Full transaction results baseline        |            5,137,056 |
| Off-chain baseline, 3 cases              |            7,767,869 |

Every row has five passing trials at its minimum and five traps one fuel below.
The finite address-page matrix dominates the baseline. Its three configurations
each test six fixed page sizes up to 1,001 entries. The added lifecycle remains
explicitly bounded at 32 cycles. Invoking it twice completes with roughly twice
the allowance. The immediate RPC transport polls once and rejects `Pending`.
Subscription draining removes a queued notification before decoding it once.
These observations support bounded suite growth crossing a nearly full cap.
They do not establish a wallclock speedup or exclude unrelated future regressions.

The diagnostic binary runs each trial in a fresh actual host:

```sh
cargo run --locked --offline --release \
  --manifest-path verification/leptoskit/Cargo.toml --bin fuel_probe \
  -- target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm 1200000000
```

An optional integer argument selects an event in a diagnostic artifact. The
ordinary guest uses event 1 for corruption. `cargo xtask leptoskit` builds the
ordinary guest and uses the release host binary. Host optimization changes no
pinned code, imports, interpreter configuration, assertions, or fuel policy.
Raw diagnostics, the fixture preparation program, and complete trial outputs
remain in the ignored Unit135 evidence store. No public SDK source or fixture
body was changed for this correction.
