---
solana_kit_browser: major
---

# solana_kit_browser: Window HTTP and WebSocket transports

Explicit browser adapters for the executor-neutral client. `BrowserHttp`
implements `Transport` with Window Fetch and local futures, bounded bodies,
deadlines, cancellation, redirect rejection, and no ambient credentials.

`BrowserSubscriptionSocket` adapts the maintained Window WebSocket to the shared
subscription session, retaining event wakers and bounded timers between polls.
Imports exist only for `wasm32-unknown-unknown`; importing the crate is the
explicit opt-in to browser JavaScript.
