---
solana_kit_client: minor
---

# Add bounded caller-owned RPC subscriptions

Drive all nine typed subscription families through explicit text-frame I/O and
caller ticks. Bound registrations, early notifications, retained bytes, and slow
readers. Preserve acknowledgement failures, cancellation, and generation gaps.
No socket adapter or automatic reconnect is included. Publication stays disabled.
