# Source provenance

The client model, PDA recipes and literal fixtures derive from the MIT-licensed
Dart package `solana_kit_mpl_token_metadata` at revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46` of
`https://github.com/openbudgetfun/solana_kit`. Its complete notice is `DART-LICENSE`.
Original Rust additions use the Unlicense in `LICENSE`.

The upstream Metaplex repository at revision
`353d01be4af3018e1daf49d0cd35ba9f731d9c9c` remains a private independent schema
oracle. Its repository license is the Metaplex NFT Open Source License, with hash
`18f025513b3ad665d57ab359e11ab593f240ecd46b59eb5092ef03418f1bf0f7`.
No primary IDL or implementation bytes are distributed by this crate. A package
manifest license string does not resolve that separate distribution boundary.

Fixed account key framing, strict privilege parsing, explicit allocation tails,
ordered payload pairs, explicit program variants and checked public helper limits
are original Rust API choices. They do not establish full Dart API or validator
parity. The exact canonical declaration table preserves its original pending status.
