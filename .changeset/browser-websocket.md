---
solana_kit_browser: minor
---

# Add caller-owned browser WebSocket operations

Connect, send bounded text, receive and close through one local browser Window
owner. Receive cancellation preserves its queue and socket. Terminal events and
Drop detach handlers. Explicit budgets reject queue overflow, oversized messages
and browser send backpressure. Typed subscriptions retain their existing caller
owned Connection. No reconnect, task, arbitrary authentication headers or worker
support is added. Component publication remains disabled.

Clone and release executor wakers outside state borrows so synchronous destructor
callbacks can dispatch browser events during cancellation and replacement.
