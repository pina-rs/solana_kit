# Changelog

## Unreleased

- Add checked codec size metadata and validated encoder/decoder pairing.
- Preserve complete-operation overrides through borrowed codecs.
- Validate both halves when importing a 64-byte Ed25519 keypair.

Solana Kit supports off-chain application clients. On-chain programs should use
Pina or official Solana program libraries.

## [0.2.1](https://github.com/pina-rs/solana_kit/releases/tag/v0.2.1) (2026-11-09)

Grouped release for `sdk`.

### Fixes

#### Publish the Codama renderer with the SDK group.

_Packages:_ 🟢 _codama_renderers_rust_solana_kit_

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #186](https://github.com/pina-rs/solana_kit/pull/186)

Add `codama_renderers_rust_solana_kit` to the `sdk` MonoChange group so it
shares the primary `v*` version and release tag. Qualify and upload the package
from `.github/workflows/publish.yml` beside the crates.io publish path.

## [0.2.0](https://github.com/pina-rs/solana_kit/releases/tag/v0.2.0) (2026-11-08)

Grouped release for `sdk`.

### Breaking changes

#### solana_kit_core: portable Solana types and wire formats

_Packages:_ 🔴 _solana_kit_core_

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #183](https://github.com/pina-rs/solana_kit/pull/183)

The foundational off-chain crate: validated addresses, hashes, signatures,
ordered instructions, account roles, bounded binary codecs, account snapshots,
sysvar layouts, program errors, exact fixed points, immutable legacy/v0/v1
transactions, and off-chain application messages.

The default graph depends only on `bs58`. Optional features add SHA-256,
validated program-derived addresses, deterministic Ed25519 signing and strict
verification, original Keccak-256, arbitrary-width integer codecs, and text
codecs. The crate is `no_std + alloc`, owns no executor or entropy source, and
never claims on-chain support.

#### solana_kit_client: executor-neutral typed RPC

_Packages:_ 🔴 _solana_kit_client_

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #183](https://github.com/pina-rs/solana_kit/pull/183)

Typed balance, blockhash, account, token, node metadata, economics, history,
submission, and simulation operations through one executor-neutral protocol
owner. Native and local futures share the same client without a runtime,
network, JavaScript, or ambient-randomness dependency.

Optional features add version-aware resource estimation, caller-driven reactive
stores, slot-tracking streams, and typed subscription families. Callers own the
transport, deadlines, cancellation, and any spawned driver.

#### solana_kit_native: Tokio HTTP, TLS, and WebSocket transports

_Packages:_ 🔴 _solana_kit_native_

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #183](https://github.com/pina-rs/solana_kit/pull/183)

Native HTTP/1 and text WebSocket transports for off-chain Solana applications.
`NativeHttp` implements the public client transport with caller-owned Tokio
execution, explicit deadlines, cancellation, authentication, and verified TLS.

Optional key support adds explicit exportable-key generation and intentional
Solana-keygen JSON backup with atomic publication and typed cleanup failures.
The optional Mobile Wallet Adapter transport keeps association, encryption, and
cancellation caller-owned.

#### solana_kit_signers: executor-neutral signing capabilities

_Packages:_ 🔴 _solana_kit_signers_

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #183](https://github.com/pina-rs/solana_kit/pull/183)

Portable asynchronous signing contracts for message, transaction, modifying,
partial, and sending signers. The crate is `no_std + alloc` and has no runtime,
entropy, filesystem, or network dependency.

Each logical resource gets a `SignerCapability`; `TransactionSigners`
deduplicates repeated handles, rejects two handles claiming one address, and
preserves local or native futures. Every call receives its own configuration,
including `aborted` and `min_context_slot`, unchanged across composed pipelines.

#### solana_kit_browser: Window HTTP and WebSocket transports

_Packages:_ 🔴 _solana_kit_browser_

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #183](https://github.com/pina-rs/solana_kit/pull/183)

Explicit browser adapters for the executor-neutral client. `BrowserHttp`
implements `Transport` with Window Fetch and local futures, bounded bodies,
deadlines, cancellation, redirect rejection, and no ambient credentials.

`BrowserSubscriptionSocket` adapts the maintained Window WebSocket to the shared
subscription session, retaining event wakers and bounded timers between polls.
Imports exist only for `wasm32-unknown-unknown`; importing the crate is the
explicit opt-in to browser JavaScript.

#### solana_kit_leptoskit: interpreted LeptosKit adapters

_Packages:_ 🔴 _solana_kit_leptoskit_

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #183](https://github.com/pina-rs/solana_kit/pull/183)

Guest and host adapters for interpreted LeptosKit applications. `GuestHttp`
connects the ordinary public client to named host services; `HttpDriver` and
`HttpHost` install fixed destinations, TLS roots, grants, deadlines, and byte
ceilings in the native host.

Every guest entry point drains unrelated requests and cancellations, while
completion delivery retains no interpreter borrow and wakes the real guest. The
optional wallet adapter exposes the portable MWA protocol to installed hosts.

#### solana_kit_wasi: WASI Preview 2 HTTP

_Packages:_ 🔴 _solana_kit_wasi_

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #183](https://github.com/pina-rs/solana_kit/pull/183)

`WasiHttp` implements the public client transport with caller-driven local
futures for `wasm32-wasip2`. A caller-owned `Reactor` registers pollables,
selects readiness and deadlines, and wakes the guest without a global executor
or detached task.

The adapter follows the pinned `wasi:http/io/clocks@0.2.12` contract, preserves
explicit deadlines and body budgets, and leaves DNS, TLS, proxy, credential, and
destination policy to the embedding host.

## sdk [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/sdk/v0.1.1) (2026-11-05)

Grouped release for `sdk`.

### Features

#### Add accounts, sysvars and typed program failures

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [3c2b8f4](https://github.com/pina-rs/solana_kit/commit/3c2b8f47231d4fd1f2e466a0c0269cc0143395c3)

Decode account snapshots with explicit missing states, owner validation and
metadata preservation. Read and write all nine pinned sysvar layouts with
bounded allocations and strict full decoding, including historical blockhash
fees. Preserve program error codes, instruction positions and original log facts.
These deterministic APIs serve off-chain application clients and perform no RPC.

#### Await caller-owned message construction during planning

_Packages:_ 🟠 _solana_kit_core_, 🟠 _solana_kit_client_

_Owner:_ Ifiok Jr. · _Introduced in:_ [77dbbd3](https://github.com/pina-rs/solana_kit/commit/77dbbd3cfaf94745a1a1666bc65eeff230275a8c)

Add concrete asynchronous construction and update futures at actual candidate
boundaries. Preserve the shared synchronous planning traversal, exact packing,
transactional failures, local futures and conditional native Send. Execution
boundaries await planning before signing or sending. Publication remains disabled.

#### Add bounded deterministic binary codecs

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [688db72](https://github.com/pina-rs/solana_kit/commit/688db722e353bdfcb94fe18b68c98a2ff3dc0280)

Encode exact-width integers through 256 bits, IEEE 754 floats, canonical
short-u16, raw bytes, strict UTF-8, sequences, tuples, explicit option layouts,
and constant or length framing. Full decoding rejects trailing data; prefix
decoding returns the next offset. Caller-selected byte and allocation budgets
apply across nested decoders. The codec layer adds no runtime dependencies.

The shared portable conformance suite grows to 31 cases. This records release
intent for off-chain client applications; component publication stays disabled.

#### Estimate and tune transaction resource limits

_Packages:_ 🟠 _solana_kit_core_, 🟠 _solana_kit_client_

_Owner:_ Ifiok Jr. · _Introduced in:_ [367a812](https://github.com/pina-rs/solana_kit/commit/367a812bad41f06f2faa36e43ed21b5a03dd9202)

Add optional client resource estimation over caller-owned RPC transports. Preserve
version-specific budgets, nonce lifetimes, explicit limits, observed usage, and
original errors. Add pure removal of version-one resource requests.

#### Expose directional codec size declarations and checked encoder/decoder pairing.

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [15c77df](https://github.com/pina-rs/solana_kit/commit/15c77df15e424f87432cc6d82dd0f69371af7bb4)

Keep metadata separate from byte operations and preserve each selected owner's
operation overrides. Component publication remains disabled in monochange.toml.

#### Add collection termination and configurable optional layouts

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [839b632](https://github.com/pina-rs/solana_kit/commit/839b63254854acb12b106af49493a9261a05a962)

Decode collection markers at item boundaries with explicit required, optional,
or omitted policies. Configure absent-value bytes and numeric boolean storage
using ordinary Rust Option values. Permit exhausted count prefixes explicitly
while retaining errors for partial prefixes and shared allocation limits.

#### Extend portable deterministic client contracts

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [cbe40e1](https://github.com/pina-rs/solana_kit/commit/cbe40e17c6be956cb09434dcb8d64adbed7e2600)

Add untagged codec selection, contextual transforms, runtime tuple and record
schemas, explicit UTF-16 radix codecs, optional arbitrary-width integers and
Keccak, public address collation, lookup metadata and canonical address constants.

#### Add explicit durable nonce lifetime metadata, deterministic advance preparation,

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [e83d103](https://github.com/pina-rs/solana_kit/commit/e83d103b4af2d3a43ebbc8a687fd5adea3362792)

compiler prerequisite validation and structural first-advance inspection for
off-chain clients. Received lifetimes remain unresolved.

#### Add optional exact decimal and binary fixed points

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [f47bd7b](https://github.com/pina-rs/solana_kit/commit/f47bd7b81f18d7202a0275f593364fda355061dc)

The `fixed-points` feature adds validated shapes, checked arithmetic, explicit
rounding, exact decimal parsing and formatting, rescaling, comparisons, ranges,
and bounded little/big-endian codecs using optional no_std BigInt primitives.
The default graph remains bs58-only. Widths and scales are bounded at 4096,
input at 8192 bytes, and ratio/scalar operands at 32768 magnitude bits.

The portable suite adds three fixed-point cases. Custom callback constructor
parity and extreme lossy float conversion parity remain explicitly open in the
126-entry capability receipt; this is not complete package parity.

Lossy presentation conversion rejects nonfinite results explicitly, preserves
finite underflow, and avoids false overflow for wide finite ratios by parsing
the exact decimal scientific representation with Rust's core float parser.

#### Add an explicit exportable key owner with zeroizing backups and caller-selected entropy.

_Packages:_ 🟠 _solana_kit_core_, 🟠 _solana_kit_native_, 🟠 _solana_kit_browser_

_Owner:_ Ifiok Jr. · _Introduced in:_ [eb1e6ac](https://github.com/pina-rs/solana_kit/commit/eb1e6ac886f2693c19894268efb6cf88d9d8c97a)

Native applications can opt into secure-provider entropy and private, atomic backups that never overwrite an existing destination.
Browser applications can opt into caller-owned Web Crypto entropy without a fallback random source.
Ordinary signing keys retain their non-exporting API.

#### Add portable instruction planning and caller-owned execution

_Packages:_ 🟠 _solana_kit_core_, 🟠 _solana_kit_client_

_Owner:_ Ifiok Jr. · _Introduced in:_ [77dbbd3](https://github.com/pina-rs/solana_kit/commit/77dbbd3cfaf94745a1a1666bc65eeff230275a8c)

Build sequential, parallel and indivisible trees with exact compiler-backed
packing. Retain ordered transaction results, original errors and partial context
through cancellation and signing or sending failures. Applications supply the
builder, signer, sender and executor. Publication remains disabled.

#### Add key grinding, constant-time equality and decompilation

_Packages:_ 🟠 _solana_kit_core_, 🟠 _solana_kit_signers_

_Owner:_ Ifiok Jr. · _Introduced in:_ [e41c7ba](https://github.com/pina-rs/solana_kit/commit/e41c7ba31d69df35e08ba1ce9245e9a325e41457)

Add caller-owned, finite-budget key grinding with validated base58 prefix and
suffix literals, predicate composition, distinct entropy and exhaustion errors,
and audited constant-time slice equality through `subtle`.

Add typed `Message::decompile` and `DecompiledMessage`: caller-owned lookup
snapshots preserve version, payer, instructions, duplicates, privileges, table
provenance, loaded ordering and V1 configuration without inventing lifetime
metadata. Recompilation reuses the existing compiler; canonical messages roundtrip
to their exact signed bytes. Grind signers consume export capability before they
are returned.

#### Import 64-byte Ed25519 keypairs through the existing optional cryptographic

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [15c77df](https://github.com/pina-rs/solana_kit/commit/15c77df15e424f87432cc6d82dd0f69371af7bb4)

owner, rejecting incorrect lengths and public keys that do not match the seed.
Keep caller buffers unchanged and retain zeroizing ownership without secret
export. Component publication remains disabled in monochange.toml.

#### Add immutable legacy and v0 transaction construction

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [b42d01e](https://github.com/pina-rs/solana_kit/commit/b42d01e41c7aa3843cda011c3602abdd8c159565)

Compile instructions with explicit blockhash expiry metadata, deterministic
privilege merging and lookup tables. Decode bounded external wire, preserve exact
signed bytes, keep ordered signature slots immutable, and encode packets within
the legacy/v0 packet limit. Optional deterministic crypto signs and verifies
those frozen bytes. This is an off-chain client SDK.

The phase 3 ledger keeps v1, durable nonces, off-chain messages, introspection and
signer capabilities explicitly pending. This slice does not claim full phase 3
parity. Publication remains disabled.

- **Add an explicitly configured Tokio HTTP/1 and verified TLS transport with bounded.** response parsing, streaming body limits, deadlines and owned cancellation.
  _Packages:_ 🟢 _solana_kit_core_, 🟢 _solana_kit_client_, 🟠 _solana_kit_native_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [064052a](https://github.com/pina-rs/solana_kit/commit/064052aa6b095a1fa0d5ab501039251d58773329)

#### Add bounded numeric and string codec capabilities

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [bb5055e](https://github.com/pina-rs/solana_kit/commit/bb5055e2feb9775bfee127b0bbb5316e472040e1)

Provide exact 256-bit numeric text conversion, explicit/default numeric byte
order, integer radix and bit-reslicing string codecs, hexadecimal pairs and odd
nibbles, configured strict/lossy UTF-8, and bounded null helpers. The optional
`text-codecs` feature adds audited Base64 conversion with pinned Dart leniency.
Default and existing capability dependency graphs remain unchanged. This is an
off-chain client SDK; no component publication is enabled.

#### Add bounded off-chain application message signing

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [cd11a82](https://github.com/pina-rs/solana_kit/commit/cd11a8292b51a15f75090505f40decefc2b56df0)

Construct and strictly decode immutable V0 and V1 application messages with
explicit byte and allocation budgets. Preserve exact message bytes, encode
ordered optional signature slots, and sign or verify deterministically with the
optional crypto feature. V0 content limits remain distinct from transaction
packet limits. Publication remains disabled.

#### Preserve insertion order in map and set codecs

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [cbe40e1](https://github.com/pina-rs/solana_kit/commit/cbe40e17c6be956cb09434dcb8d64adbed7e2600)

Encode owned ordered maps and sets through existing sequence framing. Decoded
duplicate keys replace values in their original position; duplicate set values
retain their first appearance. Original wire items retain their shared allocation
claims, and normalization reuses the decoded storage without cloning values.

#### Add portable primitives and deterministic cryptography

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [229f70b](https://github.com/pina-rs/solana_kit/commit/229f70b6748e7195b3f7d74ff6f8448679f491ba)

Provide exact-length base58 addresses, hashes, signatures, account roles, and
owned ordered instructions without host dependencies. The optional crypto
feature adds SHA-256, seed/PDA derivation with curve and seed validation, and
deterministic Ed25519 signing with strict verification.

The initial implementation has native behavioral evidence and a shared,
host-neutral 25-case conformance suite. Other runtime execution is verified in
separate portability units. This changeset records release intent and does not
authorize publication.

#### Add opt-in transactional codec reads

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [3335649](https://github.com/pina-rs/solana_kit/commit/3335649888e9b6400483deaa8881d920354f5724)

Allow a decoder to commit cursor movement and allocation claims only when its
closure succeeds. Failed reads restore both values while external closure effects
remain visible. Ordinary frame semantics remain unchanged.

The private shared generator now represents tagged, framed and TLV data layouts
from complete pinned inputs. Exact generation goldens and independent malformed
encoding controls retain their bounded scope. This is off-chain client code.

#### Compose typed bounded client codecs

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [e2efcd1](https://github.com/pina-rs/solana_kit/commit/e2efcd1c4ae3238135cb0b05339b707191c407de)

Add fallible value conversion, independent encoder/decoder pairing, checked value
observers, deterministic padding and exact sentinel frames. Pack boolean bits,
select literal values and tagged binary alternatives, prefix sequence counts and
read dependent typed pairs with the existing shared allocation budget.

These capabilities support off-chain clients. They add no dependencies or host
requirements. The scoped parity receipt records source differences and remaining
families. Components remain unpublished.

#### Add immutable version-one client transactions

_Packages:_ 🟠 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [a6384b5](https://github.com/pina-rs/solana_kit/commit/a6384b576c07126a90b396940e50374b6a09180a)

Compile and decode version-one messages with typed inline configuration, exact
message-first signature envelopes, and the inclusive 4096-byte packet limit.
Preserve received signed bytes and keep legacy/v0's 1232-byte contracts. Optional
crypto signs and verifies the frozen message. Local validator feature support is
explicit evidence, not inferred from a node version. This SDK is exclusively for
off-chain application code; use Pina or official Solana program libraries on chain.

Durable nonce lifecycle, general signer capabilities and complete Phase 3 remain
outstanding. Publication stays disabled.

- **Add caller-driven reactive action and stream stores.** Opt into cached action and stream snapshots, scoped cancellation, local callbacks, owned drivers and a latest-accepted-value observation bridge. The client starts no executor or background task.
  _Packages:_ 🟠 _solana_kit_client_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [462fccf](https://github.com/pina-rs/solana_kit/commit/462fccffdb2cf99038d4e59cb47ac596c913de7f)
- **Add composed slot-tracking reactive stores.** Combine fresh initial and subscription sources with retained slot context, scoped shared-failure cancellation, local callbacks and caller-owned drivers. Stale refreshes retain their existing contextual Arc without cloning user values.
  _Packages:_ 🟠 _solana_kit_client_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [4d00c4a](https://github.com/pina-rs/solana_kit/commit/4d00c4aefc6929559cbef8d6bac7fd67b77172b5)
- **Add typed parsed account projection.** Project node `jsonParsed` token, stake, vote, nonce, address lookup table, BPF upgradeable loader, config and sysvar payloads behind the opt-in `parsed-accounts` feature. Exact integers, opaque extension and config payloads, bounded admission and explicit unknown variants keep every original raw account untouched.
  _Packages:_ 🟠 _solana_kit_client_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [22e7416](https://github.com/pina-rs/solana_kit/commit/22e74168c57b983f5d3e62e9e839d1d3e5b0fe5c)

#### Retain original browser HTTP JavaScript failures behind redacted diagnostics.

_Packages:_ 🟠 _solana_kit_client_, 🟢 _solana_kit_browser_

_Owner:_ Ifiok Jr. · _Introduced in:_ [7813ae4](https://github.com/pina-rs/solana_kit/commit/7813ae404422e26e0d686d8f015a86941be187ab)

Permit local error sources on wasm32-unknown-unknown while preserving native and
WASI Send and Sync error bounds. Component publishing remains disabled.

- **Compose shared confirmation with fallible clocks and direct Preview2 subscription.** sockets while retaining the shared REST transports and framed Session owner.
  _Packages:_ 🟠 _solana_kit_client_, 🟠 _solana_kit_native_, 🟠 _solana_kit_browser_, 🟠 _solana_kit_wasi_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [7813ae4](https://github.com/pina-rs/solana_kit/commit/7813ae404422e26e0d686d8f015a86941be187ab)
- **Share provider REST requests and bounded JSON admission in the client.** Extend the existing platform HTTP owners with explicit named routes and body representations.
  _Packages:_ 🟠 _solana_kit_client_, 🟠 _solana_kit_native_, 🟠 _solana_kit_browser_, 🟠 _solana_kit_leptoskit_, 🟠 _solana_kit_wasi_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [7813ae4](https://github.com/pina-rs/solana_kit/commit/7813ae404422e26e0d686d8f015a86941be187ab)

#### Combine initial and subscribed RPC values by slot

_Packages:_ 🟠 _solana_kit_client_

_Owner:_ Ifiok Jr. · _Introduced in:_ [dc0e6b8](https://github.com/pina-rs/solana_kit/commit/dc0e6b81c00a7f505c220f26b5405a710382b8e2)

Add an optional reactive stream adapter that preserves complete RPC contexts,
accepts equal slots, rejects older values, and owns both source lifetimes. Retain
nonterminal typed failures with bounded pending outputs and conditional Send.

#### Add bounded caller-owned RPC subscriptions

_Packages:_ 🟠 _solana_kit_client_

_Owner:_ Ifiok Jr. · _Introduced in:_ [66288ed](https://github.com/pina-rs/solana_kit/commit/66288ed3788ecdc64ff91e72c6e5d12b7a6f836b)

Drive all nine typed subscription families through explicit text-frame I/O and
caller ticks. Bound registrations, early notifications, retained bytes, and slow
readers. Preserve acknowledgement failures, cancellation, and generation gaps.
No socket adapter or automatic reconnect is included. Publication stays disabled.

#### Add explicit transaction confirmation policy and caller-owned observation with

_Packages:_ 🟠 _solana_kit_client_

_Owner:_ Ifiok Jr. · _Introduced in:_ [b9f5349](https://github.com/pina-rs/solana_kit/commit/b9f5349ee057473effa3706ccff7c3288f406297)

bounded awaited cleanup. Preserve exact transaction bytes and reported signature
checks when submitting. Subscription factories and authenticated nonce RPC
observations remain pending; no clock, executor or retry policy is implicit.

#### Inspect typed transaction results and validated core messages through ordered

_Packages:_ 🟠 _solana_kit_client_

_Owner:_ Ifiok Jr. · _Introduced in:_ [3da7b6a](https://github.com/pina-rs/solana_kit/commit/3da7b6a6a083a7e0880bfb0e81512325949f6d5e)

account roles, raw or parsed instructions, and interleaved inner execution traces.
Preserve exact binary transactions and absent execution metadata while rejecting
inconsistent headers, signatures, lookup resolutions and instruction references.

#### Add executor-neutral typed RPC foundation

_Packages:_ 🟠 _solana_kit_client_

_Owner:_ Ifiok Jr. · _Introduced in:_ [07e5a4f](https://github.com/pina-rs/solana_kit/commit/07e5a4f3300ef34f6a09e45291769d381981fc61)

Call getBalance, getLatestBlockhash and full-base64 getAccountInfo through one
bounded, correlated JSON-RPC client. Preserve exact integers, account metadata,
explicit absence, detailed failures and concrete native or local adapter futures.
Provider methods reuse the same envelope through an open typed extension trait.
This off-chain client slice leaves the remaining 51 catalog methods and network
adapters pending. Component publication remains disabled.

#### Deploy programs from ordered explicit artifact roots

_Packages:_ 🟠 _solana_kit_native_

_Owner:_ Ifiok Jr. · _Introduced in:_ [670e480](https://github.com/pina-rs/solana_kit/commit/670e48069e6a9e4a444e72709f2a90f20e9faa56)

Select the first complete same-root program/keypair pair in caller order and snapshot
only its optional IDL. Verify a bounded keypair JSON file before program reads and
reuse sequential deployment with explicit limits, typed errors and cancellation.

#### Own explicit Surfpool children with bounded RPC lifecycle

_Packages:_ 🟠 _solana_kit_native_

_Owner:_ Ifiok Jr. · _Introduced in:_ [92dfd68](https://github.com/pina-rs/solana_kit/commit/92dfd68899f1ec6125f955415dffa454eab201f8)

Add a synchronous direct-child owner with caller-selected ports, paths, payer,
fresh genesis and explicit environment. Borrowed readiness and close futures retain
the handle on cancellation, cache reaped status and preserve typed shutdown causes.
Typed settings and runtime paths take precedence over stock environment overrides.
Caller-established OS containment and caller-owned runtime cleanup remain required.

#### Connect to an explicit remote MWA reflector

_Packages:_ 🟠 _solana_kit_native_

_Owner:_ Ifiok Jr. · _Introduced in:_ [028b8b8](https://github.com/pina-rs/solana_kit/commit/028b8b862b4a41c5c955dff19a00c21b218fe5ed)

Add a caller-owned two-stage remote association: connect once to an explicit
WSS reflector authority with selected TLS roots, validate its LEB128 reflector
ID, return a pending owner with the real remote association URI, then await the
wallet join before HELLO and the existing encrypted `NativeMobileWallet` path.
Offer only the pinned binary and base64 subprotocols, enforce one selected
protocol with no extensions, apply the encoded and decoded frame budgets, and
keep connection, join, operation and approval deadlines finite. No default
reflector, redirect, retry, background task or fallback transport is added.

Expose the existing remote reflector authority validation so a caller can reject
malformed authorities before opening a socket.

#### Add optional native Surfpool deployment from borrowed program bytes or an explicit

_Packages:_ 🟠 _solana_kit_native_

_Owner:_ Ifiok Jr. · _Introduced in:_ [ce133c6](https://github.com/pina-rs/solana_kit/commit/ce133c6e76908d8843bde2e16c337394ca68b009)

regular file path, with finite caller-selected read limits. Reuse sequential portable
program writes before opening the optional IDL file, preserve exact JSON fields, and
retain partial writes and typed causes on later failures or cancellation.
Preserve literal unknown JSON keys, including serde_json's internal token names,
without changing the parser's existing nesting ceiling.
Allow the permanent portable crate in SDK dependency manifests while keeping
normal implementation publishing disabled in Monochange configuration.

#### Expose the failed private-backup operation through KeyFileError::stage while

_Packages:_ 🟠 _solana_kit_native_, 🟢 _solana_kit_leptoskit_

_Owner:_ Ifiok Jr. · _Introduced in:_ [3aac1f4](https://github.com/pina-rs/solana_kit/commit/3aac1f44a2d696df68ed36224b4eacb69b9eafaf)

preserving the original I/O cause. Caller-supplied I/O conversions retain an
unknown stage, and library failures identify their exact operation.

Use an atomic no-replace rename for private backups on Linux and Android to avoid
Android's app hard-link restriction. Existing destinations remain untouched, and
a failed publication retains its original I/O cause and staging cleanup results.

Deliver the actual wallet service failure before waking the released lease so
failed backups reach the guest as native failures.

- **Add optional wallet view models, browser controller projections, Android Intent.** and entropy assets, and a caller-owned native binary Mobile Wallet Adapter.
  _Packages:_ 🟠 _solana_kit_native_, 🟠 _solana_kit_browser_, 🟠 _solana_kit_leptoskit_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [eb1e6ac](https://github.com/pina-rs/solana_kit/commit/eb1e6ac886f2693c19894268efb6cf88d9d8c97a)

#### Add owned browser Wallet Standard discovery, account authorization and Kit signing capabilities.

_Packages:_ 🟠 _solana_kit_signers_, 🟠 _solana_kit_browser_

_Owner:_ Ifiok Jr. · _Introduced in:_ [eb1e6ac](https://github.com/pina-rs/solana_kit/commit/eb1e6ac886f2693c19894268efb6cf88d9d8c97a)

Wallet metadata methods now return owned snapshots so changing backend state never requires an invalid borrowed reference.
Signer capabilities and local/native transaction compositions can preserve a caller-selected failure type while existing default errors retain their native Send behavior.
Browser failures retain their original JavaScript causes behind redacted diagnostics.

- **Add immutable bare signable messages, bounded whole-slice partial and modifying.** capabilities, explicit local/native composition, and a singular signing adapter.
  _Packages:_ 🟠 _solana_kit_signers_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [9c56e9d](https://github.com/pina-rs/solana_kit/commit/9c56e9ddbefb73196fdf1a8dea95e26a663a38ce)

#### Add explicit per-call `SignerConfig` and `TransactionSignerConfig` values to every

_Packages:_ 🟠 _solana_kit_signers_, 🟠 _solana_kit_browser_

_Owner:_ Ifiok Jr. · _Introduced in:_ [685fff1](https://github.com/pina-rs/solana_kit/commit/685fff156b1291de0edc3a801eb021a1cfea9c59)

message, partial, modifying and sending capability method, typed pipeline,
heterogeneous local/native composition and the singular message adapter. Aborted
calls fail with the existing cancellation error before backend or wallet effects.
Transaction backends receive the exact optional `min_context_slot` hint; the
browser wallet forwards it as Wallet Standard `options.minContextSlot`, omits it
for `None`, and rejects slots outside the JavaScript safe integer range before
prompting. Repository callers, tests, fixtures and documentation now pass the
configuration explicitly; `Default::default()` preserves prior behavior.

#### Add portable signer capability contracts

_Packages:_ 🟠 _solana_kit_signers_

_Owner:_ Ifiok Jr. · _Introduced in:_ [3f12064](https://github.com/pina-rs/solana_kit/commit/3f120649480308d60ea38c9665b0c3ac4557e099)

Add local asynchronous message, partial, modifying, and sending signer contracts
with immutable byte snapshots and deterministic keypair support. Keep submission caller controlled.

Pipeline callers create a `SignerCapability` handle for each logical resource.
Repeating one handle deduplicates, while distinct handles claiming one address
conflict even when their backends are zero-sized.

Automatically select composite roles across explicitly registered heterogeneous
backends, with local and native Send adapters. Add bounded ordered transaction
batches while preserving logical handle identity and signature snapshot checks.

#### Add owned browser HTTP transport

_Packages:_ 🟠 _solana_kit_browser_

_Owner:_ Ifiok Jr. · _Introduced in:_ [58329fd](https://github.com/pina-rs/solana_kit/commit/58329fd3ffb051ba9ff4803bc7d00b349b99c375)

Provide an explicit Window Fetch adapter with streamed Rust receive limits,
caller-owned authentication, a total deadline and abort-on-drop ownership.
This crate is exclusively for off-chain application code. Publication stays disabled.

#### Add caller-owned browser WebSocket operations

_Packages:_ 🟠 _solana_kit_browser_

_Owner:_ Ifiok Jr. · _Introduced in:_ [13e31f0](https://github.com/pina-rs/solana_kit/commit/13e31f071a4066121865789a7a5b789f3cedd87c)

Connect, send bounded text, receive and close through one local browser Window
owner. Receive cancellation preserves its queue and socket. Terminal events and
Drop detach handlers. Explicit budgets reject queue overflow, oversized messages
and browser send backpressure. Typed subscriptions retain their existing caller
owned Connection. No reconnect, task, arbitrary authentication headers or worker
support is added. Component publication remains disabled.

Clone and release executor wakers outside state borrows so synchronous destructor
callbacks can dispatch browser events during cancellation and replacement.

#### Borrow an application-owned interpreter for HTTP

_Packages:_ 🟠 _solana_kit_leptoskit_

_Owner:_ Ifiok Jr. · _Introduced in:_ [2a4c69f](https://github.com/pina-rs/solana_kit/commit/2a4c69f22df956a39fb533133012aba4a7d78e98)

Add HttpDriver and opaque HttpCompletion so installed applications can keep their
WasmHost while native HTTP waits independently. HttpHost delegates to the same
driver. Bind completions to the actual interpreter and Bridge, preserve capacity
until join, and provide explicit admission-closing abort-and-join shutdown.

- **Add explicit fixed-service interpreted guest HTTP with actual LeptosKit host.** ownership, bounded admission, native deadlines and awaited cancellation cleanup.
  _Packages:_ 🟠 _solana_kit_leptoskit_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [39ce66f](https://github.com/pina-rs/solana_kit/commit/39ce66f847a62c3e4736a50ca4fdbd9d5cb11153)

#### Add an optional application-owned wallet session service and guest leases, with

_Packages:_ 🟠 _solana_kit_leptoskit_

_Owner:_ Ifiok Jr. · _Introduced in:_ [eb1e6ac](https://github.com/pina-rs/solana_kit/commit/eb1e6ac886f2693c19894268efb6cf88d9d8c97a)

Android Intent handover around the existing binary MWA transport. Add explicit
native local-wallet selection, signing approval and private no-overwrite
persistence controls. The generated-app source adaptation installs real service
and platform registrations; compiler, peer and installed-device validation remain
required before claiming supported application journeys.

#### Add an explicit WASI Preview2 HTTP adapter and caller-owned readiness reactor for off-chain clients.

Named-host policy and lifecycle acceptance remains documented separately.

_Packages:_ 🟠 _solana_kit_wasi_

_Owner:_ Ifiok Jr. · _Introduced in:_ [b9376d3](https://github.com/pina-rs/solana_kit/commit/b9376d38d7c339e4bb2add422c5d54d2777c6784)

Preserve typed host denial and early HTTP responses during upload. Classify structured WASI HTTP stream failures without parsing diagnostic strings.

Allow Waker callbacks to inspect the reactor during polling, readiness completion and cancellation by releasing registry borrows before callbacks and destruction.

### Fixes

#### Match the pinned Dart compiler's address collation for static accounts, lookup

_Packages:_ 🟢 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [e83d103](https://github.com/pina-rs/solana_kit/commit/e83d103b4af2d3a43ebbc8a687fd5adea3362792)

table precedence, loaded accounts and lookup descriptors. Mixed-case addresses
now produce the same frozen legacy, v0 and v1 message bytes.

- **Preserve fixed and variable size declarations through checked codec observers so.** observed encoders and decoders can be paired without running their callbacks.
  _Packages:_ 🟢 _solana_kit_core_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [77e9998](https://github.com/pina-rs/solana_kit/commit/77e99980758ef7e75bcc5e4ed25e493e1cd3d030)
- **Add standalone account snapshot contracts and bounded shared error diagnostics for off-chain clients.**
  _Packages:_ 🟢 _solana_kit_core_, 🟢 _solana_kit_client_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [7cdc6cd](https://github.com/pina-rs/solana_kit/commit/7cdc6cd2b54d576e41f93090b75875f71c8678cc)

#### Declare fixed-point codec widths for composition

_Packages:_ 🟢 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [86a3ed6](https://github.com/pina-rs/solana_kit/commit/86a3ed6db42a681b074b3a14306eb6dc7aad47fc)

Report the validated fixed byte width through both codec traits so fixed-point
encoder and decoder halves compose directly through `Paired::try_new`. Mismatched
widths remain rejected before reading or writing.

#### Carry explicit lookup-table account origins through heterogeneous instruction

_Packages:_ 🟢 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [cbe40e1](https://github.com/pina-rs/solana_kit/commit/cbe40e17c6be956cb09434dcb8d64adbed7e2600)

inputs and the existing message compiler, rejecting inconsistent snapshots,
conflicting hints and static-account privilege conflicts. Keep ordinary compiler
callers source compatible. Maintain public Clock and SlotHashes fetch/decode
composition callers through the existing client transport and core codecs.

- **Clarify client validation and verification flows while preserving their wire contracts.**
  _Packages:_ 🟢 _solana_kit_core_, 🟢 _solana_kit_client_, 🟢 _solana_kit_signers_, 🟢 _solana_kit_wasi_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [af19680](https://github.com/pina-rs/solana_kit/commit/af196807568fa0fc5a34a89f074f39f0453dc475)

#### Reject completed packers that append no instructions

_Packages:_ 🟢 _solana_kit_core_

_Owner:_ Ifiok Jr. · _Introduced in:_ [77dbbd3](https://github.com/pina-rs/solana_kit/commit/77dbbd3cfaf94745a1a1666bc65eeff230275a8c)

Planning now rejects a successful custom packer that reports completion without
appending work. Synchronous planning, asynchronous planning, and append return
`NoProgress` instead of producing an empty transaction or silently losing work.

#### Bound buffered JSON during RPC parameter and complete-frame serialization.

Reject

_Packages:_ 🟢 _solana_kit_client_

_Owner:_ Ifiok Jr. · _Introduced in:_ [0bbb1a4](https://github.com/pina-rs/solana_kit/commit/0bbb1a4b9ac6d25b8373d226516ea3be3a4e503c)

oversized requests before copying the complete caller value while preserving
exact wire bytes, scalar-parameter validation and existing error distinctions.

- **Reject payer and loaded account indices as top-level raw JSON invoked programs,.** matching core wire decoding while preserving loaded programs in inner calls.
  _Packages:_ 🟢 _solana_kit_client_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [3da7b6a](https://github.com/pina-rs/solana_kit/commit/3da7b6a6a083a7e0880bfb0e81512325949f6d5e)
- **Treat an explicit null RPC enum payload as absent when invoking the context callback.**
  _Packages:_ 🟢 _solana_kit_client_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [c4851b5](https://github.com/pina-rs/solana_kit/commit/c4851b5ba41e253bab3ab126af7ee82e8f13b690)

#### Preserve signature execution failures

_Packages:_ 🟢 _solana_kit_client_

_Owner:_ Ifiok Jr. · _Introduced in:_ [66288ed](https://github.com/pina-rs/solana_kit/commit/66288ed3788ecdc64ff91e72c6e5d12b7a6f836b)

Deliver valid failed-transaction signature notifications through the shared
typed error decoder. Preserve exact future error JSON and keep other
subscriptions active. Clarify that stale-generation frames do not terminate
the replacement connection or require closing its socket.

#### Accept real MWA properties at sequence one

_Packages:_ 🟢 _solana_kit_native_

_Owner:_ Ifiok Jr. · _Introduced in:_ [886286b](https://github.com/pina-rs/solana_kit/commit/886286ba1d4b16b213efc994dc7917f59982f672)

Real wallets (including the pinned Solana Mobile reference wallet) consume
sequence 1 for the encrypted HELLO_RSP session properties. The native local and
remote clients now start their receive counter at 1, so the properties frame
authenticates and the next wallet reply is sequence 2. Previously the native
peer expected 0, so a genuine wallet's first encrypted frame failed ordering and
the pending authorization closed before any request was served.

The native local and reflector peers now encode the same real sequence, and the
public encrypted-session handshake test uses the standard properties/reply
counters. No protocol framing, crypto, deadline or configuration contract
changes.

- **Add bounded caller-owned native text WebSockets with explicit TLS roots, deadlines and cancellation ownership.**
  _Packages:_ 🟢 _solana_kit_native_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [1b4d6c6](https://github.com/pina-rs/solana_kit/commit/1b4d6c63a29a2eb7ee42fb55841e84f8c802171b)

#### Retain wallet service requests across completion wakes and bounded routing turns,

_Packages:_ 🟢 _solana_kit_browser_, 🟢 _solana_kit_leptoskit_

_Owner:_ Ifiok Jr. · _Introduced in:_ [eb1e6ac](https://github.com/pina-rs/solana_kit/commit/eb1e6ac886f2693c19894268efb6cf88d9d8c97a)

require fee-payer transaction identifiers, release browser listeners after invalid
metadata, and decode private key files into bounded zeroizing storage.

- **Label local wallet addresses for native accessibility.**
  _Packages:_ 🟢 _solana_kit_leptoskit_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [6a88d44](https://github.com/pina-rs/solana_kit/commit/6a88d44bb9c6c60715b87c2c725d4d8edd03fce3)
- **Align LeptosKit sources for generated wallet apps.** Use one framework revision for SDK transports, generated guests and native hosts, including iOS startup error layout before the first view pass.
  _Packages:_ 🟢 _solana_kit_leptoskit_
  _Owner:_ Ifiok Jr. · _Introduced in:_ [3edda48](https://github.com/pina-rs/solana_kit/commit/3edda48b17c1bdea65118497adaedeca26fde1f8)

#### Align runtime dependencies and generated application manifests with the verified

_Packages:_ 🟢 _solana_kit_leptoskit_

_Owner:_ Ifiok Jr. · _Introduced in:_ [d5962e8](https://github.com/pina-rs/solana_kit/commit/d5962e812c964bf1d43bb7077539a12f3467e42e)

LeptosKit revision. Declare Android wallet visibility using an action and URI
scheme pair, and prepare generated wallet consumers with ordinary Git dependencies.
