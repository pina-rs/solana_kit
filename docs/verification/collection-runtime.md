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

The current-main integration retains the Reader transaction and generator import
repairs. Fresh full core captures on each compiler report 154 passes. These are
148 ordinary native bodies, five executed doctests, and one expected
compile-failure doctest. Thus 153 bodies execute at runtime. The earlier
151-pass collection receipt remains historical evidence. The separate ordinary
consumer executes all 54 contract bodies.
Full workspace, consumer, and private host Clippy and documentation checks deny
warnings. Every authored SDK file meets both unchanged Monostyle 0.3.26 floors
of 90. The formatter uses authenticated cached plugins under a network-denied
boundary.

The following digests identify the preserved executed artifacts. The browser
input is transformed by wasm-bindgen before execution. The transformed Wasm,
generated JavaScript, and runner are retained separately.

| Artifact                   | SHA-256                                                            |
| -------------------------- | ------------------------------------------------------------------ |
| `browser.wasm`             | `ae9e12079035eff9a75e9363f3b0038ea69ce8f7a09f9caa91edcd7deeafc1c9` |
| `guest.wasm`               | `bedbce8bc088b8aacad660fa02b02041b45f958635823bbda5399c44042246ce` |
| `leptoskit-host`           | `5296a625fb029037c3df83a00eb29d211673eb6ec0daa2f45bd4c8c6c1b265ab` |
| `native`                   | `e956a7bad8a009294aa51f93dbbd863d92e069ca509588806ac91d7e295f60f2` |
| `wasip1.wasm`              | `cf11be3c53599e51fd969d651ed8f4d1fcdbedb26a8bd30415cdf31227ad071e` |
| `wasip2.wasm`              | `640ce31731dc873287b13d9d52a166db267f97378ccf2017824d33eb68f0f812` |
| `xtask`                    | `966715845eef2080b9f58d9a0414b78e8932b48158264e4b421a87fbfb13b2eb` |
| `browser-transformed.wasm` | `96c3cf6fb4c9a8d974a77fa67dc901f6dd7c0de54e0a01dd9be154a65d78c002` |
