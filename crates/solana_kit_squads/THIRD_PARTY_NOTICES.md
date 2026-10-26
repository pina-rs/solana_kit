# Source and distribution notices

Original Rust code is Unlicense. The source-owned normalization tables, PDA
recipes and independent literals derive from the MIT Dart Squads client at
`openbudgetfun/solana_kit`, revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`, package
`packages/solana_kit_squads`. Its complete notice is `DART-LICENSE`.

The complete primary Squads V4 IDL at revision
`af94153ff77a28b6effe46b9c94baaa93742b48c` is an external private schema/byte
oracle. The primary repository LICENSE is AGPL-3.0 while its SDK package.json
declares MIT without a separate SDK notice. This unresolved scope ambiguity is
not treated as permission to redistribute its IDL or silently change the SDK's
license. No primary archive, original IDL or copied primary implementation is
included in this crate. The read-only primary auditor requires external bytes
with SHA-256
`cb9a0a29040ec3853a5105c547ffc608bb0e3175d78dacf30d463aff939f9b9b`.

The Rust model retains all 36 MIT Dart instruction payloads and ordered role
matrices, nine complete account layouts, 11 named Dart values and 45 errors.
Argument-record flattening is wire equivalent; `newConfigAuthority` intentionally
distinguishes the new authority argument from the signing `configAuthority` role.
The grouped vault-create roles retain their distinct prefixed names. The
deprecated multisig-create variant remains encodable and reports the source
program's deprecation error; the client does not pretend to reactivate it.

Rust value equality and structural traits represent the same typed data without
promising Dart hash numbers or exact diagnostic strings. Readonly/writable/signer
policies are strict. Heterogeneous remaining accounts for config/vault/batch
execution are explicit caller inputs; stored account/action state must still
authorize them at execution. PDA helpers use exact MIT source seed order and
integer widths. Source-owned snapshot helpers add no hidden network behavior.

All 905 immutable canonical source rows remain unchanged and pending in the
scoped source inventory. Generated clients, native behavior owners and the
independent primary/source audits do not promote those rows automatically.
Per-symbol adjudication, portable execution and installed application journeys
retain their own acceptance gates.
