# Solana Kit dApp Publisher

Portable, off-chain publication attestations and prepared-transaction policy for
application clients. This unpublished source slice freezes exact attestation
bytes and checks the pinned dApp publisher's transaction policy before callers
sign. On-chain programs should use Pina or official Solana program libraries.

The default graph exposes validated 32-digit request IDs and exact ordered JSON
messages. Enable `crypto` to verify detached attestations and prepared
transactions. It adds audited Kit cryptography, the existing Kit Base64 codec,
and the portable Token Metadata decoder and PDA helpers. No runtime, network
request, entropy source, key file, signer callback or submission is started.

```rust
use solana_kit_core::{Hash, codecs::Limits};
use solana_kit_dapp_publisher::{AttestationMessage, RequestId};

let request_id = RequestId::parse("00000000000000000000000000000042")?;
let message = AttestationMessage::new(100, Hash::from_bytes([0; 32]), request_id,
    Limits::new(1024, 4096))?;
// Present message.bytes() to the application's approved Ed25519 signer.
# Ok::<(), solana_kit_dapp_publisher::Error>(())
```

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

Portal APIs, uploads, resumable workflows, CLI, media processing and complete
publisher parity remain future slices of this same package. Porting source does
not authorize uploading application artifacts or publisher/contact metadata.

The tests contain independent JSON, Borsh and legacy/v0/v1 envelope literals plus
targeted policy rejection cases. The preceding `cff45f7` source passed all 22
native tests, one Rustdoc example, strict Clippy and documentation checks in root
verification unit 840. The subsequent shared-source refactor and five appended
portable groups are unexecuted source proposals. The same complete assertion
bodies are included by the ordinary native test entry points and shared
conformance; cryptographic bodies have no conditional no-op path there.
No portable-host or cross-language execution is claimed for this new source.
