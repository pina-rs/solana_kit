# Rust registry readiness audit

This receipt separates the reviewed release-selection facts from what a local
run can prove about artifacts and registry authorization.
It records three things: the reviewed release selection now committed as data,
the narrow selection guard that checks it, and the retained archive/registry
audit with its exact provenance. It does not claim registry readiness, artifact
qualification, trusted-publishing configuration, name ownership, or
publication authorization for any channel. The whole SDK is not ready.

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

| Decision           | Count | Packages                                                                                                                                         |
| ------------------ | ----: | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `public`           |    13 | core, compute budget, memo, signers, system, wallet 0.2.0, associated token account, anchor, client, surfpool, browser, wasi, native (all 0.1.1) |
| `blocked-upstream` |     1 | leptoskit — `leptoskit_bridge 0.1.0` exists only at Git pin `7437097c52aa68a3401bd378764e051ed4c9165a`, not in the registry                      |
| `private-fixture`  |     1 | test support — deliberately private, must never publish                                                                                          |
| `private-deferred` |    19 | manifest `publish = false`; each needs a separate implementation release review                                                                  |
| `reserved`         |     1 | `solana_kit` 0.0.0 umbrella reservation, not a release target                                                                                    |

The plan also records the five publish batches derived from the actual
non-dev Cargo graph (normal, build and optional edges; no private transitive
leak): 1 core → 2 anchor, associated token account, compute budget, memo,
signers, system, wallet → 3 client → 4 browser, surfpool, wasi → 5 native.

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
depend on non-public workspace packages, and requires the five batches to
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

### Current real result: blocked, as it must be

Against the real committed record and the real readiness output
(`/tmp/solana-kit-all-release-readiness.json`, resolved at `98f34604…`), the
guard exits nonzero with `selection-mismatch` and 28 blockers: the readiness
selection is empty (13 expected, 0 observed) and the release record has no
publication entries (13 expected). `monochange publish readiness` reporting
`ready` with zero packages is correct upstream zero-selection semantics, not
project readiness — that distinction is exactly what this guard exists to
keep visible. The full stdout and the machine report are retained at
`/tmp/solana-kit-rust-release-selection/`. An independently staged simulated
fixture with the 13 exact selections and publication entries exits 0 with
`selection-exact`; that fixture lives only under the evidence root and is a
unit control for the guard, not a readiness claim.

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
  fetched 2026-11-05. All 13 public crate names exist with only the `0.0.0`
  placeholder version; `leptoskit_bridge` has no registry presence at all.
  A version being absent does not mean the name is claimable: these snapshots
  do not identify owners and are not ownership evidence.

## Trust and ownership are UNVERIFIED

Nothing here establishes trusted publishing or ownership state:

- The snapshots show no `trustpub_data` for the placeholders. A null field in
  an anonymous snapshot is not proof that trusted publishing is unconfigured,
  and its presence would not prove that a workflow is authorized. Both are
  registry-side, first-party-account facts that require an authorized owner to
  query. The guard deliberately reports nothing about trust.
- Owner and maintainer permissions for the 13 existing placeholder crates were
  never queried. Placeholder existence is not name ownership, and this
  repository has not demonstrated write access to any of them.
- Per the official crates.io trusted-publishing documentation, a trusted
  publisher is configured by crate owners on an existing crate. All 13 names
  already exist, so owners can configure trusted publishing before the first
  real version if the settings flow accepts a placeholder-only crate; the
  documentation does not address that case explicitly. If it is rejected, an
  authorized maintainer token remains an alternative for a first publish —
  an explicit, separately authorized action that this preparation neither
  performs nor assumes.

## Out-of-tree policy proposal

`/tmp/solana-kit-rust-release-selection/policy-proposal.patch` (with its
`README.md`, outside the repository, not applied to committed source) enables
`monochange` publication for exactly the 13 reviewed public crates, keeps
trusted publishing enabled through the repository's existing crates.io OIDC
flow (`.github/workflows/publish.yml` exchanges an OIDC identity via
`rust-lang/crates-io-auth-action`), and adds no maintainer-token bootstrap
override. The other 22 Cargo packages, including the umbrella, and the npm
ecosystem stay byte-identical. It was validated only in a disposable `git
archive` copy with `git apply --check` and `monochange config`, confirming
exactly 13 packages resolve `publish.enabled = true`. Applying it, configuring
registry-side trusted publishers, and preparing any new release record with
real publication entries are future authorized actions; the committed record
must never gain hand-invented ready entries.

## Limits

Local macOS evidence only; no Linux/Windows execution. No publication,
registry write, tag, workflow dispatch, or secret access occurred. The 13
placeholder crates have unverified owner permissions in this audit. The archives
exclude lockfiles. `solana_kit` 0.0.0, the fixture and the 19 deferred crates
are outside this selection by review, and LeptosKit stays blocked until its
bridge reaches a registry. Selection exactness — the only thing this unit can
check — is currently not met, and that is the correct, expected state.
