# Node RPC protocol verification

The public `solana_kit_client::methods::node` module implements all nineteen node
operations from the pinned Dart catalog. It is exclusively for off-chain
application code. Solana on-chain programs should use Pina or official Solana
program libraries instead.

Production node modules, their ten original public integration tests, and the
exact 110-ID [node ledger](../parity/rpc-node.tsv) equal independently reviewed
source `5a877b262dfdb51fdfad3d3f971354dbf893a196`. The ledger retains 98 implemented
contracts and twelve pending Dart display/hash contracts. The immutable global
54-method/398-ID catalog remains unchanged. Combined with the earlier three
foundation methods, the public crate now provides 22 catalog operations.

## Actual portable execution

Clean signed checkpoint `96494782927020a2708732abd31c56570139663e` executed
`cargo xtask all`. The shared v10 suite appends three groups after all 53 existing
cases. Native Rust, real Chrome, Wasmtime Preview 1 and 2, and the pinned actual
LeptosKit WasmHost each observe 56 successful canonical cases and 55 successes
with only the deliberately wrong SHA-256 fixture failing. See the
[runtime receipt](runtimes.md) for versions, artifact hashes and reproduction.

| Group                           | Literal public requests and asserted responses                                                                                                                                       |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `rpc-node-scalar-metadata`      | getBlockHeight, getSlot, getTransactionCount, getMaxRetransmitSlot, getMaxShredInsertSlot, getGenesisHash, getSlotLeader, getHealth, getIdentity, getVersion, getHighestSnapshotSlot |
| `rpc-node-structured-metadata`  | getEpochInfo, getEpochSchedule, getClusterNodes, getBlockProduction, getLeaderSchedule, getRecentPerformanceSamples, getSlotLeaders                                                  |
| `rpc-node-certificate-failures` | getAgGenesisCert, exact byte widths, null certificate, missing nullable snapshot member, duplicate response key and wrong correlation ID                                             |

An immediate Ready exchange compares complete expected request bytes. It polls
the real public Client directly without an executor, private decoder, production
framing copy or private Tokio pilot dependency. Typed assertions cover full-width
u64 values, owned addresses and hashes, u32/u16 metadata, missing versus nullable
fields, exact production pairs and identity-indexed leader schedules. Certificates
validate 32-byte block IDs, 192-byte aggregate signatures and byte-valued bitmaps.
This is schema validation, not BLS or consensus verification.

The first 53 canonical and corruption golden lines remain byte-identical to the
parent. Normal/build graphs, their exact-version policies and negative controls
are unchanged. No manifest or lockfile changes were needed. The plain guest
still imports only the three documented LeptosKit functions.

## Fresh local checks

The original public node tests and complete workspace tests pass. The independent
Unit57 node consumer's five unmodified behavior groups also pass against this
checkout on Rust 1.89.0 from an ordinary external workspace with repository
RUSTFLAGS, RUSTDOCFLAGS and CARGO_ENCODED_RUSTFLAGS removed. Its literal fixtures
execute every node method. A fresh auditor confirms all 110 unique node IDs,
nineteen method names, pinned Dart hashes and all nineteen official Kit source
hashes plus the two actual Agave source hashes.

Workspace Clippy with warnings denied, rustdoc with warnings denied, rustfmt,
dprint, actionlint, core default tests, MSRV core/client/System tests and client
compilation for all three Wasm targets pass. Monochange check, configuration
validation and release preview pass with publication disabled. No package was
published or reserved. Actual monostyle 0.3.26 scores every required authored and
generated file with both per-file and repository floors at least 90.

The portable runs use memory transports. They do not establish actual browser,
WASI or interpreted-guest networking, installed applications, Android/iOS journeys,
remote node behavior, subscriptions or complete ecosystem parity. Native HTTP/TLS
has its own [networking evidence](native-http.md). Genesis certificates remain
schema-only values. All twelve pending node declarations remain explicit.
