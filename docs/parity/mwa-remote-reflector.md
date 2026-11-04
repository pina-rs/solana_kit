# Native remote MWA reflector association

This receipt covers RW-04's native remote reflector path at pinned Dart source
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`, principally
`packages/solana_kit_mobile_wallet_adapter/lib/src/remote_association_scenario.dart`
(lines 69-158 and 515-528) and the association URI builder in
`packages/solana_kit_mobile_wallet_adapter_protocol/lib/src/association_uri.dart`.
The existing local binary transport remains unchanged, including its APP_PING
restart behavior.

## Lifecycle

`mwa::RemoteReflector::new(authority)` accepts only an exact bare TLS authority.
It validates that authority with the existing remote association URI contract
before any socket work, then builds one `wss://<authority>/reflect` request. The
same rule is exposed as `solana_kit_wallet::mwa::validate_reflector_authority`, so
there is no second authority parser. The caller supplies `NativeEntropy` and
`MwaConfig` explicitly; the socket uses
`WsConfig::tls`, so normal certificate-chain and destination-name verification
stay enabled. No default reflector, DNS-based discovery, redirect, proxy
credential, userinfo, fragment, arbitrary path, retry or background task is
installed.

`RemoteReflector::connect` is the first stage. Within
`WsConfig::connect_timeout` it completes TLS and the WebSocket upgrade, offers
exactly `com.solana.mobilewalletadapter.v1` and
`com.solana.mobilewalletadapter.v1.base64` in that order, requires exactly one
selected offered protocol with no extension, receives the reflector's first
payload, decodes it, and returns `PendingRemoteAssociation`. The pending owner
holds the only socket and private association/agreement state. It exposes the
real `solana-wallet:/v1/associate/remote?...` URI, whose `reflector` value is the
actual selected authority and whose `id` is the server-assigned ID.

`PendingRemoteAssociation::join` is the second stage. It waits up to
`MwaConfig::join_timeout` for exactly one empty decoded wallet-join frame. Only
then does it send the existing 129-byte HELLO, receive HELLO_RSP within the
socket operation deadline, accept the agreement, parse optional encrypted session
properties, and return the existing `NativeMobileWallet`. Calls, request
correlation, nonce history, approval deadlines, close, and borrowed-call
cancellation all reuse that existing path. Dropping the pending owner, the
connect future, the join future, or a borrowed active call closes the owned
socket and key state; no detached task survives.

All four timeout settings must be positive and representable before either
local or remote connection entry point performs I/O. Authorities and pairing
URIs remain explicit accessor data; `Debug` exposes neither.

## Wire details

- The reflector ID payload is LEB128 length-prefixed. Zero length, truncated
  payloads, more than ten varint bytes, integer overflow, and declared lengths
  past the received payload reject before allocating ID storage.
- IDs must fit the existing `ReflectorId` JavaScript-safe contract. More than
  eight raw bytes or a value above 2^53 - 1 reject.
- Binary selection requires binary WebSocket messages. Base64 selection requires
  text messages containing padded standard base64. A wrong frame type or invalid
  base64 rejects. Both the encoded text and decoded payload must fit
  `WsConfig::max_frame_bytes`.
- A nonempty join frame rejects. An empty HELLO_RSP rejects for remote
  associations, matching the pinned source, while the local APP_PING path keeps
  its existing empty-response behavior.
- Empty decoded frames are skipped only in the established encrypted call state,
  as the source does. No HELLO is sent before the wallet join.

## Deliberate bounded substitutions

The pinned Dart source accepts arbitrary raw reflector ID bytes while building
the URI and only later constrains the integer to `mwaMaxReflectorId`. Rust keeps
the existing [`ReflectorId`](mobile-wallet-protocol.md) JS-safe integer type and
rejects wider raw bytes instead of weakening it. The URI therefore carries the
canonical big-endian integer token; leading zero bytes in the payload resolve to
the same wallet-side ID but are not reproduced verbatim.

Dart retries reflector connections on the pinned delay schedule. This Rust path
performs exactly one caller-initiated connection and reports the typed failure;
retry policy stays with the application. Dart also decodes any WebSocket payload
type regardless of the selected subprotocol; this path enforces the selected
binary or base64 frame type and rejects an unselected form, as required for the
bounded transport.

## Verification

An owned loopback TLS reflector fixture uses only the repository's disposable
public test DER files: trust is restricted to `fixtures/ca.der`, and the leaf
identifies `localhost`. The fixture asserts the exact `/reflect` path, exact
offered protocol list, absence of cookie/authorization headers, and the selected
protocol. It drives both binary and base64 sessions through reflector ID, pending
URI, empty join, verified HELLO signature, encrypted correlated capability
response, encrypted wallet rejection, and graceful close.

Negative native tests cover wrong CA, wrong destination name, missing/unknown/
multiple subprotocols, unsolicited extensions, malformed/truncated/overflowed/
wide/unsafe IDs, wrong frame types, invalid base64, oversized frames, oversized
base64 wire encoding, nonempty joins, empty HELLO_RSP, stalled connection/join/
HELLO deadlines, dropping a pending association, dropping a join future, and
dropping a borrowed active call with observed peer EOF. All fixture tasks and
joins have finite bounds.

Executed with the pinned Rust 1.99.1 toolchain:

```sh
cargo test --locked -p solana_kit_native --features mwa --lib mwa --offline
```

After review, the focused target passes 26 tests, including a regression that
first failed for an invalid approval deadline and now checks zero and
unrepresentable values at both public connection entry points before TCP I/O.
The same focused target passed on Rust 1.89.0,
and the all-feature native target, workspace tests, strict Clippy, warning-denied
Rustdoc, formatting, style and dependency gates passed locally.

## Limits

This is owned loopback TLS evidence, not a public reflector connection and not an
installed-wallet journey. No public reflector or RPC endpoint, installed external
wallet, device or user account was used. Android launch/approval and browser
association remain separate caller/platform responsibilities.

The no-background-task contract concerns SDK-owned work. Hostname resolution
uses Tokio's OS resolver path; dropping the connection future does not prove
termination of an operating-system resolver callback. The fixture uses only
`localhost` and makes no arbitrary-DNS cancellation claim.
