# Source provenance

Helius wire names, model fields, defaults and operation contracts derive from the
MIT-licensed Dart Solana Kit Helius package at commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46` in
<https://github.com/openbudgetfun/solana_kit>.

The original MIT notice is retained in `MIT-DART.txt`. New original Rust code is
offered under the Unlicense. No upstream network implementation or credentials
are bundled.

Current provider wire contracts are also checked against the Helius TypeScript
SDK at commit `8560889fffe1af14d69e99d001cec29196756b87` in
<https://github.com/helius-labs/helius-sdk>. Its MIT notice is retained in
`MIT-HELIUS.txt`. Rust models and adapters are implemented independently; the
provider SDK's networking implementation is not bundled.

Withdrawal assessment uses the Kit Stake client's documented program rules at
`b37e4f76d4fbd08ad28c291f77620541489089c7`. Live-rent integer behavior is checked
against the published `solana-rent` version 4.5.0, whose archive checksum is
`6e7c7590d4e53446a00c363783d98199d8c8f5099ca60eaf2a0f1f50d0e7a31e`.
The RPC adapter and rent calculation are independently implemented with Kit-owned
types; neither upstream program code nor a Solana SDK dependency is bundled here.
