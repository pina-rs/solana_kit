# History and submission RPC slice

This is exclusively an off-chain application SDK. Use Pina or official Solana
program libraries for on-chain programs. This slice implements 13 of the 15
history/submission catalog operations; it does not complete Phase 4.

The public `methods::history` operations are `BlockTime`, `BlockCommitment`,
`Blocks`, `BlocksWithLimit`, `FirstAvailableBlock`, `MinimumLedgerSlot`, and
`SignaturesForAddress`. `methods::submission` provides `FeeForMessage`,
`SignatureStatuses`, `RequestAirdrop`, and `SendTransaction`. Their built-in
response schemas are fixed by the SDK and execute through the existing public
`Client<T>` framing, budgets, correlation and remote-error boundary.

All configuration fields for these 13 operations are typed. Historical commitment
excludes processed. Block ranges are inclusive and at most 500,000 slots apart;
block counts accept 0 through 500,000. Signature pagination retains before/until,
commitment, minimum context and the 1 through 1000 limit. Status requests accept
at most 256 signatures and preserve exact positional response cardinality.
No history is truncated or automatically paginated. Invalid request configuration
fails serialization before exchange.

Slots, stake, fees, lamports, retries and confirmation counts are exact u64.
Timestamps are signed i64. Signature and address values belong to Kit; incoming
signatures validate the alphabet and exact decoded 64-byte length. Required
nullable fields cannot disappear. Required historical nulls remain explicit; the current optional transactionIndex
preserves absence versus present u32 and rejects explicit null.
Block commitment uses the actual 32 confirmation-depth stake entries.

Submission accepts exact caller-supplied wire bytes and selects canonical base58
or base64 explicitly. It does not assume a legacy packet size or infer a message
version, lifetime, expiry or durable nonce. Fees accept message bytes and encode
canonical base64. The caller owns transaction construction and signing. Returned
send/airdrop signatures indicate node acceptance, never confirmation. The client
never retries; maxRetries controls only the remote node's policy.

Known execution failures have typed transaction and instruction variants with
validated u8 indices and u32 custom errors. Unknown future transaction variants
remain exact bounded RawValue strings or single-key objects. Malformed known
variants cannot fall through to that future branch. Unknown future instruction
variants currently reject and require a schema update. The deprecated status and
err fields must agree on success versus failure. Error payload equality is not
claimed; both original typed values are retained. Node transaction failure remains
a successful RPC result, distinct from RPC/HTTP/transport failure.

The scoped [ledger](rpc-history-submission.tsv) accounts for all 146 canonical
Dart IDs exactly once: 67 implemented and 79 pending. Pending includes every ID
for the two large operations below and Dart hashCode/toString conveniences.
The canonical global inventory and source catalog remain unchanged.

These two operations have no placeholder implementation:

| Operation                 | Remaining typed contract                                                                                                                                     |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| getTransactionsForAddress | Full/signatures page, cursor, sort, typed slot/time/signature comparisons, status/token-account filters, all encoding/version variants                       |
| simulateTransaction       | Full typed execution result, returned accounts, token/balance/loaded-address/return-data fields, inner instructions, signature/replace-blockhash constraints |

Current Kit 8.3.0 getTransactionsForAddress adds blockTime.eq beyond the pinned
Dart comparison model. Its cursor is a string shaped as slot:position; results
have required transactionIndex, data and nullable paginationToken. Full results
require typed transaction/meta variants and conditional version fields. Its
signatures/full limits are 1000/100 respectively. These facts ground the pending
work; no generic map or whole-response RawValue is substituted for this schema.

Native public fixture tests exercise every implemented operation through Client,
including exact params, omitted configuration, explicit false, integer limits,
nulls, malformed fields/signatures, ranges/cardinality and execution error payloads.
The clean downstream source consumer additionally exercises maximum 256 statuses,
current optional transactionIndex, central duplicate/correlation failures and
remote preflight failure without retry. These are local protocol executions;
three Wasm target checks are compilation only. No RPC network, validator,
installed LeptosKit application or device execution follows from these tests.

Sources are immutable Dart revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46` and official
[Kit 8.3.0 RPC schemas](https://github.com/anza-xyz/kit/tree/v8.3.0/packages/rpc-api/src).
The table records SHA256 of each actual input. Rust implementation and fixtures
are newly authored. Execution-error variant declarations were adapted from
Apache-2.0 solana-transaction-error 3.4.0 at
`342a32244651d1f9a6811ddf735e4c8c1d557c1a` and solana-instruction-error 2.5.0 at
`e658c42f7dfdbe5431fa9bbf58709824c64ea706`. Their required Apache license is retained
beside the adapted module. These are inspected schema inputs, never runtime
Solana dependencies. Official solana-rpc-client-types 4.2.2 at
`c9c6f3287e26f24e3476e13e751aebe710191e89` and transaction-status-client-types
4.2.2 confirm Rust response/config widths and historical optional metadata.
The explicit bs58 alloc-only dependency already existed through core, so the
normal/build package-version closure remains the existing 13 external pairs.

| Method                      | Pinned Dart source SHA256                                          | Kit 8.3.0 source SHA256                                            |
| --------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------ |
| `getBlock`                  | `35056b884a1be5d23fc024a764efd5bc0a9e435a6ed7d7d98e8c138effdd257d` | `dc63145cbc9078e5853e32458b7aed6d403952769910cd2b0126d74f0a76df91` |
| `getBlockCommitment`        | `d59f882800dc41a212cbf8597505e552dbfecd014712ff0c64dfd288871d3c09` | `80e897ecd0a364258831290cdafcb07211b34de32b81e6d56ee157d56ad3ddf2` |
| `getBlocks`                 | `ba14c5b3e78370576f627902cca65641f8aa462eb5cda2a05e49963706e046d6` | `309bec79f8d44668a457d9a8fab5517dc33aa65a03631638427736f62f57e20f` |
| `getBlocksWithLimit`        | `df5304a4c55bf79119fe812c16bab5f8a77231c7064025969347ef81932852be` | `3f8ba1b9e421953c331a0fc1479c074861c4d600178e902254538025dce9cf25` |
| `getBlockTime`              | `adfbd8ec93ae49777a49fa027ac80952894879570f96221cacf488fe15c1fd73` | `9384a4ffac6e7f120cc85b0ca905a59302ddcce736d568dab5c0d713937ce1f6` |
| `getFeeForMessage`          | `92e459924a1b1881ec145fed9f615e50de15b1bac072646647fca72cd7235517` | `eb433b6dd7f3007f14acc841a0b2df8de56facde84e42fa80718ede612424656` |
| `getFirstAvailableBlock`    | `760e56bc561148e4ffca80c06a4f88b47485797d6d59a211f2c057a32c479da5` | `7edd0698c97b5cb6d91d7e2677a853595c0bb2192ecc9569d26155dfc029a447` |
| `getSignaturesForAddress`   | `f4c5fbbafe29ac38a4b552bfbe5e3674f1dcee4a13601b11c12f73cbafbcbd93` | `c75fcd4a75663c3adf28e55d3fa8d4829b6d140444d2eb5a7adacf084283c620` |
| `getSignatureStatuses`      | `ab315f75f4502bb00b5cce5db8486f9beac42a52d434a88178f4cb41dca9d40a` | `e6b2fd078946c3ded455c1dbfe6afd03cc6667da3e988c901d9ec76c1e956d7a` |
| `getTransaction`            | `fa215aadfa539568a0c9791e322fce00bc0ddb67c924a5bf202c441b48dd5b08` | `a920e06dda4a8021d55321035be8aed9dce31de1c4e7f02a2c55ac83b4fbab9f` |
| `getTransactionsForAddress` | `c24a573459ff4061986f411d3d9687d63d618d2b868fe3c0f86d21b18bd16753` | `77623c39fe83845a88763deced15e86a5bc3e0816d035f9826900673746b4175` |
| `minimumLedgerSlot`         | `67489d8f2ca52c520b0bc70c368c46edeab9d11367cbe735815c6951e9824c30` | `1cfde6dde5e85a288c64b4f773173d722d23c6e98eb54caf0fe7f3244e5c734a` |
| `requestAirdrop`            | `615d85d7b83fa70add4cad427cc33688348a4f24bb7c0eefa0b0be2aef22882e` | `734c757c5c8185e651165d950fb5cd88f1d6c002d325113d7319cbd0c7a34516` |
| `sendTransaction`           | `55861cec3a49d9da0357a0d4a17b13c2dd566fa9abb3af03949ad36cf247c2e6` | `e8f716a286932f41a7c101f4ace745fb505e6f95036885a90ee38139c0cd52aa` |
| `simulateTransaction`       | `2b6b8429625cb04701824ba648d5effde8b7edfc09dc66c392a0fe1872a550c7` | `2297a051ea94b51c416570ef08ca8e6159c62088aeff278e5ae53dbf811351a6` |

## Typed block and transaction results

`methods::transaction_results::{Block, Transaction}` add `getBlock` and
`getTransaction` with fixed SDK-owned response graphs. Both support explicit
base58/base64/json/jsonParsed encoding, optional confirmed/finalized commitment,
and omitted or numbered 0/1 maximum-supported version. `Block` additionally
supports full/accounts/signatures/none details and optional rewards. Omitted
configuration fields remain omitted; explicit false remains false. Null block
or transaction results remain null. Block time and height accept required nulls,
following actual Agave rather than Kit's nonnullable block aliases.

Request-selected enums retain wire bytes, index-based JSON, parsed JSON or
accounts-only views. Binary data is decoded canonically without imposing a
legacy packet limit or pretending to verify a transaction. Structured views own
validated Kit addresses, hashes and signatures, header counts, lookup references,
compiled/parsed/fallback instructions and current V1 transactionConfig. Version
fields must match the requested maximum. Block detail and reward fields must
match the request. Optional historical/versioned fields preserve absence and null
where the node protocol permits both. The actual Agave 4.2.2
`TransactionWithStatusMeta::MissingMetadata` serializers omit the version and
emit `meta: null` even when a maximum was requested. `Block` and `Transaction`
accept this historical form; complete metadata still requires the requested
version field. Shared decoding has an explicit stricter contract for operations
whose schema requires the version independently of metadata.

Execution metadata includes typed errors/status, exact fees and balances, token
balances, rewards, loaded addresses, inner instructions, logs, return data, compute
consumption and cost units. Token raw amounts remain canonical decimal u64 strings;
UI JSON numbers remain exact serde_json::Number values and are never used for
monetary arithmetic. Their raw JSON token must be an actual number or null;
serde_json's private arbitrary-precision object representation is not a numeric
RPC scalar. Program-defined opaque objects with the same keys remain valid.
Only genuinely program-defined parsed instruction content
retains bounded exact RawValue. Nested parsed content is dispatched directly from
RawValue so its whitespace, escapes and arbitrary numeric lexemes survive.

The field layouts were adapted from Apache-2.0 official Agave
solana-transaction-status-client-types 4.2.2 at
c9c6f3287e26f24e3476e13e751aebe710191e89, with the license retained in the
module's APACHE-2.0 file. Existing Kit8.3.0 sources and hashes listed below remain
the encoding/configuration reference. No official Solana dependency was added.
These native public protocol fixtures do not establish actual validator behavior,
Wasm networking, confirmation or lifetime semantics.
