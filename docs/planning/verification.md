# Planning verification

The foundation PR contains an empty name-reservation package and no SDK APIs.
Its checks establish ledger integrity, source accounting, package validity, and
style targets, not Solana behavior or runtime compatibility.

Compile and run the dependency-free Rust checker from the repository root:

```sh
rustfmt +stable --edition 2024 --check tools/check_plan.rs
rustc +stable --edition=2024 -D warnings tools/check_plan.rs -o /tmp/solana-kit-check-plan
/tmp/solana-kit-check-plan
cargo +stable check --workspace --locked
cargo +stable package -p solana_kit --locked
monostyle check . --fail-under 90
```

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

Only `solana_kit` is registered in monochange. Cargo auto-registration is disabled
so adding future manifests cannot silently add publish targets. The requested
bootstrap command is `monochange publish placeholder --package solana_kit`;
inspect its `--dry-run --format json` output before publication. It must select
exactly that package at 0.0.0. Placeholder bootstrap requires no release record.

The architectural candidates were independently reviewed using GPT-6 Astra.
The configured non-GPT model families were unavailable. This is an independent
model review of source/design, not cross-family agreement or experimental proof.

Audit artifacts in this repository are checked against actual source pins and
files. The reviewer did not have an exported local conversation transcript;
conversational authorization is recorded from the active chat, with that review
limitation retained in the decision trail.
