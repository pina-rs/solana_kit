# Native HTTP transport receipt

This bounded Phase4 slice implements `solana_kit_native0.1.0` for off-chain
application code. On-chain programs should use Pina or official Solana program
libraries. Full RPC, subscriptions, browser/WASI/guest transports and device
journeys remain separate.

## Ownership and dependencies

Two shapes were considered: a high-level pooled client with an executor and
connection tasks, and one directly owned Hyper HTTP/1 connection per exchange.
The latter keeps socket, TLS, HTTP driver and body reception in the returned
future, with no detached driver or hidden retry. It deliberately pays connection
setup per request. A named native GAT wrapper boxes one explicitly Send future
because stable Rust cannot name an async-block associated type. The portable
Transport contract and its local futures stay unchanged.

Direct pins are Hyper1.12.0, hyper-util0.1.21 (Tokio IO only),
http-body-util0.1.3, bytes1.11.1, Tokio1.54.1, tokio-rustls0.26.6
(ring/TLS1.2; no AWS-LC), and webpki-roots1.0.10. The lock selects
Rustls0.23.46, rustls-webpki0.103.16 and ring0.17.14. No broad Solana dependency
enters this adapter. No native dependency enters core/client normal/build graphs.
The native crate intentionally requires Tokio and native sockets; applications
choose other adapters for Wasm hosts.

The pinned implementation and current primary
[Hyper parser builder documentation](https://docs.rs/hyper/1.12.0/hyper/client/conn/http1/struct.Builder.html)
were inspected. `max_header_size` bounds status line plus headers and chunk
trailers, `max_headers` bounds fields, and `max_buf_size` bounds the adaptive read
buffer (minimum8192). Hyper exposes its public too-large classification only with
its server feature, so that feature is enabled solely to retain HeaderLimit
classification; no server or executor is started by the SDK. It adds httpdate.
Certificate and name verification use
[Rustls' safe client configuration](https://docs.rs/rustls/0.23.46/rustls/struct.ClientConfig.html).
There is no verifier override or process-global provider installation.

## Contracts and executed controls

The endpoint must be absolute http/https, have a valid host and explicit valid
port when supplied, and contain neither userinfo nor a fragment. Invalid port
text rejects instead of silently selecting a default: http::Uri's optional port
accessor alone would hide overflow. Caller authorization is explicit and secret
values are omitted from configuration/transport diagnostics. Caller headers
cannot override framing, Host, content type or hop-by-hop/proxy controls.
No proxy discovery, cookies, redirects, retries or ambient credential lookup is
implemented. A307 fixture returns unchanged while its separate destination
observes no connection and receives no authorization.

The whole exchange deadline covers DNS/TCP/TLS/headers/body; the connect deadline
also bounds DNS/TCP/TLS. Owned cancellation drops actual sockets and HTTP work.
Tokio's blocking OS resolver may finish after cancellation; it cannot continue
into a connection or HTTP request after the exchange future is dropped. This
is a documented resolver limitation, not a claim to abort the OS resolver.

Body collection rejects a known oversize before awaiting body bytes and checks
every received data frame before reserve/copy. Actual fixtures cover fixed,
chunked and EOF-delimited bodies at the exact budget and one byte over, missing
and inconsistent lengths, malformed/truncated framing, non2xx status/body,
header count/bytes at exact boundaries and oversized trailers. Hyper's trailer
error retains its original source; its public classification can be Io rather
than HeaderLimit. Allocations include bounded parser/TLS buffers in addition to
the collected body; the body limit is not a whole-process memory ceiling.
A smaller Content-Length defines the HTTP response boundary; surplus bytes are
not silently added to the body. No response decompression occurs.

Actual public Client balance execution preserves u64::MAX through a generic
multiworker Tokio spawn. The fixture validates POST/path/query, JSON, content
type, Content-Length and explicit synthetic authorization. Trusted local TLS
succeeds; untrusted roots and the wrong destination name fail. A quiet TLS peer
proves the connect deadline; quiet headers and a stalled body prove the total
deadline and peer EOF. A retained quiet exchange first proves no EOF, then both
a direct future drop and an aborted/awaited spawned call produce actual peer EOF.
All fixture tasks are awaited; ports are ephemeral loopback and no remote RPC
cluster or credentials are used. Synthetic public test certificates/key are
packaged with their provenance, never read from a machine trust/key store.

## Verification and release limits

Native macOS executes14 integration tests. Default and allfeature workspace
checks, MSRV1.89, warning-denied docs/Clippy, formatting, actionlint, Monochange
check/validation/preview and the improved explicit-file monostyle gate are run
for the final unit. Linux/macOS/Windows actual journey CI uses an explicit Bash
shell, so an early native command failure cannot be masked by a later success.
Remote CI and independent final-head review remain coordinator-owned.

A clean downstream workspace uses ordinary source dependencies without patches
or repository Rust flags and executes the generic native Send balance journey
under MSRV1.89. Components remain unpublished; this proves source consumption,
not registry-install readiness. The sdk release group explicitly owns
core/client/native, and all publishing remains disabled. No component placeholder
or publication is authorized or attempted by this unit.
