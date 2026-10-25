# Source and wire contracts

The complete original IDL is retained at
`tools/codama-renderer/fixtures/compression-original.json`, SHA256
`587f3b4d7a1f4384181f1836c09870e3fcc7ee747a69c9cf0c2164eff57bec74`, from
Account Compression revision `b229799e395cb78867e3753cf27534ecac375843`.
Its Apache 2.0 license is retained as `UPSTREAM-LICENSE`. The pinned Dart source is
revision `0138bb956d2d81ccc0c56edc602401c0df2f2b46`; its MIT license is
`DART-LICENSE`. Original Rust code is Unlicense.

Regenerate with the private maintained Rust generator command
`compression-generate crates/solana_kit_account_compression/src/generated.rs`.
All seven primary eight-byte literal instruction headers are preserved. The Dart
ordinal-u8 renderer disagrees with the primary protocol; its tags are deliberately
not reproduced. `replace_leaf` exists in the pinned program despite historical
IDL prose describing its removal.

The exact role matrix comes from the pinned program's Initialize, Modify,
VerifyLeaf, TransferAuthority and CloseTree contexts. The generated client requires
exact privileges and count. Primary TypeScript wrappers permit arbitrary remaining
metadata for every instruction; this public safe builder deliberately exposes only
the three actual proof loops as readonly non-signer remaining accounts. Other
remaining wrapper metadata and privilege escalation are not equivalent accepted
inputs. Callers retain explicit control over every role address and program override;
no instruction construction authenticates those identities against a node.

The 34 valid depth/buffer pairs, V1 header size and allocation formula follow the
pinned Dart `merkle_tree_size.dart` and primary ConcurrentMerkleTree/Path/Canopy
layouts. Checked Rust errors replace unbounded integer/floating arithmetic.
The canopy inverse rejects incomplete trees instead of returning a fractional depth.

Source values expose structural equality/hash and Rust diagnostic Display through
the shared typed renderer. Rust hash values and field spellings are idiomatic
contracts, not exact Dart object-identity hash numbers or Dart toString text.
