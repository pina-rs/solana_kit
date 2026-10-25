# Provider REST ownership

This source checkpoint is exclusively for off-chain application clients. On-chain
programs use Pina or official Solana program libraries. Compilation, formatting,
runtime checks and declaration parity remain pending.

`solana_kit_client::rest` owns one request contract. `Transport<D>` accepts a
`Request<D>` and returns the existing `Reply` or `TransportError` through a
concrete GAT future. There is no universal Send requirement. Helius owns its six
`Service` slots. Jupiter and Hermes each own a distinct single-destination marker.
One local owner can implement both provider capabilities.

`Body::Absent`, `Body::Json` and `Body::Form` bind byte presence to media type.
JSON null remains a present document. Ordered decoded path segments and query
pairs remain ordered. Adapters percent encode each component independently.
Decoded `.` and `..` segments are denied because browser URL parsing normalizes
even percent-encoded dot segments. A caller-approved root's fixed query remains
first, followed by the ordered operation query. Duplicate keys are preserved.

`Budgets` checks bytes, container depth, aggregate values and keys, and encoded
string lengths before typed allocation. It reuses the client's existing unique
object-key visitor. Provider errors retain their existing budget distinctions,
status, bounded body and schema cause. There is one capped JSON writer. No third
duplicate-key parser or RPC envelope rewrite is introduced.

The alternative transport-associated destination would prevent the real outside
consumer's one Rc owner from implementing both Jupiter and Hermes. Generic
`Transport<D>` preserves that use. A universal provider enum would put independent
origin policy into the client and require unrelated providers to coordinate.

`NativeRest<D>` routes to existing `NativeHttp` owners. The existing exchange still
owns its socket, TLS, Hyper driver, deadline and bounded collection inline.
`BrowserRest<D>` routes to existing Window Fetch owners and their abort controller,
timer and streamed receive boundary. Fetch rejects GET bodies before I/O.
`WasiRest<D>` uses the existing Preview2 HTTP resources and caller-driven Reactor.
Preview1 has no networking adapter. Original fixed-POST RPC entry points remain.

`GuestRest<D>` resolves fixed named host services through actual upstream Calls.
The host installs `NativeRest<()>` under additive method1. The old method0 wire
and all socket IDs remain unchanged. A bounded versioned request carries method,
body representation, segments, ordered pairs and reply limit. It carries no URL,
headers, TLS roots or credentials. `HttpDriver` retains its original job registry,
owner/interpreter/Bridge completion identities and cancellation accounting.
Capacity remains occupied until native completion is joined. The application
routes Events and awaits shutdown before unloading its host. Drop only aborts.

The private provider consumer's `rest::journey` calls actual Helius, Jupiter and
Hermes clients with Rc across awaits. Concrete browser, Preview2 and guest
functions consume the same journey. Native peer source covers literal methods,
encoding, authentication isolation, body presence, chunk bounds and cancellation.
These bodies have not executed. Actual browser, Preview2 and production WasmHost
driver replay, current/MSRV graphs, quality gates and independent source review
remain required. Every canonical provider declaration remains pending.
