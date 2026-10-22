# Browser HTTP integration verification

The reviewed Window Fetch adapter and its original six-group browser fixture are
byte-identical to `7ec4668035eb0c9b6ee24fdb1752414767d7de43`. They now coexist
with the reviewed native transport, System client, node operations, and numeric
codecs. Executed integration source is `374d6ad`, whose numeric parent is
`9a824a11ff68fda859dbeaf337fb3079257b5bf1`. This SDK is exclusively for off-chain
application code. Use Pina or official Solana program libraries on-chain.

The original [browser receipt](browser-http.md) describes the supported API and
its boundaries. This integration adds no production behavior. Both actual Chrome
runs use installed Chrome 155.0.8059.85, driver 155.0.8059.84, and
wasm-bindgen-test-runner 0.2.128. `WASM_BINDGEN_TEST_DRIVER_TIMEOUT=30` changes
only driver startup allowance. Every SDK deadline assertion remains unchanged.

The authored six groups pass with zero ignored or filtered tests. Exact typed
requests and full-width responses, local futures, credential isolation, status,
streamed and decompressed body limits, deadlines, Drop cancellation, redirects,
and constructor validation all execute against owned loopback servers. The native
accepted-socket race regression also passes. All fixture owners join.

The unchanged independent review consumer runs again with ordinary source-path
dependencies from a separate Cargo workspace. It executes 120 completion, error,
and empty-response calls, plus six repeated pending-call Drop controls. Rust-owned
JavaScript roots remain 23 before and after. This measures retained Rust roots,
not complete browser heap use. Ambient cookies and referrers remain absent.
Explicit authentication reaches only its selected peer. The separate redirect
sink receives zero connections. All listeners and worker threads join.

Workspace default and all-feature tests, MSRV 1.89 checks, browser compilation,
warning-denied Clippy and documentation, rustfmt, dprint, actionlint, and the
ordinary consumer's MSRV browser test compilation pass. Eight core/client
normal/build graph queries retain their exact portable dependency allowlists and
reject the injected JavaScript control. Updated core feature, numeric, System,
and client graph checks also pass. The rebuilt plain guest imports exactly
`leptoskit::{node_create,node_set_props,commit}`. Three Wasm target checks are
compilation evidence. Actual-host protocol execution remains covered by the
unchanged numeric parent source; this integration does not rerun that protocol
suite or claim guest networking.

Monochange has one SDK version owner for core, client, native, and browser. System
keeps independent ownership. Every publication route remains disabled, and the
umbrella reservation stays at 0.0.0. Offline archive preparation stops at the
explicitly unpublished `solana_kit_client` registry prerequisite. Source consumer
success does not imply registry release readiness.

Window Fetch HTTP is the accepted scope. Workers, WebSockets, positive browser
HTTPS and IPv6 fixtures, WASI or interpreted-guest networking, installed desktop
or mobile journeys, wallet integration, and complete ecosystem parity remain
separate work. Browser CORS, trust, network allocation, event-loop scheduling,
and internal networking behavior retain the limits in the original receipt.
