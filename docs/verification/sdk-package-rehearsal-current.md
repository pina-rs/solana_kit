# SDK package rehearsal (current)

This receipt covers the local RW-07 package and release rehearsal for the
implemented `crates/*` packages. It is not a publication, a registry reservation,
or a whole-SDK completion claim. Normal publishing remains disabled: Monochange
keeps `release: false`, every package publication is off, and twenty crates retain
`publish = false`.

The executable gate is `cargo xtask package-rehearsal CONFIG.json`. It packages the
selected crates in dependency order, reviews their actual archives, installs the
complete closure into a fresh offline local registry, and builds and executes an
external consumer. The current run used Rust 1.99.1 and MSRV 1.89.0, a fresh
task-owned `CARGO_HOME` with no credentials, and an exact `git archive` of commit
`6606f80f` as the source tree. The source-tree digest over `crates/`,
`Cargo.toml`, `Cargo.lock`, `monochange.toml` and `rust-toolchain.toml` is
`8730aad3609ce00b5464951deba232a6f980d9c774aea881ba365a14ef462629`.
No archive, registry cache, staging tree or build output is committed. The
transient archives, registry namespaces, Cargo cache and compiler targets were
removed after the hashes below were recorded; the retained command logs, summaries
and JSON evidence remain outside the repository.

## Monochange planning and apply

`monochange check` passed with zero issues: 109 Cargo files under seven rules and
111 changesets under twenty rules. `monochange preview --format json` reported 35
packages, 29 release targets, 111 changesets, 62 planned changed files, and zero
`package_publications`. Every one of the 29 targets had `release: false`. The plan
is one major bump (`solana_kit_wallet 0.2.0`) and 34 minor bumps (`0.1.1`); the
0.0.0 umbrella reservation has no bump. The preview JSON SHA256 is
`2e647114249e94b746c2b8237cabfa4b7064baa591d82a57546539a8ab02159d`;
the human diff SHA256 is
`497c85fcd3e4808941b232b89ba135b6de130e0e013ce52bae9099c3f77209c5`.

The supported apply was attempted only in a disposable staging tree made from the
same archived source. `monochange prepare` wrote 34 Cargo versions, 26 changelogs
and the Codama package/lock pair (62 files), then failed in its configured
`cargo update --workspace` command:

```text
error: failed to select a version for the requirement `solana_kit_wallet = "^0.1.0"`
candidate versions found which didn't match: 0.2.0
location searched: crates/solana_kit_wallet
required by package `solana_kit_browser v0.1.1 (crates/solana_kit_browser)`
```

The diff contains 33 `0.1.1` Cargo versions and one `0.2.0` version, but no changed
internal dependency requirement and no `Cargo.lock` change. `cargo update` therefore
cannot resolve the stale browser-to-wallet requirement. `monochange versions sync
--dry-run --format json` also reports zero changes. This is a release-plan
prerequisite, not a packaging failure: the computed major bump is not propagated
to the explicit path dependency. No dependency requirement was edited by hand and
no successful apply is claimed.

`monochange publish packages --dry-run` also fails with
`no monochange release record found in first-parent ancestry from HEAD`. The apply
diff SHA256 is
`4bce5a3d40f297a14037534aeae345f20e2f88c1ea0e06fb7a6c1dc9b9caf060`;
its command log SHA256 is
`1c768e58a1b961480971071de2ef827314c9939dc96116d184c7b6f943d9f6d5`.
Because the staged tree is not a valid post-apply workspace, the package rehearsal
uses the exact unmodified 0.1.0 source snapshot.

## Package set and archive review

Cargo metadata identifies 35 SDK roots under `crates/`. Thirty-four enter the local
rehearsal: twenty retain `publish = false`, thirteen are publishable components, and
the remaining publishable entry is the 0.0.0 `solana_kit` reservation. The only
excluded root is `solana_kit_leptoskit 0.1.0`, whose pinned upstream
`git+https://github.com/ifiokjr/leptoskit?rev=7437097c52aa68a3401bd378764e051ed4c9165a`
is not a registry package. It remains an external release prerequisite.

Cargo created the 34 archives in dependency order with
`cargo package --locked --offline --exclude-lockfile --no-verify`. The consumer
then proves the resulting bytes build; archive creation alone is never treated as
compile acceptance. The maintained [archive gate](../../xtask/PACKAGE_REVIEW.md)
performed 32 fresh analyses over 1,090 regular members. Every supported source and
document met the 90-point readability and complexity floors; no floor was lowered.

Two archives could not enter the maintained gate because of archive form, not a
score or a bad source byte:

- `solana_kit_wallet 0.1.0` contains `assets/coinbase wallet.uri` and
  `assets/huobi wallet.uri`. The gate rejects filenames containing spaces.
- `solana_kit_token_2022 0.1.0` uses GNU long-name records for three instruction
  source paths longer than 100 bytes. The gate explicitly rejects GNU `L`/`K`
  records.

Both archives are still materialized and checksum-locked in the registry. The
consumer executes Wallet Standard metadata and MWA behavior from the wallet archive;
no style review is claimed for either archive. The gate now distinguishes a GNU
long-name rejection from a nonregular member, but it still rejects the archive.

Representative reviewed archive hashes:

| Package                 | Version | SHA256                                                             | Members      |
| ----------------------- | ------- | ------------------------------------------------------------------ | ------------ |
| `solana_kit_core`       | 0.1.0   | `25d56493e18f6a4a517199eb15ae30763729eb20cef34f9507b584e0cf25362e` | 103          |
| `solana_kit_client`     | 0.1.0   | `e92a9fb84204bfbe764debb3c67b5d086352c90c5fa14c01daea47217804ad65` | 191          |
| `solana_kit_signers`    | 0.1.0   | `77aa81a238d53ecf4366b67e5ae3cb02caf9098844cd78ac0bb527fd58926524` | 24           |
| `solana_kit_native`     | 0.1.0   | `58ca348b8956f3c85aaa8513cebb9a6f511116fd94d2e341d36b18b989a4eabc` | 68           |
| `solana_kit_system`     | 0.1.0   | `70929fbd5ceb21ad2cdcce7af05898af1a4b06989f86b017ed09601ba8eb4799` | 11           |
| `solana_kit_token`      | 0.1.0   | `8a691cb6dabf8a0e805926bff26e9be0c530ff0162c70d27f6af96456140110e` | 14           |
| `solana_kit_anchor`     | 0.1.0   | `470a77164e380b6f4449a9bae514b51f11029ee7adb040a03bb9049abc3eafa9` | 17           |
| `solana_kit_wallet`     | 0.1.0   | `e75cc64d16f46f9e59a66ad969cc635e9f277bf1f0cf447cb6ea1b5253737180` | form-limited |
| `solana_kit_token_2022` | 0.1.0   | `08ab548098496c23ae1727e3e427e0dfdeca8e5c8ee4ec26b7271271158c679c` | form-limited |

The complete archive-set digest
`b9eff25dd008efd478b5b451864cfb28f2e6d65b306d5c0d862eb4c54bdac50a` is the SHA256
of the sorted per-archive SHA256 list.

## Offline registry and external consumer

`registry-fixture` installed all 34 selected archives plus 173 checksum-verified
dependencies from the exact closure lock, for 207 packages. Each external archive
matched both its `Cargo.lock` checksum and its cached sparse-index record. The
consumer uses only a `[source.crates-io]` local-registry replacement with exact
`=0.1.0` version requirements. It contains no `[patch]`, path dependency, original
checkout path or registry credential.

The consumer depends on Core, Client, Signers, Wallet, Native, Token, System, Memo,
Associated Token Account and Anchor. Its default closure executed 9 tests and its
selected feature closure executed 12. The selected closure enables Core
crypto/fixed-points/text-codecs/bigint-codecs/keccak, Client
confirmation/resource/parsed/reactive features, Signers crypto, Wallet MWA, Native
MWA and Surfpool, Token plans and ATA crypto. The tests assert independent literals:
address and legacy message bytes, System/Memo/ATA instruction bytes, Anchor
discriminators, Wallet metadata and MWA association round trips, immutable signer
messages, a scripted `getBalance` client exchange, a caller-owned
`127.0.0.1:0` Native HTTP exchange, Core signing and PDA rejection, and a two-leaf
Token transfer plan. No production network or user key is used.

Ten resolved SDK packages were audited through Cargo metadata. Every selected package
has a registry source, its manifest is under the fresh Cargo home rather than this
checkout, and its consumer lock checksum equals the reviewed archive hash. The audit
rehashed 446 unpacked members for the nine packages with a fresh review, including all
103 Core, 191 Client, 24 Signers, 68 Native, 14 Token, 11 System, 17 Anchor, 10 ATA
and 8 Memo members. Wallet is checksum-locked but has no member audit because its
archive is form-limited.

The same consumer passes under Rust 1.89.0 in both default and selected modes. A
portable consumer depending on Core, Client, Signers and Wallet passes
`cargo check --target wasm32-unknown-unknown --locked --offline`.

Fresh negative registries use separate Cargo homes and target directories. The
missing-Core control fails while downloading
`registry-negative-missing/solana_kit_core-0.1.0.crate` with `No such file or
directory`. The corrupted-Client control changes only a gzip timestamp byte and
fails with `failed to verify the checksum of solana_kit_client v0.1.0`.

## Publish dry-run outcomes

Only dry-run was executed, offline, with the fresh Cargo home, `--no-verify`, and
no `--registry` or `--index`. Twenty packages fail with `publish = false`. The
remaining fourteen reach Cargo's registry API check and fail with
`attempting to make an HTTP request, but --offline was specified`; no upload or
registry mutation occurred. The umbrella is included in that fourteen because its
manifest is publishable, but its current 0.0.0 reservation is not a release.

A direct `cargo package -p solana_kit_client` without the local registry fails on
`no matching package named solana_kit_compute_budget found` in the crates.io index.
This is the concrete absent-registry-dependency evidence for a dependent package.
No local registry API fixture was used, because a fixture pass would simulate
availability rather than prove it. The local registry consumer is separate local
evidence, not a crates.io readiness result.

## Remaining release prerequisites

1. Fix the Monochange major-bump dependency update before any apply: the Wallet
   0.2.0 plan must update browser's explicit `solana_kit_wallet` requirement, or
   the release must choose a compatible bump. Then rerun apply and lock refresh.
2. Publish or replace the pinned `leptoskit` Git upstream in a real registry before
   any whole-SDK closure can be assembled from registries.
3. Resolve the twenty `publish = false` decisions and the 0.0.0 umbrella
   reservation for the crates that should actually ship.
4. Provide actual registry versions for every internal dependency; the direct
   dependent package currently has no crates.io Core/Compute Budget versions.
5. Complete `cargo publish --dry-run` against a real registry API with authorized
   credentials only during an approved release, never from this gate.
6. Decide whether the archive gate should support spaces and GNU long-name records;
   until then Wallet and Token-2022 have no style claim.

## Reproduction

The configuration is an absolute-path JSON file with `workspace`, `evidence`,
`target`, `registry`, `consumer`, `cargo_home`, `cache`, `sparse_index`, `engine`,
`engine_sha256`, `policy`, `gzip`, `gzip_sha256`, optional `msrv`, optional
`wasm_targets`, and optional `publish_dry_run`. `sparse_index` names the Cargo
sparse-index root (the directory containing `.cache`), not `.cache` itself.

```sh
cargo xtask package-rehearsal /absolute/path/to/rehearsal.json
```

The command creates one fresh evidence tree containing `packages.json`,
`metadata.json`, `closure-Cargo.lock`, `summary.json`, `consumer-audit.json`,
`commands/`, `archives/` and `reviews/`. Re-run `cargo xtask package-review` on any
retained archive for an independent membership and source review. The
[existing core receipt](core-package-delivery.md) remains the single-core consumer
record; `registry-fixture` keeps its original single-archive configuration, and its
`additions` array is optional.

Dependent archives use Cargo's `--exclude-lockfile` option so internal unpublished
versions are not required during assembly. The archive source and normalized
manifest are real; the lockfile is generated metadata and must be rechecked when a
release actually packages immutable publish artifacts. The rehearsal proves registry
resolution through PackageID sources and fresh-Cargo-home manifest paths, not through
a read-denial sandbox around the original checkout.

This rehearsal does not prove publication, a clean installation of the CLI, Linux
or Windows execution, or every feature combination of every SDK crate. It proves the
selected archive closure, the offline registry consumer, the negative controls, and
the exact blockers above for the recorded source and tool versions.
