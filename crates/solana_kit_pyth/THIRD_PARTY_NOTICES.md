# Source provenance

Wire names, fields and helper contracts derive from the MIT-licensed Dart Pyth
package at commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46` in
<https://github.com/openbudgetfun/solana_kit>. Its original notice is retained in
`MIT-DART.txt`. New original Rust code is offered under the Unlicense.

Current wire structure is checked against Pyth's authoritative definitions at
commit `918664828173a60a2cde1f8431474efb02a6b75f` in
<https://github.com/pyth-network/pyth-crosschain>, specifically
`pythnet/pythnet_sdk/src/wire.rs` and its serializer and deserializer. The upstream
implementation is not bundled or linked.

The test fixture originates in that upstream file's accumulator golden test.
Its Apache-2.0 notice is retained beside the fixture. Test fixtures are excluded
from the published library archive.
