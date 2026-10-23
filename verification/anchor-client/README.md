# Anchor public consumer and boundary checks

This ordinary standalone Cargo workspace consumes the public Anchor/Core APIs
without repository-specific flags or patches. `cargo test` executes the public
wire suite plus novel instruction, deferred-type and nested-option contracts.
Its binary is a real portability probe, compiled for all three promised Wasm
targets. Compilation and import validation do not establish host runtime support.

The separate `audit` workspace is native verification tooling. It validates
actual module/component bytes, checks root import allowlists, rejects core and
component capability injection, and checks closed normal/build dependency lists
with an injected Tokio negative. It is absent from SDK dependency graphs.

All compiler commands in unit 258 use explicit frozen project-private producers,
separately owned Cargo/target directories and retained executable captures. The
production library is an off-chain application client. On-chain users use Pina
or official program libraries.
