# Async and standalone System boundary experiments

These private workspaces test proposed adapter contracts. They publish no client
crate and are absent from SDK application normal/build graphs.
They prove portability for off-chain application clients; they make no on-chain
program support claim.

Published receipts replace only the private checkout prefix with `<workspace>`;
relative paths, commands, results, versions, source pins and artifact hashes are preserved.

## Async evidence

Run from the repository root:

```sh
cargo test --manifest-path verification/boundaries/Cargo.toml --workspace --locked
cargo clippy --manifest-path verification/boundaries/Cargo.toml --workspace --all-targets --locked -- -D warnings
cargo fmt --manifest-path verification/boundaries/Cargo.toml --all --check
monostyle check . --fail-under 90 --no-color
```

One `Client<T>::call` awaits `T::Exchange<'_>`, validates correlation, decodes
exactly one little-endian u64, and returns separate transport, correlation and
body errors. This is a protocol experiment, not an RPC implementation. The GAT
has no universal Send bound. Native generic callers require `T: Send + Sync`,
`T::Error: Send`, and `for<'a> T::Exchange<'a>: Send`; the same method then runs
inside a multi-thread Tokio spawn. A LocalPool executes a transport retaining
`Rc` through Pending. A compile-negative doctest creates that local transport
inside a spawned block and proves the awaited local work cannot enter Tokio.

Six pure capability tests cover entropy Pending/wake/completion, unavailable and
short entropy replies without output mutation, manual-clock deadlines, denied
clock capability, and independent cancellation waking a quiet future. Cancellation
drops the reply receiver; late completion is rejected and cannot mutate state.
Two native tests observe actual TCP EOF after owner Drop aborts its task and
prove a bare JoinHandle instead detaches. OwnedTask promises local abort, not a
synchronous join or remote cancellation acknowledgement.

These capability fixtures do not execute guest entropy, timers or unload hooks.
They do not prove installed-host HTTP, WebSocket, async ABI, mobile, or device
behavior. The runtime/transport track owns that integration.

## Official System constructor

`solana-system-interface =3.3.0`, default features disabled, selects only
`bincode`; `solana-address =2.7.0` also disables defaults. The same `verify(bool)`
function calls the official CreateAccount constructor on every target and checks
all 52 data bytes, program ID, account order, signer flags and writable flags.
The [independent oracle provenance](../../verification/boundaries/system/PROVENANCE.md)
describes its pinned Dart schema and retained license. Corruption changes the
independent expected discriminator and must produce the exact serialization error.

Local execution on 2026-10-22 used Rust 1.99.1 on macOS ARM64:

| Environment                      | Canonical                   | Corruption control                            |
| -------------------------------- | --------------------------- | --------------------------------------------- |
| Native                           | exact oracle, exit 0        | exact error, exit 1                           |
| Chrome 155.0.8059.85             | browser test passed         | rejection test passed                         |
| Wasmtime 50.0.0, WASI p1         | exact oracle, exit 0        | exact error, exit 1                           |
| Wasmtime 50.0.0, WASI p2         | exact oracle, exit 0        | exact error, exit 1                           |
| Actual pinned LeptosKit WasmHost | report and commit 1 checked | Press event, exact error and commit 2 checked |

The host uses `leptoskit_wasm` commit
`fecf8628d326ff7b03f295523e6bb584c3fdf88d` through its own locked workspace;
this executes upstream wasmi, without JavaScript. The compiled guest imports
only `leptoskit::node_create`, `leptoskit::node_set_props`, and `leptoskit::commit`.
The runtime track's closed binary import checker was executed on this artifact;
its receipt is in `verification/boundaries/evidence/system-guest-imports.txt`.
The System guest uses the same pinned rendering ABI as that runtime harness.

For browser execution install `wasm-bindgen-cli =0.2.128`, provide actual Chrome
and matching ChromeDriver executables, and run:

```sh
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner WASM_BINDGEN_USE_BROWSER=1 WASM_BINDGEN_TEST_ONLY_WEB=1 cargo test --manifest-path verification/boundaries/Cargo.toml -p solana_kit_system_browser_boundary --target wasm32-unknown-unknown --locked
```

For each WASI target, build and run both modes using `WASMTIME` pointing to the
pinned 50.0.0 executable. Success must exit 0; corruption must exit 1:

```sh
cargo build --manifest-path verification/boundaries/Cargo.toml -p solana_kit_system_boundary --target wasm32-wasip1 --locked
"$WASMTIME" run verification/boundaries/target/wasm32-wasip1/debug/solana_kit_system_boundary.wasm
"$WASMTIME" run verification/boundaries/target/wasm32-wasip1/debug/solana_kit_system_boundary.wasm --corrupt-fixture
```

Repeat with `wasm32-wasip2`. Execute the actual LeptosKit guest:

```sh
cargo build --manifest-path verification/boundaries/Cargo.toml -p solana_kit_system_guest_boundary --target wasm32-unknown-unknown --release --locked
guest="$PWD/verification/boundaries/target/wasm32-unknown-unknown/release"
cargo run --manifest-path verification/boundaries/leptoskit/Cargo.toml --locked -- \
  "$guest/solana_kit_system_guest_boundary.wasm"
cargo fmt --manifest-path verification/boundaries/leptoskit/Cargo.toml --check
cargo clippy --manifest-path verification/boundaries/leptoskit/Cargo.toml --locked -- -D warnings
```

The dedicated boundaries workflow repeats native checks on Linux, macOS and
Windows, and System execution in checksum-pinned Linux browser/WASI hosts and
the actual pinned LeptosKit host. Local workflow validation passed actionlint;
remote CI execution is still required before merge.

## Dependency closure and reuse decision

The checked-in `evidence/system-*-tree.txt` files record the complete selected
normal/build graph and features for native, plain Wasm, WASI p1 and WASI p2.
Regenerate each with:

```sh
cargo tree --manifest-path verification/boundaries/Cargo.toml -p solana_kit_system_boundary --locked --edges normal,build --format '{p} features=[{f}]' --target wasm32-unknown-unknown
```

The external closure contains 22 package/version pairs: autocfg 1.5.1, bincode
1.3.3, five8/five8_const/five8_core 1.0.0, num-traits 0.2.19, proc-macro2
1.0.107, quote 1.0.47, serde/serde_core/serde_derive 1.0.229, solana-address
2.7.0, solana-instruction 3.5.1 and 4.0.0, solana-instruction-error 2.5.0,
solana-msg 3.1.0, solana-program-error 3.0.1, solana-pubkey 4.3.0,
solana-sanitize 3.0.1, solana-system-interface 3.3.0, syn 3.0.9 and
unicode-ident 1.0.27. Proc macros and autocfg execute as build tools. Both
instruction generations are selected by upstream's compatibility dependency.

The minimal constructor feature enables alloc, serde and solana-instruction,
including instruction std and serde std through bincode. Consequently this is
not a strictly no_std dependency closure even though our oracle library uses
`#![no_std]`. It adds no Tokio, browser APIs, ambient RNG or validator/client
stack. The separate browser runner, guest JSON reporting and LeptosKit host have
additional private harness dependencies; the host graph is retained separately.

Recommend reusing this pinned standalone constructor behind private conversions
in a future std-capable System adapter, with only Kit-owned public values.
Its exact constructor bytes and account metadata are executed on every target.
Keep it outside deterministic core; if the System program crate must itself
promise a strictly no_std graph, use a small owned wire implementation or the
pinned IDL generator and retain this constructor as an oracle. No public API or
registry component has been added by this experiment. No transaction was sent
to a validator, and these results claim no other System instruction parity.
