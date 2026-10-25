# Address Lookup Table clients

Deterministic, exclusively off-chain application clients for all five ALT
instructions, initialized account data and authority/slot PDA derivation.
Use Pina or official Solana libraries for on-chain programs.

Kit owns addresses, instructions, codecs and transaction lookup snapshots.
Enable `pda` to use core's deterministic crypto. The default graph contains only
core and bs58. Publication is disabled.

Pinned source: address-lookup-table revision
`03b77b9e852c6f4c361f47f49823b61fc2088f49`, `idl.json` and
`program/src/{instruction,state,error}.rs`; Dart revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. Notices are retained alongside this file.
`generated` contains reproducible data codecs emitted from the exact pinned IDL by
the shared typed wire model. Public instruction codecs use those emitted layouts.
The account adapter preserves the Rust padding contract below while using the
emitted account codec. Account roles, optional funding and PDA wrappers remain
handwritten. Existing Token and System outputs remain byte-exact.

The fixed account header follows the Rust source: absent authority puts the
padding immediately after the zero option tag, followed by 32 reserved bytes.
The Dart fixed nullable codec places padding after those reserved bytes; zero
padding agrees, nonzero absent-authority padding differs and remains a stated
source-contract difference. Decode ignores reserved bytes, preserves padding,
and rejects uninitialized/unknown state tags and tables over 256 addresses.

Extend accepts either both funding accounts or neither, matching the pinned Rust
constructor; Dart requires both. Parsers require exact positional account counts,
accept privilege upgrades, preserve duplicate positions and verify an explicit
program identity. `active_addresses` takes an explicit bank slot and bounded
SlotHashes history; entries added in the current slot stay unavailable and a
deactivated table remains usable only through its cooldown history.
`decode_account` preserves the caller's fetched metadata and validates its owner.
Structural equality/hash and Rust diagnostic display use owned value semantics;
they do not preserve Dart List object identity or diagnostic punctuation.

The maintained69-body program registry executes on native Rust, Chrome, both
WASI previews and the pinned LeptosKit interpreter. Network fetching, actual
validator lifecycle and installed application journeys remain separate gates.
