# Publisher CLI v0.1.1 release candidate (current)

On 2026-11-05, the current base plus the external-consumer fix was prepared in a
disposable Monochange copy and requalified locally. This is a review candidate,
not a published release: no tag, registry upload, GitHub release, binary upload,
push, secret access or publication authorization occurred. The prepared
whole-workspace tree is retained for review; the repository keeps its current
versions.

This receipt is about the Publisher library and CLI packages and the packaging
gate. It does not claim whole-SDK release readiness. The SDK remains
450 complete, 72 partial and 17,426 pending parity rows. A locally installable
GitHub CLI archive is also distinct from registry publication, prebuilt GitHub
Release binaries and whole-SDK parity.

## Candidate identity

- Base: `91314cc6` (`test(addresses): qualify constants and curve contracts`).
- Candidate source commit: `e348f0be` (signed OpenPGP, `G`) with tree
  `71f00a241d5fe6839e825cda060ebab2012e358c`. It adds only the external
  packaged-core consumer correction. This receipt is a documentation-only commit
  on top; it changes no package, lock or release metadata, so the prepared digest
  and archives below still identify the candidate.
- Monochange: `0.21.0`, configured `monochange.toml`; preparation ran in a
  disposable `git archive` copy, not in the parent checkout.
- Prepared source tree digest:
  `b82379ebf021914de9495cbb94ac57b5735dd1416435b9d107681f555d9348c8`.
  The digest covers `crates/`, `Cargo.toml`, `Cargo.lock`, `monochange.toml` and
  `rust-toolchain.toml`, including the staged versions and changelogs.
- Preparation result: 35 included packages, 64 file writes and 114 consumed
  changeset deletions. The root `Cargo.lock` was refreshed.
- No package, publish, workflow or security setting was changed. All release
  targets retain `release: false`.

| Component                         | Prepared version |
| --------------------------------- | ---------------: |
| `solana_kit_dapp_publisher`       |            0.1.1 |
| `solana_kit_dapp_publisher_cli`   |            0.1.1 |
| SDK group (`sdk`)                 |            0.1.1 |
| `solana_kit_wallet`               |            0.2.0 |
| `solana_kit_core` and 30 siblings |            0.1.1 |
| `solana_kit` umbrella             |            0.0.0 |
| Codama renderer                   |            0.1.1 |
| `solana_kit_leptoskit`            |            0.1.1 |

The root requirement for Wallet moved to `0.2.0` through its configured
`versioned_files` entry. Other internal root requirements stay at `^0.1.0`,
which admits the prepared `0.1.1` versions. The freshly resolved lockfile records
the prepared versions; `cargo metadata --locked` succeeds in the disposable tree.

The prepared tree is `/tmp/solana-kit-cli-release-readiness/prepared/source`.
The reviewable release patch (without generated `.monochange` state) is
`/tmp/solana-kit-cli-release-readiness/prepared/candidate-release.patch`, SHA256
`c7fa59501c69b1f4618d2c6d17d8cc094570c6cbaa284f2a3fdc6ddffaba0d49`.
It applies cleanly with `patch -p1` to a fresh `e348f0be` archive. The complete
prepared diff, including Monochange's release record and local index, is
`/tmp/solana-kit-cli-release-readiness/prepared/candidate-full.patch`, SHA256
`34f597ccf9ac152163fbe1f7fac22d0f4a391c0976f728d13f3b1098c5de60d0`.
The `.monochange/local` index is disposable state and is not a release artifact.
After preparation, a second `monochange preview` reports `no markdown changesets
found`; that is expected after the 114 changesets are consumed, not a version
failure.

## Packaged-core consumer defect and correction

The old external consumer had two independent `0.1.0` literals: it extracted
`target/package/solana_kit_core-0.1.0.crate` and generated a dependency on
`solana_kit_core-0.1.0`. A prepared `0.1.1` workspace therefore packaged `0.1.1`
correctly and then failed to extract it. If a stale `0.1.0` archive happened to be
present, the old consumer silently compiled and tested the stale package while
Cargo had just packaged `0.1.1`.

`xtask/src/consumer.rs` now:

1. reads the core package name and version from Cargo workspace metadata;
2. requires the exact metadata-derived `.crate` file and reports the actual
   package directory when it is absent;
3. extracts the archive and parses the extracted `Cargo.toml`, failing when its
   name or version disagrees with metadata;
4. substitutes the exact extracted directory into
   `xtask/consumer/Cargo.toml.txt`, which no longer contains a version.

Three focused tests cover the template, Cargo's resolution of an unequal staged
version (`0.2.7`) and rejection of an extraction mismatch. Regression evidence:

- Old consumer on the prepared `0.1.1` tree fails at
  `tar .../solana_kit_core-0.1.0.crate: No such file or directory`.
- Old consumer with a deliberately stale `0.1.0` archive in the target succeeds
  but compiles `solana_kit_core-0.1.0` under the consumer while Cargo packaged
  `0.1.1`.
- New consumer with the same stale archive present packages and uses
  `solana_kit_core-0.1.1`, and all 10 external tests pass.

Logs are under `/tmp/solana-kit-cli-release-readiness/logs/`:
`consumer-current.log`, `consumer-prepared-new.log`,
`consumer-prepared-legacy.log` and `consumer-prepared-legacy-stale.log`. The
current consumer passes 10 tests and the prepared consumer passes 10 tests; no
repository flags or path patches are used.

## Release-note scope

The CLI package has `changelog = false`, so preparation does not render a
CLI-specific changelog. The library changelog for `solana_kit_dapp_publisher`
0.1.1 contains the version-display fix under Fixes and the typed portal and
normalized-model features under Features. The CLI-only changesets
(`publisher-cli.md` and the CLI half of `publisher-package-version.md`) are
consumed without a generated CLI note.

Concrete CLI behavior for this candidate is therefore documented by the CLI
README and executable help, not by a generated release note: it starts or resumes
a caller-owned publication, uploads a local APK through one presigned PUT,
validates and signs prepared mint and collection-verification transactions,
creates the attestation from portal block data, and submits store review. It does
not retry mutations or submit transactions automatically, and it never prints or
checkpoints signing or API keys. A human-authored CLI release note is still
needed if the release announcement must describe these commands; that is a
documentation gap, not a code failure.

## Archive and install evidence

The rehearsal config is
`/tmp/solana-kit-cli-release-readiness/rehearsal-config.json`; evidence is under
`/tmp/solana-kit-cli-release-readiness/rehearsal/evidence`. All 34 archives were
built from the prepared source with
`cargo package --locked --offline --exclude-lockfile --no-verify` and received a
fresh maintained-gate analysis: 1,293 regular members, no archive-form exception.
The lowest archive aggregates are 92.04 readability and 94.76 complexity.
Every scored member and archive category is at least 90.

| Package                               | Version | SHA256                                                             | Members | Readability | Complexity |
| ------------------------------------- | ------- | ------------------------------------------------------------------ | ------: | ----------: | ---------: |
| `solana_kit`                          | 0.0.0   | `273a038af1ca4a46c6bf6ee687feea4af0519b1d6331ea624e2d8c5adaae6fbf` |       5 |      100.00 |     100.00 |
| `solana_kit_core`                     | 0.1.1   | `7704d73442b5b343925dec37232d63b519f147346c98b824611b51886e0d4229` |     103 |       98.24 |      99.53 |
| `solana_kit_account_compression`      | 0.1.1   | `51e1144ca666194942e38d9635b97a74985ce50e9a0c6e4f5cb1bbfcebb67fab` |      16 |       97.87 |     100.00 |
| `solana_kit_address_lookup_table`     | 0.1.1   | `2b24b17e5b1796ff8d0e63de774a51a8105767da41cf67b2d0ad982c14b21a13` |      15 |       98.24 |      97.26 |
| `solana_kit_anchor`                   | 0.1.1   | `704f1802f2bc260364360acd5b5f93ef09a31a9fc66519b1a02e695a5a6f4005` |      17 |       99.07 |     100.00 |
| `solana_kit_associated_token_account` | 0.1.1   | `6017190fdb60cc2e93dc2cdece2f92f125b00ff25026aa8e971c7a7965ec3acc` |      10 |       97.96 |     100.00 |
| `solana_kit_attestation_service`      | 0.1.1   | `f77622507ebe0202bd6430d4587359e03b4fb818cc0d26c6fee975025d55e515` |      19 |       95.23 |      96.63 |
| `solana_kit_compute_budget`           | 0.1.1   | `93a98dfae610892ffc777cde02ef9382a2556d1f808c98db9ce182a5edc6d38e` |       8 |      100.00 |     100.00 |
| `solana_kit_config`                   | 0.1.1   | `d6fc67b4ca916cd4d15bde19ed945a0a60b39485a6fc44f7828c217bd44150d1` |      11 |       97.27 |     100.00 |
| `solana_kit_loader`                   | 0.1.1   | `989ae763577d1fa022590ae7e1c8f8a16d98e2a67cb99028d413e86a02222a49` |      33 |       99.39 |      98.92 |
| `solana_kit_memo`                     | 0.1.1   | `e63b9b17028b83fe94410aaacce4f99e080108a77167e75decec94828b4db1db` |       8 |      100.00 |     100.00 |
| `solana_kit_mpl_core`                 | 0.1.1   | `5be7b051bd11e3a11b9bbec7c8123a73f9d1cda377b5cd1a65fc8df3339dee8a` |      17 |       96.11 |      97.86 |
| `solana_kit_mpl_token_metadata`       | 0.1.1   | `39f53ab12fcb04e383771d44025c732e0370676d2d8fc636ce7b020c6c344754` |      20 |       92.04 |      97.80 |
| `solana_kit_signers`                  | 0.1.1   | `7cf66874448c9377de36c6a64bb38eac844e3fc46fbd314734fc06a5d70ce2a8` |      24 |      100.00 |     100.00 |
| `solana_kit_sns`                      | 0.1.1   | `8659290d31865732f71b2757187c0166a742e356c92dc54c82850fe66e45bffe` |      13 |       99.45 |      98.29 |
| `solana_kit_squads`                   | 0.1.1   | `1fbe8da92e3eb33cfa607f83dc90bb212c5e081a748933e44e7c7a634925f17d` |      19 |       94.41 |      96.81 |
| `solana_kit_subscriptions`            | 0.1.1   | `0c3645d2fda520933606891919785d06f1ceef516bfb5658a292923067878df9` |      24 |       93.91 |      94.76 |
| `solana_kit_system`                   | 0.1.1   | `5e2dfefc3b0264d7eef5c99eed52e261b7782fa462c062dd800c94c072ec20d0` |      11 |       96.42 |     100.00 |
| `solana_kit_client`                   | 0.1.1   | `a22e3cb05bcb0f1caf8974b3200d44906afc76f65aabbe3a020803ef13ba13fc` |     197 |       98.00 |      99.10 |
| `solana_kit_dapp_publisher`           | 0.1.1   | `166f1d59386d756f4c12d5bf8228e85eec8541e6fccab82386cb3f0b60a67a32` |      90 |       98.84 |      97.97 |
| `solana_kit_dapp_publisher_cli`       | 0.1.1   | `b953d1562cf26e9ab1702104eb5fd49e3c9c1cfc53136326efe555c9e4e405f2` |       9 |       99.40 |     100.00 |
| `solana_kit_jupiter`                  | 0.1.1   | `3da13a81c04be5c042c76f98b79e1dccb02190b6fbbb3d66761621a5f7a3bc07` |      22 |       99.45 |     100.00 |
| `solana_kit_mpl_bubblegum`            | 0.1.1   | `4b38cd550ba69275565f6e3098a1d7e837e656f89b95c47eac7bf14211a87e0a` |      37 |       95.36 |      97.38 |
| `solana_kit_pyth`                     | 0.1.1   | `5b45ce4fcd0713a0c5480eacfc57fc99784625c008be17e82336d44f66475789` |      27 |       97.79 |     100.00 |
| `solana_kit_stake`                    | 0.1.1   | `438b5832df7e89658ef72deeca5c8e368ec6f24baa251d13c7739f1be5670f84` |      41 |       98.88 |      99.36 |
| `solana_kit_helius`                   | 0.1.1   | `3b24a22bc8abac20f674ca32c813acedc1438d9e25c6ebced4d08c478af087f5` |     143 |       98.52 |      99.62 |
| `solana_kit_surfpool`                 | 0.1.1   | `a8bed322dd24af2a96a6bdfb4f62b15bf82763dc6c414ef785ea259fe0a40947` |      37 |       99.80 |     100.00 |
| `solana_kit_test_support`             | 0.1.1   | `00859fc548538f87e73e39f790ee882d708f7c0a73e2e864824398db29eb2073` |      12 |       97.53 |      98.28 |
| `solana_kit_token`                    | 0.1.1   | `7586e70c3922a1b847b90a9bbde5e0223837a1f81b1411e7167da183c818cb69` |      15 |       97.70 |      99.10 |
| `solana_kit_token_2022`               | 0.1.1   | `44a7072a1f69db72c94f16fb2c72b2ce6e277e34a81d95485346e2d29a36e411` |     124 |       96.45 |      97.50 |
| `solana_kit_wallet`                   | 0.2.0   | `9fc4b8eebec3d4083d992ef9ae2373ff2b3e98d0751e55638130aa2d386ab2e6` |      59 |       97.69 |     100.00 |
| `solana_kit_browser`                  | 0.1.1   | `91984930c26a518a29c451254d980b374fed2237a7ad92d978a3b3550a117bbd` |      24 |       97.46 |      99.44 |
| `solana_kit_native`                   | 0.1.1   | `c6a9e18122641b0e93b561cd98d992c88c5f07bdf2e6d994b00dc5d662186d5f` |      68 |       98.73 |      99.06 |
| `solana_kit_wasi`                     | 0.1.1   | `d3ca758df9fd7f82a2d264d2a14fbd436418b90f3367720a296adc1951d64a11` |      15 |       98.70 |     100.00 |

The local checksum registry contains 207 exact packages. The external consumer
passes 9 default and 12 selected-feature tests; Cargo metadata audits 10 and 12
SDK PackageIDs with registry sources and matching archive checksums. Real Rust
1.89.0 (`29483883e`, 2025-08-04) runs the same 9 and 12 tests with explicit
`CARGO`, `RUSTC` and `RUSTDOC` binaries. The portable Core/Client/Signers/Wallet
closure checks on `wasm32-unknown-unknown`. Missing-Core and corrupted-Client
controls use independent namespaces and fail with `failed to download` and
`checksum` diagnostics naming the affected package.

## Isolated CLI install

A fresh credential-free Cargo home with only a source replacement to the local
registry installed the corrected CLI archive:

- PackageID: `solana_kit_dapp_publisher_cli 0.1.1 (registry+https://github.com/rust-lang/crates.io-index)`
- Install root: `/tmp/solana-kit-cli-release-readiness/install/root`
- Binary SHA256:
  `3c6b6f71d4f5235415b85bb321ae440d2a02ea028e0fb3a156e54699f340a5e6`
- `--version`: `solana-kit-publisher 0.1.1`
- `--help` heading: `solana-kit-publisher 0.1.1`
- Black-box `version_display` tests: 2 passed, 0 failed.

The installed extracted CLI source matches the reviewed archive for every member
except Cargo's own `.cargo-ok` marker. The installed `src/main.rs` SHA256 is
`1ecfb12b63519061d40b3b11de7ce63c1db44ff5976b6be0bd0f74528777a298`
and the shared library `src/cli.rs` hash is
`aff60d7bfc11b280c0f9971117fed25f9041a1c533d18300cbcab18ffe3bc4c1`,
matching the reviewed member bytes. Installation resolved 80 packages because
the archive excludes `Cargo.lock`; it proves the local archive install and
version display, not immutable published-artifact behavior or crates.io
availability.

## Check results

Concrete toolchain: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`,
`cargo 1.99.1 (5f94df478 2026-08-27)`,
`rustdoc 1.99.1 (58cf8f9c3 2026-10-13)`. Every child command received explicit
`CARGO`, `RUSTC` and `RUSTDOC` paths, `CARGO_INCREMENTAL=0`,
`CARGO_BUILD_JOBS=2` and `CARGO_NET_OFFLINE=true`.

| Check                                                                                   | Result                                                                            |
| --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `cargo test --locked -p solana_kit_xtask --offline`                                     | Passed: 73 tests, 2 ignored (one explicit MSRV replay and one style-CI-only test) |
| `cargo clippy --locked -p solana_kit_xtask --all-targets --offline -- -D warnings`      | Passed                                                                            |
| `RUSTDOCFLAGS="-D warnings" cargo doc --locked -p solana_kit_xtask --no-deps --offline` | Passed                                                                            |
| `cargo xtask consumer` on `0.1.0` current                                               | Passed, 10 tests                                                                  |
| `cargo xtask consumer` on prepared `0.1.1` with stale `0.1.0` archive                   | Passed, 10 tests, used `0.1.1`                                                    |
| Publisher library + CLI `cargo test --all-features`                                     | Passed: 139 tests and doctests                                                    |
| Publisher library + CLI Clippy and rustdoc                                              | Passed                                                                            |
| `cargo fmt --all --check`                                                               | Passed                                                                            |
| `dprint check`                                                                          | Passed                                                                            |
| `monochange check` (current and prepared)                                               | Passed, 0 issues                                                                  |
| `cargo xtask style` (current)                                                           | Passed, 2,334 files, readability 97.24, complexity 98.54, every category >= 90    |
| Prepared workspace Clippy                                                               | Passed                                                                            |
| Prepared workspace rustdoc                                                              | Passed                                                                            |
| Prepared workspace tests excluding `solana_kit_xtask`                                   | Passed                                                                            |
| Initial prepared workspace tests including `solana_kit_xtask`                           | Failed at the internal-version policy; historical log retained                    |
| Final prepared workspace tests including `solana_kit_xtask`                             | Passed after the parent policy correction                                         |

The initial prepared-workspace failure was
`client_gate::current_client_and_jupiter_graphs_reject_tokio_on_every_target`.
`xtask/policy/client.txt` is an exact-version golden that still names
`solana_kit_client v0.1.0` and `solana_kit_core v0.1.0`, while the prepared tree
resolves `0.1.1`. The actual dependency closure is otherwise identical.
The parent correction compares the three explicitly named workspace packages
by name while retaining exact versions for every external dependency. Cargo.lock
governs workspace versions. New controls reject external-version drift and an
unexpected SDK package. The prepared source includes this tooling correction;
the package-source digest and all reviewed SDK archives remain unchanged.
The historical failing log is retained alongside the final parent rerun.
The final all-feature workspace test command exited 0 in
`logs/parent-prepared-workspace.log`. The final current-source xtask test,
warning-denied Clippy and docs also passed (`parent-xtask.log`,
`parent-clippy.log`, `parent-doc.log`); `parent-style.log` records all 2,334
authored files above the existing floors. These follow-up tooling changes do
not modify SDK package contents, version preparation or the archive hashes.

## Remaining limits

1. Candidate version files are retained for review and have not yet landed on
   the release branch. A release tag must identify committed version preparation.
2. `solana_kit_leptoskit` still depends on the unpublished
   `leptoskit_bridge` Git revision
   `7437097c52aa68a3401bd378764e051ed4c9165a`; it is excluded from the 34
   archives and must be published or replaced for registry closure.
3. Twenty included crates retain `publish = false`. The offline dry run records
   20 publish-disabled packages and 14 registry-API-offline packages; no
   publication decision or authorization exists.
4. Archives use `--exclude-lockfile`, so the CLI install resolves a fresh graph of
   80 packages instead of proving the immutable published dependency set.
5. No Linux/Windows execution, prebuilt GitHub Release binary, real crates.io
   availability, registry credential, signing operation, device run, RPC call or
   publication occurred. The CLI note for GitHub binary readiness must be
   authored separately from registry publication.
6. The SDK remains 450 complete / 72 partial / 17,426 pending. This candidate does
   not advance or claim whole-SDK parity.

## Evidence paths

- Prepared source and patches:
  `/tmp/solana-kit-cli-release-readiness/prepared/`
- Preview and next-version JSON:
  `/tmp/solana-kit-cli-release-readiness/preview-current.json` and
  `next-current.json`
- Monochange logs:
  `/tmp/solana-kit-cli-release-readiness/logs/monochange-prepare.log` and
  `monochange-check-prepared.log`
- Rehearsal:
  `/tmp/solana-kit-cli-release-readiness/rehearsal/evidence/`
  (`summary.json`, `archives/`, `reviews/`, command stdout/stderr and audit JSON)
- CLI install:
  `/tmp/solana-kit-cli-release-readiness/install/` and
  `/tmp/solana-kit-cli-release-readiness/logs/cli-install.log`
- Required check logs:
  `/tmp/solana-kit-cli-release-readiness/logs/`

## Committed version preparation receipt

The prepared versions are now committed on `chore/prepare-publisher-cli-0.1.1`
instead of living only in a disposable copy. The package source commit is
`125c261d` (signed OpenPGP, `G`) with tree
`cd3d0c655bb390e594adc77aae1808c247c1c887`. The source digest for the package
rehearsal is
`94a5f417a42d4264f2063dc51ccd79f93f0db961334bb1c9cae5f053379aa083` over
`crates/`, `Cargo.toml`, `Cargo.lock`, `monochange.toml` and
`rust-toolchain.toml`. A documentation-only successor adds this receipt and
`docs/releases/publisher-cli-v0.1.1.md`; neither file is a package member.

Configured `monochange prepare` produced 35 package targets, 64 file writes and
114 consumed changeset deletions. The umbrella `solana_kit` remains `0.0.0`, the
SDK group is `0.1.1`, Wallet is `0.2.0`, and the publisher library, CLI, Codama
renderer and remaining implemented packages are `0.1.1`. All 29 release targets
(28 packages plus the SDK group) retain `release: false` and `tag: false`. The
canonical record is `.monochange/releases/683bff58248659a0/release.json`; the
root lockfile records the prepared package identities, and internal requirements
remain `^0.1.0` where that admits `0.1.1`.

After the workspace bump, 31 tracked standalone workspace lockfiles were
resolved with real Cargo metadata. Every lockfile diff changes only
`solana_kit_*` package identities or dependency edges; no third-party version,
source or checksum changed. All 31 roots pass `cargo metadata --locked
--offline`. The live signer audit, WASI HTTP graph policy and confirmation-factory
golden now name the prepared workspace versions, and their external-version and
Tokio negative controls are unchanged.

Required local gates on the committed tree:

| Check                                                                                          | Result                                                                                                                 |
| ---------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `cargo test --locked --workspace --all-features --offline`                                     | Passed                                                                                                                 |
| `cargo clippy --locked --workspace --all-targets --all-features --offline -- -D warnings`      | Passed                                                                                                                 |
| `RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps --offline` | Passed                                                                                                                 |
| `cargo fmt --all --check`, standalone consumer fmt checks                                      | Passed                                                                                                                 |
| `dprint check`, `monochange check`, `cargo xtask style`                                        | Passed, 2,246 files, readability 97.24, complexity 98.54                                                               |
| `cargo xtask all`                                                                              | Passed: Native, real Chrome, Wasmtime 50 `wasip1`/`wasip2`, pinned LeptosKit, dependency gates and fresh Core consumer |
| Conformance reports                                                                            | 481/481 success on Native, `wasip1` and `wasip2`; 480/481 sole-hash corruption on each                                 |
| Publisher tests                                                                                | Passed, all features, including the black-box version tests                                                            |
| Actual release CLI                                                                             | `--version` and `--help` heading both `solana-kit-publisher 0.1.1`                                                     |

The fresh package rehearsal ran from the committed tree with disjoint outputs:
34 dependency-ordered archives, 34 fresh reviews over 1,327 regular members, a
207-package local checksum registry, 9 default and 12 selected consumer tests on
Rust 1.99.1 and Rust 1.89.0, the portable Wasm check, and missing-archive and
checksum negative controls. Minimum archive scores are 92.04 readability and
94.76 complexity. The same five expected release blockers remain: the
unpublished LeptosKit Git prerequisite, 20 `publish = false` crates, 20
publish-disabled dry runs, 14 offline registry-API dry runs, and the
whole-client package check that resolves LeptosKit.

The publisher library and CLI archive hashes are new because the committed
archives now include Cargo's `.cargo_vcs_info.json`. Member-by-member comparison
against the `e348f0be` candidate shows the library and CLI payloads are otherwise
byte-identical, and the Core archive differs only by the generated changelog's
owner metadata plus the same VCS file. The previous archive hashes were therefore
not reused; this 34-archive run is authoritative for the committed tree.

`Publisher CLI v0.1.1` release-note review is in
`docs/releases/publisher-cli-v0.1.1.md`. The workflow now fails closed before the
draft release or any target upload when the `publisher-cli-v*` tag does not equal
`publisher-cli-v<crate version>`; local positive `0.1.1` and negative `0.1.0` /
`0.2.0` cases were exercised. Publication remains disabled and no workflow was
dispatched.
