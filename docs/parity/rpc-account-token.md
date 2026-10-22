# Account and token RPC slice

This is an off-chain application client SDK. Use Pina or official Solana
program libraries for on-chain programs.

Twelve source catalog rows are represented: seven account methods and five token
methods. Nine operations are new; Balance and LatestBlockhash retain their
existing contracts. The existing AccountInfo still requests and validates a
complete canonical base64 snapshot. AccountInfoWithConfig exposes the wider
standard getAccountInfo configuration with a fixed typed result.

AccountConfig preserves omitted encoding, commitment, minimum slot and data slice.
MultipleAccounts' omitted encoding defaults to base64, whereas AccountInfoWithConfig,
ProgramAccounts and token-account queries use the legacy binary default. The
response must match the selected encoding. JsonParsed permits the standard
base64 fallback when a program parser is unavailable. ProgramAccounts selects
an ordinary array or context wrapper from withContext; callers cannot substitute
a response schema. MultipleAccounts preserves request order, exact cardinality
and the requested identity of null accounts. Keyed responses validate addresses,
reject duplicate keys and enforce the requested program owner when known.

Owned core account metadata stays separate from representation. Bytes explicitly
record the requested slice; full allocation remains space, so a sliced result
is never treated as a complete account snapshot. Zstd retains decoded base64
compressed bytes explicitly: decompression and stream validity are separate
capabilities. ParsedAccount owns typed program/space metadata and bounded exact
RawValue for the genuinely program-defined parsed payload. Standard account
fields remain typed. Complete space and rentEpoch are required, matching the
pinned Dart contract; compatibility with older nodes omitting these fields is
not promised. JsonParsed combined with slicing and overflowing slice ranges
fail before transport access. Program requests permit at most four filters. Validated memcmp compares at most 128 bytes and
supports audited base58, base64 and literal bytes. Token selectors require
exactly one typed mint or program ID.

Raw token amounts parse unsigned decimal strings directly to u64, rejecting
negative, fractional, exponent and overflow values. Decimals is u8. Required
uiAmount can be finite or explicit null, while uiAmountString retains the
node's spelling, including future scaled UI semantics. Neither display field
is used to derive the exact raw quantity. Unknown encoding tags fail explicitly;
unknown program payload fields remain intact within their byte/depth budgets.

The scoped ledger maps all 100 canonical IDs exactly once. It distinguishes typed
operation/config equivalents from pending Dart hash/equality contracts and
partial idiomatic Debug formatting. Those language-specific convenience members
are not claimed behaviorally complete. The global catalog and capability ledger
are unchanged for coordinator reconciliation.

Sources: Dart Solana Kit revision
0138bb956d2d81ccc0c56edc602401c0df2f2b46, the exact twelve rpc-catalog.tsv source
hashes, official solana-rpc-client-types and solana-account-decoder-client-types
4.2.2 schemas, and
[Agave v3.1.8 RPC implementation](https://github.com/anza-xyz/agave/blob/v3.1.8/rpc/src/rpc.rs)
for operation-specific encoding defaults. The
[official account](https://solana.com/docs/rpc/http/getaccountinfo),
[multiple account](https://solana.com/docs/rpc/http/getmultipleaccounts),
[program account](https://solana.com/docs/rpc/http/getprogramaccounts), and
[token account](https://solana.com/docs/rpc/http/gettokenaccountsbyowner)
documentation corroborates the envelopes, filters and parsed payload structure.
Rust implementation and literal fixtures are newly authored; source definitions
were consulted as protocol facts. Existing upstream notices are retained.

Native public-client fixtures exercise all twelve operations, exact params and
omission, full-width integers, strict malformed data, missing/null, cardinality,
identity, representation/slice bounds, contexts, token precision and filters.
These are executed protocol fixtures, not host networking proofs. MSRV and Wasm
compiles do not establish browser/WASI/LeptosKit networking. Production adapters,
remote clusters, validator behavior and device journeys are separate work.
