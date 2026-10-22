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
