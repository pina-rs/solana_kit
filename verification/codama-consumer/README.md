# Generated Codama consumer

This private fixture contains nine client contract groups against the output of the packed NPM renderer. They cover literal instruction bytes and malformed inputs, borrowed codec composition and size rejection, imported signing keys, complete account/event discriminator predicates, positional optional roles, omitted suffixes, literal defaults without PDA recipes, PDA helper names and account-free builders, and numeric layouts.

Generate all six program modules through the maintained packed-package test before running Cargo. The default `pda` feature ensures resolver helpers compile. The `guest` exports the ordinary LeptosKit ABI and runs these same assertions before emitting its result. Source alone is not evidence of native, browser, WASI or LeptosKit runtime execution.

The numeric AST in `packages/codama-renderers-rust-solana-kit/test/numeric.json` is consumed by both the public native-renderer test and the packed NPM renderer. The compiled consumer checks both byte orders of signed integers, 128-bit integers and f32, including IEEE 754 bit patterns and truncated-input rejection. Literal big-endian vectors cover scalar 258, count-prefixed booleans, byte-size framing and variable/fixed options. Two u32 instruction tags use distinct non-symmetric bytes; identification, dispatch, truncation and reversed-byte controls exercise the generated headers. It uses the generated codecs, rather than duplicating their composition in the test.

The source contract is Dart commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46`, `packages/codama-renderers-dart/`:

| Source                                                                                          | SHA-256                                                            |
| ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `src/visitors/getTypeManifestVisitor.ts` (number visitor at line 135)                           | `94bda1971f1cc849e8d6af9346341529e5a45a55b19659ea4f207265ed25dd29` |
| `test/visitors/getTypeManifestVisitor.test.ts` (widths at lines 80–123, byte order at line 136) | `00df669e8308364c1d3f249bac2d2d1d9e7e4322a2c161eb14cf7a364225b054` |
| `test/type-manifest-security.test.ts` (prefix contracts at line 124, byte vectors at line 232)  | `29dd2d08cbd016368220240727c9fa2f6315e1754c315fbc46a268431cb457e3` |

Instruction discriminator byte order follows the same pinned source: `src/utils/discriminators.ts:107` encodes constants through their declared codec (SHA-256 `d0b990d9b601ad902820ffc7ef58053af30e10fd89865a3e1e67af41dd2bef04`), and `src/fragments/instructionPage.ts:127` retains each argument's manifest (SHA-256 `496c87612eebfb5a7f2a0a52438f62b729a990ad2061123e1c274fb9b73c9e56`).

This is application client code. Use Pina or the official Solana program libraries for on-chain programs.
