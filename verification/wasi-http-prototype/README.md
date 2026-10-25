# WASI HTTP verification

This isolated source consumer uses the real public Client and WasiHttp transport
for off-chain application code. It is not for Solana on-chain programs. Use Pina
or official Solana program libraries for those programs.

The designs binary executes two ownership prototypes. A direct owner drives one
future's pollable; a local registry composes arbitrary Client futures through
cancellation guards. Both observe Pending, actual host readiness, one wake and
resource cleanup. The registry retains Rc across Pending and unregisters a
pending timer on Drop. The public adapter uses the registry design.

The runner requires Wasmtime50.0.0 and a real wasm32-wasip2 component. It checks
four exact normal/build closures, four injected Tokio negatives, actual outer
imports and thirteen independent peer/component journeys. The peer validates
exact POST bytes and caller Authorization. Requests stay on loopback.

Executed source c067aaade3221982713dbd673ae1ddb9924b6bf1 passes exact u64MAX,
eleven-byte chunking, HTTP503, HTTP302 without following, body overflow, malformed
HTTP, whole header/body deadlines, response/body Drop versus retained negative,
private-CA TLS rejection at the valid localhost name and corrupt-reply detection.
A polled guest call must return Pending within50ms and later observe a real
counting-waker callback. Registry ownership reaches zero on success/error/drop.
Peer EOF arrives before250ms while the cancelled guest stays alive another300ms.

The deliberately public TLS fixture key/certificate is reused directly from the
prior wasi-sockets fixture. Its validity, origin and deployment prohibition are
documented there. No TLS verifier is overridden by guest code.

Run from the repository root after provisioning the pinned Wasmtime executable:

```sh
cargo +1.99.1 build --locked \
  --manifest-path verification/wasi-http-prototype/Cargo.toml \
  --bin solana_kit_wasi_http_prototype --target wasm32-wasip2
cargo +1.99.1 run --locked \
  --manifest-path verification/wasi-http-prototype/Cargo.toml \
  --bin runner -- /path/to/wasmtime
```

The `embedded` binary pins the official Wasmtime, wasmtime-wasi and
wasmtime-wasi-http crates to50.0.0. It instantiates `embedded_guest`, which uses
the public Client and WasiHttp. Its host owns destination grants, fixture roots,
ordinary Rustls name verification and request lifetime accounting. A denied
destination and an independent redirect target observe no connection. Trusted
localhost succeeds. An unrelated CA and a trusted CA with the wrong hostname
fail separately. Stalled TLS and upload, controlled pre-resolution suspension,
early413, malformed chunking, truncation, repeated success/denial/cancellation,
Store drop and replacement have executable assertions. This does not test an
operating-system DNS stall or a late callback from an uncancellable resolver.

Wasmtime50's official outgoing handler consumes request-options in WIT but leaves
that owned resource in its table. The private `embedded_outgoing` binding
delegates the actual official handler and deletes the consumed options handle.
It does not replace the network or TLS implementation. After each completed
journey the complete typed table must match the exact handles created by the
official CLI stdin, stdout and stderr bindings and the executed no-network warm
baseline. Resource type alone never grants CLI ownership. An extra output stream
fails the checker even though legitimate CLI output uses the same type. Dropping
the Store releases those streams. Owned network flights and Tokio task counts
return to baseline.

Rust1.89's bundled Preview1 compatibility adapter initializes stdin, stdout and
stderr together on the first stdout write. Rust1.99.1 retains only stdout and
stderr in these components. The former two-output-stream assertion rejected
Rust1.89's legitimate stdin handle. `embedded_resources` records provenance by
delegating the official CLI bindings, then checks every retained handle's type
and exact warm identity. RequestOptions must still be zero. Unit negative controls
inject RequestOptions, a non-CLI resource and an extra stream of a CLI type.

Build `embedded_guest` for wasm32-wasip2, then run the native `embedded` binary
with the same manifest. No host executable argument is required. The default
executes every case. An optional case name selects one journey after the baseline.
The original CLI runner retains all thirteen journeys and the exact outer
sixteen-interface policy now rejects a validated unknown-interface component.
The separate portable core retains its original three-import grant.

The independent `cli_guest` component contains only stdout and stderr writes.
Build it with each toolchain and run `embedded --cli-only` to isolate compiler
stdio initialization from the SDK and HTTP. Focused CI runs both components with
Rust1.99.1 and Rust1.89 through the same native host.

`evidence/embedded-execution.txt` records the actual assertions. To reproduce the
official50 owned-options leak, run `embedded denied --unpatched-host`. That
deliberate negative must fail with one retained request-options entry. The
default reference host enables the narrow consumed-resource correction.

The guest queries `wasi:http/types.http-error-code` before retaining a stream
error as generic I/O. Wasmtime50 stores malformed-chunk and truncated-body
failures as its internal Hyper error and returns no structured code. Both
journeys therefore assert a failed I/O result with a diagnostic source, never a
successful reply. A stricter InvalidResponse expectation failed in an actual
design probe. No diagnostic-string parsing or host categorization override is
used. A host that exposes the structured HTTP framing code receives the
adapter's InvalidResponse classification.

Native MSRV1.89 endpoint tests pass. Local Rust1.89 Preview2 execution now passes
all twenty embedded cases and the independent CLI-only component. Actual Linux
Preview2 compilation and embedded MSRV execution remain focused CI gates before
merge. Ordinary source
consumption works through this isolated workspace. Registry archive preparation
stops at deliberately unpublished solana_kit_client0.1.0. No release was attempted.

The production reactor's focused callback tests build with
`cargo +1.99.1 test -p solana_kit_wasi --test reactor --target wasm32-wasip2 --no-run --locked`.
Run the emitted `reactor-*.wasm` component through the pinned Wasmtime50 CLI with
`--test-threads=1`. Six executed tests cover legal clone reentrancy, replaced and
cancelled Waker destruction, ready and expired-timer removal, capacity rejection
and cleanup, and wake delivery. The unchanged independent public safe Wake/Drop
consumer also passes after failing against the preserved original component.
See `evidence/waker-execution.txt` for the actual bounded receipt.

The CLI runner takes the concrete Wasmtime executable and actual Cargo-selected
component path as its two arguments. The embedded host requires
`SOLANA_KIT_WASI_HTTP_GUEST` to name that actual component. Neither assumes a
repository-local target directory.
