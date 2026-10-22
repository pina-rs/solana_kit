# Interpreted conformance fuel

This private harness executes the complete off-chain client suite in the pinned
upstream `WasmHost`. Every canonical and corruption report must match the native
case names, order, errors, pass count, and suite identity.

The host grants fuel separately to each guest entry. Starting the guest and
pressing its report each execute the whole suite. Wasmi 2 meters Wasm operations,
dynamic copying, and first-use lazy function translation. It does not meter
elapsed time. The upstream application default and production source are unchanged.

The harness grants a pooled `15_000_000 × EXPECTED_CASE_COUNT` allowance with
checked multiplication. The 81-case suite receives 1,215,000,000 fuel per entry.
This scales the aggregate allowance with verified workload. It is not an
individual case limit. The harness also requires a fresh guest with one fuel to
trap with `OutOfFuel` before committing anything.

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
