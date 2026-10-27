# Receiver instruction clients

The client builds `post_update` and `post_update_atomic`, encodes their parameter
records, exposes the receiver/Wormhole/push-oracle identities, and derives receiver
configuration, treasury and Wormhole guardian-set addresses. These are exclusively
off-chain APIs. On-chain programs should use Pina or official Solana libraries.

The reference is Dart revision `0138bb956d2d81ccc0c56edc602401c0df2f2b46`, including
`update_price_feeds.dart`. All nine Pyth library sources match their pinned hashes.
An independent Dart execution with OS networking denied captures complete data
and account vectors for both default instructions and every explicit account,
authority, treasury and program override. It also derives the treasury endpoints,
guardian-set index endpoints and custom-program addresses. The input VAA is the
existing pinned upstream fixture; all message, proof and account values are
synthetic. Dart source, package configuration and the user's SDK cache remain
unchanged during this execution.

The primary Pyth source is revision
`918664828173a60a2cde1f8431474efb02a6b75f`:

- [Parameter records and program identities](https://github.com/pyth-network/pyth-crosschain/blob/918664828173a60a2cde1f8431474efb02a6b75f/target_chains/solana/pyth_solana_receiver_sdk/src/lib.rs)
- [Exact account order and privileges](https://github.com/pyth-network/pyth-crosschain/blob/918664828173a60a2cde1f8431474efb02a6b75f/target_chains/solana/pyth_solana_receiver_sdk/src/cpi/accounts.rs)
- [Configuration and treasury seeds](https://github.com/pyth-network/pyth-crosschain/blob/918664828173a60a2cde1f8431474efb02a6b75f/target_chains/solana/pyth_solana_receiver_sdk/src/pda.rs)

Both instruction variants preserve all seven positions and signer/write roles.
Write authority defaults to the payer; repeated addresses are retained rather
than collapsed. The enum's distinct guardian-set and encoded-VAA fields prevent
silently choosing the other variant's discriminator or verification role.
Receiver accounts support explicit overrides and optional derivation under a
custom receiver program. Guardian derivation separately selects its Wormhole
program and uses the exact big-endian unsigned index.

The default normal dependency graph remains Kit Core and bs58. Optional `crypto`
only enables the existing Kit Core implementation. Borsh encoding reuses the core
writer and fallible owned encoder. VAA and message bytes remain borrowed until
output encoding; proof hashes have a fixed 20-byte Rust type. Every vector count,
size composition and allocation is checked. `Limits.max_bytes` covers instruction
data including the discriminator. `Limits.max_items` counts proof hashes plus the
seven instruction accounts, or just the proof hashes for parameter-only encoding.

Treasury IDs and guardian indices use unsigned wire types. Random treasury
selection consumes one caller-provided uniform byte and propagates its failure;
it adds no ambient random source. Construction verifies no guardian, account
owner, Merkle path, price authenticity or freshness and sends no transaction.
Opaque and empty parameter fields retain their ordinary wire meaning.

Eleven native tests cover complete independent byte/account vectors, parameter
encoding, PDA boundaries/custom programs, Anchor hash domains, exact byte/item
limits, duplicate positions, unchanged inputs and entropy success/failure.
The default feature set executes the seven tests that do not need PDA crypto.
Three appended conformance groups advance the shared suite to v43 with 373 cases,
preserving every earlier case and the sole corrupt-hash control. Each browser,
WASI and production LeptosKit runtime cell needs actual execution before acceptance.

This does not establish full Pyth service integration or complete ecosystem
parity. In particular, host-network composition and authenticated provider
configuration for the new Hermes client still need their separate evidence.
