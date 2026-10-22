# WASI validating TLS and WebSocket socket experiment

This private workspace proves one off-chain client networking path: Rust
`wasm32-wasip2` components use `wasi:sockets@0.2.12` through Wasmtime 50.0.0,
with TLS and WebSocket protocol logic inside the guest. It adds no public SDK
adapter and never targets Solana on-chain programs. Use Pina or official Solana
program libraries for on-chain programs.

## Chosen contract

The direct socket prototype successfully compiled and executed rustls and
Tungstenite in the actual component, so it avoids introducing a custom WIT bridge
and native embedding dependency. The existing HTTP-only experiment still rejects
Upgrade. An alternative host WebSocket bridge would move TLS/protocol ownership
into the host but require a new interface, resource lifecycle, and embedding;
that alternative was evaluated structurally, not implemented or executed.

| Item                | Pin                                                                |
| ------------------- | ------------------------------------------------------------------ |
| Rust                | 1.99.1                                                             |
| Executed host       | Wasmtime 50.0.0 (`ebe5e8df1`, macOS ARM64)                         |
| Host binary SHA-256 | `d5bd3748633fcfb39a11c90c7225f981103d82c1141a6f10382e27fbf404571f` |
| Imported interfaces | WASI 0.2.12, listed in the execution receipt                       |
| TLS                 | rustls 0.23.46, ring 0.17.14, default features disabled            |
| WebSocket           | tungstenite 0.29.0, handshake feature only                         |
| Private tooling     | anyhow 1.0.104, wasmparser 0.240.0                                 |

The isolated lockfile pins the full graph; none enters the SDK workspace or any
public dependency graph. The runner validates real component encoding, requires
TCP imports, and rejects non-WASI or HTTP outer imports. Rust std also imports
UDP, filesystem, and terminal interfaces. UDP is explicitly denied; no directories
are preopened, no environment is inherited, and no name lookup is granted.

Positive processes grant `tcp=y`, `inherit-network=y`, `udp=n`, and
`allow-ip-name-lookup=n`. **The CLI's inherited-network grant permits every TCP
destination.** The guest connects only to a supplied IPv4 loopback port, but this
is not a host-enforced destination allowlist. An application adapter must use an
embedding with an address-check policy before claiming destination confinement.
The denied control removes inherited networking, and the unavailable control
sets `tcp=n`; neither peer may accept a connection.

The guest validates the fixture certificate chain and DNS name `localhost` using
normal rustls verification, with exactly one fixture CA. No dangerous verifier,
validation bypass, ambient trust store, or DNS request is involved. The actual
network address is loopback; the TLS name and upgrade authority are explicit.

## Executed behavior

The [receipt](../../verification/wasi-sockets/evidence/execution.txt) contains all
12 cases and the exact outer imports. The native peer independently asserts the
upgrade path/authority, text `subscribe:7`, binary `[0,7,255]`, and close frame.
It supplies distinct notification and binary values rather than echoing input.
The guest requires status 101, both values, and close acknowledgement. The
corrupt peer supplies `notification:42`; only a failing guest with the expected
wrong-payload diagnostic passes that negative control.

Other controls require exact UnknownIssuer and NotValidForNameContext errors,
a real TLS ClientHello rejected by a plaintext peer, HTTP 403 upgrade rejection,
and rejection of a 65-byte frame with both frame and message limits set to 64.
Each journey has one guest process and one peer thread, a five-second process
limit, four-second accept limit, two-second socket I/O limits, and bounded
captured output. The guest has no detached networking task or dispatch queue.

For quiet cancellation, a nonblocking receive first returns WouldBlock. Dropping
its sole socket owner causes the peer to observe EOF within 240ms while the guest
stays alive another 350ms. Keeping the owner alive is the negative control and
must time out without EOF. Process unload kills the guest only after its real
pending-receive marker and observes the same closure. These are resource-drop
and command-process teardown proofs, **not an async Future wakeup contract or an
embedded Wasmtime Store unload proof**.

A peer may successfully buffer a write after observing EOF. The receipt exposes
that result. A closed guest resource cannot consume that stream, but this does
not prove application reply generations/correlation or remote cancellation
acknowledgement. Graceful close is separately proved in the exchange case;
resource drop intentionally need not send TLS close_notify or a WebSocket close.
The experiment does not implement subscriptions, reconnection, DNS, proxies,
fragment/queue orchestration, caller authentication, or mobile device journeys.

## Reproduce

From the repository root, set `WASMTIME` to the pinned executable:

```sh
sockets_manifest=verification/wasi-sockets/Cargo.toml
cargo +1.99.1 fmt --manifest-path "$sockets_manifest" --all --check
cargo +1.99.1 clippy --locked --manifest-path "$sockets_manifest" --all-targets -- -D warnings
cargo +1.99.1 build --locked --manifest-path "$sockets_manifest" --bin guest --target wasm32-wasip2
cargo +1.99.1 clippy --locked --manifest-path "$sockets_manifest" --bin guest \
  --target wasm32-wasip2 -- -D warnings
cargo +1.99.1 run --locked --manifest-path "$sockets_manifest" --bin runner -- "$WASMTIME"
cargo +1.99.1 xtask style
```

The dedicated Linux workflow repeats these checks with a SHA-256-pinned host
archive. Local execution does not establish Linux CI success; that remains
required before merge. This experiment alone does not complete Phase 1 or
establish generic WASI WebSocket support.

## Fixture provenance and next boundary

The DER certificates and PKCS#8 key are new disposable test fixtures, generated
locally with OpenSSL EC P-256 keys and no personal subject data. The CA subject is
`kit-local-test-ca`; the server subject/SAN is `localhost`, with serverAuth EKU,
critical digitalSignature key usage, and CA:false. A second unrelated CA supplies
the wrong-trust control. The server certificate expires on 2036-10-19; regenerate
these fixtures before then. The committed server key is intentionally public,
valid only for this fixture and never suitable for deployment. CA signing keys
remain ignored scratch artifacts and are unnecessary to reproduce execution.

The next public adapter unit must provide host-enforced destinations, readiness
and cancellation futures, bounded multi-connection dispatch, embedded Store
unload and replacement, late-result rejection, deadlines during handshake/body,
and clean application-owned lifecycle. Keep those requirements open.

Pinned upstream sources: [Wasmtime network context](https://github.com/bytecodealliance/wasmtime/blob/v50.0.0/crates/wasi/src/ctx.rs),
[rustls ring provider](https://github.com/rustls/rustls/blob/v/0.23.46/rustls/src/crypto/ring/mod.rs).
These facts explain the chosen contract; the actual component/peer assertions
supply its runtime evidence.
