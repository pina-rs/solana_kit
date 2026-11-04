# Current public WASI text socket and clock execution

On 2026-11-04 the public `solana_kit_wasi` `websocket` feature executed in an
actual `wasm32-wasip2` component on macOS ARM64. The component ran under the
pinned Wasmtime 50.0.0 CLI and under the private embedded Wasmtime host. Source
was `4f3c949aa49b622e3b207f0f729e11c76cc8cb4b`; this unit adds evidence and
status text only, with no production SDK or harness source change.

These are actual loopback WS/WSS peers, a typed `subscriptions::Session` driven
by `WasiClock`, explicit TLS trust/name negatives, bounded frame failures,
quiet-stream cancellation, CLI process unload and embedded Store
destruction/replacement. They are not compilation-only or simulated replies.
The harness's path dependency resolves to this workspace crate; it does not
implement a second socket codec. No production RPC endpoint or credential was
used, and every peer is an owned localhost fixture.

## Pins and artifacts

| Item                | Actual value                                                                                                                                       |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| Rust                | 1.99.1 (`58cf8f9c3`, 2026-10-13), host `aarch64-apple-darwin`                                                                                      |
| Host                | Wasmtime 50.0.0 (`ebe5e8df1`, 2026-10-20), macOS ARM64                                                                                             |
| Host binary SHA-256 | `d5bd3748633fcfb39a11c90c7225f981103d82c1141a6f10382e27fbf404571f`                                                                                 |
| Guest target        | `wasm32-wasip2`, C11 wasi-sdk producer plus `rustc 1.99.1`                                                                                         |
| Guest SHA-256       | `617c7b7b0f01546e2a1483aeeb231e579d8324b5f6d82accc5639f7a5be0ff2d`                                                                                 |
| Guest size          | 49,251,581 bytes                                                                                                                                   |
| Contract            | `wasip2 = 1.0.4+wasi-0.2.12`, `tungstenite = 0.29.0`, `rustls = 0.23.46` with `ring = 0.17.14`                                                     |
| Harness             | `verification/wasi-text-sockets` at this source                                                                                                    |
| Fixtures            | Existing public disposable test CA/server/wrong-CA DER files from the earlier `wasi-sockets` experiment; the server certificate expires 2036-10-19 |

The committed [execution transcript](../../verification/wasi-text-sockets/evidence/execution.txt)
contains every mode line and the exact component imports. The
[artifact hashes](../../verification/wasi-text-sockets/evidence/artifacts.sha256)
record the guest, host executable and fixtures. Full raw logs and the JSON build
record were retained outside every `target` directory at
`/tmp/solana-kit-rw05-evidence/`; they are disposable local evidence, not
committed build output.

## Executed CLI modes

The CLI runner starts one real peer thread and one Wasmtime guest process per
mode. It grants `allow-ip-name-lookup=y`, `udp=n` and, except for `denied`,
`inherit-network=y`. Each positive guest must print `PASS <mode>` and exit 0;
the corrupt control must exit nonzero with its exact payload diagnostic.

| Mode           | Actual assertion                                                                                                                                                                                     |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plain`        | `ws://` without TLS sends `subscribe:7`, receives `notification:41`, completes the close handshake                                                                                                   |
| `session`      | Typed `Session` plus `WasiClock`: a root notification arrives before its subscribe response, is drained as `u64::MAX`, then unsubscribe and shutdown complete with no remaining Reactor registration |
| `exchange`     | Trusted `wss://`: exact text request, independent text reply and graceful close                                                                                                                      |
| `duplex`       | `poll_receive` is pending and registers readiness, then `start_send`/`poll_flush` writes while that receive remains pending                                                                          |
| `fragment`     | One text message split across a final-empty continuation frame and a final continuation frame is reassembled                                                                                         |
| `corrupt`      | A peer reply of `notification:42` makes the guest fail with `wrong text payload`; peer assertions and exit 1 are both required                                                                       |
| `wrong-ca`     | An unrelated fixture CA produces `SocketError::Tls`; the peer observes a real TLS rejection alert                                                                                                    |
| `wrong-name`   | A certificate for `localhost` reached as `127.0.0.1` produces `SocketError::Tls`; the peer observes the alert                                                                                        |
| `bad-upgrade`  | An HTTP 403 upgrade response produces `SocketError::Protocol`                                                                                                                                        |
| `oversized`    | A 65-byte frame under a 64-byte frame and message limit produces `SocketError::Limit`                                                                                                                |
| `binary`       | A binary frame on the text-only receive path produces `SocketError::Binary`                                                                                                                          |
| `invalid-utf8` | An invalid UTF-8 text payload produces `SocketError::Protocol`                                                                                                                                       |
| `masked`       | A masked server frame produces `SocketError::Protocol`                                                                                                                                               |
| `abrupt`       | EOF without a WebSocket close produces `SocketError::AbruptClose`                                                                                                                                    |
| `cancel`       | A pending receive is dropped, `shutdown` returns registrations to zero, and the peer observes EOF before 300 ms                                                                                      |
| `retain`       | Retaining the socket owner is the negative control: the peer observes no EOF for at least 240 ms                                                                                                     |
| `unload`       | The guest reaches `PENDING unload`, stays alive, then the supervisor kills the process; the peer observes EOF and the exit status is SIGKILL                                                         |
| `denied`       | `inherit-network=n` denies the socket before a peer connection; the peer accepts nothing                                                                                                             |

All 18 CLI modes passed. `unload` is command-process destruction, not Rust
resource-drop or embedded Store destruction.

## Executed embedded modes

The embedded host instantiates the same component in a fresh `Store` per mode.
It grants name lookup, denies UDP, and checks every socket address use with one
policy: wildcard ephemeral TCP bind and the exact loopback TCP port are allowed;
everything else is denied. The guest's endpoint still says `localhost`; the
policy observes both `127.0.0.1` and `::1` loopback connects.

| Mode set                                                  | Actual result                                                                                                            |
| --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `plain`, `exchange`, `session`, `duplex`, `fragment`      | Positive journeys complete with one resource in the table before Store drop and restored task counts                     |
| `wrong-ca`, `wrong-name`                                  | Real TLS rejection alerts, with the same policy and task/resource checks                                                 |
| `oversized`, `binary`, `invalid-utf8`, `masked`, `abrupt` | Exact typed failures, with the same policy and task/resource checks                                                      |
| `cancel`                                                  | Pending receive dropped, shutdown unregisters, peer EOF, then Store drop                                                 |
| `retain`                                                  | Retained-owner negative control times out without EOF, then Store drop                                                   |
| `denied`                                                  | The host denies the bind/connect policy; no peer connection; Store still drops cleanly                                   |
| `unload`                                                  | `PENDING unload` is polled as pending; the Store is dropped with 9 table resources; the peer observes EOF; tasks restore |
| Final `exchange`                                          | A fresh replacement Store instantiates the component again and completes the journey                                     |

All 17 embedded executions passed. The embedded run covers 16 of the CLI's 18
distinct modes; `corrupt` and `bad-upgrade` are CLI-only. The final duplicate
`exchange` is intentional: it proves the host can replace a Store destroyed
during pending I/O. Every completed embedded mode reported one resource before
Store drop; the pending `unload` Store reported nine. "Tasks restored" is the
harness's exact alive-task count comparison after Store drop and peer join.

## Cancellation, unload and replies

Quiet cancellation is application-owned and bounded. The guest polls a receive
to `Pending`, verifies one Reactor registration, drops the receive future,
verifies the parser's wait registration is still owned, then either shuts the
socket down (`cancel`) or keeps it alive (`retain`). Only `cancel` closes the
stream early. This proves pending-receive cancellation and resource-drop
teardown, not operating-system resolver cancellation. No DNS stall, retry,
background resolver callback or arbitrary-host lookup is injected.

Process destruction and Store destruction are separate:

- CLI `unload` kills the live child process after its pending marker. That proves
  command-process teardown only.
- Embedded `unload` polls the component to a suspended state, confirms it stays
  pending beyond the Store-drop interval, drops the actual `Store`, joins the
  peer, checks the table resources are released, restores the runtime task count,
  then instantiates a fresh Store for the final exchange.

The `session` mode proves a notification that arrives before its subscribe
response is correlated and drained, followed by unsubscribe and shutdown. This
harness does not inject a further reply after a cancelled receive; the
cancellation evidence is the observed pending-drop/shutdown/peer-EOF sequence.
No remote cancellation acknowledgement is claimed.

## TLS, DNS and import audit

Positive WSS uses exactly one fixture CA and normal rustls verification of the
`localhost` name. The two negatives are an unrelated CA and a name mismatch;
each peer observes an actual TLS alert. This is not a revocation, TLS
version/cipher-suite, hostname-edge-case or production trust-store test. The
CLI's `inherit-network=y` grant is not a destination allowlist; only the embedded
policy provides exact destination confinement. Raw socket tests remain separate
from authorization or configurable trust roots a public embedder might add.

DNS is bounded to the fixture name `localhost`. The CLI grants name lookup; the
embedded host grants it and then applies its address policy. Neither run
practices arbitrary OS DNS, DNS cancellation or resolution of a public host.

The audit validated the actual component encoding and recorded its complete
outer imports:

```text
wasi:cli/environment@0.2.12
wasi:cli/exit@0.2.12
wasi:cli/stderr@0.2.12
wasi:cli/stdin@0.2.12
wasi:cli/stdout@0.2.12
wasi:cli/terminal-input@0.2.12
wasi:cli/terminal-output@0.2.12
wasi:cli/terminal-stderr@0.2.12
wasi:cli/terminal-stdin@0.2.12
wasi:cli/terminal-stdout@0.2.12
wasi:clocks/monotonic-clock@0.2.12
wasi:clocks/wall-clock@0.2.12
wasi:io/error@0.2.12
wasi:io/poll@0.2.12
wasi:io/streams@0.2.12
wasi:random/insecure-seed@0.2.12
wasi:random/random@0.2.12
wasi:sockets/instance-network@0.2.12
wasi:sockets/ip-name-lookup@0.2.12
wasi:sockets/network@0.2.12
wasi:sockets/tcp-create-socket@0.2.12
wasi:sockets/tcp@0.2.12
```

There is no `wasi:http/` import, no non-WASI service import and no UDP import.
The audit also wrote a harness-generated component with `wasi:sockets/tcp` plus
`env:fetch/network` and required its inspector to reject it with
`unexpected service import`; that negative passed. The component producer
metadata contains `rustc 1.99.1` from this build. Older dependency producer
entries remain in the same custom section and are not the SDK guest compiler
identity.

## Reproduction and gates

From the repository root, with the concrete 1.99.1 toolchain bins first on
`PATH` and `CARGO`, `RUSTC` and `RUSTDOC`:

```sh
export WASMTIME=/path/to/wasmtime-v50.0.0
export TC=/path/to/rustup/toolchains/1.99.1-aarch64-apple-darwin
export PATH="$TC/bin:$PATH" CARGO="$TC/bin/cargo" RUSTC="$TC/bin/rustc" RUSTDOC="$TC/bin/rustdoc"
export CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 CARGO_HTTP_USER_AGENT=build
manifest=verification/wasi-text-sockets/Cargo.toml

cargo fmt --manifest-path "$manifest" --all --check
cargo test --locked --manifest-path "$manifest" --bin runner embedded::tests
cargo clippy --locked --manifest-path "$manifest" --bin runner --bin audit -- -D warnings
cargo clippy --locked --manifest-path "$manifest" --bin guest --target wasm32-wasip2 -- -D warnings
cargo build --locked --manifest-path "$manifest" --bin guest --target wasm32-wasip2 \
  --message-format=json > /tmp/solana-kit-rw05-evidence/guest-build.json
component="$(jq -sre '
  [.[] | select(.reason == "compiler-artifact" and .target.name == "guest" and
    (.target.kind | index("bin")) != null and
    (.manifest_path | endswith("/wasi-text-sockets/Cargo.toml"))) |
    .filenames[] | select(endswith(".wasm"))] |
  if length == 1 then .[0] else error("expected one text socket component") end
' /tmp/solana-kit-rw05-evidence/guest-build.json)"
cargo run --locked --manifest-path "$manifest" --bin runner -- "$WASMTIME" "$component"
cargo run --locked --manifest-path "$manifest" --bin runner -- --embedded "$component"
cargo run --locked --manifest-path "$manifest" --bin audit -- \
  "$component" /tmp/solana-kit-rw05-evidence/forbidden-service.wasm
RUSTDOCFLAGS="-D warnings" cargo doc --locked -p solana_kit_wasi --features websocket \
  --no-deps --target wasm32-wasip2
dprint check
monochange check
cargo fmt --all --check
cargo xtask style
```

| Gate                                     | Actual result                                               |
| ---------------------------------------- | ----------------------------------------------------------- |
| Harness `cargo fmt --all --check`        | Exit 0                                                      |
| `embedded::tests`                        | 3 passed, 0 failed                                          |
| Native Clippy (`runner`, `audit`)        | Exit 0, warnings denied                                     |
| Guest Clippy (`wasm32-wasip2`)           | Exit 0, warnings denied                                     |
| Guest build and single-artifact selector | Exit 0, one emitted `.wasm` component                       |
| CLI runner                               | 18/18 modes passed                                          |
| Embedded runner                          | 17/17 executions passed, including replacement Store        |
| Component audit                          | Imports recorded, rustc producer present, negative rejected |
| `solana_kit_wasi` doc                    | Exit 0 with `RUSTDOCFLAGS=-D warnings`                      |
| `dprint check`                           | Exit 0                                                      |
| `monochange check`                       | 0 issues                                                    |
| Root `cargo fmt --all --check`           | Exit 0                                                      |
| `cargo xtask style`                      | 2258 files; every repository and per-file category >= 90    |

## Scope and remaining limits

This receipt is the named-host execution required by RW-05 for the public
socket and clock path. It does not prove Windows or Linux execution, Wasmtime
versions other than 50.0.0, every WASI host or runtime, browser execution,
installed Android/iOS applications or physical devices. The Ubuntu job in
`.github/workflows/client-sessions.yml` runs the same harness steps, but this
receipt is the local macOS ARM64 run; no Linux CI claim is made. It also does not
validate a public RPC provider, arbitrary destinations, UDP, proxies, retries,
reconnection, pooling, authentication, message replay or server-side effects.

The earlier `wasi-sockets` and `wasi-contract` documents are separate historical
experiments. `wasi-sockets` predates the public adapter and proves a private
prototype; `wasi-contract` covers the HTTP-only contract. Neither is current
evidence for the public `WasiWebSocket`/`WasiClock` path, and neither substitutes
for the embedded Store replacement run above.

No canonical capability status, declaration ledger, source hash, conformance
case, root manifest or lockfile changed in this unit.
