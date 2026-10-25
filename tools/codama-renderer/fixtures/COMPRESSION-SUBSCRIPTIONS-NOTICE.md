# Program preparation source notices

`compression-original.json` is unchanged from Account Compression revision
`b229799e395cb78867e3753cf27534ecac375843`, at
`account-compression/sdk/idl/spl_account_compression.json`.
SHA256: `587f3b4d7a1f4384181f1836c09870e3fcc7ee747a69c9cf0c2164eff57bec74`.
The upstream Apache 2.0 distribution license is retained as `COMPRESSION-LICENSE`.
The canonical source archive SHA256 is
`6c1b15cdfda6357a5aa4df16a862e21e20c1b83a61ee0e80820126dc8a6892e7`.

`subscriptions-original.json` is unchanged from Subscriptions revision
`5a347ffaa969036061d274d3c91e0277962e2b51`, at `idl/subscriptions.json`.
SHA256: `461e1c41da256906644d3e20ab185e97de9d9c141661e1521bf5ef42e255ac40`.
The upstream MIT license is retained as `SUBSCRIPTIONS-LICENSE`.
The canonical source archive SHA256 is
`93b3f30c1b765a9431e6f6abeeac6c29d5a73fcdee24a57b28714c237f853914`.

The new Rust preparation code is an independent transformation under this
repository's Unlicense. It preserves source layout declarations and documentation.
It does not replace the upstream distribution licenses.

Compression instruction discriminators use the first eight bytes of
SHA256(`global:<snake_case_name>`), matching the pinned primary TypeScript SDK's
seven literal discriminator arrays. The pinned Dart package instead emits
instruction-order u8 tags. Those Dart bytes are not canonical Compression wire
oracles. The Rust tests use the primary literal arrays, including
`replaceLeaf = cc a5 4c 64 49 93 00 80`.

This preparation primitive is not a runtime client. The source includes seven
Compression instructions, ten named types, nine errors and no declared accounts.
Subscriptions contains eighteen instructions, six accounts, six PDA recipes,
seventy-eight errors and seven events. Its original tree remains complete;
this checkpoint validates its PDA recipes and twenty-eight role defaults only.
Events, fixed strings, account dispatch and runtime client generation still
require explicit typed admission and tests.

The three Compression proof-list policies are based on the actual pinned
program's `remaining_accounts` proof-node loops for `replace_leaf`, `verify_leaf`
and `insert_or_append`. Other primary TypeScript wrappers also permit arbitrary
remaining account metadata. Normalization records the proof-specific readonly
policy; it does not establish full source-wrapper account privilege equivalence.

All resulting SDK work is exclusively for off-chain client applications. Use Pina
or official Solana program libraries for on-chain programs.
