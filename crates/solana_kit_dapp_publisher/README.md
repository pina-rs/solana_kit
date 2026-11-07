# Solana Kit dApp Publisher

Portable, off-chain publication attestations, prepared-transaction policy and portal protocol for
application clients. This unpublished source slice freezes exact attestation
bytes and checks the pinned dApp publisher's transaction policy before callers
sign. On-chain programs should use Pina or official Solana program libraries.

The default graph exposes validated 32-digit request IDs and exact ordered JSON
messages. Enable `crypto` to verify detached attestations and prepared
transactions. It adds audited Kit cryptography, the existing Kit Base64 codec,
and the portable Token Metadata decoder and PDA helpers. No runtime, network
request, entropy source, key file, signer callback or submission is started.

<!-- {=publisherAttestationExample|trim|codeBlock:"rust"} -->

```rust
use solana_kit_core::{Hash, codecs::Limits};
use solana_kit_dapp_publisher::{AttestationMessage, RequestId};

let request_id = RequestId::parse("00000000000000000000000000000042")?;
let message = AttestationMessage::new(100, Hash::from_bytes([0; 32]), request_id,
    Limits::new(1024, 4096))?;
// Present message.bytes() to the application's approved Ed25519 signer.
# Ok::<(), solana_kit_dapp_publisher::Error>(())
```

<!-- {/publisherAttestationExample} -->

`validate_prepared_transaction` takes complete wire bytes, a typed `ReleaseMint`
or `VerifyCollection` policy and explicit limits. Caller expectations bind the
blockhash token, payer and local signer. The canonical outer program allowlist
contains only Compute Budget and Token Metadata, and Metadata must be invoked.
All present signatures are verified before a private immutable seal is returned.
The seal exposes an immutable transaction and exact signing request. Signature
attachment consumes it, verifies the supplied detached signature and requires
every signature slot before returning a normal Kit transaction. Conflicting
bytes for an occupied caller slot return `OccupiedSignatureMismatch`; identical
bytes are verified again and may fill duplicate empty identity slots. Every
existing signature remains byte-for-byte unchanged. The caller owns
any review, approval, encoding, network submission and confirmation.

Release policy requires the exact payer/mint signer set, a valid existing mint
signature and the expected collection in the first Metadata Create V1 payload.
Verification policy requires the exact payer/authority signer set and the
expected NFT metadata, collection mint, collection metadata, master edition and
authority accounts. These preserve the source's narrower presence checks: they
do not establish instruction intent, account authenticity, economic effects,
blockhash freshness or validator acceptance. Lookup descriptors are rejected
until an authenticated snapshot contract is added. Malformed and trailing Create
data fail; unsupported collection variants never become a successful seal.

Transaction `max_bytes` is checked before parsing. `max_items` conservatively
reserves three elements per wire byte for the core copied message, parsed tree
and v1 temporary headers; Metadata decoding receives only the remaining items.
Core's own 1232-byte legacy/v0 and 4096-byte v1 caps also apply. Attestation limits
cover the eventual base64 size and the aggregate JSON, signed bytes and base64
characters before constructing JSON. Scalar formatting has fixed-size scratch.

Callers issue fresh request IDs and supply exact slot/blockhash values. They sign
the returned bare JSON bytes. Verified attachment emits the canonical padded
`base64(signature || JSON)` and cannot adopt a different signed message.

Enable `cli` for pure publisher command resolution: valued/boolean/equals
flags, new-version and resume validation, selector aliases, self-update policy,
portal/API endpoint derivation with HTTPS and localhost checks, API-key
environment/stdin selection, local/remote source selection and summary lines.
It performs no file read, stdin read, keypair load, network request, signing or
workflow execution; the native binary and concrete adapters live in the separate
`solana_kit_dapp_publisher_cli` crate.

The default graph also exposes a caller-owned funding preflight. It resolves an
explicit RPC endpoint, skips the check for local development, and otherwise names
the default mainnet endpoint without contacting it. `check_signer_balance`
compares a caller-observed lamport balance with the 16,000,000-lamport minimum;
`format_sol_amount` uses integer six-decimal rounding. RPC transport, deadlines,
retries and failed-read warnings remain caller policy.

Enable `portal` for thirteen fixed typed procedures over the existing Kit
`rest::Transport<PortalService>` and explicit JSON budgets. The caller's transport
owns its approved API root, authentication, TLS and cancellation. No default
origin, credential lookup, HTTP adapter or executor is provided. GET queries
carry JSON in `input`; POST mutations require explicit calls. Prepared
transaction strings remain untrusted proposals for the separate policy seal.

Raw bundle/session DTOs preserve missing, null and present fields and exact JSON
numbers. Wrong known-field types, duplicate keys, mixed or null protocol branches
and invalid envelopes reject within byte/depth/string/item budgets. Unknown DTO
fields are ignored. Ordinary error diagnostics redact provider text and bodies;
bounded evidence requires explicit field access. GET does not hydrate metadata,
cache sessions, choose signing policy or submit anything.

The same feature exposes deterministic normalized projections in `publication`:
typed dApp, publisher, bundle, release, install-file, media, source,
publication-session and ingestion-session records; checkpoint, stage and status
translators; and safe progress helpers. Projection performs no I/O and samples no
clock, so callers supply any timestamp fallback. Absent/null/blank and unknown
values keep the pinned source's precedence; unknown stage, source-kind and
publisher-kind strings are retained instead of being silently coerced. Provider
error text stays explicit data and never enters progress messages or `Debug`
output. The workflow still exposes the original raw DTOs and the CLI keeps its
existing raw-record composition.

Only explicit `create_ingestion_with_retry` supports at most three attempts with
the same nonempty idempotency key and caller-owned Clock/deadline. A 1500 ms base
retains the source's 1500/3000 ms waits. Permanent schema, admission and auth
failures do not retry. Completion at or after the deadline loses, including
decoding. Binary uploads use a separate caller-owned `Uploader`, approved origin,
explicit bytes/type/cap and one PUT; it receives no portal authentication object.
Dropping futures drops owned exchanges and timers.

Enable `workflow` for caller-driven ingestion, resume, polling, metadata hydration
and explicit cleanup. Callers own bounded source reads, atomic durable storage,
Clock and the independent uploader. Pending effects are saved before admission;
ambiguous outcomes stop resume before replay. Public metadata and receipt URLs
must be query-free HTTPS and differ from presigned PUT capabilities. The feature
preserves metadata schema 0.4.0, immutable APK hashing, six image header formats
and exact public R2 reuse. It adds only portable URI parsing, with no HTTP client.

The 41 retained workflow bodies now return fallible outcomes in five shared
groups appended after the unchanged 443-case prefix. The current v55/448
identity includes the completion group and is checked in.
Native Send witnesses remain confined to the earlier portal's native coverage.
Nine native completion bodies add caller-driven mint preparation, signed
transaction submission, receipt save, collection verification and attested store
submission. Each mutation records durable intent before the call; no method signs,
retries or enables normal publishing. Funding preflight and the native CLI are
implemented at their documented boundaries. Porting source does not authorize
uploading application artifacts or contact data.

A separate native-only publication suite adds 32 behavioral bodies for the
normalized projection: full/empty source bundles, display/contact/media
precedence, exact numbers, checkpoints/status/receipt precedence, absent/null/
blank/unknown values, caller-supplied timestamps, source kinds, attached
ingestion records, metadata URI precedence, safe progress messages and redacted
diagnostics. Its JSON and string literals are independently authored from the
pinned Dart fixtures and edge cases. These bodies are not registered in the
shared conformance suite, so the checked-in v55/448 identity is unchanged.
No Dart oracle, actual portal request or publication was executed.

The tests contain independent JSON, Borsh and legacy/v0/v1 envelope literals plus
targeted policy rejection cases. The preceding `cff45f7` source passed all 22
native tests, one Rustdoc example, strict Clippy and documentation checks in root
verification unit 840. Unit 891 then executed the shared-source refactor: all 22
native bodies, the Rustdoc example, strict Clippy and documentation checks passed.
Its historical v51 execution passed the five groups within the 432-group native suite.
Deliberate corruption passed 431 groups and exited one. Those reports qualified
that historical checkpoint. Native tests and shared conformance include the same
complete assertion bodies; cryptographic bodies have no conditional no-op path.
No portable-host or cross-language execution is claimed for this new source.

The portal reconciliation preserves all twenty-three retained portal bodies and
their predicates as Result-returning source. Twenty-one runtime bodies are
shared across hosts; two Send compile witnesses and their registration are
native-only. Four portable groups cover routes, envelopes/admission,
retry/deadlines and uploads, without claiming native Send on Wasm.
A CI unit control changes the independent expected procedure and verifies later
real portal groups continue after that reported failure. Root unit 1074 executed
the integrated `7c3ca424` producer: all 22 policy bodies, all 23 retained portal
bodies, the uploader regression control and conformance tests passed natively.
Strict Clippy and documentation checks passed. The v53 native suite passed
443/443 groups; deliberate SHA-256 corruption passed 442/443 and exited one.
Current checked-in reports are those actual captured outputs. Browser, both
WASI targets, production wasmi and three-OS CI remain unqualified for this
producer. The 2026-11-04 normalized-projection pass executed formatting, both
package test suites, strict Clippy, documentation and the repository style gate
locally; see the parity records for exact commands and remaining limits.
See the portal parity record for source anchors, native receipts and remaining checks.
