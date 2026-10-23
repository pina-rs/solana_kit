# Shared data generator

This private native tool renders Kit-owned data codecs from pinned Codama inputs.
It shares the typed wire model, shape validation, codec expressions and atomic
single-file writer with the existing instruction generator. It is not a runtime
dependency or a complete Token-2022 client generator.

`token-2022-data <output.rs>` renders the complete pinned data model.
`token-2022-proof <directory>` also writes the ordinary downstream byte-oracle
consumer. The output directory must exist or be creatable by that command.

`alt-config` first validates and renders both complete inputs, then replaces each
output atomically. Its existing output directories must already exist under
`crates/solana_kit_address_lookup_table/src` and `crates/solana_kit_config/src`.
The set is not transactional if a later filesystem replacement fails. These
program clients are not yet part of the public workspace.

The two `fixtures/*-generated.rs` files retain the exact accepted ALT and Config
outputs from commit `442316b6b855cdd1eae38d4984d5ef31687628e4`. They are immutable
generator goldens. The drift test compares newly rendered bytes with these
snapshots. Later public program clients must use these same outputs.
`fixtures/alt-config-prior-receipt.tsv` retains the accepted 113-row bounded
private receipt from that commit. It is test input, not the current public parity
ledger or a claim that its named client APIs are available. The receipt audit
command requires the original source cache and installed program notices.

The source fixtures retain their original license notices. The source auditor
retains all 1,829 Token-2022 declaration rows without promoting any to complete.
The 528 Dart rows compare 400 instruction and 12 base-account frames, plus 116
separate bare extension tags and payloads. They are not 528 canonical account
TLV equivalences. Nine original Dart tests are historical source-identical proof.

Tagged variants have contiguous discriminants. Standalone zero remains legal.
TLV account collections reject a zero entry before writing their first extension.
Preflight rejection takes precedence over a later oversized payload. Decoder
termination and spare capacity remain supported. Named structural codecs do not
implement complete program-state validation or reserved-size account dispatch.

`Reader::transaction` is an opt-in cursor and item-budget transaction. Successful
closures commit those two values. Failed closures restore them. External closure
effects remain visible. Ordinary frame and take operations keep their existing
consuming behavior.

All generated code is exclusively for off-chain client applications. Use Pina
or official Solana program libraries for on-chain programs. Validator and installed
desktop, Android and iOS journeys remain separate requirements.
