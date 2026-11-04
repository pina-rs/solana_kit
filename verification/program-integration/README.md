# Program integration verification

Private, standalone workspace that executes the real Solana Kit program clients against one owned
Agave test validator per test.

```sh
fixture_root=$(mktemp -d)
git -C "$DART_SOURCE" archive 0138bb956d2d81ccc0c56edc602401c0df2f2b46 config/programs \
  | tar -x -C "$fixture_root"
export SOLANA_KIT_PROGRAM_FIXTURES="$fixture_root/config/programs"
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 cargo +1.99.1 test --locked --offline \
  --manifest-path verification/program-integration/Cargo.toml -- --test-threads=1
```

Set `DART_SOURCE` to a local checkout of the reference Dart repository containing that revision.
`SOLANA_TEST_VALIDATOR` can explicitly select the Agave 4.2 executable. The harness validates its
version and all selected artifact hashes before accepting any result. No fixture path is inferred
from another run. The extraction is caller-owned; the harness removes only its own ledgers.

- `src/node.rs` owns the validator, ledger, ports, config and bounded cleanup.
- `src/lib.rs` owns the RPC client, disposable keys, transaction submission, confirmation and typed
  `jsonParsed` reads.
- `src/fixtures.rs` records the pinned artifact hashes and validates the deployed ProgramData.
- `tests/program_integration/` contains the built-in and local pinned external program journeys.

Full evidence, observed errors and limitations are in
[`docs/verification/program-integration.md`](../../docs/verification/program-integration.md).

The validator and fixture directory are prerequisites. Missing or mismatched inputs fail the suite; no
test silently skips. The workspace has its own lockfile and `publish = false`; it is not a member of the
root workspace.
