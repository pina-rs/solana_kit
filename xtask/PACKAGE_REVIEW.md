# Cargo archive review

`cargo xtask package-review CONFIG.json` reviews a real Cargo `.crate` file.
It never publishes or contacts a registry. This command supports arbitrary
regular-file member sets rather than a placeholder package's fixed inventory.

The configuration requires absolute `archive`, `source`, `receipt`, `engine`,
`policy` and `gzip` paths, plus the admitted `engine_sha256` and `gzip_sha256`.
The receipt directory must not exist. `source` names the reviewed crate source.
Use the repository's exact `monostyle.toml` policy. Authenticate tools and resource
ownership in the invoking driver before starting this command.

The gate checks raw GNU and USTAR headers after bounded decompression. It rejects tar
extensions, duplicate or escaped names and nonregular members. Each actual
member's type, size and SHA256 is preserved in `members.json`.
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
offline inputs. Its configuration names `archive`, packaged `lockfile`, actual
Cargo `metadata`, cached archive directory `cache`, cached `sparse_index`, a new
`destination`, and the selected `package` and `version`.

Only lockfile-selected packages enter the fixture. Each dependency's actual
archive digest must match both its lockfile checksum and its exact cached sparse
index record. The implementation's record comes from actual Cargo metadata;
local source paths are excluded. Archives are hardlinked into the owned fixture.
The tool creates no Git repository and performs no network request.

Optional `omit_archive` and `corrupt_archive` package names construct real Cargo
failure controls in fresh destinations. The latter changes a gzip timestamp byte,
preserving decompression while invalidating the archive checksum. It copies that
one control archive before changing bytes; cached source archives remain intact.

Consumers use an exact dependency version and a Cargo source replacement whose
`local-registry` points at the fixture. Keep positive and negative compiler targets
separate. Record the resolved package identity, actual compiler invocation and
selected artifact bytes before reusing a target with another producer. A failed
build proves a control only when its diagnostic names the intended missing source
or checksum failure.
