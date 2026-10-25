# solana_kit_native

Tokio HTTP/1 and text WebSocket transports with verified TLS for off-chain Solana application code.
Use Pina or official Solana program libraries for on-chain programs.

`NativeHttp::new(endpoint, HttpConfig::default())` implements the public
`solana_kit_client::Transport`. Each polled exchange owns its connection and
HTTP driver. It creates no runtime, pool, detached task, redirect, retry, proxy,
cookie store or ambient authentication. Caller headers and DER trust roots are
explicit. Diagnostics omit endpoint and header values. HTTPS verifies both the
certificate chain and destination name; verification cannot be disabled.

The total deadline covers DNS, TCP, TLS, response headers and complete body.
The connect deadline also bounds DNS/TCP/TLS. Dropping the exchange closes owned
network work; Tokio's blocking OS name resolution may finish after cancellation,
but cannot subsequently send HTTP. Dropping a spawned Tokio JoinHandle detaches
it: abort and await the handle to cancel that call.

Response body bytes are checked before each reserve and copy against the
request budget, independent of Content-Length. Hyper's parser enforces header
byte and field-count limits, including the status line; its byte limit also
bounds chunk trailers. The separately configured read buffer is at least 8192
bytes. These are bounded buffers in addition to the bounded collected body;
the body budget is not a promise of an exact whole-process allocation ceiling.
Non-success statuses and their bounded bodies return unchanged for the client
policy. Compressed bodies are not automatically decompressed.

The named native future boxes one Send future because stable Rust cannot name
an async-block associated GAT. Generic Tokio multiworker callers retain Send;
portable/local client futures are unchanged. HTTP/2, connection pooling,
browser/WASI/guest socket adapters and device journeys are separate work.

```rust
use solana_kit_client::{Client, Limits};
use solana_kit_native::{HttpConfig, NativeHttp};

let transport = NativeHttp::new("https://rpc.invalid/", HttpConfig::default())?;
let client = Client::new(transport, Limits::default());
# let _ = client;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Construction performs no network request. See the repository's native HTTP
verification receipt for executed local controls and remaining platform limits.

`NativeWebSocket::connect(endpoint, WsConfig::default()).await` opens one
caller-owned text socket. The endpoint explicitly selects `ws://` or `wss://`.
Use WSS for untrusted networks. There is no private-destination filter. Accept
endpoints only from trusted application configuration. Caller headers supply
authentication. Userinfo, fragments, protocol overrides and cookie headers reject.
Certificate chain and destination-name verification cannot be disabled.

`send(&str)` checks its payload length before copying and flushes within a
deadline. Incoming frame and reassembled-message limits apply during parsing.
The write buffer has a hard encoded-byte limit. Ping and pong frames flush while
receiving. Binary data, invalid frames, abrupt EOF and exceeded budgets return
redacted typed errors. The fixed Tungstenite handshake guard permits at most
65,536 input bytes plus one bounded read chunk and limits tiny packet attacks.
An observed receive/send error or timeout drops the owned socket. Oversized local
send input rejects before touching the socket. This is not a whole-process memory
ceiling. Do not enable dependency wire-level trace logging for credentialed
connections. Tungstenite trace logs can include complete requests and frames.

Use a separate `solana_kit_client::subscriptions::Connection`. Send each outbound
UTF-8 frame, ingest each complete received text with that socket's generation,
and drive acknowledgement deadlines with caller-owned monotonic ticks. Drop the
socket on a current-generation ingest failure. Dropping Connection releases its
local state and does not drop this independent socket.

Cancelling a borrowed `receive()` retains the persistent socket and its bounded
parser. Cancelling a borrowed `send()` can leave a partial write. Drop the socket
after a cancelled send and do not retry uncertain delivery automatically.
Cancelling `connect()` drops in-progress I/O. OS DNS resolution may finish later,
but it cannot send the handshake after cancellation. `close(self)` consumes the
socket and waits for peer close within one operation deadline. Cancelling that
consuming future drops its socket. Dropping the socket owner always releases its
connection locally. No implicit unsubscribe, pool, owned task, reconnect, replay,
automatic ping timer or retry exists.

The native WebSocket verification receipt records actual loopback WS and fixture
WSS tests. Browser, WASI, LeptosKit and installed device sockets remain pending.

The source-only Session extension exposes `receive_frame()` for explicit text or
binary messages and `NativeSubscriptionSocket` for the client `Session`. It reuses
the same tungstenite parser, budgets, deadlines and native socket ownership.
`receive()` continues to reject binary on its text-only contract. The maintained
Helius journey shares one physical JSON registration, forwards pre-ack binary,
checks a quiet last-handle Drop wake and separately awaits actual close. These
new bodies have not executed in this checkpoint.

The unverified REST source extension adds `NativeRest<D>` over these same HTTP
owners. Each named route owns an approved `NativeHttp` root, headers, TLS policy
and deadlines. Request segments and ordered query pairs append to that root with
separate percent encoding. Fixed root query values remain first. Decoded dot
segments are denied. Body presence owns its media type. The exchange retains its
inline socket and driver, bounds response copies, follows no redirects and sends
once. Normal REST errors redact request values while retaining the original cause
chain for explicit inspection. Original fixed-POST RPC behavior remains separate.
Current/MSRV compilation, quality and actual REST peer replay remain pending.
