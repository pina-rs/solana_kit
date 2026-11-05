# Codama NPM 0.1.1 release acceptance

On 2026-11-05, the actual `codama-renderers-rust-solana-kit` 0.1.1 archive passed
local packed-package acceptance and the preserved generated client executed on
every promised host. This is local macOS execution evidence for the off-chain
application client fixture. It does not publish the package, qualify the full
SDK workspace, or complete on-chain, validator, device, wallet, registry
ownership or live network work.

Base checkout was `98f34604fb880ab1003a75c6fad5f3a85c34ce12` on branch
`test/codama-0.1.1-release-readiness`. The reviewable draft release note is
[Codama Rust Solana Kit renderer v0.1.1](../releases/codama-renderers-rust-solana-kit-v0.1.1.md).

Only release documentation, package and consumer README links and the manual npm
release workflow changed. Renderer source, generated output, writer policy,
package version 0.1.1, the package manifest and the disabled monochange publish
policy are unchanged. The old 0.1.0 receipt remains historical evidence for the
earlier archive; it does not qualify this 0.1.1 archive.

## Environment and isolation

The run was on macOS 27.0.1 (build 26A434), arm64. Node was 26.13.0 and npm was
11.19.0. Rust 1.99.1 (`58cf8f9c3`, cargo `5f94df478`) and Rust 1.89.0
(`29483883e`, cargo `c24e10642`) each supplied matching `cargo`, `rustc` and
`rustdoc`; Clippy was installed for both toolchains. Wasmtime was 50.0.0
(`ebe5e8df1`), Chrome and ChromeDriver were 156.0.8078.80, and
wasm-bindgen-test-runner was 0.2.128. The LeptosKit host was pinned to
`7437097c52aa68a3401bd378764e051ed4c9165a`. dprint was 0.60.1 and monostyle was
0.3.27.

The npm user configuration contained only `user-agent=build`; the global
configuration was a separate `user-agent=build` file, not the machine's
configuration. The npm cache was an isolated directory under the evidence root.
The package installation and every packed-consumer installation were
credential-free. `NODE_AUTH_TOKEN` and `NPM_TOKEN` were absent. `npm publish`
was executed only with `--dry-run`.

`CARGO_HOME` was the parent-owned public cache at
`/tmp/solana-kit-address-cargo-cache`. `CARGO_TARGET_DIR` was an owned directory
outside `HOME`, `CARGO_NET_OFFLINE=true` was used after the public cache was
prepared, `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=2` and
`CARGO_HTTP_USER_AGENT=build` were set. No secret, login, wallet, validator,
device or live RPC service was used.

## Package acceptance

`npm ci --offline --ignore-scripts --no-audit --no-fund` installed the exact
lock and 50 packages from the isolated cache. The actual bundled engine was
built from `tools/codama-renderer` with the existing source-path remaps. The
final package then passed:

| Gate             | Actual result                                                   |
| ---------------- | --------------------------------------------------------------- |
| `build`          | Real Rust WASI engine built and selected from Cargo output      |
| `typecheck`      | Production and test projects passed                             |
| `test`           | 9 tests passed, 0 failed                                        |
| `format:dist`    | Emitted JavaScript and declarations formatted with pinned tools |
| `check:package`  | Real 72-file pack inventory, engine imports and privacy checks  |
| `pack --offline` | `codama-renderers-rust-solana-kit-0.1.1.tgz`, 545,039 bytes     |
| `test:packed`    | Archive-only consumer passed on Rust 1.99.1 and Rust 1.89.0     |

The nine package tests covered:

- complete record and resolver contracts through the normal Codama boundary;
- deterministic Rust engine fragments;
- the public render-map visitor through the normal Codama factory;
- the real Codama CLI loading the default renderer;
- regeneration replacing the selected modules while preserving caller files;
- response validation rejecting escaping and duplicate modules;
- destination files and directories that are symlinks failing before writes;
- a symlinked destination ancestor failing before output creation.

## Archive identity

The packed manifest reports name `codama-renderers-rust-solana-kit`, version
`0.1.1`, license `Unlicense`, `engines.node >=24`, `publishConfig.access`
`public` and the existing repository value
`https://github.com/pina-rs/solana_kit/tree/main/packages/codama-renderers-rust-solana-kit`.
The version was read from the actual archive, not the working manifest.

| Artifact                       | Actual value                                                                                      |
| ------------------------------ | ------------------------------------------------------------------------------------------------- |
| Archive bytes                  | 545,039                                                                                           |
| Archive SHA-256                | `af7da8220428ed1ce84972a1fcd5f04d4fc73718e9791b6d55d73f9955c6433b`                                |
| npm `shasum` (SHA-1)           | `360a379d75223539092e10967d59e684c3f01e41`                                                        |
| npm integrity                  | `sha512-gRxeRlQxA4LJZOzTXrCFnMJplG+eqC+lUSpUipa1IQP3ZACBPDdtTl6m7un+FSf0sYGfwkfYJbS19ubY3lJ4RQ==` |
| `package/package.json` SHA-256 | `e27a06967ef75be8ed69d5add8c49910e1d36b624c66306d2d681bdc75c47668`                                |
| Packaged engine SHA-256        | `fb9f6af748a78c3c7c4a6edea93dd3fb409be979b0f2a3b27bf88b083f159ec0`                                |
| `dist/index.js` SHA-256        | `efbf422af2c942feff706e60bba08819d1c948c4d0e0e1e8a0ff5de77aebc972`                                |
| `dist/index.d.ts` SHA-256      | `7dd084406ca02d99ca0974f6354dd6d2615fc4429070b47918b0438f653fbc8a`                                |

The engine is 1,842,390 bytes. It imports exactly five functions from
`wasi_snapshot_preview1`: `environ_get`, `environ_sizes_get`, `fd_read`,
`fd_write` and `proc_exit`. No other host import module is present. The archive
contains no `src`, `test`, `target` or `node_modules` path and no text match for
a home directory, `/Users/` or `/home/`. The 72-file inventory contains only
`LICENSE`, `README.md`, `THIRD_PARTY_NOTICES.md`, `dist`, `engine`, `licenses`
and `package.json`.

A credential-free `npm publish <archive> --dry-run --ignore-scripts --access
public` reported the same name, version, 72 files, size, shasum and integrity.
It warned that a real publish needs registry login and uploaded nothing.

## Packed consumer

`test:packed` installed only the prepared archive closure and its exact locked
dependency artifacts (36 packages) in a disposable consumer with an empty
`PATH` for renderer execution. The normal Codama visitor and the real Codama CLI
generated all six modules and `mod.rs`; the seven files matched byte-for-byte.
The run preserved a caller-owned file across repeated generation and required a
malformed IDL to raise before any generated module changed.

The consumer then executed the nine contract groups and printed
`generated-client-contracts:9`. Strict warning-denied Clippy passed under both
Rust 1.99.1 and Rust 1.89.0. The generated bytes preserved for the runtime hosts
were byte-identical between the two package runs and to the 0.1.0 acceptance
set:

| Module                   | SHA-256                                                            |
| ------------------------ | ------------------------------------------------------------------ |
| `counter.rs`             | `f89b2b5b77502c107b2e8f401479d360b5bb9d2766c09a9f89a1dc102572c817` |
| `empty_pda_contracts.rs` | `8e198db546171e405c0d2a9d9e3ddff404eff11661fc30627be1adbc58fd09de` |
| `literal_contracts.rs`   | `05965721f1dc324cd3e7a6af8279b6ed623ee423ae0a548fd3f7feca3c2575ec` |
| `numeric_contracts.rs`   | `3f77de410ecc3b93279e58a63c162f803a13c7e67250a4b2a51ab824331055f7` |
| `record_contracts.rs`    | `89d9008992a97bfb7900349f301d9a3d52e3ec3e9fbb22e87866baaece2bf975` |
| `resolver_contracts.rs`  | `f9562bf9ac4569348cd77299f4d888478bb311a23007ab7681357c1e611e8ac0` |

## Actual host execution

Every host ran the same nine contract groups and produced
`generated-client-contracts:9`. Native and both WASI commands exited 0. The
Chrome run reported one test passed and none failed. The LeptosKit host rejected
one fuel with `OutOfFuel` and zero commits, then ran with the finite
250,000,000 fuel budget, required exactly one commit and read the complete report
text. The guest imports exactly `leptoskit::{commit,node_create,node_set_props}`.

| Host                   | Runtime                                    | Artifact SHA-256                                                   | Result                    |
| ---------------------- | ------------------------------------------ | ------------------------------------------------------------------ | ------------------------- |
| Native current         | Rust 1.99.1 / cargo `5f94df478`            | `6fa7f3a976df162d2245390e988d40232a68854d055c35be48ba50cb0b7eef06` | exit 0                    |
| Native minimum         | Rust 1.89.0 / cargo `c24e10642`            | `e0b9c7bfe22b51e227645c67815062822a6b0efd6f46d431b21048d34da17521` | exit 0                    |
| Chrome wasm-bindgen    | Chrome and ChromeDriver 156.0.8078.80      | `ca488aa7a93c42c42c242867164fd56bf6ee13767c08236dda72aff6236b33b4` | 1/1 passed                |
| Wasmtime Preview 1     | Wasmtime 50.0.0                            | `a81772916df5c3aa0c555b85b71223d526005e25bad090c908dc82c9faf575ff` | exit 0                    |
| Wasmtime Preview 2     | Wasmtime 50.0.0                            | `9e4e33f4c42d3eea8523f108702bdff069c21cf75afc118fab3b936a78284445` | exit 0                    |
| Pinned LeptosKit guest | `7437097c52aa68a3401bd378764e051ed4c9165a` | `10d7723035df3ed2d9584af9b66079c1499a4e3e08ac15aaca3ae45ee7033df6` | one-fuel trap; one commit |
| Pinned LeptosKit host  | Production `WasmHost` at the same revision | `911d7a5a41e61e46d718067364b89f5ad2dd4d5bc8d93126c11f9597c7680e70` | reports 9                 |

## Registry state

An anonymous `GET https://registry.npmjs.org/codama-renderers-rust-solana-kit`
at 2026-11-05T09:52:03Z returned HTTP 404 with `{"error":"Not found"}`. The
version endpoints `0.1.1` and `0.1.0` also returned HTTP 404. The package is
absent from the public registry at that time. This is a registry observation,
not an artifact qualification: it does not prove name availability, ownership,
future publication or trusted-publisher readiness.

Registry prerequisites from the official [trusted-publisher documentation](https://docs.npmjs.com/trusted-publishers/)
and [`npm trust` prerequisites](https://docs.npmjs.com/cli/v11/commands/npm-trust/):

- Trusted publishing requires npm CLI 11.5.1 or later and Node 22.14.0 or later.
  This workflow uses a GitHub-hosted runner. The separate `npm trust` setup
  command requires npm 11.15.0 or later.
- A package must already exist before a trusted publisher can be configured
  (`npm trust` prerequisites). The first publish therefore needs a maintainer
  with 2FA or a granular access token with bypass-2FA, or a staged publish. A
  staged publish of a new package creates a publicly visible placeholder.
- The trusted publisher must name `.github/workflows/npm-publish.yml`, match the
  `pina-rs/solana_kit` repository, and explicitly allow direct `npm publish`;
  configurations created now are stage-only unless direct publishing is added.
  The configuration expires if its first successful publish does not complete
  within two days.
- The package `repository.url` must match the GitHub repository. The current
  field is the monochange-required subdirectory URL
  `https://github.com/pina-rs/solana_kit/tree/main/packages/codama-renderers-rust-solana-kit`.
  The registry-side match has not been exercised and must be verified by the
  maintainer before enabling the trusted-publisher publish step. A manual first
  publish with 2FA or a token does not depend on that trusted-publisher check.

No npm login, ownership check, token, trusted-publisher configuration, OIDC
exchange or registry write occurred. `npm publish` ran only with `--dry-run`.

## Release workflow behavior

`.github/workflows/npm-publish.yml` is new and manual-only. It has no pull
request, main-push or tag-push trigger.

- The default dispatch is `dry_run: true`. It checks the tag suffix against the
  package version before building, builds the actual WASI engine, runs
  `build`, `typecheck`, `test`, `format:dist`, `check:package`, `npm pack` and
  `test:packed` on the actual archive under Rust 1.99.1 and 1.89.0, verifies the
  archive version, name, 72-file inventory, engine imports and privacy, then
  runs `npm publish --dry-run` with a credential-free npm config.
- A real publish requires an explicit dispatch on `main`, `dry_run: false`, the
  exact release tag and no `checkout_ref`. The checked-out commit must be an
  ancestor of `origin/main`. Tag/version or checkout disagreement and any failed
  qualification step stop before upload.
- The publish step is last and uses the workflow OIDC identity with
  `id-token: write`. It does not read a repository token. The job names the
  `npm-release` GitHub environment; maintainers must protect it with required
  reviewers and use the same environment (or none) in the trusted publisher.
- The registry-side prerequisites above, including the first manual publish, the
  trusted-publisher configuration and the unproven `repository.url` match,
  remain external and are not claimed as ready.

## Other gates

`cargo test --manifest-path tools/codama-renderer/Cargo.toml --all-features
--locked --offline` passed 116 tests with 0 failures. `cargo doc --no-deps` with
`RUSTDOCFLAGS=-D warnings` passed for the consumer/guest workspace and the
LeptosKit host. `dprint check` passed with dprint 0.60.1, `monochange check`
passed with 0 issues across the cargo, changesets, dart and npm suites, and
`cargo xtask style` passed for 2,249 files with repository readability 97.25 and
complexity 98.54; every scored category was at least 90.

## Reproduce

`REPO` is this checkout, `R1_99` and `R1_89` are the installed toolchain `bin`
directories, `WASMTIME` is Wasmtime 50.0.0, and `CHROME`/`CHROMEDRIVER` are the
matching 156.0.8078.80 executables.

The formatter inputs are dprint 0.60.1, monostyle 0.3.27 and the TypeScript
0.96.1 plugin with SHA-256
`9c52244de2c25a33addc7ddacff4443234a2dc4b5d3f34131262d1e0df35007c`.

```sh
cd "$REPO/packages/codama-renderers-rust-solana-kit"
export PATH="$R1_99:$PATH" CARGO="$R1_99/cargo" RUSTC="$R1_99/rustc" RUSTDOC="$R1_99/rustdoc"
export CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2
npm ci --offline --ignore-scripts --no-audit --no-fund
npm run build && npm run typecheck && npm test
export DPRINT=/path/to/dprint MONOSTYLE=/path/to/monostyle \
  MONOSTYLE_CONFIG="$REPO/monostyle.toml" \
  DPRINT_TYPESCRIPT_PLUGIN=/path/to/typescript-0.96.1.wasm
npm run format:dist
npm run check:package && npm pack --ignore-scripts --offline
npm run test:packed -- "$PWD/codama-renderers-rust-solana-kit-0.1.1.tgz" \
  "$REPO/verification/codama-consumer/generated"
```

Repeat `test:packed` with the 1.89.0 `CARGO`, `RUSTC`, `RUSTDOC` and `PATH` for
the minimum-toolchain run. Later host commands reuse the preserved modules
without regenerating source; use a fresh checkout because the preserve
destination must not already exist.

## Limits

These are local macOS arm64 executions. They do not establish Linux or CI
behavior, cross-machine reproducibility, a npm login, package ownership,
trusted-publisher configuration, a published artifact or live network behavior.
No tag, GitHub release, binary upload, registry upload or secret was created.
