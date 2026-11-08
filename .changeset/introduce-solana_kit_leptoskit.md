---
solana_kit_leptoskit: major
---

# solana_kit_leptoskit: interpreted LeptosKit adapters

Guest and host adapters for interpreted LeptosKit applications. `GuestHttp`
connects the ordinary public client to named host services; `HttpDriver` and
`HttpHost` install fixed destinations, TLS roots, grants, deadlines, and byte
ceilings in the native host.

Every guest entry point drains unrelated requests and cancellations, while
completion delivery retains no interpreter borrow and wakes the real guest. The
optional wallet adapter exposes the portable MWA protocol to installed hosts.
