# Portable Token runtime

The shared `solana-kit-core-v25` suite executes all 105 ordered groups on native
Rust, Chrome, Wasmtime50.0.0 Preview1 and Preview2, and the pinned LeptosKit
`fecf8628d326ff7b03f295523e6bb584c3fdf88d` headless WasmHost. Each canonical
report passes 105/105. Each corruption report passes 104/105 and fails only
`sha256-fips-abc`.

Executable source is signed `c00a2edc646d0d66ba8db640c576b74e34ae35ea`.
The two new groups follow all 103 existing groups without changing their
assertion bodies or order.

- `token-literal-oracles` executes 93 literal byte, program identity, account
  address and ordered role comparisons. This includes all 28 instruction
  constructors and all three account layouts at three values each.
- `token-strict-errors` executes the accepted six strict groups, including
  every fixed account truncation, malformed options, booleans and enums,
  instruction tags, byte and item budgets, duplicate roles, custom program
  identity and all 23 source error rows.

The original 93-row fixture remains byte-exact. The portable assertions use the
same public Token and core APIs as the ordinary consumers. Failures return an
error in the complete report. The SDK dependency closure remains
Token to core to bs58.

The actual interpreter rejects a one-fuel allowance before committing a report.
Its finite source-owned allowance is 6,675,000,000 for each entry. It charges 102
ordinary groups at 15,000,000 each, 257 authenticated transactions and 772 message
crypto calls at 5,000,000 each. The two pure Token groups add 30,000,000 ordinary
fuel. The 256 distinct message workload and 256 cached transaction cases remain
intact. This is a conservative allowance, not a measured minimum.

Run the maintained complete journey with its pinned local host tools:

```sh
CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=2 cargo xtask all
```

`CHROME`, `CHROMEDRIVER` and `WASMTIME` must identify the required installed
tools. Missing tools and incomplete or mismatched reports fail the command.
The [canonical report](../../verification/conformance/evidence/native-success.txt)
and [negative report](../../verification/conformance/evidence/native-corruption.txt)
are the exact expected streams.

The [Token declaration receipt](token.md) retains the authority-role,
remaining-account, leading-BOM and Batch-account differences. These checks
do not establish validator or device execution, complete handwritten helpers,
all renderer contracts or full Token parity. This SDK is exclusively for
off-chain application clients.
