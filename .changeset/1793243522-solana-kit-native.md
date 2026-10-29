---
solana_kit_native: minor
---

# Own explicit Surfpool children with bounded RPC lifecycle

Add a synchronous direct-child owner with caller-selected ports, paths, payer,
fresh genesis and explicit environment. Borrowed readiness and close futures retain
the handle on cancellation, cache reaped status and preserve typed shutdown causes.
Typed settings and runtime paths take precedence over stock environment overrides.
Caller-established OS containment and caller-owned runtime cleanup remain required.
