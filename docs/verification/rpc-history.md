# Complete RPC catalog runtime receipt

All 54 source-audited RPC catalog methods have typed public implementations.
The final fifteen history, submission, full-result, address-history and simulation
operations use the existing `Client<T>` framing and transport owner. This SDK is
exclusively for off-chain application code. On-chain programs use Pina or official
Solana program libraries.

## Accepted source and actual hosts

Clean signed executable source `f4e0364bbd900912a677277e9ee5853d3994c6d9`
builds on corrected account/token head `8b0980d0c39891f6940dbec7a19f08048d46cdc6`,
which includes the accepted LeptosKit host-test correction.
Its history production and existing tests are byte-identical to reviewed final
`75edc3506938207ded68e0ae5ef4ab710b7ffa54`. Current core, account/token,
node/economics, central JSON/client, adapters, System, source pins, lockfiles,
features, dependency policies and package owners remain unchanged.

Signed ancestry-only merge `d79dc8c63faa50cc4f8413a7b223b60105092da0` also
includes actual main `66122fd00f1a4408272d0e83629a84def0d214d3`. Main's complete
tree equals the accepted account/token parent. The merge preserves every byte
of executed source `f4e0364`, with the same complete tree
`74c3ebf8321778d2e71a6c7192685b80c8fbe0b1`. The evidence remains bound to the
executed source and this exact tree identity.

Four cohesive groups append after the original 68 cases, preserving their names,
order and outcomes. `solana-kit-core-v15` has 72 cases. On 2026-10-22, the complete
`cargo +1.99.1 xtask all` command exited 0 with the following actual results.

| Actual host                                   | Canonical                      | Corruption control                |
| --------------------------------------------- | ------------------------------ | --------------------------------- |
| Native Rust command                           | 72/72, exit 0                  | 71/72, exit 1                     |
| Chrome 155.0.8059.85 and driver 155.0.8059.84 | Full 72/72 checked             | Full 71/72 checked                |
| Wasmtime 50.0.0 Preview 1                     | 72/72, exit 0                  | 71/72, exit 1                     |
| Wasmtime 50.0.0 Preview 2                     | 72/72, exit 0                  | 71/72, exit 1                     |
| Actual pinned LeptosKit WasmHost              | Full 72/72 JSON and one commit | Full 71/72 JSON and second commit |

Only `sha256-fips-abc` fails under corruption. LeptosKit remains pinned to
`fecf8628d326ff7b03f295523e6bb584c3fdf88d`. The compiled guest retains exactly
`leptoskit::{node_create,node_set_props,commit}` imports. All existing dependency
queries and import/dependency negative controls pass. Default core remains
bs58-only and client normal/build graphs retain thirteen external package/version
pairs on each of the four targets. All five external packaged-core consumer
groups pass without repository flags or patches.

Executed artifact SHA-256 values follow.

```text
53c0dd2d0eae13687cf40d7d608ff7fbb1215e1b948281003be1221a71caedaf  wasm32-wasip1
a06dd168b759654ef587e430aba58803ba9f6d1eb108c193cd9c58704dfbb5c9  wasm32-wasip2
2f5659798d297b5ef244c3213aa9e687cc7a810874c50d8061cf9442559d3627  leptoskit-guest
```

Use the actual executable paths described in the [runtime guide](runtimes.md),
then run the following command. The driver startup timeout is explicit.

```sh
CARGO_NET_OFFLINE=true WASM_BINDGEN_TEST_DRIVER_TIMEOUT=30 cargo +1.99.1 xtask all
```

## Executed contracts

The four appended groups call all fifteen new method names through the existing
literal-byte public Client fixture. They check exact u64 quantities, signed i64
block time, required nullable history/status members, positional status nulls,
nonzero 64-byte signatures, range ordering, canonical opaque submission encodings,
4096-byte submission without a legacy cap, typed instruction failures and bounded
exact future-error JSON containing 1e400 and -1e-400. Remote preflight failures
retain exact data and are never retried.

Full transaction/block results keep their historical missing-metadata version
omission. Current address full pages reject omitted requested versions. Parsed
program data retains exact opaque numbers and Unicode spelling; token annotations
reject synthetic serde Number objects while accepting genuine extreme scalars.
Simulation preserves explicit false, exact fee/compute quantities, requested
account cardinality and incompatible signature/blockhash option failures.

Actual populated signature pages execute 0, 1, 100, 101, 1000 and 1001 boundaries
under omitted config, empty config and explicit limit 1000. Accepted pages retain
the Signatures variant, positions and cursor. Explicit Full accepts 100 entries
and rejects 101. Its cap remains separate from the default 1000 signature cap.

The unchanged independent Unit97 ordinary-source consumer additionally passes all
sixteen groups under Rust 1.89 from outside the repository. Only manifest source
paths change; its original source, tests and lock remain exact. It retains all
18 historical cases, four encodings, requested-version matrices, detail limits,
filters, scalar/opaque distinctions, rich shared accounts and malformed shapes.
Existing public tests and the unchanged independent history consumer additionally
cover configuration omission, 256 signatures, all 39 known failure variants and
4096-byte opaque submissions in both encodings.

## Accounting and limits

The canonical catalog retains all 398 declaration IDs and their immutable Dart
source hashes. The [history ledger](../parity/rpc-history-submission.tsv) has
exactly 146 IDs, 130 implemented and 16 pending. Fourteen pending IDs cover Dart
hashCode/toString conveniences and two cover response equality. The
[account/token ledger](../parity/rpc-account-token.tsv) remains exactly 100 IDs,
88 typed fixtures, four Debug partial, four equality pending and four Hash pending.
All retained source licenses and provenance remain intact.

These results execute local memory-transport protocol behavior. Accepted network
adapters have their own receipts. This receipt establishes no new HTTP/socket,
validator, device, installed application, registry, complete-declaration or full
Phase 4 claim. SendTransaction remains opaque, with no client retry, signing,
expiry inference or confirmation behavior. Component publication stays disabled.
