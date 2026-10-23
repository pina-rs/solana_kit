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
| Native Rust 1.99.1                   | 215/215   | 214/215, only SHA-256 |
| Chrome 155                           | 215/215   | 214/215, only SHA-256 |
| Wasmtime 50 Preview 1                | 215/215   | 214/215, only SHA-256 |
| Wasmtime 50 Preview 2                | 215/215   | 214/215, only SHA-256 |
| Pinned LeptosKit WasmHost with wasmi | 215/215   | 214/215, only SHA-256 |

The first 161 case names, assertions and expected outcomes remain unchanged. All
54 collection bodies append to that prefix. The actual interpreter also rejects
a one-fuel allowance before committing a report. Ordinary consumer captures run
all 54 bodies with both Rust 1.99.1 and 1.89.0. MSRV Wasm execution is not claimed.

A separate copied-source build changes the item-boundary marker comparison. The
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
