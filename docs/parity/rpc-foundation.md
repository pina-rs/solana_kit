# Typed RPC foundation receipt

This slice implements public typed getBalance and getLatestBlockhash reads and
getAccountInfo with complete canonical-base64 account data. It is exclusively
for off-chain clients; use Pina or official Solana libraries on chain.

The exact 54-row catalog in rpc-catalog.tsv retains the pinned Dart source hashes
and 398 unique canonical declaration IDs. The canonical capabilities.tsv is not
marked complete by this slice. Public-read status describes the implemented API;
it does not establish network adapters, every account encoding or full Phase 4.

The three operations preserve context, required values, exact u64 amounts and
owned domain values. Account null becomes a core Missing(requested_address);
non-null accounts retain their address, metadata and validated decoded bytes.
Invalid base64, wrong encoding, missing data or allocation mismatch is an error.
Data slices, zstd and jsonParsed account variants remain pending.

One Client owns checked request IDs and validates status, body/depth budgets,
JSON syntax, duplicate keys at every depth, version, correlation and exclusive
result/error envelopes before a typed decoder runs. Unknown remote data retains
its exact bounded JSON, including present null versus absent data. An open Method
trait lets provider integrations share these checks without duplicating framing.

The native public tests and private consumers exercise real concrete Send and
local Rc futures, observed Pending, concurrent unique IDs, exhaustion, malformed
responses, account metadata and typed downstream extensions. The private HTTP
fixture verifies exact requests and socket EOF after pending-call cancellation,
with a retained-call negative. It is a fixture, not a production HTTP adapter.

The isolated verification/rpc-design/src/conformance.rs contains small portable
protocol cases ready for host integration. This slice executes them natively and
compiles the public crate for three Wasm targets; it does not add them to the
39-case portable-core suite or claim actual browser/WASI/LeptosKit RPC execution.

Remaining 51 typed catalog operations:

- getAgGenesisCert
- getBlock
- getBlockCommitment
- getBlockHeight
- getBlockProduction
- getBlocks
- getBlocksWithLimit
- getBlockTime
- getClusterNodes
- getEpochInfo
- getEpochSchedule
- getFeeForMessage
- getFirstAvailableBlock
- getGenesisHash
- getHealth
- getHighestSnapshotSlot
- getIdentity
- getInflationGovernor
- getInflationRate
- getInflationReward
- getLargestAccounts
- getLeaderSchedule
- getMaxRetransmitSlot
- getMaxShredInsertSlot
- getMinimumBalanceForRentExemption
- getMultipleAccounts
- getProgramAccounts
- getRecentPerformanceSamples
- getRecentPrioritizationFees
- getSignaturesForAddress
- getSignatureStatuses
- getSlot
- getSlotLeader
- getSlotLeaders
- getStakeMinimumDelegation
- getSupply
- getTokenAccountBalance
- getTokenAccountsByDelegate
- getTokenAccountsByOwner
- getTokenLargestAccounts
- getTokenSupply
- getTransaction
- getTransactionCount
- getTransactionsForAddress
- getVersion
- getVoteAccounts
- isBlockhashValid
- minimumLedgerSlot
- requestAirdrop
- sendTransaction
- simulateTransaction

Platform adapters, host permissions/deadlines/header budgets, successful TLS,
subscriptions and application/mobile network journeys require their own evidence.
The SDK release group explicitly owns core and client; all component publishing
remains disabled and the umbrella placeholder is unchanged.
