# Planning and native pilot verification

The foundation established ledger integrity and a dependency-free name reservation.
The native pilot implemented deterministic primitives in `solana_kit_core`;
its [historical receipt](../verification/core-native.md) records the original
25-case suite. The current [runtime receipt](../verification/runtimes.md)
covers all 35 primitive, codec, and transaction cases with independent fixtures,
negative controls, actual hosts, and dependency inspection. These results do
not complete Phase 1 or establish transport or device behavior.

Compile and run the dependency-free Rust checker from the repository root:

```sh
rustfmt +1.99.1 --edition 2024 --check tools/check_plan.rs
rustc +1.99.1 --edition=2024 -D warnings tools/check_plan.rs -o /tmp/solana-kit-check-plan
/tmp/solana-kit-check-plan

cargo +1.99.1 fmt --all --check
cargo +1.99.1 test --workspace --all-features --locked
cargo +1.99.1 clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo +1.99.1 check -p solana_kit_core --no-default-features --locked
cargo +1.99.1 check -p solana_kit_core --no-default-features --features crypto --locked
cargo +1.99.1 run -p solana_kit_conformance --locked
cargo +1.99.1 run -p solana_kit_conformance --locked -- --corrupt-fixture

cargo +1.89.0 check -p solana_kit_core --no-default-features --locked
cargo +1.89.0 check -p solana_kit_core --no-default-features --features crypto --locked
cargo +1.99.1 package -p solana_kit_core --all-features --locked
cargo +1.99.1 package -p solana_kit --locked
monostyle check . --fail-under 90
```

The canonical CLI must exit 0 with 35/35 cases. The corrupt-fixture command must
exit 1 with the SHA-256 comparison failure and 34/35 cases; CI checks the exact
committed output as well as the status, so a compilation failure cannot satisfy
the negative control. Rust 1.99.1 is pinned in `rust-toolchain.toml` and CI.
The native CI matrix executes on Linux, macOS, and Windows. The local integration
checks execute on macOS ARM64; Linux/Windows results require their CI runs.
The core's declared MSRV is Rust 1.89.0, checked with and without `crypto`.

Optionally pass the root of a clean archive or checkout at the Dart commit in
`references.toml`. This also checks package names, source manifest existence,
and an exact match against all top-level package manifests:

```sh
/tmp/solana-kit-check-plan /path/to/pinned/dart-reference
```

The checker enforces one row per package and source manifest, package counts
from the reference pins, known phase/status values, and evidence for implemented
or verified claims. Current rows are all `inventoried`. A package-level row is
an index; implementation phases must add public-capability and runtime evidence
before marking a whole package verified.

The audit ran this check against Dart commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46` and found all 71 top-level packages.
Reference project tests and generated apps were not run. System Program crate
metadata was inspected with `cargo info`; its selected target graph and behavior
remain to be tested.

Monostyle 0.3.26 is pinned in CI. `monostyle.toml` sets both per-file floors to
90, and CI also enforces 90 for both repository categories. Generated code is
included; no scoring rules are disabled. The initial Rust checker was simplified
into package parsing, state validation, and reference comparison so it meets the
target through clearer ownership rather than adjusted scoring.

Monochange 0.16.0 explicitly registers `solana_kit` and `solana_kit_core`.
The Ubuntu planning job downloads its official Linux x86_64 GNU release archive
into `$RUNNER_TEMP/monochange-0.16.0`, checks SHA-256
`26ff7caf1ce4951c0309ff8227d59ea218d01b189545b0aa30360d54a2ff237a`
before extraction, prepends that directory to `PATH`, and asserts version 0.16.0
before running every existing ownership and release gate. This avoids the
release's source-build dependency on an unstable Rust coverage attribute;
SDK builds remain on stable Rust 1.99.1 and the core MSRV stays 1.89.0.
The archive digest and single-file `monochange` layout were checked on macOS
ARM64, including rejection of a deliberately wrong digest. Linux binary
execution requires the Ubuntu CI run.

Cargo auto-registration stays disabled. Private `solana_kit_conformance` has
`publish = false` and is visible in discovery, but is not a registered release
or publish target. Component and ecosystem publishing are disabled. Umbrella
publishing is also disabled after its completed reservation.

Only `solana_kit` 0.0.0 was [reserved on crates.io](https://crates.io/crates/solana_kit/0.0.0),
at 2026-10-22T10:39:37Z. No component reservation or publication is authorized.
The umbrella stays dependency-free and outside the `sdk` lockstep group while it
is only a reservation. The group initially owns implemented core 0.1.0; future
implemented clients/adapters join when their contracts and versions converge.

Run the built-in commands below; no custom workflow names are configured:

```sh
monochange check
monochange step validate
monochange discover --format json
monochange preview --format json
monochange preview --diff
```

The pending core minor changeset previews `sdk` and core from 0.1.0 to 0.1.1.
Monochange's pre-1.0 policy shifts minor intent to a patch component increment.
The observed planned paths are root `Cargo.toml`, core `Cargo.toml`, and the group's
single changelog at `crates/solana_kit_core/CHANGELOG.md`. The release note belongs
to output `default`, stream `default`, owner `sdk`. `package_publications` is empty.
Umbrella remains 0.0.0; conformance is not a release target. A preview computes
future writes without applying them or publishing. It does not execute the
configured lockfile refresh; full release preparation, dependency propagation,
and clean-consumer rehearsal remain Phase 8 gates.

The architectural candidates were independently reviewed using GPT-6 Astra.
The configured non-GPT model families were unavailable. This is an independent
model review of source/design, not cross-family agreement or experimental proof.

Audit artifacts in this repository are checked against actual source pins and
files. The reviewer did not have an exported local conversation transcript;
conversational authorization is recorded from the active chat, with that review
limitation retained in the decision trail.
