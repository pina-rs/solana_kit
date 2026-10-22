# Actual interpreted guest HTTP

This optional adapter serves off-chain applications. On-chain programs use Pina
or official Solana program libraries. A plain portable core guest still requires
exactly three renderer imports. This separate application guest requires exactly
those three plus the eight upstream `leptoskit_plugins_v1` functions. The compiled
artifact gate checks every import and integer signature. No JavaScript, WASI or
ambient socket import is permitted.

The real guest uses public `Client<GuestHttp>`. The real host uses public
`HttpHost`, pinned `leptoskit_wasm::WasmHost`, interpreter wasmi2 and `NativeHttp`.
All LeptosKit crates resolve to immutable revision
`fecf8628d326ff7b03f295523e6bb584c3fdf88d`. No recreated runtime or test-only
resume export supplies readiness. The application owns `LocalPool`, exports
`leptoskit_poll`, and owns the singleton upstream Events receiver. Upstream Call
registers real wakers; `WasmHost::wake_plugins` invokes readiness and the poll
export. A custom event-based completion design was rejected because it duplicated
Call and imposed singleton event routing on every transport operation.

Installed hosts explicitly grant named services. A name selects a fixed host
NativeHttp instance with endpoint, TLS roots, authentication, parser budgets and
deadlines. Guest requests contain only JSON and a receive budget. Native tasks
carry private instance/call tokens, never guest memory pointers. The upstream ABI
validates guest linear-memory ranges before copying. Host admission validates its
own ceilings before an additional request-body copy. Native receive budgets apply
while receiving. A completed reply is checked again in the guest.

The owner counts aborting jobs until `join_next_with_id` observes termination.
`shutdown().await` aborts and joins every job before unloading the actual host.
Drop of the guest call cancels through the real bridge; Drop of the host owner
aborts native tasks as a fallback. Only awaited shutdown establishes completed
unload cleanup. No task owns the guest, callback channel or renderer pointer.
Replacement creates a fresh private instance identity and task owner.

Execute from the repository root:

```sh
cargo +1.99.1 build --locked --offline \
  --manifest-path verification/leptoskit-http/Cargo.toml \
  -p solana_kit_leptoskit_http_guest --target wasm32-unknown-unknown
cargo +1.99.1 run --locked --offline \
  --manifest-path verification/leptoskit-http/Cargo.toml \
  -p solana_kit_leptoskit_http_host -- \
  verification/leptoskit-http/target/wasm32-unknown-unknown/debug/solana_kit_leptoskit_http_guest.wasm
cargo +1.99.1 test --locked --offline -p solana_kit_leptoskit --all-features
```

The required artifact must exist; a missing file fails. Two sequential real
instances each execute authenticated HTTPS balance/u64::MAX, non-2xx rejection,
streaming body limits, a quiet native deadline, local Rc across observed Pending
and wake, retained-call no-EOF control, application-event cancellation and EOF,
unrelated event routing, then pending-I/O unload with actual abort/join and EOF.
Six native adapter tests exercise actual upstream pointer guards, malformed and
over-budget admission, preserved unrelated requests/cancellations, stale instance
rejection and completed native failure arriving after cancellation. The stale
token test deliberately injects a private task result; it does not claim an
installed application callback journey.

This is headless actual-host evidence. Generated desktop, Android emulator and
iOS simulator app loops have not been wired or executed by this unit. The package
requires unpublished pinned LeptosKit source; registry release prerequisites remain
open and publishing is disabled. No mobile/device or whole-ecosystem completion
is implied.

## Integrated-source replay

The accepted adapter and fixture production source remain unchanged after
integration with the current core, client, native and browser implementations.
Both the Rust 1.99.1 and Rust 1.89.0 rebuilt guests execute the two-instance
HTTP/TLS, Pending/wake, cancellation, peer EOF and awaited-unload journeys above.
The six adapter source controls and four compiled-artifact ABI controls pass.

The executed Rust 1.99.1 guest has SHA-256
`4aa4154d6a3e6bf0f4f33b88b6fe8a52551b70e398c248acb247a0ef1e1d03e1`.
The separately rebuilt and executed Rust 1.89.0 guest has SHA-256
`103f9b89912da753da5faaba11bac36db635569e3a69a6b63e047eb30a745008`.
These identify the local debug builds, without a cross-machine hash promise.
The committed fixture receipt records direct stdout and the 1.99.1 artifact.

An unchanged independent consumer passes four ordinary public-API test groups
under Rust 1.89.0 from a separate Cargo workspace, with compiler flags and
wrappers absent. It observes host-owned destinations and authorization, delayed
application delivery after real native completion, cancellation before join,
cancellation of a proven successful ready reply, retained-call capacity and EOF,
awaited unload, and a real guest destination trap that preserves ready bytes.
All successful fixture peers and tasks finish and are awaited.

The separate plain-core guest still passes its exact three-import policy and
unchanged dependency negative controls. The existing portable protocol suite is
unchanged by this adapter. Its host evidence remains in the
[runtime receipt](runtimes.md). This HTTP replay adds no protocol cases or device
claim. A clean offline archive attempt fails on unpublished
`leptoskit_bridge` 0.1.0, so it provides no registry-install evidence.
