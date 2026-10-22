# RPC pilot provenance

The catalog, method parameter signatures, declared-field summaries and canonical
IDs derive from the Dart Solana Kit repository at immutable revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. The upstream MIT notice is preserved
in DART-LICENSE. Source SHA-256 values and defining paths remain in
`docs/parity/rpc-catalog.tsv`; no Dart method implementation is copied.

The executable Rust designs, strict JSON vectors and loopback fixtures are newly
authored. The three method response shapes were cross-checked against the primary
protocol links in docs/architecture/rpc.md. Literal sample values deliberately
exercise zero-address encodings and numeric limits, rather than account history
or live user data. No live RPC endpoint is called by this workspace.
