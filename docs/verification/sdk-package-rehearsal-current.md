# SDK package rehearsal (current)

On 2026-11-04, a fresh local RW-07 rehearsal completed release preparation,
assembled 34 SDK archives in dependency order, and ran an external
offline-registry consumer. All 34 archives received a fresh member/style
analysis: 1,287 regular members, no archive-form exception. This is local
package evidence, not publication or whole-SDK release readiness. All publish
settings remain disabled; twenty crates retain `publish = false`.

The executable gate is `cargo xtask package-rehearsal CONFIG.json`. The fresh
snapshot is a disposable `git archive` of `8b1b342a` (merged main `886286ba`
plus the archive-form support in this change), prepared with Monochange only in
that copy. Its source-tree digest is
`2f01b423bd6152f82c4c41870036e8501d56e768a84850d5715d06c1a0eeca99`. That digest
covers `crates/`, `Cargo.toml`, `Cargo.lock`, `monochange.toml` and
`rust-toolchain.toml`, including the staged versions and changelogs. It includes
the native MWA sequence correction, so the historical archive hashes from the
earlier prepared run do not qualify this source by themselves. The repository
keeps its current versions; no release was committed, tagged or uploaded.

## Release preparation and the fixed dependency gap

The original `6606f80f` rehearsal exposed a real preparation failure: Wallet was
planned at 0.2.0, while Browser's target-specific dependency still required
`^0.1.0`. Monochange's discovered graph omitted that target-specific edge. It
wrote package versions but could not refresh the workspace lock.

Browser now inherits Wallet from `workspace.dependencies`, and Wallet's typed
`versioned_files` entry updates `workspace.dependencies.solana_kit_wallet` during
preparation. The shared dependency disables default features, preserving Browser's
previous setting; Wallet's default feature set is empty, so other workspace
callers gain no feature change.

With those changes, the fresh `monochange prepare` exited 0 in its disposable
copy: 35 packages and 64 changed files, including the root requirement and
refreshed lockfile. Wallet becomes 0.2.0; the other 33 included Cargo components
become 0.1.1; the dependency-free umbrella stays at 0.0.0. The excluded LeptosKit
crate also plans 0.1.1. The NPM package's planned files are preparation evidence
only; its packed-consumer proof remains a separate receipt.

The preview has 113 changesets, 29 release targets and no package publications.
Every target retains `release: false`. The initial failed apply and its logs
remain historical evidence; the fresh prepared copy is the input to this archive
rehearsal. No publish policy was enabled.

## Package set and archive review

Cargo metadata finds 35 SDK roots under `crates/`. Thirty-four enter this
rehearsal. The excluded `solana_kit_leptoskit` depends on the pinned LeptosKit Git
upstream `7437097c52aa68a3401bd378764e051ed4c9165a`, which is not a registry
package. That remains an external release prerequisite.

Cargo produced actual archives with
`cargo package --locked --offline --exclude-lockfile --no-verify`.
The [maintained archive gate](../../xtask/PACKAGE_REVIEW.md) freshly analyzed all
34 archives and 1,287 regular members. Every scored member met both 90-point
floors; the lowest member readability was 90.02 and the lowest member complexity
was 90.36. The lowest archive aggregates were 92.04 and 94.76. The gate now
accepts the real Cargo forms that previously remained outside it:

- Wallet's `assets/coinbase wallet.uri` and `assets/huobi wallet.uri` use
  ordinary interior ASCII spaces.
- Token-2022's four instruction paths use bounded GNU `L` long-name metadata for
  the immediately following regular file.

Those two archives were reviewed as fresh analyses, not as form exceptions.
Wallet has 59 regular members with readability 97.69 and complexity 100.00.
Token-2022 has 124 regular members with readability 96.45 and complexity 97.50.
Their archive SHA256 values are unchanged from the earlier checksum custody:
`9fc4b8eebec3d4083d992ef9ae2373ff2b3e98d0751e55638130aa2d386ab2e6` and
`c0f5c54696286d72d218add6499947b4bf554720eb1ffacd5a5378a3e1e010db`.

| Package                               | Version | SHA256                                                             | Members | Readability | Complexity |
| ------------------------------------- | ------- | ------------------------------------------------------------------ | ------: | ----------: | ---------: |
| `solana_kit`                          | 0.0.0   | `273a038af1ca4a46c6bf6ee687feea4af0519b1d6331ea624e2d8c5adaae6fbf` |       5 |      100.00 |     100.00 |
| `solana_kit_account_compression`      | 0.1.1   | `4deb2ce9293f2682d83714405294b28c6d17deda51afbbd17f513928d0982cfa` |      16 |       97.87 |     100.00 |
| `solana_kit_address_lookup_table`     | 0.1.1   | `a3fc36b8428c40ea487a5267cfa849a5ce882dd274f24a0785436fa6b5f657e3` |      15 |       98.24 |      97.26 |
| `solana_kit_anchor`                   | 0.1.1   | `704f1802f2bc260364360acd5b5f93ef09a31a9fc66519b1a02e695a5a6f4005` |      17 |       99.07 |     100.00 |
| `solana_kit_associated_token_account` | 0.1.1   | `6017190fdb60cc2e93dc2cdece2f92f125b00ff25026aa8e971c7a7965ec3acc` |      10 |       97.96 |     100.00 |
| `solana_kit_attestation_service`      | 0.1.1   | `1e4639fe452cfc23da00d265f82dc77f2d83240f0b6f33206a05463cd9f41c1e` |      19 |       95.23 |      96.63 |
| `solana_kit_browser`                  | 0.1.1   | `91984930c26a518a29c451254d980b374fed2237a7ad92d978a3b3550a117bbd` |      24 |       97.46 |      99.44 |
| `solana_kit_client`                   | 0.1.1   | `3b704b8d481acdd333f344af6ffc0db2c52aecc854c390fb2eb55f14a9f7396b` |     191 |       98.02 |      99.07 |
| `solana_kit_compute_budget`           | 0.1.1   | `93a98dfae610892ffc777cde02ef9382a2556d1f808c98db9ce182a5edc6d38e` |       8 |      100.00 |     100.00 |
| `solana_kit_config`                   | 0.1.1   | `f8317b973c14473be5089ad827306268930c65f11a3301c1fbc1835de5190f45` |      11 |       97.27 |     100.00 |
| `solana_kit_core`                     | 0.1.1   | `a4973d3f1c997dd7f2a4be1e2bc9811d49124a0875618f1a3fdec6f4e48b6c0a` |     103 |       98.28 |      99.53 |
| `solana_kit_dapp_publisher`           | 0.1.1   | `166f1d59386d756f4c12d5bf8228e85eec8541e6fccab82386cb3f0b60a67a32` |      90 |       98.84 |      97.97 |
| `solana_kit_dapp_publisher_cli`       | 0.1.1   | `b953d1562cf26e9ab1702104eb5fd49e3c9c1cfc53136326efe555c9e4e405f2` |       9 |       99.40 |     100.00 |
| `solana_kit_helius`                   | 0.1.1   | `3b24a22bc8abac20f674ca32c813acedc1438d9e25c6ebced4d08c478af087f5` |     143 |       98.52 |      99.62 |
| `solana_kit_jupiter`                  | 0.1.1   | `3da13a81c04be5c042c76f98b79e1dccb02190b6fbbb3d66761621a5f7a3bc07` |      22 |       99.45 |     100.00 |
| `solana_kit_loader`                   | 0.1.1   | `8d748cbecff2c5db8fca132d7ca58dfeaad5a4ad0964145a7b9cc64e08c5d722` |      33 |       99.39 |      98.92 |
| `solana_kit_memo`                     | 0.1.1   | `e63b9b17028b83fe94410aaacce4f99e080108a77167e75decec94828b4db1db` |       8 |      100.00 |     100.00 |
| `solana_kit_mpl_bubblegum`            | 0.1.1   | `f726736a627e5c79e4680550c42f3bc899f55a8ddc672e1ff59facd1e3ab3fa0` |      37 |       95.36 |      97.38 |
| `solana_kit_mpl_core`                 | 0.1.1   | `cc8f2615b3096289f6a449276da5a79cce0303469b1f049f5414094fe331034b` |      17 |       96.11 |      97.86 |
| `solana_kit_mpl_token_metadata`       | 0.1.1   | `e998842906ffc43566d859bc39e8f14829249fca382a6f56b343077890dc42f7` |      20 |       92.04 |      97.80 |
| `solana_kit_native`                   | 0.1.1   | `c6a9e18122641b0e93b561cd98d992c88c5f07bdf2e6d994b00dc5d662186d5f` |      68 |       98.73 |      99.06 |
| `solana_kit_pyth`                     | 0.1.1   | `5b45ce4fcd0713a0c5480eacfc57fc99784625c008be17e82336d44f66475789` |      27 |       97.79 |     100.00 |
| `solana_kit_signers`                  | 0.1.1   | `7cf66874448c9377de36c6a64bb38eac844e3fc46fbd314734fc06a5d70ce2a8` |      24 |      100.00 |     100.00 |
| `solana_kit_sns`                      | 0.1.1   | `aad5eb43cd850729a2187087695a434178a5a44061058c850e90db65a56dd49a` |      13 |       99.45 |      98.29 |
| `solana_kit_squads`                   | 0.1.1   | `a3c28cb21f3e9a44d800e810d6de195508520d8ec74de5d15e37e50580149300` |      19 |       94.41 |      96.81 |
| `solana_kit_stake`                    | 0.1.1   | `6a418cbf3a6688711036d841de2fd4d333851369ebf2005111f78c4ef2e7cb27` |      41 |       98.88 |      99.36 |
| `solana_kit_subscriptions`            | 0.1.1   | `2574b05e9bcfebf04d516c1e91c8f6c6cfdcf257fe4798b83e554032e4806cb7` |      24 |       93.91 |      94.76 |
| `solana_kit_surfpool`                 | 0.1.1   | `a8bed322dd24af2a96a6bdfb4f62b15bf82763dc6c414ef785ea259fe0a40947` |      37 |       99.80 |     100.00 |
| `solana_kit_system`                   | 0.1.1   | `5e2dfefc3b0264d7eef5c99eed52e261b7782fa462c062dd800c94c072ec20d0` |      11 |       96.42 |     100.00 |
| `solana_kit_test_support`             | 0.1.1   | `00859fc548538f87e73e39f790ee882d708f7c0a73e2e864824398db29eb2073` |      12 |       97.53 |      98.28 |
| `solana_kit_token`                    | 0.1.1   | `dac1f4d1bd59f2d754f09330cd17f46b90343d2ee0a599f79a126f9efadfc1c8` |      15 |       97.70 |      99.10 |
| `solana_kit_token_2022`               | 0.1.1   | `c0f5c54696286d72d218add6499947b4bf554720eb1ffacd5a5378a3e1e010db` |     124 |       96.45 |      97.50 |
| `solana_kit_wallet`                   | 0.2.0   | `9fc4b8eebec3d4083d992ef9ae2373ff2b3e98d0751e55638130aa2d386ab2e6` |      59 |       97.69 |     100.00 |
| `solana_kit_wasi`                     | 0.1.1   | `d3ca758df9fd7f82a2d264d2a14fbd436418b90f3367720a296adc1951d64a11` |      15 |       98.70 |     100.00 |

The earlier prepared snapshot started from `17311782`, added three
release-metadata changes and had source digest
`398a35ae80b0e76b2a4b023cbd451de8a3ef4a9a9feef6f811cb0fe7d774ad8a`. Its
32 fresh reviews covered 1,103 members; Wallet and Token-2022 were checksum-only
form limitations. Four archive hashes differ in the fresh snapshot because the
source changed after that run:

| Package                         | Historical SHA256                                                  | Fresh SHA256                                                       |
| ------------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------ |
| `solana_kit_core`               | `4c31af052a50d38e82631e8e9c525f0b1a353f145b2d02bc106f3cfbd2ad4e8d` | `a4973d3f1c997dd7f2a4be1e2bc9811d49124a0875618f1a3fdec6f4e48b6c0a` |
| `solana_kit_dapp_publisher`     | `30dbbdd15c41bb4fbb7cf21babc7f37dc336bc56c3bcd17c7a1eef6e116ca58c` | `166f1d59386d756f4c12d5bf8228e85eec8541e6fccab82386cb3f0b60a67a32` |
| `solana_kit_dapp_publisher_cli` | `59fe4f91e055f4a68cc36d73eb08632e9a29251ab914a8a280fe44964568fdd8` | `b953d1562cf26e9ab1702104eb5fd49e3c9c1cfc53136326efe555c9e4e405f2` |
| `solana_kit_native`             | `5c9ad1c0b56998039c7d9671be26004837198eeb2fa0979b3a39985377f8f29f` | `c6a9e18122641b0e93b561cd98d992c88c5f07bdf2e6d994b00dc5d662186d5f` |

The other 30 archive hashes are unchanged. Historical evidence remains under
`/tmp/solana-kit-package-prepared-run/evidence`; corrected publisher archives
and the isolated install remain under
`/tmp/solana-kit-publisher-version-run/evidence`.

## Offline registry and consumer evidence

The fresh local registry contains 207 packages: the 34 SDK archives and 173
checksum-verified cached dependencies. Each external archive matches both its
lockfile checksum and exact sparse-index record. The Cargo home admits cached
inputs, but rejects credentials, configuration files and pre-existing unpacked
registry sources. Output namespaces cannot overlap or resolve into the source
tree through aliases or parent traversal.

The consumer uses exact registry versions with a Cargo source replacement,
without SDK path dependencies or `[patch]`. Its default configuration executes
9 tests and the selected-feature configuration executes 12. Assertions cover
literal address/message/System/Memo/ATA bytes, Anchor discriminators, wallet
metadata, signer message ownership, scripted client correlation, owned-loopback
Native HTTP, crypto, MWA association metadata and a two-leaf Token plan.

Cargo metadata is audited separately for both configurations: 10 resolved SDK
packages by default and 12 with selected features. The latter adds Compute Budget
and Surfpool. Every resolved SDK PackageID has a registry source, a manifest under
the fresh Cargo home, and a consumer-lock checksum matching its archive. All
reviewed unpacked members are rehashed against archive receipts; there is no
checksum-only archive exception. A successful command with no executed tests is
rejected.

Actual Rust 1.89.0 (`29483883e`, 2025-08-04) executes the same 9 default and
12 selected tests. The harness resolves Cargo, rustc and rustdoc independently
through `rustup which --toolchain` and sets all three explicitly, so an inherited
Rust 1.99 compiler override cannot invalidate the MSRV result. The compiler's
verbose version and both executions are retained separately.
The consumer build script compares Cargo's actual compiler with the selected
version. The replay passes on 1.89; deliberately using 1.99 with a 1.89 expectation
fails with `MSRV compiler mismatch` before any consumer tests can pass.

The portable Core/Client/Signers/Wallet consumer also passes a locked offline
`wasm32-unknown-unknown` check. This is compilation, not Wasm execution or a proof
of every feature combination.

Fresh missing-Core and corrupted-Client controls use independent Cargo homes,
registry namespaces and compiler targets. They fail for the intended missing
archive and checksum diagnostics, each naming the affected package. No cached
positive build is accepted as a negative result.

## Dry-run outcomes and remaining release prerequisites

Only offline `cargo publish --dry-run --no-verify` was executed, with no registry
credential or upload. Twenty packages refuse because of `publish = false`;
fourteen stop at the offline registry API requirement. A direct dependent package
without the local registry reports
`no matching package named solana_kit_compute_budget found`.
Unknown failures are recorded as failures, not mislabeled as missing packages.

The initial `monochange publish packages --dry-run` had no committed release
record in its ancestry. Successful disposable preparation is not a published
release, and this rehearsal does not manufacture a release commit or credentials.

Remaining prerequisites are:

1. Publish or replace the LeptosKit Git upstream before a whole-SDK registry closure
   can exist.
2. Decide which of the twenty private crates should ship and resolve the umbrella's
   0.0.0 reservation. This gate deliberately leaves all publication disabled.
3. Make the required internal versions available in the real registry, then
   perform authorized registry-API dry runs.
4. Recheck immutable publish archives with their generated lockfiles; this
   rehearsal uses `--exclude-lockfile` to assemble unpublished dependencies.

The earlier correction run also installed the CLI archive into an isolated root
from its fresh 207-package registry, and `--help` exited 0. The first install
exposed a version-display defect: Cargo recorded package version 0.1.1, but help
and version output reported 0.1.0 from a hard-coded library constant. The
correction and its requalification follow.

## Publisher CLI version correction

`solana_kit_dapp_publisher_cli` now reports its own Cargo package version in
`--version` and the help heading. The executable uses its own
`env!("CARGO_PKG_VERSION")`; the retained public `CLI_VERSION` constant follows
the library package metadata and documents that it is not an executable version.
No runtime dependency, argument parser or public function changed.

`tests/version_display.rs` runs the actual executable and compares both outputs
with the version read from that binary package's authored `Cargo.toml`. A
disposable staged copy with CLI 9.8.7 and library 1.2.3 also passes, and its
built executable prints `solana-kit-publisher 9.8.7`; that split was never
committed. The old installed reproducer still shows the defect: its package
record says 0.1.1 while its output says 0.1.0.

The fresh rehearsal includes both corrected archives. The library has 90 regular
members with readability 98.84 and complexity 97.97; the CLI has 9 members with
readability 99.40 and complexity 100.00. Exact archive SHA256 values are:

| Package                         | SHA256                                                             |
| ------------------------------- | ------------------------------------------------------------------ |
| `solana_kit_dapp_publisher`     | `166f1d59386d756f4c12d5bf8228e85eec8541e6fccab82386cb3f0b60a67a32` |
| `solana_kit_dapp_publisher_cli` | `b953d1562cf26e9ab1702104eb5fd49e3c9c1cfc53136326efe555c9e4e405f2` |

The earlier correction run used the same lock closure, metadata and cached
inputs, replacing only these two archive inputs. Every unpacked corrected member
matched its reviewed receipt. A fresh Cargo home and install root then installed
package ID
`solana_kit_dapp_publisher_cli 0.1.1 (registry+https://github.com/rust-lang/crates.io-index)`;
`--version` and `--help` both start with `solana-kit-publisher 0.1.1`. The
installed binary SHA256 is
`d925eb1f70a93649c382a955155e73bfac64c08313c3c466f232fd0f46a154eb`.

These archives are still created with `--exclude-lockfile`. The install resolved
80 packages because no lockfile is published, so this is not immutable publish
artifact evidence and does not prove real crates.io availability. The fresh
34-archive run now requalifies every archive against the current source; the
earlier correction run remains the isolated CLI install proof.

The rehearsal does not prove public-registry CLI availability, Linux/Windows execution,
production network behavior, publication, or every feature of every SDK root.
Registry resolution is evidenced by PackageIDs, checksums and manifest paths,
not a filesystem read-denial sandbox around the source checkout.

## Local gates and reproduction

The xtask unit/CLI suites, warning-denied Clippy and rustdoc, repository formatting,
Monochange check and the full style gate pass. Workspace checks and the existing
dependency-graph and single-core consumer controls are recorded alongside the
package evidence. The [single-core receipt](core-package-delivery.md) and Codama's
packed-consumer receipt retain their separate scopes.

The JSON configuration supplies absolute paths for `workspace`, `evidence`,
`target`, `registry`, `consumer`, `cargo_home`, cached archives, sparse index,
and exact analyzer/gzip binaries and SHA256 values. The sparse-index path is the
directory containing `.cache`. It optionally selects `msrv`, `wasm_targets`
and `publish_dry_run`.

```sh
monochange check
monochange preview --format json
# Run prepare only in an owned disposable source copy.
monochange prepare
cargo xtask package-rehearsal /absolute/path/to/rehearsal.json
```

The gate emits the source digest, package plan, archive/review records, lock
closure, default and selected consumer audits, command outputs and negative
controls. `registry-fixture` preserves its single-archive configuration;
`additions` is optional.

The fresh worker's logs and standalone Wallet/Token-2022 reviews are under
`/tmp/solana-kit-rw07-final-run`; reviewed preparation, corrected MSRV and local-gate
logs from the earlier run remain under `/tmp/solana-kit-package-parent-evidence`.
The fresh prepared registry evidence is under
`/tmp/solana-kit-rw07-final-run/evidence`. Build targets are disposable; hashes, public
source snapshots and receipts are retained outside them.
