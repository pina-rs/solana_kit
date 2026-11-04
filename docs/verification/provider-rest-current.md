# Current provider REST transport execution

The public REST transport adapters executed locally on 2026-11-04 at SDK source
`9694851f546c051080aecc42f070ae08b1c07271`. These are actual owned loopback
requests through the named runtimes, not compilation-only or fabricated replies
inside the adapters. No production provider endpoint or credential was used.

| Adapter          | Actual execution                                                                                     | Result                                                                                                  |
| ---------------- | ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `NativeRest<D>`  | Workspace `solana_kit_native` HTTP and REST-policy tests, including the native TLS peer              | Passed                                                                                                  |
| `BrowserRest<D>` | `solana_kit_browser_http_verification` in Chrome/ChromeDriver 156.0.8078.80                          | 8 tests passed, including both REST groups; all fixture threads joined                                  |
| `WasiRest<D>`    | Actual Preview 2 component in the private Wasmtime 50.0.0 embedding                                  | All 23 journeys plus the CLI baseline passed, including four REST journeys                              |
| `GuestRest<D>`   | Actual guest in production LeptosKit wasmi, framework pin `7437097c52aa68a3401bd378764e051ed4c9165a` | REST success, body bound and host grant denial passed in both borrowed and owned host forms, twice each |

## Boundaries exercised

The native peer checks bodyless GET, JSON null, form media type, encoded path
segments, ordered duplicate query pairs, denied unknown routes without a
connection, actual response bounds, framing errors and pending-socket Drop.
The TLS peer checks that credentials and submission bytes stay behind verified
TLS. The native REST-policy target also exercises configuration rejection before
I/O. These tests passed in the full local Rust 1.99.1 workspace run.
The same native HTTP and REST-policy targets also passed on Rust 1.89.0
(20 and four tests respectively).

The browser REST groups send actual Fetch GET, JSON POST and form POST requests,
retain an `Rc` across await, reject unknown routes and GET bodies, bound streamed
responses and observe peer closure after dropping a pending exchange. The six
existing RPC groups also passed, preserving their authentication, redirect,
deadline, decompression and failure checks.

The Preview 2 REST groups check exact POST bytes and target encoding, an unknown
route, an oversized body and an actually truncated body retaining its diagnostic
source. The guest's Reactor registration count returns to zero. The embedding
checks the exact warm resource baseline, no remaining network flights, restored
task counts and Store destruction. The full run also executes the existing
HTTP/TLS, early-response, cancellation, repeated-use and replacement cases.
This private host retains its documented narrow correction for Wasmtime 50's
consumed request-options handle; it does not establish leak-free behavior in an
uncorrected embedding.

The LeptosKit guest sends the bounded additive method1 frame through actual
upstream Calls and application-routed Events. Independent peers check the JSON
null body and encoded target. A denied host grant starts no native job; accepted
requests and response limits use the existing HTTP job owner. Every host form
also completes its existing pending/wake, cancellation, replacement and awaited
shutdown assertions. This is interpreted-host execution, separate from installed
OS UI acceptance.

## Reproduction and artifacts

- Run `cargo test --locked -p solana_kit_native --all-features --test http --test rest_policy`.
- Set the actual matching `CHROME` and `CHROMEDRIVER`, use
  `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner`, then
  `cargo run --locked -p solana_kit_browser_http_verification`.
- Build `embedded_guest` with the manifest at
  `verification/wasi-http-prototype/Cargo.toml` for `wasm32-wasip2`.
  Set `SOLANA_KIT_WASI_HTTP_GUEST` to that actual emitted component and run the
  same manifest's native `embedded` binary without a case filter.
- Build `solana_kit_leptoskit_http_guest` with
  `verification/leptoskit-http/Cargo.toml` for `wasm32-unknown-unknown`.
  Run `solana_kit_leptoskit_http_host` from that workspace with the emitted guest
  path as its required argument.

All four commands returned exit 0. The WASI dependency initially was not cached;
the offline attempt failed before compilation, then the pinned dependency was
fetched and the actual build and host run succeeded. Full command output is
retained locally in `/tmp/solana-kit-browser-rest-host.log`,
`/tmp/solana-kit-wasi-rest-host.log` and
`/tmp/solana-kit-leptoskit-rest-host.log`. Preview 2 public-adapter Clippy and
LeptosKit guest Clippy also passed with warnings denied.

Executed guest SHA-256 values:

```text
e0a7b35c926e1a5e851b39568e1488a4afbf835511f933751bd62565c67d5fb3  embedded_guest.wasm
cb5087bdc670943ef8c7b40a41bdb37ad97b3c929ec1c0424b11493d0017e0e4  solana_kit_leptoskit_http_guest.wasm
```

## Limits

Transport success does not adjudicate every Helius, Jupiter or Hermes operation,
nor establish production-provider compatibility. Their typed request/response
contracts and declaration evidence remain separate. No canonical capability
status, source hash, shared golden or conformance-suite version changed.
Browser CORS, host destination/TLS policy, operating-system DNS behavior and
installed platform journeys retain their documented limits.
