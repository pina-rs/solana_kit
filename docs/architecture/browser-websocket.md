# Browser WebSocket ownership

This adapter runs off-chain application clients in a browser Window. The existing
`Connection` owns typed subscription correlation, generations and acknowledgement
budgets. The application drains `Connection::outbound`, sends complete UTF-8
frames once, awaits `receive`, and calls `Connection::ingest`. Socket I/O and
protocol state remain separate owners. There is no receive task or reconnect.

Two compiling ownership probes compare a borrowed operation future with a cloned
queue observer. The borrowed future prevents simultaneous receivers and makes
cancellation release only its timer and waker. A cloned observer admits competing
receivers and requires runtime arbitration. The borrowed representation is chosen.
A single owner retains four JavaScript event closures. Weak callback references
avoid an ownership cycle. Terminal transitions detach handlers before waking a
caller. Closure storage survives callback return and is destroyed by owner Drop.

`BrowserWebSocket::connect(endpoint, config).await` owns construction until open.
`send(&mut self, text)` checks the encoded message and browser bufferedAmount
before calling the browser synchronously. `receive(&mut self).await` borrows the
owner. `close(self).await` consumes it. Cancellation of connect or close drops the
socket. Cancellation of receive preserves queued messages and the socket.

Every operation has a positive finite deadline. Expiry ends the socket. Explicit
close discards queued messages. Terminal events clear the receive queue, detach
all handlers and wake the pending operation. An error after terminal state cannot
replace its first cause. Browser network events hide TLS and handshake detail, so
those failures use a redacted Browser category rather than invented diagnostics.

The browser handles RFC6455 framing, fragmentation, ping/pong, TLS and network
buffering. Rust queue and message limits apply at the event boundary, not before
the browser allocates an incoming message. Binary ArrayBuffer and Blob events
are explicitly rejected because this transport accepts JSON text only. Text is
never decoded lossily. Browsers cannot set arbitrary Authorization headers.
Caller-supplied URL query parameters or subprotocols are supported, subject to
browser policy. Browser-managed cookies may be sent. No endpoint, subprotocol,
close reason or message is included in adapter Debug or errors. Applications must
avoid logging their own credentials or frames.
