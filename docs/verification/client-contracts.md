# Public client contract execution

These adapters serve off-chain client applications. On-chain programs should use
Pina or the official Solana program libraries.

The maintained jobs below select each component from actual Cargo JSON output.
Selection requires its manifest, target name, target kind and a unique emitted
Wasm filename. Native runners receive that concrete path; they do not infer a
repository-local target directory. Each runner retains the existing bounded peer,
host policy and joined teardown checks.

## Maintained bodies awaiting execution

- `native-http.yml` runs native direct REST tests on current Rust and Rust 1.89.
  Owned peers inspect POST/PUT, encoded paths, duplicate queries, JSON and form
  bytes. Additional bodies check denied routes, reply limits, malformed framing
  and cancellation EOF.
- `wasi-http.yml` runs the original CLI suite and embedded guests from both Rust
  producers. Direct REST adds exact authenticated wire delivery, denied routes,
  reply limits and truncated framing through the existing Preview2 HTTP owner.
- `client-sessions.yml` runs typed confirmation factories on Preview1 and
  Preview2. It registers the reactor's no-signal, timer and unwakeable controls.
  Its real TLS peer also drives typed root subscription, an early notification,
  unsubscribe acknowledgement and close through the public Session in both the
  Wasmtime CLI and embedding. Existing denial and malformed socket cases remain.
- `wasi-sockets.yml` passes the selected component to the original socket proof.
- `leptoskit-http.yml` composes direct REST with the production interpreted guest
  and native task owner. The same host suite runs borrowed and owned application
  lifetimes, replacement, quiet cancellation and awaited shutdown.

Adding a job or assertion does not establish a passing contract. The new bodies,
ordinary feature and documentation gates, network journeys and their exact Rust
producers require execution before merge. Existing browser and deterministic
portable receipts retain their original source qualification. Generated desktop,
Android and iOS end-to-end application journeys remain separate required work.

Normal releases and registry publication remain disabled. These verification
packages are private fixtures and must never receive placeholder reservations.
