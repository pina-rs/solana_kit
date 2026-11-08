---
solana_kit_wasi: major
---

# solana_kit_wasi: WASI Preview 2 HTTP

`WasiHttp` implements the public client transport with caller-driven local
futures for `wasm32-wasip2`. A caller-owned `Reactor` registers pollables,
selects readiness and deadlines, and wakes the guest without a global executor
or detached task.

The adapter follows the pinned `wasi:http/io/clocks@0.2.12` contract, preserves
explicit deadlines and body budgets, and leaves DNS, TLS, proxy, credential, and
destination policy to the embedding host.
