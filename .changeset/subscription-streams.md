---
solana_kit_client: minor
solana_kit_native: minor
solana_kit_browser: minor
---

# Add Rust streams and callbacks for RPC subscriptions

`Session::notifications` returns a fused stream of typed RPC notifications. `Session::run_callback` delivers those notifications to a Rust closure in the caller's task. Both support explicit unsubscribe and preserve protocol and socket errors.

Use `TokioClock` on native Tokio, `BrowserClock` in browser WebAssembly, or `WasiClock` with the WASI Preview 2 reactor.
