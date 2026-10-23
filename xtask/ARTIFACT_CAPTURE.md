# Native Cargo artifact capture

Run `solana_kit_xtask artifact-capture /absolute/config.json` using a separately
preserved xtask executable. This private verification command adds no SDK dependency.
A configuration contains:

```json
{
  "source": "/absolute/consumer",
  "source_files": ["Cargo.toml", "Cargo.lock", "src/lib.rs"],
  "target": "/absolute/exclusive-target",
  "archive": "/absolute/new-archive",
  "toolchain": "/absolute/toolchain/bin",
  "expected_release": "1.99.1",
  "args": ["test", "--lib"],
  "required_tests": true
}
```

Obtain the concrete toolchain directory from `rustup which --toolchain 1.99.1 cargo`.
Use an exclusively owned target; the command serializes its own invocations through
a directory lock. Every prior archive recorded in the target ledger must validate
before any producer or Cargo child starts. An interrupted or malformed capture
blocks reuse. An ordinary completed failing Cargo run remains a failure and retains
its executable bytes and logs. Do not remove the history or bypass its ownership.
Unrelated Cargo processes must never write the owned target.

The command selects concrete Cargo, rustc and rustdoc binaries, records their
versions and SHA256 hashes, hashes the explicitly inventoried source files, and
passes offline/locked/jobs2/JSON/verbose flags. Include every relevant source and
configuration file in `source_files`; this is an explicit inventory, not automatic
source discovery. Paths and argument values are JSON strings and never shell text.
Source hashes snapshot those files; they do not prove that a Cargo `Fresh`
executable was built from those bytes. Review source-linked mutation evidence
separately. Direct producer identities do not establish a nested launcher's compiler.

Cargo JSON executable paths are copied immediately into unique numbered files,
including executables outside the usual debug/deps layout. Libraries with null
executables are ignored. Ordinary test output is retained verbatim, malformed
claimed Cargo artifacts reject, and only test-profile executables receive `--list`.
An artifact claim is a top-level JSON `reason` value of `compiler-artifact`,
including a decoded discriminator before malformed remaining fields. Ordinary
text and JSON strings mentioning that word are retained as output.
`required_tests` requires a nonempty listed inventory. It does not prove that test
bodies executed; `--no-run` and filters can leave a nonempty inventory with no execution.
The original Cargo stdout/stderr and exit status survive failures. Parser rows are
bounded to 8 MiB while raw streams continue to disk. A fresh archive is mandatory;
JSON files use flushed temporary siblings and atomic renames. Archive byte hashes
are validated again after capture and before the next owned-target command.

This captures new executions. It cannot recover overwritten historical bytes or
prove the identity of arbitrary nested compiler processes launched by build scripts.
Review verbose producer commands when a consumer launches its own toolchain.
Manifests contain raw local paths and belong in ignored evidence. Publish a separate
normalized report. Capturing native tests does not establish browser, WASI,
validator, or on-chain execution. Solana Kit is an off-chain client SDK; use Pina
or official Solana program libraries for on-chain programs.
