# Private typed RPC experiment

This workspace compares two async boundaries and proves a small typed JSON-RPC
pilot for off-chain application clients. It is not a public SDK crate and must
not be used in on-chain programs; use Pina or official Solana program libraries.

Run from the repository root with cached dependencies:

```sh
cargo +1.99.1 test --manifest-path verification/rpc-design/Cargo.toml --locked --offline
cargo +1.99.1 clippy --manifest-path verification/rpc-design/Cargo.toml \
  --all-targets --locked --offline -- -D warnings
cargo +1.99.1 fmt --manifest-path verification/rpc-design/Cargo.toml --check
cargo +1.99.1 run --manifest-path verification/rpc-design/Cargo.toml \
  --bin catalog --locked --offline -- /path/to/pinned-dart
```

The catalog command requires the immutable clean Dart source revision and checks
all 54 methods, exact canonical IDs and source SHA-256 against the committed plan.
Add `--write` only to regenerate docs/parity/rpc-catalog.tsv. The lexical field
summary is an audit aid, not a general Dart semantic parser. Exact IDs come from
the existing tree-sitter capability inventory.

Tests execute the same generic client through a native Send exchange and an Rc
exchange that actually suspends. They also execute the erased-future alternative
locally and reject its native spawn at compile time. Typed JSON cases cover
balance, latest blockhash, and nullable base64-account wire shapes, with malformed
and precision controls. Real loopback HTTP observes a typed response and pending
call abort followed by socket EOF; retaining the call first produces no EOF.

See [the design](../../docs/architecture/rpc.md) for the selected boundary,
proposed public crate ownership, all 54 implementation scopes and precise limits.
No browser, WASI, interpreted guest or mobile RPC runtime is claimed here.
