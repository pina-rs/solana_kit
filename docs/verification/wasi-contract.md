# WASI Preview 2 host contract experiment

This private Rust workspace executes off-chain client boundary probes in a real
`wasm32-wasip2` component. It is evidence for one named host, not a published SDK
adapter or an on-chain program support promise.

## Pins and capabilities

| Item                         | Pin                                                                                       |
| ---------------------------- | ----------------------------------------------------------------------------------------- |
| Rust                         | Repository toolchain, 1.99.1                                                              |
| Wasmtime                     | `50.0.0 (ebe5e8df1 2026-10-20)`                                                           |
| Executed platform            | macOS ARM64                                                                               |
| Executed host binary SHA-256 | `d5bd3748633fcfb39a11c90c7225f981103d82c1141a6f10382e27fbf404571f`                        |
| Guest bindings               | `wasip2 = 1.0.4+wasi-0.2.12`, `wit-bindgen = 0.57.1`                                      |
| WIT imports                  | WASI `0.2.12`, listed in [imports](../../verification/wasi-contract/evidence/imports.txt) |
| Private host tooling         | `anyhow = 1.0.104`, `httparse = 1.10.1`, `wasmparser = 0.240.0`                           |

The private workspace has its own lockfile and `publish = false` packages. None
of these dependencies enters the application's normal/build dependency graph.
The runner validates the component encoding and records only its outer imports.
There are no `wasi:sockets` imports.

Each positive invocation grants `-S http=y`, plus the CLI's default CLI, clock,
random, and stdio services. It grants no preopened directories, environment
inheritance, or socket network inheritance. HTTP imports are component-wide, so
even the entropy/clock mode requires the HTTP linker implementation. The CLI's
HTTP transport is separate from socket permissions: this experiment's destinations
are loopback, but these flags do not implement an HTTP destination allowlist.

## Repeat locally

From the repository root, set `WASMTIME` to the pinned executable and run:

```sh
cargo fmt --manifest-path verification/wasi-contract/Cargo.toml --all --check
cargo test --locked --manifest-path verification/wasi-contract/Cargo.toml -p solana_kit_wasi_contract_runner
cargo clippy --locked --manifest-path verification/wasi-contract/Cargo.toml -p solana_kit_wasi_contract_runner --all-targets -- -D warnings
cargo build --locked --manifest-path verification/wasi-contract/Cargo.toml -p solana_kit_wasi_contract_guest --target wasm32-wasip2
cargo clippy --locked --manifest-path verification/wasi-contract/Cargo.toml -p solana_kit_wasi_contract_guest --target wasm32-wasip2 -- -D warnings
cargo run --locked --manifest-path verification/wasi-contract/Cargo.toml -p solana_kit_wasi_contract_runner -- "$WASMTIME"
monostyle check . --include-generated --fail-under 90
```

The runner binds ephemeral IPv4 loopback ports. It requires fixture acceptance,
exact guest markers, expected exit status, and server observations. Each child
has a four-second limit; missing fixtures fail. The dedicated workflow pins the
Linux release archive by SHA-256; Linux execution is pending its first CI run.

## Executed observations and negative controls

The [stored transcript](../../verification/wasi-contract/evidence/execution.txt)
records every case. The following assertions all execute inside the component:

| Probe             | Observation and control                                                                                                                                                                                           |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| POST              | Exact eight-byte `{"id":7}`, method, path, content length, and two application headers verified by the server; guest checks status 201, response header, and `reply:7`. A `reply:8` control must trap.            |
| Application error | HTTP 503 stays an HTTP response and retains its exact bounded body.                                                                                                                                               |
| Body bound        | A 30-byte response exceeds the guest's 16-byte bound and returns the explicit local limit error. The success response fits.                                                                                       |
| Malformed HTTP    | Invalid response syntax requires `HttpProtocolError`; successful syntax returns 201.                                                                                                                              |
| Header deadline   | An 80ms first-byte budget with a 200ms stalled fixture requires `ConnectionReadTimeout`; cancellation probes use a separate two-second budget.                                                                    |
| Cancellation      | Guest waits 40ms, confirms the response remains pending, drops it, then stays alive another 300ms. Server requires early TCP EOF within 250ms. The retained-response control requires no EOF during 250ms.        |
| Entropy           | Host returns 32 bytes, copied into the caller's 32-byte buffer. With `max-random-size=16`, the same request must trap. No deterministic value or entropy-quality claim is made.                                   |
| Monotonic wake    | A 20ms timer is initially unready, becomes ready after blocking, and elapsed monotonic time is at least 20ms. Initial unreadiness is its early-wake control.                                                      |
| HTTP unavailable  | `-S http=n` must fail component instantiation with missing HTTP resources, before a request can run. This is not a per-destination denial test.                                                                   |
| TLS failure       | HTTPS emits a TLS handshake record. A plaintext reply requires `TlsProtocolError`; the HTTP success control uses the same fixture transport. Successful TLS and certificate validation are not proved.            |
| WebSocket attempt | Setting `Upgrade: websocket` and `Connection: Upgrade` requires `HeaderError::Forbidden`. An unsolicited 101 is observable only as an HTTP response; no upgraded byte stream or WebSocket messages are exercised. |

The cancellation result applies to a pending response before headers on a fresh
HTTP/1 connection. It does not prove cancellation of DNS, TLS, a partially read
body, a pooled connection, or server-side work already performed. The fixture's
EOF observation proves this request connection is closed while the guest lives;
it does not inventory all host tasks.

## WebSocket follow-up contract

WASI subscriptions remain unsupported by this experiment. Keep the forbidden-header
and unsolicited-101 cases as executable failures of the HTTP-only approach.
The next unit must choose explicit socket/TLS imports or a host WebSocket bridge
and execute a complete trusted TLS WebSocket connection before making a support
claim. A bridge must expose a caller-granted destination, certificate policy,
bounded send/receive messages, pollable readiness, explicit close, and cancellation
on resource drop. Host unload must terminate its network task and reject late
completions. Prove text/binary exchange, rejected certificate/upgrade, oversized
frame, quiet receive cancellation, and local teardown with a still-running guest.

Wasmtime's [pinned response resource implementation](https://github.com/bytecodealliance/wasmtime/blob/v50.0.0/crates/wasi-http/src/p2/types.rs)
uses an abort-on-drop task handle for pending responses. Its
[pinned HTTP hooks](https://github.com/bytecodealliance/wasmtime/blob/v50.0.0/crates/wasi-http/src/ctx.rs)
define the default header filter and HTTP/HTTPS handling. These source facts
support interpretation of the executed probes; they do not replace execution.
