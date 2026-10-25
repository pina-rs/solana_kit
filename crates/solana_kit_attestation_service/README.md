# Solana Attestation Service

Portable off-chain application/client code for the pinned Solana Attestation
Service. On-chain programs should use Pina or official Solana program libraries.
`no_std + alloc` enables portable clients and does not promise on-chain support.

The source model covers all 12 instructions, three accounts, the close-event
payload, 26 schema data types and 12 custom errors. Instruction builders take
explicit ordered Kit `AccountMeta` values and enforce the complete role policy.
Canonical and explicit program IDs share the same codecs. The `pda` feature adds
all seven deterministic seed recipes without entropy or host dependencies.

Account and instruction `data` fields stay opaque bytes owned by the caller.
`AttestationDataCodec::from_schema` opts into checked encoding for the actual
source-defined schema. Its `FieldValue` enum preserves exact integer widths and
Unicode scalars. Names are strict UTF-8 and unique, and values must have exactly
the declared names and types. Named input order does not change wire order.

```rust
use solana_kit_attestation_service::{CreateAttestation, CreateAttestationCodec};
use solana_kit_core::{Address, codecs::{Decoder, Encoder, Limits}};

let data = CreateAttestation {
    nonce: Address::from_bytes([7; 32]),
    data: vec![0xff, 0x80],
    expiry: 0,
};
let bytes = CreateAttestationCodec.encode(&data, 4096)?;
assert_eq!(CreateAttestationCodec.decode(&bytes, Limits::new(4096, 256))?, data);
# Ok::<(), solana_kit_core::codecs::Error>(())
```

`decode_credential`, `decode_schema` and `decode_attestation` consume already
fetched core accounts. They preserve owner, allocation, metadata and caller-limit
errors. Applications fetch through the existing typed Kit client and own their
transport, runtime, account snapshots, authority selection and all effects.

Schema display text replaces malformed UTF-8. Attestation string fields follow
the reference's explicit presentation rule. Malformed UTF-8 becomes `0x` hex text;
encoding that text writes its UTF-8 characters and does not recover the original
binary. Keep the opaque bytes when lossless persistence is needed.

```rust
use solana_kit_attestation_service::{AttestationDataCodec, AttestationField, FieldValue, Schema};
use solana_kit_core::{Address, codecs::{Encoder, Limits}};

let schema = Schema {
    credential: Address::from_bytes([7; 32]),
    name: b"counter".to_vec(),
    description: Vec::new(),
    layout: vec![0],
    field_names: vec![5, 0, 0, 0, b'c', b'o', b'u', b'n', b't'],
    is_paused: false,
    version: 1,
};
let codec = AttestationDataCodec::from_schema(&schema, Limits::new(1024, 64))?;
let values = [AttestationField { name: "count".into(), value: FieldValue::U8(9) }];
assert_eq!(codec.encode(&values, 1024)?, [9]);
# Ok::<(), solana_kit_core::codecs::Error>(())
```

The source defines expiry as signed i64, including zero. Encoding does not assert
that a validator will accept a timestamp, signer, paused schema, close or token
account. Closing is the source operation for revocation. There is no separate
revoke instruction, optional account sentinel or remaining proof tail at this pin.

The reproducible model lives in `tools/codama-renderer/src/attestation` and
lowers into the existing shared renderer. `attestation-generate OUTPUT` owns
emission. No generated file may be handwritten. The native suite executes all 22
original behavior owners and both public examples. The shared portable registry
imports those same bodies for browser, WASI and interpreted-host verification.

Publication is disabled. See `THIRD_PARTY_NOTICES.md` and the exact source table at
`docs/parity/remaining-programs-source.tsv`. Source membership remains distinct
from completed declaration-level parity. Validator transitions and installed
application journeys require their own execution evidence.
