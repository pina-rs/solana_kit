# solana_kit_native

Tokio HTTP/1 and text WebSocket transports with verified TLS for off-chain Solana application code.
Use Pina or official Solana program libraries for on-chain programs.

The optional `keys` feature adds `keys::NativeEntropy` for explicit exportable-key
generation and `keys::write_keypair_new` for intentional Solana-keygen JSON backups.
The entropy owner uses a caller-selected rustls secure provider; its default is
the same ring provider used by TLS. The original provider failure remains typed.
No global provider or executor is installed.

Backup writes are blocking. Callers select their blocking executor and join its
work before teardown. On Unix, the destination appears only after the complete
file is flushed in a new private staging directory. Linux and Android publish it
with an atomic no-replace rename because Android apps cannot create hard links.
Other Unix platforms use a hard link. Both operations refuse existing files,
directories and symbolic links. The caller must supply a trusted parent
directory. Private files use mode 0600 and staging directories use 0700. An error
reports whether publication preceded cleanup failure, and retains the original
I/O cause and all cleanup errors. Unsupported filesystem operations fail
explicitly. Other platforms return Unsupported before creating a file. This does
not promise power-loss durability or protection against a hostile owner of the
parent path.

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

## Optional Surfpool file deployment

Enable `surfpool` to call `surfpool::deploy(&surfnet, options)` with an existing
`solana_kit_surfpool::Surfnet`. `surfpool::DeployOptions::new(program_id, source)`
accepts exactly one `ProgramSource::Bytes(&bytes)` or `ProgramSource::Path(&path)`.
The address is supplied explicitly. `idl_path` optionally selects an explicit
Anchor IDL JSON file. This API does not discover artifacts or keypair files,
search project/home directories, consult environment paths, or own a process.

Program input defaults to at most 16 MiB and IDL input to at most 1 MiB. The
caller can override `max_program_bytes`, `max_idl_bytes` and the nonzero
`chunk_bytes` field. The default 3 KiB chunks reuse portable Surfnet deployment
and its existing client wire budget. Program limits apply to borrowed bytes too.
Native paths must name regular files. Tokio metadata checks reject known
oversized files before reading; incremental reads catch growth and retain at
most the selected limit in the vector plus a fixed 8 KiB read buffer. Allocation
capacity and parsed JSON have additional bounded overhead. Callers supply
trusted paths; concurrent path replacement is not prevented.

Every program write succeeds before the IDL is opened or parsed. The IDL must
be a JSON object. Unknown fields and exact integers are retained, and its
`address` is overwritten with the supplied program address before registration.
Literal keys matching serde_json's internal number/raw-value tokens remain object
keys, including inside arrays. Parsing retains serde_json's existing nesting ceiling.
Duplicate keys retain its last-wins behavior, with every earlier value validated.
The returned address is the supplied one. A later IDL read, parse or RPC failure
leaves earlier writes visible; there are no retries or rollback. Dropping the
future drops its active RPC exchange and starts no later write or IDL access.
An already submitted Tokio filesystem operation may finish after cancellation.
Implicit diagnostics omit paths and content, while public error variants retain
the original typed I/O, JSON and portable deployment causes for explicit inspection.

Seventeen native contract cases cover explicit file/byte equivalence, deferred
IDL access, literal JSON, partial writes, bounds, cancellation and a native
transport future that retains `Send`. Strict all-feature Clippy and API
documentation also pass. Isolated feature graphs and current-source CI remain
required; real program execution is a separate integration check.

`surfpool::discover_program_artifacts(name, &target_roots).await` selects the first
same-root regular `deploy/<name>.so` and `deploy/<name>-keypair.json` pair in caller
order. Supply absolute target directory paths explicitly. Names must be one nonempty
filename stem; separators, controls, dot/dot-dot and Windows colon/prefix syntax reject.
Every root is validated before filesystem access. Discovery neither sorts nor
canonicalizes paths and performs no environment or ancestor search. Missing and
nonregular candidates count as absent. Permission and other metadata errors return
their candidate index, role and original typed cause, a stricter policy than Dart's
existence checks. `ProgramArtifacts` owns private paths with explicit accessors and
redacted Debug. Only the winning root's IDL presence is snapshotted.

With both `surfpool` and `keys`, `surfpool::deploy_program(&surfnet, &artifacts, options)`
reads a bounded Solana-keygen JSON array into zeroizing storage and verifies its
64-byte seed/public-key pair through Core. A malformed selected key fails without
falling back to another root. The signer, raw input and decoded bytes are dropped
before program reads or RPC work. `DeployProgramOptions` defaults to a 1 KiB key-file
cap plus the existing program, IDL and chunk limits, with explicit caller overrides.
The raw input allocation is reserved once to avoid reallocating copied secret bytes;
the read scratch and decoded array also use zeroizing storage. This does not promise
erasure of copies held by the OS or dependency I/O machinery.

Deployment reuses the explicit path API above. An IDL absent at discovery stays
excluded if created later. A selected IDL removed later fails after program writes.
Selected files replaced with nonregular files fail with typed errors. No process,
runtime, key backup, fallback, retry or rollback is installed. These discovery and
key-backed deployment source tests have not executed in this checkpoint. Earlier
native checks do not establish the new feature combinations or artifact behavior.

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

## Optional mobile wallet transport

Enable `mwa` for a caller-owned binary Mobile Wallet Adapter session. After an
explicit user action, generate `mwa::NativeAssociation` with `NativeEntropy`,
present its URI to an Android wallet, and consume the proposal in `connect`.
Use `call` with the existing portable MWA method and typed response contracts.
Wallet rejection remains a protocol response; transport errors preserve their
original source. Dropping a pending call closes local authority. Await `close`
for the bounded peer close, or drop the unique owner for immediate local close.

Authorization and signing use `MwaConfig::approval_timeout`, which defaults to
five minutes and includes the user's approval time. Capability queries, revocation
and peer close use the socket operation deadline. Applications can configure both.

This transport does not launch Android activities or grant signing approval.
Desktop and iOS applications must select their actual available wallet backend;
Android MWA availability is never inferred from a Wasm target. A generated app
still needs to compose the native session with its application service factory.
