# Wallet Standard verification

This independent Rust 1.89 consumer executes a concrete native backend in a real
Tokio task and a local backend holding Rc across Pending and wake. The backends
are deterministic fakes. They do not prove real user authorization or deployed
wallet support. The production crate has no Tokio or SHA dependency.

Run the consumer from its workspace with
`cargo +1.89.0 run --offline --bin wallet_consumer`.

From the repository root, run the `graphs` binary with its manifest path to check
the actual default dependency closure. It accepts only the wallet crate and
base64. Injected Tokio, getrandom, Solana SDK, web-sys, and reqwest rows must fail.

The `receipt` binary takes the repository and immutable Dart reference paths.
It verifies the catalog's source hashes before writing all 203 declaration IDs.
The `logos` binary takes the reference and repository paths in that order and
extracts all 36 fallback icons plus the generic glyph. Run rustfmt afterward.
The package license and asset attributions remain in the public crate notices.

`execution.tsv` normalizes commands and records their measured scope. Raw logs
remain in the coordinator's ignored primary evidence store.
