# Portable collection contracts

Solana Kit is an off-chain application client SDK. On-chain programs use Pina or
Solana's program libraries. These collection codecs retain `no_std + alloc` and
add no normal or build dependencies.

The [ordinary consumer](../../verification/collection-layouts-consumer) executes
54 public contract bodies. It retains 42 original collection, Option, framing,
composition and accounting bodies, plus 12 independent boundary controls. The
[body map](../../verification/collection-layouts/runtime-map.tsv) identifies every
original function. The maintained [auditor](../../verification/collection-layouts/body-audit.rs)
compares all 54 complete bodies and all 12 modules, including helpers. Its 281
ordered body assertion sites retain their predicates and literals. No body is
excluded as native-only.

Adapters change test visibility, explicit prelude imports and fixture paths. Two
empty byte-array comparisons add a type annotation to resolve an ambiguity from
serde_json's comparison implementations. Shared framing and composition helpers
use their existing module definitions. Original assertions still execute and
abort on a mismatch; registration cannot turn an assertion failure into success.

| Executed host                        | Canonical | Deliberate corruption |
| ------------------------------------ | --------- | --------------------- |
| Native Rust 1.99.1                   | 259/259   | 258/259, only SHA-256 |
| Chrome 155                           | 259/259   | 258/259, only SHA-256 |
| Wasmtime 50 Preview 1                | 259/259   | 258/259, only SHA-256 |
| Wasmtime 50 Preview 2                | 259/259   | 258/259, only SHA-256 |
| Pinned LeptosKit WasmHost with wasmi | 259/259   | 258/259, only SHA-256 |

The first 205 case names, assertions and expected outcomes remain unchanged.
These include the completed-packer correction and all 43 Anchor groups. All
54 collection bodies append to that prefix in suite `solana-kit-core-v31`. The actual interpreter also rejects
a one-fuel allowance before committing a report. Fresh ordinary consumer captures run
all 54 bodies with both Rust 1.99.1 and 1.89.0 from Cargo cwd `/`, using an absolute
manifest path and no repository flags or patches. MSRV Wasm execution is not claimed.

A fresh copied-source build of this aggregate changes the item-boundary marker
comparison. The
independent multi-byte-marker assertion then fails with `Truncated`; its unchanged
positive control passes. The negative build owns a separate compiler target and
retains its actual executable, core dependency bytes and producer records.

The original 35-row Dart oracle and 144 Dart tests remain carried evidence from
the exact pinned source. This integration freshly replays all 35 literal rows in
the full body suite. The [73-ID disposition table](../parity/collection-layouts.tsv)
keeps 17 native Option mappings and 56 partial contracts. Rust hash values, generic
formatting, dynamic type inspection, heterogeneous recursive traversal, metadata,
callback semantics and larger count domains retain their stated limits. No
canonical declaration status is promoted by this runtime proof.

The core's three Wasm links and closed dependency graphs pass. The plain guest
imports only the three declared LeptosKit rendering operations. Valid forbidden
import and dependency controls reject. These deterministic host tests establish
neither installed desktop/mobile journeys, validator integration nor Phase 2
completion.

The pooled host allowance is 11,850,000,000 fuel units. The 54 new ordinary
cases add 810,000,000 units. Existing crypto categories, rates, and work counts
remain unchanged. All three host fuel-accounting tests execute successfully.
This fixed allowance does not measure performance.

Fresh full core captures on each compiler report 151 passes. These are 145
ordinary native bodies, five executed doctests, and one expected compile-failure
doctest. The separate ordinary consumer executes all 54 contract bodies.
Full workspace, consumer, and private host Clippy and documentation checks deny
warnings. Every authored SDK file meets both unchanged Monostyle 0.3.26 floors
of 90. The formatter uses authenticated cached plugins under a network-denied
boundary.

The following digests identify the preserved executed artifacts. The browser
input is transformed by wasm-bindgen before execution. The transformed Wasm,
generated JavaScript, and runner are retained separately.

| Artifact                   | SHA-256                                                            |
| -------------------------- | ------------------------------------------------------------------ |
| `browser.wasm`             | `d3ddcd8b062acb62184b409fc04fb3fd3701905327d97538fa505268c8b3a8a0` |
| `guest.wasm`               | `066f9e30c77aee0bcbf925780ccb98d7c67a5bddf8c017e10657b73eb4c0f85c` |
| `leptoskit-host`           | `d0c0ab3a0dc02008aa53b903171ec2ed57b396d64c867df7bfaefffa5937410e` |
| `native`                   | `944d17385e4431a9a0af8bbc6d2ffddff37df0cabc1f9d3061c13b78356b32e8` |
| `wasip1.wasm`              | `8cbe0f6e431bd8f9ca94c96c7e8854000a60abdf3064a25cd1efcabf3e06527d` |
| `wasip2.wasm`              | `68b2d4c1b13d6f24b17f84c49df91d2086303b5295c121b9de8bc9e1e53ff261` |
| `xtask`                    | `6462c266f0eb4a117f15fa7d7124045299c61af73d9a929d43c9a4a199c1384c` |
| `browser-transformed.wasm` | `7b4555ba6a8fbf64649d7e0449ea24e06b5d94791bf5013c370ae3c6dfc322b0` |
