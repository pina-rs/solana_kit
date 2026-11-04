# Cargo archive review

`cargo xtask package-review CONFIG.json` reviews a real Cargo `.crate` file.
It never publishes or contacts a registry. This command supports arbitrary
regular-file member sets rather than a placeholder package's fixed inventory.

The configuration requires absolute `archive`, `source`, `receipt`, `engine`,
`policy` and `gzip` paths, plus the admitted `engine_sha256` and `gzip_sha256`.
The receipt directory must not exist. `source` names the reviewed crate source.
Use the repository's exact `monostyle.toml` policy. Authenticate tools and resource
ownership in the invoking driver before starting this command.

The gate checks raw GNU and USTAR headers after bounded decompression. It accepts
ordinary regular files, including real Cargo member names with interior ASCII
spaces, and bounded GNU `L` long-name metadata for the immediately following
regular file. A long name must be NUL-terminated, unambiguous UTF-8, within its
bound, and match the truncated name in the next header. Orphaned, repeated,
oversized or mismatched metadata is rejected. Every header and metadata checksum,
payload offset, padding bound and end marker is validated before any member is written.
All other tar extensions and every nonregular member are rejected, including GNU
`K`, links, directories and pax records. Duplicate effective names, conflicting
roots, escaped or ambiguous paths and control characters remain failures.

Each actual member's type, size and SHA256 is preserved in `members.json`.
It compares all source members with the crate source. Cargo's normalized manifest,
generated lockfile and optional VCS metadata are recorded as generated inputs;
`Cargo.toml.orig` must match the authored manifest.

Every supported source, generated source and document receives finite readability
and complexity scores of at least 90. The archive aggregate has the same floors.
Metadata, binary and data members unsupported by the analyzer remain explicit
inventory entries with actual skipped rows. Source templates such as `.rs.txt`
use byte-identical language-suffixed mirrors. Mirrors cannot collide.

The engine receives the complete member partition, with `--no-ignore` and
`--include-generated`. The SDK policy uses width 100 and disables generated-file
ignoring. A score omission, supported-file skip, duplicate, unrelated row or low
score rejects the archive. Raw diagnostics survive failure.

`cargo xtask package-report CONFIG.json` additionally requires a `report` path.
It checks retained report structure and coverage, including deliberate omission
controls. It does not certify fresh source quality. `package-review` rejects a
supplied report and always executes the analyzer on the actual archive bytes.

The gate rechecks actual archive and source bytes after analysis. Review normalized
Cargo features, dependency versions, notices and outgoing metadata separately.
Registry consumption must use the exact reviewed archive by name and version,
under a fresh local registry namespace with original source access denied.

`cargo xtask registry-fixture CONFIG.json` builds that local registry from exact
offline inputs. Its configuration names `archive`, packaged or derived `lockfile`,
actual Cargo `metadata`, cached archive directory `cache`, sparse-index root
`sparse_index`, a new `destination`, and the selected `package` and `version`.

Only lockfile-selected packages enter the fixture. Each dependency's actual
archive digest must match both its lockfile checksum and its exact cached sparse
index record. The implementation's record comes from actual Cargo metadata;
local source paths are excluded. Archives are hardlinked into the owned fixture.
The tool creates no Git repository and performs no network request.

Optional `additions` names further reviewed archives in the same registry. Each
entry supplies `archive`, `package` and `version`; additions not present in the
derived lock input are installed with metadata records after the lock pass. The
original single-archive configuration remains unchanged.

Optional `omit_archive` and `corrupt_archive` package names construct real Cargo
failure controls in fresh destinations. The latter changes a gzip timestamp byte,
preserving decompression while invalidating the archive checksum. It copies that
one control archive before changing bytes; cached source archives remain intact.

`cargo xtask package-rehearsal CONFIG.json` composes these commands into the
current SDK package gate. It selects `crates/*` workspace roots from Cargo
metadata, packages them in dependency order, reviews each archive, installs the
complete lock closure into a fresh local registry, and executes a clean external
consumer with default/selected features, MSRV, Wasm and negative controls. It
records the exact publish-dry-run blockers without uploading anything. See the
[SDK package rehearsal receipt](../docs/verification/sdk-package-rehearsal-current.md).

The rehearsal requires fresh, disjoint output namespaces and a cache-only Cargo
home without configuration, credentials or unpacked registry sources. It audits
resolved SDK PackageIDs and member bytes separately for default and selected
features, rejects empty test runs, and selects the MSRV compiler explicitly.

Consumers use an exact dependency version and a Cargo source replacement whose
`local-registry` points at the fixture. Keep positive and negative compiler targets
separate. Record the resolved package identity, actual compiler invocation and
selected artifact bytes before reusing a target with another producer. A failed
build proves a control only when its diagnostic names the intended missing source
or checksum failure.
