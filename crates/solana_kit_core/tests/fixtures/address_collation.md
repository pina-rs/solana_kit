# Address collation literals

These nine rows are literal full compiler outputs from Dart commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`, executed with Dart 3.13.3.
Columns are `M`, version, scenario, comma-separated static accounts and complete
message hex. No Rust serialization produced expected bytes.

The defining source is
`packages/solana_kit_address/lib/src/address_comparator.dart`, SHA256
`d096642ea02001eeffdc754c4ef79be5fe2c1c69b45ca55de374a1a580b21079`.
The implementation ranks each character, with digits first and lowercase before
uppercase at each letter. It compares lengths only after an equal prefix.
Its prose about the base58 alphabet and runtime ordering is contradictory;
these fixtures follow the executed implementation.

`compile_accounts.dart`, SHA256
`9ef07581a76fdcbbeb0b2e6c1e2b2cc9d63c88c166623d1ad8776d67fab9313a`,
uses that comparator for competing lookup metadata and static/loaded accounts.
`compile_address_table_lookups.dart`, SHA256
`e3969234d49d9c85896ce366079d9f63072e3631f2db09d3d07f3475156c1f92`,
sorts descriptors with it.

Scenario 0 invokes byte-9 and RecentBlockhashes programs. Scenarios 1 and 2 add
mixed digit/case addresses, privilege upgrades, a reused payer, duplicate
instruction positions and multiple programs. Scenario 2 uses lookups in v0.
Its byte-17 account appears in both tables and twice in the byte-9 table.
The first byte-9 index is selected. The Dart input uses explicit lookup metadata
for both competing tables across instructions. Rust accepts snapshots and must
produce those same bytes regardless of snapshot order.

The local Unit162 evidence retains the generating Dart script, explicit package
configuration, full stdout, original comparator/compiler/decompiler test results,
two prototype comparisons and old-failure/new-pass logs. The source was not
modified and dependencies were not resolved. Fixtures are synthetic public data.
No reference implementation code was copied. The reference MIT license remains
in the pinned checkout.
