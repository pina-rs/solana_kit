# solana_kit_wasi

WASI Preview2 HTTP for off-chain application clients. This crate is not for
Solana on-chain programs. Use Pina or official Solana program libraries there.

`WasiHttp` implements the public `solana_kit_client::Transport` with local futures.
A caller-owned `Reactor` registers WASI pollables, selects readiness and deadlines,
and wakes the caller. No global executor, thread, detached task, automatic retry,
redirect following, ambient credentials or guest-owned TLS configuration exists.
`Future::poll` uses only nonblocking WASI operations. The caller explicitly runs
`Reactor::drive` outside polling, or uses its synchronous `block_on` convenience.
This convenience runs WASI readiness futures and immediately ready computations;
a Pending future that registers no WASI readiness returns `DriveError`.

The adapter APIs exist only for wasm32-wasip2. Preview1 and plain/browser Wasm
retain the portable client and endpoint parser, without WASI HTTP imports.
The pinned contract is wasi:http/io/clocks@0.2.12 through wasip2 1.0.4.
Wasmtime 50.0.0 is the currently exercised named host. This does not establish
support for every WASI host or for installed Android/iOS applications.

A nonzero total timeout spans request streaming, headers, body and trailers.
The caller supplies endpoint and application headers explicitly. Receive calls
request at most the remaining body budget plus one probe byte, capped at64KiB
plus one. Oversized bytes are rejected before copying into the reply Vec. Host
buffers and allocations are separate. The host owns the HTTP parser, response
header/trailer limits, DNS and TLS verifier. Guest code cannot independently
bound host parser allocations or grant destinations, install trust roots or
turn off certificate/name validation. Configure those policies in the embedding
host. Wasmtime CLI `-S http=y` permits HTTP destinations without an allowlist.
Missing required imports fail component instantiation and cannot become a guest
TransportError. A host-reported HttpRequestDenied becomes Denied; I/O and deadline
errors remain observable with the original host diagnostic.

Upload and response readiness are polled together. An early HTTP response can
stop the upload, and a typed host denial takes precedence over a body stream
closed by that denial. Both retain the original total deadline. Stream failures
use the structured HTTP error code when the host supplies one. Wasmtime50
currently supplies no structured code for malformed chunks or truncated bodies,
so those failures remain I/O errors with their diagnostic source.

Dropping a polled exchange removes its readiness registration before releasing
request/response/body resources. Actual named-host fixtures observe peer EOF for
pending headers and body reception while the guest remains alive, alongside a
retained-future negative control. No remote cancellation acknowledgement, pooled
connection, DNS cancellation or embedded Store unload accounting is promised.

Waker clone, replacement, destruction and wake callbacks run without a registry
borrow. They may inspect the reactor, including during cancellation or readiness
completion. Focused Preview2 tests exercise those callback boundaries with the
real host pollables.

The private embedded Wasmtime50 reference host executes destination denial,
trusted/untrusted/name TLS, early HTTP responses, bounded TLS/upload stalls,
repeated success/error/cancellation and Store drop/replacement. Its explicit
host correction deletes owned request-options retained by the official50
handler. See the private verification README and execution receipt for the
precise policy, resource baselines and host limits. Operating-system DNS stalls,
uncancellable late resolver callbacks and local MSRV Preview2 execution remain
unverified. Focused CI provisions the MSRV target and executes that component.

The unverified REST source extension adds `WasiRest<D>` only on Preview2. Named
routes reuse existing `WasiHttp`, Reactor registrations, outgoing HTTP resources,
concurrent upload/response progress, typed host denial and bounded body reception.
Approved base queries remain first; decoded operation segments and ordered pairs
are separately encoded. Body presence determines media type. Original RPC POST
framing remains unchanged. The public provider consumer must run under its actual
caller-driven component host. Preview1 has no networking adapter. Compilation,
actual provider REST replay, resource checks and negative controls remain pending.

## Framed WebSockets and subscription time

Enable the explicit `websocket` feature for `WasiWebSocket`, `SocketConfig` and
`SocketError` on Preview2. The owner uses the same caller-owned Reactor,
nonblocking WASI TCP/DNS resources, Tungstenite0.29 framing and validating
rustls0.23 TLS. WSS requires explicit DER CA roots. The host grants DNS and TCP
destinations. Preview1, browser and plain guests gain no socket imports.

`connect` has one deadline across DNS, TCP, TLS and the strict HTTP upgrade.
Send, receive and graceful close have finite deadlines. Frame, complete-message
and encoded write-buffer limits are explicit. Text-only `receive` and
`poll_receive` reject binary. `receive_frame` and `poll_receive_frame` return
`ReceivedFrame::Text` or `ReceivedFrame::Binary`, checking payload limits before
copying parser-owned bytes. Ping/pong and graceful close retain the existing
bounded control-frame turn. The socket owns one parser and separate read/write
readiness slots.

`WasiWebSocket` implements the shared `TextSocket` directly. Pass the connected
socket to `subscriptions::Session::new`. Session owns correlation, generations,
provider sharing and bounded local queues. Outgoing commands remain text.
Transient send/flush/receive/close polls allow a quiet receive to coexist with
unsubscribe progress. Cancelling a send is uncertain and never permits replay.
Shutdown unregisters readiness before releasing parent socket resources.

`WasiClock::new(reactor.clone())` uses monotonic milliseconds and one existing
Reactor registration per sleep. Drop unregisters its pollables. Checked conversion
reports `ClockError::DeadlineRange`. Readiness admission reports
`ClockError::Capacity`. Both propagate as `SessionError::Clock`, distinct from a
successfully elapsed deadline. No separate timer scheduler or executor exists.

These socket/clock and `verification/wasi-subscriptions` source paths await
admitted current/MSRV and named-host execution. They add no runtime acceptance.
The application drives its Reactor and separately awaits Session shutdown.
Ordinary P2 socket packaging also needs a WASI-capable C compiler and archiver
for ring. The existing private fixture's loopback policy and original331 tests
remain separate from installed-host, resolver-cancellation and broad WASI support.
