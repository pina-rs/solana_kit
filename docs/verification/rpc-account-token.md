# Account and token RPC runtime verification

The public client implements nine additional account/token methods and richer
accountInfo, alongside the existing full-base64 compatibility operation. The
combined client implements 39 of the 54 catalog methods, with 15 still pending.
This is exclusively an off-chain application SDK. On-chain programs use Pina or
official Solana program libraries.

Clean signed source `f7f1107a8770bba12544aac10586ebefcdd33438` adds three groups to
the shared `solana-kit-core-v14` suite. Its first 65 canonical and corruption case
lines remain identical. The complete suite contains 68 cases, and only the
deliberate SHA-256 fixture fails in corruption mode.

The groups use the public `Client` and the existing literal memory transport.
Every host executes getMultipleAccounts, getProgramAccounts,
getMinimumBalanceForRentExemption, isBlockhashValid, getTokenAccountBalance,
getTokenAccountsByOwner, getTokenAccountsByDelegate, getTokenLargestAccounts,
getTokenSupply and richer getAccountInfo.

Exact request bodies cover omitted and empty configurations, explicit commitments,
minimum slot, contextual program results, mint and program selectors, filters,
and slices. Typed results retain full-width slots, lamports, rent epochs and raw
token quantities, decimals255, nullable display values, nonzero keyed identities
and positional null identities. The account representations include legacy
base58, tagged base58/base64, parsed exact1e400 JSON, parsed-to-base64 fallback,
slices with their complete allocation, and explicitly compressed bytes.

Malformed controls reject noncanonical base64, mismatched or unknown encoding,
wrong allocation, missing/null metadata, invalid owners, duplicate keys, wrong
cardinality/context shape, invalid raw token strings, amount overflow, decimals256,
nonfinite or missing UI values, wrong token program, overflowing or parsed slices,
129-byte memcmp, 101 accounts and five filters. Positive boundaries exercise
128-byte memcmp, 100 accounts and four filters.

| Actual host                      | Canonical         | Corruption control |
| -------------------------------- | ----------------- | ------------------ |
| Native command                   | 68/68, exit0      | 67/68, exit1       |
| Actual Chrome155                 | Full68/68 checked | Full67/68 checked  |
| Wasmtime50 Preview1              | 68/68, exit0      | 67/68, exit1       |
| Wasmtime50 Preview2              | 68/68, exit0      | 67/68, exit1       |
| Pinned actual LeptosKit WasmHost | Full68/68 JSON    | Full67/68 JSON     |

Run `cargo +1.99.1 xtask all` with Chrome, its matching driver and Wasmtime50
configured as described in the [runtime instructions](runtimes.md#reproduce).
Raw local outputs remain under ignored `target/runtime-evidence`. All existing
dependency/import controls and five packaged-core consumer tests remain required.
Default core stays bs58-only; the client keeps13 external normal/build package
pairs. No adapter, central JSON owner or shared scalar validation changed.

Executed local artifacts have these SHA256 values:

```text
38f7aff141ee078cc1d342dd4b120a30bc5a792282c9b277df79334f1a927f2e wasm32-wasip1
fee7f2ded20b84d2cd0e9f7d5a92ed3ae1752ed4ac327fdf130af6590b81d332 wasm32-wasip2
2906eed01215d6861b97222e66aeeb77719192c0fd0180e724a821a013553412 leptoskit-guest
```

The unchanged independent account consumer executes all four groups with ordinary
source dependencies under Rust1.89 outside repository flags or patches. Exact
100-ID/source-hash accounting remains88 typed, four Debug partial, four equality
pending and four Hash pending. The [accepted contract receipt](../parity/rpc-account-token.md)
retains source pins, licenses and representation boundaries.

These are actual local protocol executions. Compressed-stream validity and
decompression, application account decoding, network/device/validator journeys,
all catalog declarations and registry readiness remain separate work.
