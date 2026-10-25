# Config clients

Deterministic Config account codecs and Store builders for off-chain applications.
Use Pina or official Solana libraries for on-chain programs.

`ConfigCodec` owns the shared account/Store wire layout: canonical shortU16 key
count, 32-byte address plus strict bool per key, and opaque remaining bytes.
There is no instruction discriminator. Caller byte and allocation budgets apply.
`store` accepts explicit signer positions. `store_config` derives them exactly as
the original Dart helper, including duplicates and the Config account itself.
Parsing checks explicit program identity, writable Config and signer privileges;
stronger privileges are accepted. It does not infer authorization from new keys:
actual authorization depends on the previous account state.

Pinned Config revision `b60f4f22ea5ace8ae15336f3dd4cbad9ee50dcce`,
`program/idl.json` and `interface/src/{instruction,state}.rs`; Dart revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`.
The Rust upstream convenience builder excludes the Config key itself and uses
writable signer metas; this client preserves the actual Dart readonly signer
contract. Notices are retained. The `generated` module is emitted from the exact
pinned discriminator-free IDL using shared typed compact-count and tuple codecs.
`ConfigCodec` decodes through the emitted account codec. Encoding calculates the
compact count and fixed-width key size without an owned conversion, then writes
borrowed fields through the same core codecs. The opaque tail is copied directly
into the caller's output. Store builders derive account metas without a temporary
signer vector. Account role and signer derivation wrappers remain handwritten.

`decode_account` preserves every fetched metadata field through core's owner and
allocation validation. Structural hashing and Rust diagnostic display accompany
owned value equality; exact Dart diagnostic formatting is not a wire contract.
The maintained69-body program registry executes on native Rust, Chrome, both WASI
previews and the pinned LeptosKit interpreter. Publication is disabled. Actual
network fetching, validator lifecycle and installed applications remain separate.
