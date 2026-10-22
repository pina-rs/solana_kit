# Typed RPC foundation and design evidence

The public solana_kit_client implements three typed off-chain RPC operations;
verification/rpc-design retains private executable design controls over that same
implementation. Solana Kit must not be used for on-chain programs;
use Pina or the official Solana program libraries there.

## Grounding and worklist

- Ground: done. Read the pinned Dart catalog, request builders, convenience layer,
  response models, boundary experiments, and committed architecture.
- Sketch: compare concrete GAT exchange futures with a local erased future.
- Agree: use executable evidence; no human approval checkpoint is required.
- Implement: typed balance, blockhash and base64 account paths only.
- Scrap: discard the erased boundary if it prevents native Send propagation.

The public caller shape is `client.call(Balance(address, config))`.
A single client owns protocol framing, response correlation and typed decoding.
Adapters own endpoints, authentication, streaming body bounds and cancellation.
A request carries its response type; callers never choose an arbitrary return
schema for a catalog method. Request IDs belong to one client owner, not adapters.
The client allocates IDs from one per-client AtomicU32 counter. Concurrent calls
reserve unique IDs with checked advancement; exhaustion is an explicit error,
never a wrap or reuse. No global counter, OS entropy, timer or runtime is needed.

## Candidate contracts

A: `Transport::Exchange<'a>: Future<Output = Result<HttpReply, TransportError>>`;
`Client<T>::call<R: Method>(&self, request) -> R::Response` is an ordinary
async method. The native generic caller requires `T: Send + Sync`, each associated
exchange future Send, and the method and response Send. Local adapters retain Rc
through Pending without a universal Send bound or runtime dependency.

B: `ErasedTransport::exchange(...) -> Pin<Box<dyn Future<...> + '_>>` reduces
implementation syntax, but erases Send even for native adapters. A separate Send
trait or duplicate client path would be required. Execute this candidate locally
and require a compile-negative native spawn control before choosing A.

Neither candidate starts a runtime or detaches work. Both share one protocol
codec, so the comparison changes the async boundary rather than JSON semantics.

## Decision and ownership

Choose candidate A. The executed generic spawn helper requires `T: Send + Sync`,
`for<'a> T::Exchange<'a>: Send`, `R: Method + Send`, and `R::Response: Send`.
It invokes the same `Client<T>::call` that the local Rc test invokes. The
alternative loses Send at the erased exchange, as its E0277 doctest demonstrates.
No unsafe Send assertion or duplicated request algorithm is needed. This is a
native proof; JSON behavior in browser/WASI/interpreted guests remains to execute
when real public adapters exist.

Keep one `solana_kit_client` crate for typed RPC, HTTP contracts, pubsub,
confirmation and account fetching. It owns request identity allocation, framing,
JSON schemas, context and protocol errors. Built-in operations fix their response
types; callers cannot substitute a schema. The open typed Method trait also lets
provider integrations supply their own params and result decoder while reusing
the central envelope. Serde is part of this explicit client extension contract;
core domain types remain Kit-owned. Opaque remote data retains exact RawValue
text, including whitespace, after duplicate-key and budget validation. IDs reserve 1 through u32::MAX - 1, remain exactly representable
in JavaScript, and fail with IdExhausted before transport access at the limit.
The native concurrency test observes 32 distinct replies from one shared client;
the exhaustion control never calls the transport. Cross-target compilation proves
the selected atomic is available; runtime execution on Wasm remains pending.

Adapters remain separate `solana_kit_native`, `solana_kit_browser`,
`solana_kit_wasi` and `solana_kit_leptoskit` crates. Each adapter owns an explicitly
chosen endpoint and caller-provided authentication; neither creates a runtime,
reads ambient credentials nor retries automatically. A bounded request transfers
owned JSON bytes and the response limit. Adapters must enforce that limit while
receiving, before growing the body allocation; the client checks again before
parsing. Cancellation drops owned operations; a separately owned driver must
explicitly abort its task. Dropping a Tokio JoinHandle detaches it.

The public error contract needs typed capability/denial, transport source, HTTP
status, deadline, body/depth limit, malformed JSON, correlation, protocol envelope,
remote JSON-RPC and typed domain/schema variants. Preserve remote code/message
and bounded unknown data, including missing versus explicit null. Do not treat
simulation `value.err` as a JSON-RPC framing error, or infer success from HTTP 200.
The public TransportError categorizes unavailable, denied, deadline, cancellation,
I/O, body/header limit and invalid HTTP failures while retaining adapter causes.

## JSON boundaries and scope

The pilot checks exact u64 text through u64::MAX without any f64 round trip,
requires numeric matching IDs and JSON-RPC 2.0, rejects duplicate envelope fields,
requires exactly one result/error, and distinguishes absent fields from JSON null.
It rejects missing context/slot/value, invalid base58 address/hash, wrong account
encoding labels, wrong HTTP status, excess bytes, nesting depth and trailing JSON.
A lexical container guard runs before serde allocations; serde then validates
syntax. No recursion-limit disabling, JavaScript JSON.parse, silent zero slot,
empty account or fallback balance is used.

The public base64 account path validates canonical base64 bytes and the reported
full allocation size, returning core present/missing account types with the
requested address. Slices, zstd and jsonParsed remain unsupported. Those are
explicit remaining operations, not a claim that arbitrary serde_json establishes
a complete schema. In the public API, encoding and transaction-details choices
must select explicit result variants; parsed account fallback and absent versus
null transaction metadata need distinct owned variants. Use confirmed/finalized
commitment types where processed is invalid, and a mint-or-program enum for token
filters. Do not expose the reference's untyped comparison/filter maps as the
finished Rust API. Integer widths and nullable members require per-method oracles.

The loopback adapter is a fixture for a single success HTTP/1 response and owned
socket drop, not a production HTTP parser. It has no TLS, pooling, redirects,
chunking, authentication, timeout policy, WebSocket, or installed-host bridge.
The retained-call control observes no EOF before abort; dropping the actual
pending call then produces EOF. Neither this nor native compilation proves a
browser, WASI, LeptosKit or mobile networking journey.

## Exact catalog and implementation units

[rpc-catalog.tsv](../parity/rpc-catalog.tsv) contains all 54 literal methods from
Dart revision `0138bb956d2d81ccc0c56edc602401c0df2f2b46`, their exact parameter
builder IDs, all canonical declarations in each defining file, source SHA-256,
request signature and declared field summaries. Literal values have no invented
canonical declaration IDs. The list constants themselves remain at the IDs in
capabilities.tsv. This derived worklist never edits that canonical inventory.
Only 14 methods have convenience wrappers; 40 require new typed caller paths.
All response type names in this worklist are proposals. The three marked pilots
have public typed implementations, with getAccountInfo limited to full base64.
Runtime adapter verification and the remaining 51 methods are still pending.

| Unit                             | Catalog methods | Verification obligation                                                                                            |
| -------------------------------- | --------------: | ------------------------------------------------------------------------------------------------------------------ |
| Protocol owner and account reads |               7 | Context, nullable accounts, all encodings/slices, program-account withContext, minimum rent and blockhash validity |
| Node and epoch reads             |              19 | Each scalar/record/array, optional version fields, Alpenglow method-unavailable versus supported-null              |
| Token reads                      |               5 | Exact raw amount strings, decimals, nullable uiAmount, owner/delegate filters and encoding variants                |
| Economics and voting             |               8 | Context, per-address null rewards, bounded rates, supply and voting schemas                                        |
| History                          |              10 | Nullable blocks/transactions/times, details and encoding variants, ranges, pagination and errors                   |
| Submission and simulation        |               5 | Signature validation, message fee null, full simulation schema, uncertain-send semantics and no silent retry       |

Split these families into cohesive smaller PRs when schema size warrants it.
Each method requires exact request vectors including omission/null positional
policy, independent successful responses, numeric boundaries, nullable/missing
cases, remote errors and malformed/domain controls. The existing 14 convenience
methods do not define the finish line. Submission policy remains separate from
idempotent reads; preflight failure is not a successful send.

After the protocol owner is public, verify each adapter with the same typed
fixtures and real loopback HTTP success/status/error/deadline/body-limit/drop
journeys. Run browser fetch in Chrome, named WASI HTTP capabilities in Wasmtime,
and actual LeptosKit guest-to-host HTTP with denied capability/destination,
quiet cancellation, unload and late replies. Native must preserve generic Send;
browser/guest must execute local futures. TLS/WebSocket/subscription lifecycle
and the later desktop/mobile journeys need their own evidence.

## Protocol grounding

The three pilot shapes were checked against primary documentation on 2026-10-22:
[getBalance](https://solana.com/docs/rpc/http/getbalance),
[getLatestBlockhash](https://solana.com/docs/rpc/http/getlatestblockhash),
[getAccountInfo](https://solana.com/docs/rpc/http/getaccountinfo), and the
[JSON-RPC specification](https://www.jsonrpc.org/specification).
The [shared JSON structures](https://solana.com/docs/rpc/json-structures) identify
encoding-dependent and nullable fields that the history units must verify.
[getAgGenesisCert](https://solana.com/docs/rpc/http/getaggenesiscert) documents the
Agave 4.3 capability boundary. The getTransactionsForAddress documentation URL
was inaccessible during this audit; its typed response/config plan is grounded
in pinned Dart source and still requires an authoritative upstream schema pin
before implementation acceptance. The other 49 methods are source-audited
planning entries, not an independently verified exhaustive protocol schema.
