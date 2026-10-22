# Economics RPC runtime receipt

The public client implements all eight reviewed economics operations alongside
the three foundation and nineteen node operations. This accounts for 30 of the
54 catalog methods. The [economics ledger](../parity/rpc-economics.tsv) retains
all 42 exact declaration IDs, with 39 implemented, two partial and one pending.
The [source contract](../parity/rpc-economics.md) preserves all eight defining
source hashes and the remaining diagnostic and hashing differences.

## Public client behavior

Three portable groups append to the unchanged first 62 case lines. The
`solana-kit-core-v13` suite contains 65 cases. Every economics operation executes
through the public `Client` and the existing memory transport fixture, which
checks complete literal request bytes before returning an independently specified
response. No second protocol parser or production dependency was added.

- Accounts and votes execute `getLargestAccounts`, `getSupply`,
  `getRecentPrioritizationFees` and `getVoteAccounts`. Typed results retain
  validated addresses, both vote partitions, exact `u64::MAX` quantities and
  credit tuples, commission 255 and basis points 65535.
- Inflation and rewards execute `getInflationGovernor`, `getInflationRate`,
  `getInflationReward` and `getStakeMinimumDelegation`. They preserve finite
  nonnegative quantities, integer-valued foundation years, full-width epochs,
  nullable reward positions, nullable commission and exact reward cardinality.
- Malformed and configuration probes reject bad addresses, missing required
  members, negative and overflowing integers, nonfinite and wrong-type inflation,
  bad commission widths and credit tuples. Omitted and empty configs differ;
  explicit false remains present. Fee filters accept 128 addresses and reject
  129 before transport access. Optional basis-point null differs from zero.

The existing central JSON owner still enforces budgets, correlation, remote
errors and duplicate-key rejection. Accepted economics production source,
original tests and scoped ledgers remain byte-identical to reviewed source
`70117cf8cfa0f060f9b590ac0121773de2283558`. No commission-100 cap, supply-sum
invariant or upper rate constraint was introduced.

## Actual execution

Clean signed source `42d89dedcd5bed621fecf9dba440d499b1c1e95a` executed
`cargo +1.99.1 xtask all` on 2026-10-22. The five hosts checked every case.

| Actual host                      | Canonical                   | Corruption control             |
| -------------------------------- | --------------------------- | ------------------------------ |
| Native command                   | 65/65, exit 0               | 64/65, exit 1                  |
| Chrome 155.0.8059.85             | Full 65/65 report           | Full 64/65 report              |
| Wasmtime 50 Preview 1            | 65/65, exit 0               | 64/65, exit 1                  |
| Wasmtime 50 Preview 2            | 65/65, exit 0               | 64/65, exit 1                  |
| Pinned actual LeptosKit WasmHost | Full 65/65 JSON, one commit | Full 64/65 JSON, second commit |

ChromeDriver is 155.0.8059.84. LeptosKit remains pinned at
`fecf8628d326ff7b03f295523e6bb584c3fdf88d` with wasmi 2. Only
`sha256-fips-abc` fails in corruption mode. The unchanged 32 core normal/build
queries, 16 numeric normal-only queries, four System queries, four client queries
and all dependency and import controls pass. Compiled guest imports remain exactly
`leptoskit::{node_create,node_set_props,commit}`. All five extracted packaged-core
consumer tests pass without repository flags or patches.

Executed artifact SHA-256 values follow.

```text
ef4d4ba61de4a6f5b03e02cb19ef11878373d4b151102463d11d97ba556c7d9f  wasm32-wasip1
8757467a3812092fc3633998ac9b086437258196f0c7f8d45b5c73797cdd20d1  wasm32-wasip2
933eeecc07881d305e13e47ef31c7e450f1fbb6682e1025d24b2048f4654c173  leptoskit-guest
```

The unchanged independent economics consumer executes all eight methods in five
groups on Rust 1.89 from an external workspace. Only ordinary dependency paths
change; `RUSTFLAGS`, `RUSTDOCFLAGS` and `CARGO_ENCODED_RUSTFLAGS` are absent.
Its source assertions remain unchanged. An exact-set auditor rechecks every
economics ID, status, source pin and source digest against the canonical catalog.

Workspace default and all-feature tests, bare-core tests, MSRV 1.89, three Wasm
compile targets, warning-denied Clippy and docs, rustfmt, dprint, actionlint,
Monochange validation, API validation and read-only preview pass. Improved
Git-inventory monostyle 0.3.26 enforces both 90 floors for every file, including
generated source. Package ownership, dependency policies, manifests, lock pins
and checksums remain unchanged. All publication paths remain disabled.

These are off-chain application client protocol fixtures. They do not establish
networking on browser, WASI or interpreted hosts, remote-cluster behavior,
validator support, installed desktop or mobile journeys, full catalog parity,
registry readiness or phase completion. On-chain programs use Pina or official
Solana program libraries.
