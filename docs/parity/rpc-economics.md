# Economics RPC slice

This SDK is exclusively for off-chain application clients. Use Pina or official
Solana libraries in on-chain programs.

The eight economics operations use the public `Client` and `Method` contract.
Request config fields preserve omission; no config serializes as `[]`, while an
explicit empty config serializes as `[{}]`. Address lists preserve order. Results
use Kit-owned validated addresses and exact `u64` integers, with no numeric JSON
round trip through floating point. Inflation quantities use finite nonnegative
`f64`; no undocumented upper bound is imposed. Reward array nulls retain their
input-address positions and commission null remains distinct from zero.

## Source contracts

The full eight Dart defining files are pinned at
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`, with exact file hashes and all42
canonical IDs retained in `rpc-economics.tsv`. Source-derived declarations retain
the [Dart MIT notice](../../verification/rpc-design/DART-LICENSE). Rust code and
literal tests are newly authored.

Official response/config grounding is `solana-rpc-client-types`4.2.2, Git revision
`c9c6f3287e26f24e3476e13e751aebe710191e89`, paths
`rpc-client-types/src/response.rs` and `rpc-client-types/src/config.rs` in Agave.
It is inspected as a source oracle only and adds no dependency. Public method
specifications were checked on2026-10-22 at the eight linked official pages:

- [Inflation governor](https://solana.com/docs/rpc/http/getinflationgovernor)
- [Inflation rate](https://solana.com/docs/rpc/http/getinflationrate)
- [Inflation rewards](https://solana.com/docs/rpc/http/getinflationreward)
- [Largest accounts](https://solana.com/docs/rpc/http/getlargestaccounts)
- [Recent prioritization fees](https://solana.com/docs/rpc/http/getrecentprioritizationfees)
- [Minimum delegation](https://solana.com/docs/rpc/http/getstakeminimumdelegation)
- [Supply](https://solana.com/docs/rpc/http/getsupply)
- [Vote accounts](https://solana.com/docs/rpc/http/getvoteaccounts)

Current official additions beyond Dart configs are `sortResults` for largest
accounts and `minContextSlot` for minimum delegation. New commission-basis-point
fields are optional, typed and retained. Legacy commission is `u8`, including255;
the pinned official type explicitly permits the post-SIMD-0291 derived range.
No undocumented commission percentage or supply-sum invariant is invented.

Official source SHA-256: response.rs
`2d42c92682cc037e49b45311ffc281c5c5ca920fd3735b7d2a4cf0f6fbd2eb04`;
config.rs `78b26ca126c79d0138f60b212933551841abfba504485298760ca6db3c034a00`.
[Immutable official source](https://github.com/anza-xyz/agave/tree/c9c6f3287e26f24e3476e13e751aebe710191e89/rpc-client-types/src).

## Evidence and remaining boundaries

`tests/economics_inflation.rs` and `tests/economics_accounts.rs` execute all eight
methods through the actual public client. An in-process transport checks exact
literal JSON request bodies before serving independently specified result data.
Tests retain maximum-width integers, config omission and explicit empty configs,
false booleans, context, nullable reward positions, complete vote records,
optional schema evolution and full address validation. They reject malformed
numeric/domain/schema values, mismatched reward cardinality, unknown-key
collisions and wrong request correlation. Prioritization filtering rejects more
than128 writable addresses before transport access. All specified economics
integer fields are unsigned64-bit except commission widths; no signed64-bit
field exists in these eight official result schemas.

The scoped42-ID table records39implemented,2partial and1pending declarations.
Vote-config diagnostics are available through idiomatic Rust `Debug`, without
promising Dart's exact text; hashing remains pending. This is a scoped capability
receipt, not completion of the complete RPC54-method catalog or Phase4.

Native fixture execution and three Wasm target compilation checks do not prove
production networking, adapters, browser/WASI/LeptosKit execution, installed app
or device journeys. Those verification units remain separate. No component is
published or reserved by this change.
