# Browser HTTP ownership

This adapter is exclusively for off-chain browser windows. Workers and interpreted
guests require their own explicit host contracts.

Two API sketches were compared before implementation. A custom exchange state
machine could own fetch, timer and stream states directly, but every JavaScript
promise transition would duplicate polling and cleanup logic. A local boxed async
future can own a single guard containing the AbortController, timer callback and
stream reader. The guard drops on every exit and aborts the operation. The latter
keeps the public GAT concrete without promising Send or starting a detached task.

`Client::new(BrowserHttp::new(endpoint, config)?, limits)` preserves the public
protocol owner unchanged. BrowserHttp owns endpoint and headers. Request owns the
receive byte budget. Fetch owns browser networking and connection reuse. The
exchange guard owns its timer and abort signal until body reception finishes.
All JavaScript failures become redacted portable categories. The browser hides
CORS, TLS and network failure detail, so the adapter does not invent distinctions.

The adapter reads Uint8Array chunks and checks checked-add against the request
budget before Rust reservation or copying. The browser may allocate a larger
chunk or decompress before Rust receives it. Browser parser and header buffers
cannot be controlled through Fetch. A hidden Content-Encoding header cannot prove
that Content-Length describes the decoded body. Only an explicitly exposed
identity encoding enables the Content-Length early check.
