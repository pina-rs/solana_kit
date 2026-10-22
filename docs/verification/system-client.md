# System off-chain client slice

Source parent is the verified System data layer from
[PR19](https://github.com/pina-rs/solana_kit/pull/19). This adds application client
APIs; it does not support on-chain programs. Use Pina or official Solana program
libraries for those programs.

All14 instruction data types now build owned instructions with generated typed
positional accounts. One validated account IR supplies both construction and
strict parser metadata. Wire operations remain exclusively in the original data
codecs. Canonical System identity and Rent/RecentBlockhashes sysvars are fixed;
optional funding/base accounts and duplicate positions are preserved. Semantic
seed lengths use UTF-8 bytes. Signing-base consistency and absent-payer funding
are checked. Derived-address correctness remains caller policy and validator
validation; callers can use the existing optional core crypto API.

All14 constructors compare bytes and every ordered address/signer/writable bit
with official solana-system-interface3.3.0 constructors over0/1/u64::MAX amounts,
UTF-8/NUL/32-byte seeds, optional payer/base and duplicate-address examples.
These are independent native oracles, not validator execution. Strict public tests
reject wrong programs, account roles/counts, sysvar identities, tags, truncation,
trailing data, byte limits, seed length and base/funding inconsistencies.

Nonce account snapshots use explicit Legacy/Current and initialized/uninitialized
states. Official solana-nonce3.2.1 serde/bincode confirms both versions and both
states, authority/hash/fullwidth fees and every truncated prefix. Initialized
wire occupies80bytes. Uninitialized bincode serializes8bytes; actual allocated
80-byte snapshots retain72 unused storage bytes without inventing authority or
hash meaning. Only initialized Current snapshots expose usable durable fields.
Owner and reported allocation are checked before account decode. No nonce upgrade,
signing, confirmation, RPC or durable transaction journey is claimed.

Generated errors retain all9 pinned numbers/messages, with future codes preserved
as Unknown(u32). Unsupported selected account/error properties, wrong kinds,
unsafe names, changed manual seed-policy bindings, duplicates and unsupported
defaults fail before atomic output replacement. Negative controls preserve output
and pending sentinels. This remains a focused System generator, not arbitrary
Codama support. Nonce layout is a handwritten official-state policy adapter because
the flat pinned IDL cannot express uninitialized allocated storage accurately.

The [241-ID receipt](../parity/system-client.tsv) preserves every canonical ID once.
Fixed discriminators/default constructors, richer nonce state shapes and missing
standalone enum codecs remain explicitly partial/pending. Source receipt33 stays
historical. Full package parity and global completion are not claimed.

The two portable cases in system_client.rs are registered after the preceding
51 cases as suite v9. Clean signed source
`ef8c553bcd1a349ee937a45644d1fa6e8fc31558` executes all 53 cases on native Rust,
real Chrome, Wasmtime Preview 1 and Preview 2, and the pinned actual LeptosKit
WasmHost. Each host also observes the sole corrupted hash failure, 52/53.
The [runtime receipt](runtimes.md) records the exact tools, artifacts and graphs.
Pinned validator execution and durable nonce transaction journeys remain pending.
Public application normal/build dependencies remain System -> core -> bs58;
all official Solana oracle dependencies are private native dev dependencies.
Publishing remains disabled; unpatched source consumers are distinct from a
published-registry consumer while core is unpublished.
