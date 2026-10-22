# Per-file style verification

Run `cargo xtask style` after installing `monostyle` with
`cargo install monostyle --version 0.3.26 --locked`. The command verifies that
exact version, executes a fresh JSON analysis of the repository with
`--include-generated --fail-under 90`, and retains its report at
`target/style-evidence/report.json`. CI runs the same command.

In 0.3.26, `--fail-under` compares the repository's two aggregate scores,
computed from code-line-weighted penalty densities. It can succeed while an individual file is below 90. The xtask also
requires `readability.value` and `complexity.value` in every `files` entry to be
within 90 through 100, inclusive. Generated sources receive the same floor;
`monostyle.toml` keeps `rules.ignore.generated = false`.

Missing categories, invalid JSON, an empty file list, duplicate or empty file
paths, and file line totals inconsistent with repository totals fail closed.
Command failure or a different installed version is a hard error. Unsupported
languages remain monostyle's `skipped` entries and have no invented score.

Validate an existing report independently with
`cargo xtask style-report <report.json>`. A retained negative control is:

```sh
cargo xtask style-report xtask/tests/fixtures/style-file-below-floor.json
```

It must fail: the synthetic report uses the pinned JSON score structure and a
repository complexity of 99.887, while `generated.rs` scores 88.7.
There is no deliberately poor Rust source in this fixture. The public-command
test exercises this rejection, and unit tests cover each category, exact floors,
malformed/truncated/empty JSON, missing cells/files, and invalid scores.
