# MPL Core client

This crate is exclusively for off-chain applications and clients. On-chain
programs use Pina or official Solana program libraries. The deterministic
`no_std + alloc` codecs and builders use Kit-owned addresses, instructions,
accounts and explicit byte/item budgets. Optional `pda` enables deterministic
address derivation without transport, runtime or ambient entropy.

The maintained source model describes all 42 instructions, six account bases,
plugin and external-plugin variants, and 57 errors in the pinned MIT Dart client.
Builders accept explicit ordered account metadata. Omitted optional accounts use
readonly program-ID sentinels. Caller-owned transports fetch accounts through
`solana_kit_client`; the decoded snapshot helpers preserve its existing account
boundary. The `AccountData` wrapper retains trailing extension bytes permitted
by the Dart account decoder. `inspect_asset_plugins` and
`inspect_collection_plugins` separately validate a complete dynamic registry.
Unknown typed discriminators are rejected.

Regenerate the committed module through `mpl-core-generate OUTPUT` in the private
`solana_kit_codegen` command. The adapter declares each account's fixed key in its
wire layout before lowering the model through the common renderer. Byte-level and
interpreter contracts do not establish validator or installed-device acceptance.

The public model and fixture boundary is Dart revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. Its notice is retained in
[DART-LICENSE](DART-LICENSE). New original Rust code is Unlicense. The separate
Metaplex primary IDL remains a private schema oracle under its actual NOSL notice;
no primary IDL or implementation bytes are distributed here.
