# Codama Rust Solana Kit renderer

Generate off-chain Rust program clients with Kit-owned addresses, codecs,
instructions, builders, parsers and PDA resolvers. The NPM package bundles one
maintainer-built Rust WASI engine. Generating clients needs Node24 or later and
Codama; it needs no Rust compiler, install hook or network operation.

This package is for application/client code. Do not use generated clients in
Solana on-chain programs. Use Pina or official Solana program libraries there.

```typescript
import { createFromRoot } from "codama";
import renderVisitor from "codama-renderers-rust-solana-kit";

createFromRoot(root).accept(renderVisitor("src/generated"));
```

A Codama CLI script can use the package's default export:

```json
{
  "idl": "program.json",
  "scripts": {
    "rust": {
      "from": "codama-renderers-rust-solana-kit",
      "args": ["src/generated"]
    }
  }
}
```

Run the script through the normal Codama CLI. For in-memory output, use
`visit(root, getRenderMapVisitor())`. It returns Codama's standard
`RenderMap<BaseFragment>`, with flat Rust module paths and resolved source.

Repeated generation updates the selected generated modules. It preserves
unrelated files and does not delete stale modules from an earlier IDL. Only the
current modules appear in `mod.rs`. IDL resolution and path validation complete
before any output write. Existing symlink destinations are rejected. Filesystem
write failures can leave a partially updated generated set, following Codama's
ordinary writer behavior; callers needing a release transaction can generate in
a separate directory first.

The shared Rust frontend accepts the pinned Codama1.8 schema emitted by
`@codama/nodes`1.10.2 and the legacy1.0 fixtures. It expands the documented omitted
empty collections without discarding unknown semantic properties. The existing
finite typed engine rejects unsupported nodes, program constants, ambiguous
names, invalid addresses, unsupported wire options and missing instruction
headers explicitly. This is not an arbitrary JSON-to-Rust template processor.

Node's `node:wasi` runs this package's trusted engine with isolated stdio, empty
environment and no preopened directories. It is not a security sandbox for
arbitrary third-party Wasm modules. Node tooling is absent from generated Rust
application dependency graphs.

## Maintainer verification

The engine lives in `tools/codama-renderer`. Its `verification` feature enables
private proof commands; the bundled WASI binary is built without that feature.
Existing native program adapters and the NPM visitor use the same typed IR and
emitter. The production file-map boundary formats Rust with portable
`prettyplease`/`syn`, shared by native and WASI generation. Formatting errors
abort generation; no rustfmt subprocess or raw-source fallback is used. The
package contains no fallback or stub engine.

Prepare the pinned Cargo/NPM dependency caches explicitly, then run these steps
from this directory with the admitted concrete Cargo/rustc and owned caches:

```sh
npm run build
npm run typecheck
npm test
npm run format:dist
npm run check:package
npm pack --ignore-scripts --offline
npm run test:packed -- /absolute/path/to/the-built-package.tgz
```

`format:dist` requires explicit verified `DPRINT`, `DPRINT_TYPESCRIPT_PLUGIN`,
`MONOSTYLE` and `MONOSTYLE_CONFIG` inputs. Prepare their offline caches first.
It formats emitted JavaScript and declarations before the archive is inspected.

`build` selects the actual Cargo-reported WASI artifact. It remaps personal source
prefixes. If extra Rust flags are needed, provide `CARGO_ENCODED_RUSTFLAGS` so they
can be retained without ambiguous shell parsing. `check:package` inspects the
real pack inventory and engine imports. The packed-consumer test installs
only from an already prepared offline cache and generates with an empty PATH.
It compares the normal visitor with the real Codama CLI, preserves unrelated
files and checks that a rejected IDL leaves generated source unchanged. Empty
PATH is a tool-discovery control; an external execution-denial boundary is
required to prove that no installed Rust executable can run.

The maintained `verification/codama-consumer` fixture contains eight generated-client
contract groups: literal bytes and malformed inputs, borrowed codec metadata,
imported signing keys, complete account/event discriminator predicates, positional
optional roles, omitted suffixes, literal defaults without PDA recipes, and
collision-free PDA helpers including account-free programs. The `pda` feature is
enabled in this consumer so those helpers must compile. `test:packed` requires concrete `CARGO`, `RUSTC`
and `RUSTDOC` paths, executes those groups and runs strict Clippy. Its guest
contains the same assertions behind the ordinary LeptosKit ABI; maintaining that
source does not prove that the portable guest has executed.

A later admitted verification must compile and execute generated Rust consumers
on native Rust, browser Wasm, both WASI previews and the normal LeptosKit wasmi
route; compare native/NPM output, malformed-IDL controls, dependency/import
closures and all supported packaged/generated source scores. Source-only work
does not establish those results. Registry publication remains disabled.
