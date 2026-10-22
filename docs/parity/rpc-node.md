# Typed node RPC catalog

This off-chain client slice implements all 19 `node` methods in the pinned
54-method catalog. On-chain programs should use Pina or the official Solana
program libraries. The public entry points live under
`solana_kit_client::methods::node` and execute through the existing
`Client<T>::call`; callers cannot choose a standard method's response schema.

| RPC method                  | Rust operation           | Owned response                      |
| --------------------------- | ------------------------ | ----------------------------------- |
| getAgGenesisCert            | AgGenesisCert            | Option<AlpenglowGenesisCertificate> |
| getBlockHeight              | BlockHeight              | u64                                 |
| getBlockProduction          | BlockProduction          | Contextual<BlockProductionValue>    |
| getClusterNodes             | ClusterNodes             | Vec<ClusterNode>                    |
| getEpochInfo                | GetEpochInfo             | EpochInfo                           |
| getEpochSchedule            | GetEpochSchedule         | EpochSchedule                       |
| getGenesisHash              | GenesisHash              | core::Hash                          |
| getHealth                   | GetHealth                | Health                              |
| getHighestSnapshotSlot      | HighestSnapshotSlot      | SnapshotSlots                       |
| getIdentity                 | Identity                 | NodeIdentity                        |
| getLeaderSchedule           | LeaderSchedule           | Option<LeaderScheduleValue>         |
| getMaxRetransmitSlot        | MaxRetransmitSlot        | u64                                 |
| getMaxShredInsertSlot       | MaxShredInsertSlot       | u64                                 |
| getRecentPerformanceSamples | RecentPerformanceSamples | Vec<PerformanceSample>              |
| getSlot                     | Slot                     | u64                                 |
| getSlotLeader               | SlotLeader               | core::Address                       |
| getSlotLeaders              | SlotLeaders              | Vec<core::Address>                  |
| getTransactionCount         | TransactionCount         | u64                                 |
| getVersion                  | Version                  | NodeVersion                         |

`ReadConfig` is shared where both commitment and minimum-context-slot apply.
Absent config means `[]`; an explicitly empty config means `[{}]`. No default
commitment is inserted. LeaderSchedule retains Dart's explicit positional null:
`LeaderSchedule(None, None)` sends `[null]`. Identity filters and production slot
ranges serialize only supplied fields. Performance sample limits are 0..=720;
leader limits are 1..=5000. Invalid limits fail before transport access.

All counters use exact u64, feature identifiers use u32, and shred versions and
sample durations use u16. Leader schedules and production maps have validated
Address keys and exact integer values; production pairs have exactly two entries.
Cluster endpoints parse to SocketAddr, including IPv6. Parsing opens no sockets.

Required nullable snapshot incremental slot and epoch transaction count reject
missing fields. Version feature-set, node advertisements and the older optional
non-vote performance count retain Missing versus Present(None) versus
Present(Some(value)). Unknown object fields are ignored for forward compatibility;
known fields still reject wrong types, null where forbidden and overflow. The
central client rejects duplicate object keys and enforces body/depth budgets.
Health accepts the protocol's sole successful value `"ok"`; unhealthy nodes and
unsupported methods remain distinct remote errors.

The Alpenglow schema has required block and signature objects, exact 32-byte
block ID, exact 192-byte aggregate BLS signature, u64 slot and byte-valued rank
bitmap. Bitmap allocation remains bounded by the complete reply budget. This
parses a certificate; it does not verify BLS, validator membership or finality.
Explicit result null remains distinct from Method not found (-32601).

## Source pins and differences

Dart source is pinned to 0138bb956d2d81ccc0c56edc602401c0df2f2b46. Original
per-file hashes and canonical IDs remain in rpc-catalog.tsv. All 19 official
Kit schemas were read at tag v8.3.0, resolved commit
7dfaf8827c217105d47aa1940893d301ad9f63c0, under
[packages/rpc-api/src](https://github.com/anza-xyz/kit/tree/7dfaf8827c217105d47aa1940893d301ad9f63c0/packages/rpc-api/src).
The official [Alpenglow schema](https://github.com/anza-xyz/kit/blob/7dfaf8827c217105d47aa1940893d301ad9f63c0/packages/rpc-api/src/getAgGenesisCert.ts)
provides the certificate fields; the official
[upgrade guide](https://solana.com/upgrades/alpenglow) distinguishes certificate,
null and unsupported-node error. These are schema facts; Rust code and literal
fixtures are newly authored, with no source algorithm copied.

Official solana-rpc-client-types4.2.2 and agave-votor-messages4.2.2 source also
informs exact integer widths, socket endpoints, optional historical fields and
certificate wire structure, at repository commit
c9c6f3287e26f24e3476e13e751aebe710191e89. These are private research inputs,
not SDK normal/build dependencies.

The actual Agave v4.3.0 RPC handler and its certificate fixture were additionally
read at tag commit 825efd18292aff6ffcf9daa0f7612f21b3531a72. Its
[RPC source](https://github.com/anza-xyz/agave/blob/825efd18292aff6ffcf9daa0f7612f21b3531a72/rpc/src/rpc.rs)
has SHA-256 ea327e7db369929af46f8fbe5810b70a5d15ff8a6cd1efadfc309886083b518d;
[consensus message source](https://github.com/anza-xyz/agave/blob/825efd18292aff6ffcf9daa0f7612f21b3531a72/votor-messages/src/consensus_message.rs)
has SHA-256 fe4593dcdb7d63dfda727096b85169c307db2279210013bbae3a7406af5c3f0e.
The newer Block schema explicitly renames block_id to blockId, matching Kit and
the RPC fixture; older internal 4.2 wire naming is not used as the JSON schema.
The server returns its finalized bank's optional certificate. Reading the handler
does not claim this unit ran an Agave4.3 validator.

The pinned Dart file contains no Alpenglow response model. Its cluster endpoint
fields are plain strings, now validated as protocol socket addresses. Its
unrestricted integer request limits are narrowed to documented server bounds.
The Kit TypeScript schema requires version feature-set and non-vote sample count;
the official Rust source allows null/older absence, which this SDK explicitly
preserves. Production response lastSlot is required by official Rust's response
schema, although the TypeScript response reuses an optional request-range type.
LeaderSchedule uses the conservative nullable result for both slot forms, as the
Dart catalog does. No JavaScript numeric precision limitation is inherited.

[The scoped ledger](rpc-node.tsv) maps all 110 canonical IDs to named Rust
contracts. Dart-specific hashCode and toString semantics remain pending; deriving
Eq for actual values does not claim those runtime-specific behaviors. The global
catalog/auditor and canonical capability ledger are unchanged for deterministic
central integration.

## Evidence and limits

Ten downstream integration tests execute every operation through public Client
using literal schema fixtures. They check exact request bodies, config omission,
null positions, all owned result shapes, u64 maximum, invalid numeric/domain
values, nullable presence, tuple arity, certificate byte widths, malformed result
fields, correlation, duplicate fields and remote errors. Existing client and
workspace tests remain enabled. Native tests, warning-denied docs/Clippy and
monostyle pass; MSRV1.89 and all three Wasm targets compile. Compilation does not
prove browser, WASI or interpreted-guest RPC execution.

No live external RPC, production adapter, subscription, mobile/device journey,
registry installation or full Phase4 completion is claimed. Core and transport
machinery and dependency manifests are unchanged. This slice adds schemas and
behavior through the existing executor-neutral boundary only.

## Official schema SHA-256

getAgGenesisCert.ts `0695f4b8e5fca854c0474fb718316457df5fa2f493a3df0759c2e40f0a5a3894`

getBlockHeight.ts `d7e40847440463bad0019650ce09c10431c9d853cf90320374c23dd997bb8841`

getBlockProduction.ts `b643d4c1f03b2e026c3652bbad91657023e8c0d0886e1ab2ec3f9de32dc79016`

getClusterNodes.ts `8dff46c5bd0654237a07cfab2a2a5e7581fd76db11aec7f9e1e19a78d5af7a96`

getEpochInfo.ts `bdffd213db3f150c1d5e307ddef64b7dff330a3d2822a1c14b541314fd12e77a`

getEpochSchedule.ts `0cfdb896a05b681f34117b500d1af26f9158be24cb6d8bacb25919bc7d559109`

getGenesisHash.ts `36533e338ad2405ca798f2c80643d3fad00cf98323c74a696f9ef05644f67b5e`

getHealth.ts `14878266242e7568304c2c656dca9ce006e366b87f5162f9f0f9aeaf41c0d0f4`

getHighestSnapshotSlot.ts `81a6fb10288c5deb8d4f382baf38d7168441b48965089dba92c15635cd49e630`

getIdentity.ts `0061cdee9a8bd1ca9ead8d11f5a7994bb723f047a5e80f97bb638e938fe3a6de`

getLeaderSchedule.ts `512070356f82e5e8d50a216ca9e23cf38c7ef1d3886fb37b31a8c71232abaa78`

getMaxRetransmitSlot.ts `36a1967a0a23573a7a89727733ae2429c010d40ae1cc39cf0b8f316cebca6aaf`

getMaxShredInsertSlot.ts `6d16a72cdb57be050c8665a0485f83103bf0a0f00fc95d0c6e79c6df4f99e2d2`

getRecentPerformanceSamples.ts `dd6fd885519e5497785d329692b00102b5406e255ac7e61d07e94b932520dbe3`

getSlot.ts `87742f7f1b9c51183e24f5f28eab4150f0e345f8368d073f2258b6ce2383ff30`

getSlotLeader.ts `912ba369ec3e767e41268ad43b452bc9178f9d9e1bc7206adc5ad34afe6da601`

getSlotLeaders.ts `8a3aa2cd5ace4d31a0f8dbda42d524b764f35050ea27cfcb3db9bc0c1206640b`

getTransactionCount.ts `9d7e4514dc1f05b53492996e29d6a86a25d52411b70048c8cb5e22e77b4d3b59`

getVersion.ts `8e3b584028f38b04fddb30db848fe00a4127f91eb321da8c86d01e49163f6794`
