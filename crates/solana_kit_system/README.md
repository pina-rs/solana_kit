# solana_kit_system

Bounded, deterministic instruction-data codecs for all fourteen instructions in
the pinned System Program IDL. This crate is exclusively for off-chain client
applications. Do not use it in on-chain programs; use Pina or official Solana
program libraries there. `no_std + alloc` supports portable clients.

Each data type has an `Encoder`/`Decoder` codec from `solana_kit_core::codecs`.
`SystemInstructionDataCodec` selects the exact u32 little-endian tag. Full decode
rejects trailing bytes, and callers supply byte and allocation budgets. String
prefixes count UTF-8 bytes using u64 little-endian lengths. Amounts are exact u64
wire units; no floating-point SOL conversion occurs. Optional IDL defaults do
not silently fill data fields: callers supply every encoded field explicitly.

This initial slice provides data codecs only. Instruction account builders,
account decoding, PDAs, program errors and RPC integration remain unimplemented.
Codec validity is structural; seed/account/program semantic constraints belong to
future validated builders and the on-chain program. No on-chain execution or
validator compatibility is inferred from byte equality.

Generated from System IDL commit `e9cb05138ea802b01f1ddf79ea140dcd765c9db6`;
input SHA-256 `bb1a83194457a7fc735f9e5a806ecaced94a17276fc7b45f356171810f15b63a`.
The upstream Apache-2.0 notice is retained in `SYSTEM-LICENSE`. Regenerate with
`cargo run --manifest-path verification/codegen-design/Cargo.toml -- system`.
