# Per-file style verification

Run `cargo xtask style` after installing `monostyle` with
`cargo install monostyle --version 0.3.26 --locked`. The command verifies that
exact version, obtains tracked and nonignored new files from Git, and executes a fresh JSON analysis with
`--include-generated --fail-under 90`, and retains its report at
`target/style-evidence/report.json`. CI runs the same command.

Files are passed explicitly: monostyle 0.3.26's directory discovery silently
omits `src/bin` and other built-in directory names even with `--no-ignore`.
Git inventory retains every tracked file even when its path matches ignore
rules, including Rust source under `src/target` or `.monochange/local`.
Untracked ignored build caches and local execution evidence stay outside the
inventory through Git's ignore rules. Every inventoried Rust path must appear exactly once in the scored
report. Missing, unreadable, unknown, or aliased scored paths fail the gate.
No ignore rules or score thresholds are relaxed. To analyze another checkout,
use `cargo xtask style <absolute-repository-directory>`.

In 0.3.26, `--fail-under` compares the repository's two aggregate scores,
computed from code-line-weighted penalty densities. It can succeed while an individual file is below 90. The xtask also
requires `readability.value` and `complexity.value` in every `files` entry to be
within 90 through 100, inclusive. Generated sources receive the same floor;
`monostyle.toml` keeps `rules.ignore.generated = false`.

Missing categories, invalid JSON, an empty file list, duplicate or empty file
paths, and file line totals inconsistent with repository totals fail closed.
Command failure or a different installed version is a hard error. Unsupported
languages remain monostyle's `skipped` entries and have no invented score.

Validate an existing report independently of the current checkout's inventory with
`cargo xtask style-report <report.json>`. A retained negative control is:

```sh
cargo xtask style-report xtask/tests/fixtures/style-file-below-floor.json
```

It must fail: the synthetic report uses the pinned JSON score structure and a
repository complexity of 99.887, while `generated.rs` scores 88.7.
There is no deliberately poor Rust source in this fixture. The public-command
test exercises this rejection, and unit tests cover each category, exact floors,
malformed/truncated/empty JSON, missing cells/files, and invalid scores.
The style CI job also executes the real pinned analyzer in an isolated Git
repository containing binary targets, generated files, ignored tracked files
(including cache-like path names),
new authored files, unreadable Rust source, and a missing tracked input.
