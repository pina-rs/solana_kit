# Codama Rust Solana Kit renderer v0.1.1

Reviewable release note for `codama-renderers-rust-solana-kit` 0.1.1. This is a
draft, not a published release. No npm publication, git tag, GitHub release or
registry metadata change occurred.

## Install

Once the package is published:

```sh
npm install codama-renderers-rust-solana-kit@0.1.1
```

The renderer needs Node 24 or later and Codama. It bundles a maintainer-built
Rust WASI engine, so generating a client needs no Rust compiler, install hook or
network operation.

## What changed since 0.1.0

- The package version is now 0.1.1.
- Numeric rendering accepts `i32`, `u128`, `i128` and `f32` in addition to the
  existing widths.
- Scalar, prefix and `u32` instruction discriminator byte order is retained.
- Big-endian array, size and option prefixes use the existing Kit codecs.
- Float-bearing generated clients derive `PartialEq` and `Display` without
  implementing the impossible `Eq` or `Hash` contracts.
- Repeated generation replaces the selected modules and preserves unrelated
  caller files. A rejected IDL leaves the generated set unchanged.

The six generated modules emitted by this release are byte-identical to the
0.1.0 acceptance set. The rebuilt engine and declarations are recorded in the
[0.1.1 package receipt](../verification/codama-0.1.1-release.md).

## Compatibility

- Node: 24 or later.
- Codama: 1.10.2, matching the package dependencies and packed consumer.
- Generated client minimum Rust: 1.89.0, qualified on Rust 1.89.0 and 1.99.1.
- This is off-chain application code. Do not use generated clients in on-chain
  Solana programs; use Pina or an official Solana program library there.

## Qualification

The actual packed archive passed the local acceptance on macOS arm64:

| Check                   | Result                                                                                               |
| ----------------------- | ---------------------------------------------------------------------------------------------------- |
| Package tests           | 9 passed, 0 failed                                                                                   |
| Packed archive          | 72 files, 545,039 bytes                                                                              |
| Archive SHA-256         | `af7da8220428ed1ce84972a1fcd5f04d4fc73718e9791b6d55d73f9955c6433b`                                   |
| Packed manifest version | `0.1.1`                                                                                              |
| Bundled engine SHA-256  | `fb9f6af748a78c3c7c4a6edea93dd3fb409be979b0f2a3b27bf88b083f159ec0`                                   |
| Packed consumer         | Visitor and Codama CLI generated equal bytes                                                         |
| Consumer contracts      | 9 of 9 contract groups on every executed host                                                        |
| Strict Clippy           | Rust 1.99.1 and Rust 1.89.0 passed                                                                   |
| Hosts                   | Native 1.99.1, native 1.89.0, Chrome, Wasmtime 50 WASI previews 1 and 2, pinned LeptosKit `WasmHost` |

The engine imports only `wasi_snapshot_preview1`. The archive has no `src`,
`test`, `target` or `node_modules` inventory and no personal filesystem prefix.
The full command, hash and runtime evidence is in the
[0.1.1 package receipt](../verification/codama-0.1.1-release.md).

## Registry state and publication prerequisites

An anonymous `GET https://registry.npmjs.org/codama-renderers-rust-solana-kit`
on 2026-11-05 returned HTTP 404. No package version is present, so this draft is
not a registry claim.

Publishing is a separate maintainer operation:

- The npm account must own the package name and have 2FA enabled, or use a
  granular access token with bypass-2FA for the first direct publish.
- The first publish cannot use trusted publishing. npm only configures trusted
  publishers for packages that already exist. A staged publish of a new package
  creates a publicly visible placeholder version and still needs 2FA approval.
- After the package exists, a trusted publisher for `.github/workflows/npm-publish.yml`
  needs npm CLI 11.5.1 or later, Node 22.14.0 or later and a GitHub-hosted
  runner. The configuration must allow direct `npm publish`, not only staged
  publishing, and expires if its first successful publish does not complete
  within two days. The package `repository.url` must match the GitHub repository.
  This package still uses the monochange-required subdirectory URL
  `https://github.com/pina-rs/solana_kit/tree/main/packages/codama-renderers-rust-solana-kit`;
  the registry-side match has not been exercised and must be verified by the
  maintainer before enabling the trusted-publisher publish step.
- The prepared workflow defaults to `dry_run: true`; it never publishes on pull
  requests, main pushes or tag pushes. A real publish is an explicit manual
  dispatch on main with `dry_run: false` and the exact release tag.

These are prerequisites, not completed registry actions. No login, ownership,
token, OIDC identity, trusted-publisher configuration or registry write was
exercised here.

## Limits

This is local macOS arm64 evidence. It does not establish Linux or Windows
behavior, cross-machine reproducibility, a CI run, registry ownership or a live
publication. The archive was not uploaded. No device, wallet, validator, secret
or live network service was used.
