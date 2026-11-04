# SDK package rehearsal (current)

On 2026-11-04, the local RW-07 rehearsal completed release preparation, assembled
34 SDK archives in dependency order, and ran an external offline-registry consumer.
This is local package evidence, not publication or whole-SDK release readiness.
All publish settings remain disabled; twenty crates retain `publish = false`.

The executable gate is `cargo xtask package-rehearsal CONFIG.json`. The recorded
prepared source starts from `17311782`, adds the three release-metadata changes
described below, and runs Monochange only in a disposable copy. Its source-tree
digest is `398a35ae80b0e76b2a4b023cbd451de8a3ef4a9a9feef6f811cb0fe7d774ad8a`.
That digest covers `crates/`, `Cargo.toml`, `Cargo.lock`, `monochange.toml` and
`rust-toolchain.toml`, including the staged versions and changelogs. The repository
keeps its current versions; no release was committed, tagged or uploaded.

## Release preparation and the fixed dependency gap

The original `6606f80f` rehearsal exposed a real preparation failure: Wallet was
planned at 0.2.0, while Browser's target-specific dependency still required
`^0.1.0`. Monochange's discovered graph omitted that target-specific edge.
It wrote package versions but could not refresh the workspace lock.

Browser now inherits Wallet from `workspace.dependencies`, and Wallet's typed
`versioned_files` entry updates `workspace.dependencies.solana_kit_wallet` during
preparation. The shared dependency disables default features, preserving Browser's
previous setting; Wallet's default feature set is empty, so other workspace
callers gain no feature change.

With those changes, `monochange prepare` exited 0 in the disposable copy:
35 packages and 64 changed files, including the root requirement and refreshed
lockfile. Wallet becomes 0.2.0; the other 33 included Cargo components become
0.1.1; the dependency-free umbrella stays at 0.0.0. The excluded LeptosKit crate
also plans 0.1.1. The NPM package's planned files are preparation evidence only;
its packed-consumer proof remains a separate receipt.

The initial preview had 111 changesets, 29 release targets and no package
publications. Every target retained `release: false`. The initial failed apply
and its logs remain historical evidence; the successful prepared copy is the
input to the current archive rehearsal. No publish policy was enabled.

## Package set and archive review

Cargo metadata finds 35 SDK roots under `crates/`. Thirty-four enter this
rehearsal. The excluded `solana_kit_leptoskit` depends on the pinned LeptosKit Git
upstream `7437097c52aa68a3401bd378764e051ed4c9165a`, which is not a registry
package. That remains an external release prerequisite.

Cargo produced actual archives with
`cargo package --locked --offline --exclude-lockfile --no-verify`.
The [maintained archive gate](../../xtask/PACKAGE_REVIEW.md) freshly analyzed
32 archives and 1,103 regular members. Every scored member met both 90-point
floors. Two archives remain outside that gate's supported forms:

- Wallet contains `assets/coinbase wallet.uri` and `assets/huobi wallet.uri`;
  the gate rejects filenames containing spaces.
- Token-2022 uses GNU long-name records for three instruction paths; the gate
  rejects those extension records.

Both archives are checksum-locked in the registry. Wallet's public behavior is
executed by the consumer, but neither archive has a fresh style/member audit.
No score threshold or existing rejection control was weakened.

| Package                 | Version | SHA256                                                             | Reviewed members |
| ----------------------- | ------- | ------------------------------------------------------------------ | ---------------: |
| `solana_kit_core`       | 0.1.1   | `4c31af052a50d38e82631e8e9c525f0b1a353f145b2d02bc106f3cfbd2ad4e8d` |              103 |
| `solana_kit_client`     | 0.1.1   | `3b704b8d481acdd333f344af6ffc0db2c52aecc854c390fb2eb55f14a9f7396b` |              191 |
| `solana_kit_signers`    | 0.1.1   | `7cf66874448c9377de36c6a64bb38eac844e3fc46fbd314734fc06a5d70ce2a8` |               24 |
| `solana_kit_native`     | 0.1.1   | `5c9ad1c0b56998039c7d9671be26004837198eeb2fa0979b3a39985377f8f29f` |               68 |
| `solana_kit_system`     | 0.1.1   | `5e2dfefc3b0264d7eef5c99eed52e261b7782fa462c062dd800c94c072ec20d0` |               11 |
| `solana_kit_token`      | 0.1.1   | `dac1f4d1bd59f2d754f09330cd17f46b90343d2ee0a599f79a126f9efadfc1c8` |               15 |
| `solana_kit_anchor`     | 0.1.1   | `704f1802f2bc260364360acd5b5f93ef09a31a9fc66519b1a02e695a5a6f4005` |               17 |
| `solana_kit_wallet`     | 0.2.0   | `9fc4b8eebec3d4083d992ef9ae2373ff2b3e98d0751e55638130aa2d386ab2e6` |     form-limited |
| `solana_kit_token_2022` | 0.1.1   | `c0f5c54696286d72d218add6499947b4bf554720eb1ffacd5a5378a3e1e010db` |     form-limited |

All 34 archive hashes and per-member receipts are retained with the run summary.
Archive creation alone is not compile acceptance; the consumer proves the
selected dependency closure builds.

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
the fresh Cargo home, and a consumer-lock checksum matching its archive. Reviewed
unpacked members are rehashed against archive receipts; Wallet is checksum-bound
but remains form-limited. A successful command with no executed tests is rejected.

Actual Rust 1.89.0 (`29483883e`, 2025-08-04) executes the same 9 default and
12 selected tests. The harness resolves Cargo, rustc and rustdoc independently
through `rustup which --toolchain` and sets all three explicitly, so an inherited
Rust 1.99 compiler override cannot invalidate the MSRV result. The compiler's
verbose version and both corrected executions are retained separately.
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
4. Extend archive-form support before claiming fresh Wallet and Token-2022 audits.
5. Recheck immutable publish archives with their generated lockfiles; this
   rehearsal uses `--exclude-lockfile` to assemble unpublished dependencies.

The prepared CLI archive also installs into an isolated root from this registry,
and `--help` exits 0. Cargo records package version 0.1.1, but help reports 0.1.0
because the executable imports a hard-coded library constant. That exposed a
separate version-display defect; correct version reporting remains unqualified.
Cargo also warns that the archive has no published lockfile, so `--locked` here
does not prove an immutable published dependency lock.

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

The initial worker's logs remain outside build targets. Reviewed preparation,
corrected MSRV and local-gate logs are under
`/tmp/solana-kit-package-parent-evidence`; the prepared registry evidence is under
`/tmp/solana-kit-package-prepared-run/evidence`. Build targets are disposable;
hashes, public source snapshots and receipts are retained outside them.
