# Browser WebSocket execution

The browser adapter is exclusively for off-chain application code. On-chain
programs use Pina or official Solana program libraries.

The scoped Chrome fixture runs with:

```sh
cargo run -p solana_kit_browser_websocket_verification --offline
```

Set CHROMEDRIVER to a matching installed driver and use the existing
WASM_BINDGEN_TEST_WEBDRIVER_JSON option to select an installed Chrome binary.
The fixture owns each accepted connection and joins every thread before exit.
It uses only loopback WS and WSS. TLS verification remains enabled. The supplied
synthetic certificate and disposable public key retain their fixture notice.

The actual Chrome run covers all nine typed subscription families, complete
subscribe and unsubscribe frames, notifications before acknowledgements, failed
signature termination and Root survival on the same socket. It executes borrowed
receive cancellation and subsequent resumption, Pending and a real event wake,
explicit close, owner Drop, peer close, queue overflow, oversized messages,
binary ArrayBuffer rejection, explicit Blob rejection, malformed JSON propagation,
URL and protocol bounds, send backpressure, deadlines, cancelled connect and close,
constructor and refused handshake failures, late events, and handler detachment.
It also executes a reentrant waker that delivers a terminal event while the
message callback returns, and close-before-open and UTF-8 byte boundary controls.
Six additional lifecycle tests use safe `Arc<Wake>` owners whose final destructor
dispatches a real DOM error event through a separately captured JavaScript socket.
They cover receive cancellation and replacement, ready delivery, terminal wake,
connect cancellation and close replacement. Waker cloning precedes each state
borrow. Replaced and cancelled wakers drop after the borrow ends. Terminal
callbacks retain their existing take-then-wake boundary and first failure.
The cancellation and replacement tests each reproduce `RefCell already borrowed`
against the original adapter in separate actual Chrome runs.
Browser test instrumentation retains sockets and observes all four handler
properties. The final fixture counts completed connections and joins tasks.

Actual Chrome rejects the untrusted WSS certificate. A trusted positive WSS path
has not executed. This macOS installation has no isolated browser-profile trust
setup in this verification package. No certificate bypass, global trust change,
or claim of trusted WSS execution is made. Browser failure events conceal whether
the underlying cause is TLS, HTTP handshake, CORS policy or another network error.

The source pin remains Dart 0138bb956d2d81ccc0c56edc602401c0df2f2b46. Its original
98 WebSocket tests execute in the native Dart VM with the preserved package
configuration and no dependency resolution. They are separate from Chrome proof.
The scoped declaration receipt retains all 173 canonical IDs and 23 defining
source hashes, with 76 partial, 97 pending and zero complete. Regenerate it with
`rustc --edition 2024 verification/browser-websocket/receipt.rs` followed by the
capability inventory and output path arguments. Partial records describe bounded
Window behavior, not complete Dart API compatibility. Pooling, coalescing,
autoping, broadcast convenience, default destination policy and reconnect remain
pending. The canonical inventory is unchanged.

Core, client, subscription schema, existing browser HTTP implementation and all
101 portable cases retain exact source identity with the accepted nonce parent. This
unit does not rerun the five portable runtime hosts. Three target compile checks
and exact normal/build graph checks retain their distinct compile-only scope.
Workers, native UI webviews, WASI, interpreted Wasm, installed mobile devices,
validators and production endpoints have no socket execution claim here.

The integrated Window boundary executes 18 WebSocket and six unchanged HTTP
groups on both Rust 1.99.1 and 1.89.0 in actual Chrome. A separate public consumer
executes six independent controls on both compilers: cancelled receive queue-byte
reuse, replaced waker destruction, queued Ready delivery, first clean peer close,
cumulative queue-byte overflow, and reentrant wake/Drop terminal ordering. All
accepted fixture connections complete and every owner joins. Deliberately
injected DOM events remain distinct from network frames.

The HTTP verification launcher inherits the selected Rustup toolchain, as the
WebSocket launcher does. The executed Wasm producer metadata confirms Rust 1.99.1
and 1.89.0 respectively; an outer launcher version alone is not compiler proof.

The browser production and original test sources retain exact accepted lifecycle
identity. The current native transports, private hosts and shared 101-case suite
remain unchanged; their existing portable runtime proof carries by source identity.
The original failing adapter's independent borrow-panic receipt remains retained.
The earlier independent review also records an accidental refresh of a preexisting
local Dart test cache; its contents were retained and the 98 tests were rerun from
an owned directory with an explicit package configuration.
