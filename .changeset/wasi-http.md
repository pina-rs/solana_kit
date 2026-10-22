---
solana_kit_wasi: minor
---

Add an explicit WASI Preview2 HTTP adapter and caller-owned readiness reactor for off-chain clients. Named-host policy and lifecycle acceptance remains documented separately.

Preserve typed host denial and early HTTP responses during upload. Classify structured WASI HTTP stream failures without parsing diagnostic strings.

Allow Waker callbacks to inspect the reactor during polling, readiness completion and cancellation by releasing registry borrows before callbacks and destruction.
