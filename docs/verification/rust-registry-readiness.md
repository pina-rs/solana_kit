# Rust registry readiness audit

This receipt separates the reviewed release-selection facts from what a local
run can prove about artifacts and registry authorization.
It records three things: the reviewed release selection now committed as data,
the narrow selection guard that checks it, and the retained archive/registry
audit with its exact provenance. The preparation audit below did not claim
registry readiness, artifact qualification, trusted-publishing configuration,
name ownership, or publication authorization for any channel. The whole SDK is
not ready.

## First publication completed

The reviewed selection was published on 2026-11-08 from `bebf4cb`:

- `solana_kit_core` and its twelve companion crates were accepted by crates.io
  at `0.2.0`; `solana_kit_wallet` was accepted at `0.3.0`.
- The `v0.2.0` primary tag and seven namespaced tags name the release, with
  matching GitHub releases. `v0.2.0` is the latest release.
- `.github/workflows/publish.yml` used crates.io trusted publishing for every
  crate. The repository's existing trusted-publisher enrollments therefore
  matched the workflow and owner permissions in practice.
- `solana_kit` remains the `0.0.0` umbrella reservation. The nineteen deferred
  crates and the private test fixture were not published.

Read the "unverified" and "preparation-time" statements below as the state
before this publication. The release workflow is the executable evidence that
the fourteen trusted publishers and owner permissions work.

## Scope and provenance

The audit facts below were produced at source commit
`98f34604fb880ab1003a75c6fad5f3a85c34ce12` (Publisher CLI 0.1.1 preparation,
the audit baseline) by the first-pass review whose full evidence
remains retained outside the repository:

- `/tmp/solana-kit-rust-registry-readiness/` — audit evidence root with
  `evidence/MANIFEST.sha256` (file hashes) and `evidence/TOOLCHAIN.txt`
  (cargo/rustc/rustdoc 1.99.1, monochange 0.21.0, dprint 0.60.1, monostyle
  0.3.27).
- `/tmp/solana-kit-rust-registry-readiness/rehearsal/evidence/summary.json` —
  the fresh candidate rehearsal summary, hashed by the manifest.
- `/tmp/solana-kit-rust-release-selection/` — this unit's evidence root
  (guard configs, stdout/stderr, reports, and the corrected out-of-tree policy
  proposal).

Those audits were executed once and are retained with hashes; this unit did not
rerun them and re-states only their results. Everything current on `main` that
this unit changed is documentation plus the selection guard; no SDK source,
manifest, lockfile, workflow, publication policy, conformance case, fuel,
golden, status, or source hash changed. The Codama npm channel qualification
and Publisher CLI preparation receipts are separate and unchanged.

## The reviewed selection is data, not a readiness stamp

The committed review artifact
[docs/planning/rust-release-plan.json](../planning/rust-release-plan.json)
holds all 35 configured Cargo packages with an explicit decision, version and
reason. Its scope is the reviewed first-publish set, not whole-SDK acceptance:

| Decision           | Count | Packages                                                                                                                                                    |
| ------------------ | ----: | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `public`           |    14 | core, compute budget, memo, signers, system, associated token account, anchor, client, surfpool, browser, wasi, native, leptoskit (all 0.2.0), wallet 0.3.0 |
| `private-fixture`  |     1 | test support — deliberately private, must never publish                                                                                                     |
| `private-deferred` |    19 | manifest `publish = false`; each needs a separate implementation release review                                                                             |
| `reserved`         |     1 | `solana_kit` 0.0.0 umbrella reservation, not a release target                                                                                               |

The plan also records the six publish batches derived from the actual
non-dev Cargo graph (normal, build and optional edges; no private transitive
leak): 1 core → 2 anchor, associated token account, compute budget, memo,
signers, system, wallet → 3 client → 4 browser, surfpool, wasi → 5 native → 6 leptoskit.

## The selection guard and its exact contract

`cargo xtask rust-release-selection CONFIG.json`
(`xtask/src/rust_release_selection.rs`) is the only automated check this unit
adds. It compares the committed plan with the configured `monochange.toml`
Cargo package set, real `cargo metadata --locked --offline --no-deps` output,
the committed release record's `package_publications`, and the actual
`monochange publish readiness` JSON. It rejects duplicates, invented, missing,
wrong-version, wrong-target (`cargo`/`crates_io`), non-ready and empty
selections on either side, rejects a public plan entry whose manifest disables
publication or carries a non-dev Git dependency, rejects public packages that
depend on non-public workspace packages, and requires the six batches to
partition the public set exactly. It binds each input's monochange `kind` and,
when the config supplies `record_commit`, the readiness `record_commit`.

The contract is deliberately narrow:

- The command's only decisions are `selection-exact` and `selection-mismatch`.
  It does not label artifacts qualified or publishing authorization configured.
- Exit 0 qualifies the selection alone. Artifact verification, registry
  ownership, trusted-publishing setup and publication authorization remain
  separate external prerequisites, whether or not this command passes.
- It reads no archive, registry or credential input, because none of those can
  prove selection, and it never writes to any input path.

### Preparation-time result

At the audit baseline, against the committed record and the readiness output
(`/tmp/solana-kit-all-release-readiness.json`, resolved at `98f34604…`), the
guard exited nonzero with `selection-mismatch` and 28 blockers: the readiness
selection was empty (14 expected, 0 observed) and the release record had no
publication entries (14 expected). `monochange publish readiness` reporting
`ready` with zero packages was correct upstream zero-selection semantics, not
project readiness — that distinction is exactly what this guard exists to
keep visible. The full stdout and the machine report are retained at
`/tmp/solana-kit-rust-release-selection/`. An independently staged simulated
fixture with the 14 exact selections and publication entries exits 0 with
`selection-exact`; that fixture lives only under the evidence root and is a
unit control for the guard, not a readiness claim. The first publication
recorded above supplies the real publication entries.

## Retained archive and registry audit at `98f34604…`

Re-executed once during the first-pass review, retained under the hashed
manifest, not re-run since:

- 34 dependency-ordered archives were packaged offline with `--locked` from a
  prepared tree and each archive passed a fresh member review — 1,327 members
  in the committed-preparation run — against the same offline rehearsal gate
  recorded in [the package rehearsal receipt](sdk-package-rehearsal-current.md)
  and [the Publisher CLI 0.1.1 receipt](publisher-release-candidate.md).
- The 207-package offline registry fed consumers that passed 9 default and 12
  selected-feature tests on current Rust 1.99.1 and actual Rust 1.89.0, plus a
  portable Wasm build check. Negative controls passed: a missing package
  fails with the expected download error, a corrupted checksum is rejected.
- Known blocked provenance: packaging the Client offline with `--locked`
  requires the not-yet-published Client 0.1.1, so no fabricated lockfile was
  produced. The archives therefore exclude their lockfiles; installing them
  resolves a fresh dependency graph instead of proving an immutable published
  lock. This remains an immutability limit of the retained evidence.
- Anonymous crates.io snapshots (38 files under `evidence/registry/`) were
  fetched 2026-11-05. All 14 public crate names existed at that time: the
  original thirteen carried only the `0.0.0` placeholder version, and
  `leptoskit_bridge` 0.3.0 was published on crates.io. The first publication
  later replaced those placeholders for the reviewed set.
  A version being absent does not mean the name is claimable: these snapshots
  do not identify owners and are not ownership evidence.

## Trust and ownership at preparation time

Nothing in the preparation audit established trusted publishing or ownership
state; the first publication later verified both for the fourteen public crates:

- The snapshots show no `trustpub_data` for the placeholders. A null field in
  an anonymous snapshot is not proof that trusted publishing is unconfigured,
  and its presence would not prove that a workflow is authorized. Both are
  registry-side, first-party-account facts that require an authorized owner to
  query. The guard deliberately reports nothing about trust.
- Owner and maintainer permissions for the fourteen existing placeholder crates
  were not queried in this audit. Placeholder existence is not name ownership.
  The first publication later exercised write access for all fourteen.
- Per the official crates.io trusted-publishing documentation, a trusted
  publisher is configured by crate owners on an existing crate. All 14 names
  already exist, so owners can configure trusted publishing before the first
  real version if the settings flow accepts a placeholder-only crate; the
  documentation does not address that case explicitly. If it is rejected, an
  authorized maintainer token remains an alternative for a first publish —
  an explicit, separately authorized action that this preparation neither
  performs nor assumes.

## Applied publication policy

`/tmp/solana-kit-rust-release-selection/policy-proposal.patch` was the
preparation-time proposal. The committed configuration now enables
`monochange` publication for exactly the fourteen reviewed public crates, keeps
trusted publishing enabled through `.github/workflows/publish.yml` and
`rust-lang/crates-io-auth-action`, and adds no maintainer-token bootstrap
override. The other 21 Cargo packages, including the umbrella, and the npm
ecosystem remain outside the publish set. The release record contains the real
publication entries produced from the reviewed changesets.

## Limits

The preparation audit used local macOS evidence only; Linux and Windows were
not executed for that audit. It performed no publication, registry write, tag,
workflow dispatch, or secret access. Those limits describe the audit baseline,
not the release recorded above. The archives exclude lockfiles. `solana_kit`
0.0.0, the fixture and the nineteen deferred crates remain outside this
selection by review, and the LeptosKit dependencies are published on crates.io.
Selection exactness was not met at the baseline; the first publication covered
the reviewed fourteen-crate selection.
