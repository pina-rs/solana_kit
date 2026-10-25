# LeptosKit HTTP for Solana Kit

This is exclusively an off-chain application client adapter. For Solana on-chain
programs use Pina or the official Solana program libraries.

Enable `guest` only for interpreted LeptosKit Wasm, never browser Wasm. Construct
`GuestHttp::new("balance-service")`, then use the ordinary public Kit `Client`.
The application owns its local executor, exports `leptoskit_poll`, and routes its
single upstream `Events` receiver. The adapter never takes that receiver.

Enable `host` in the installed native application. `HttpDriver` borrows your actual
`WasmHost` for construction, installation, draining and completion delivery.
It owns service and task state; `next_completion().await` retains no interpreter
borrow. `HttpHost` owns a host and this same driver for headless callers.
Install named services using preconfigured
`NativeHttp` values and explicit grants before starting the guest. Service names
select fixed host destinations, TLS roots, authentication and deadlines. Guest
bytes contain no endpoint or headers. Independent host ceilings bound concurrent
work, request bytes and response bytes.

After every guest entry point, call `drain` and route its returned unrelated
requests/cancellations. Await `next_completion` in the application's event loop,
then pass the opaque owned `HttpCompletion` to `deliver` with a short host borrow.
The owning `HttpHost` performs both operations together.
This wakes the actual upstream registered guest wakers and application poll export.
When `Routed::guest_woken` is true, drain again before awaiting native work.
Keep other event sources in the same application-owned `select!` loop. Do not
replace the bound host's plugin bridge. Driver operations check both the actual
interpreter and Bridge identities, including after a Bridge transplant.
An idle driver wait remains Pending. Joining releases job capacity before returning
the completion; dropping that completion abandons delivery without retaining a job.
`shutdown().await` closes admission, aborts and joins native work. The driver leaves
your interpreter alive; `HttpHost` then unloads its interpreter. Drop aborts work as a last resort; only awaited
shutdown proves that accounting and native resources have terminated.

The current source selects the compatible public LeptosKit pin
`0af2913e57d680db74ee8d07bff1b54357f4d906` for the required guest Call identity API.
This selection still requires lock resolution and current native/guest replay.
Registry publication requires an
available compatible upstream release. Component publishing remains disabled.
Headless interpreted execution does not establish generated desktop, Android or
iOS application integration. Those installed application loops must wire this
owner and guest executor explicitly.

The [actual-host receipt](../../docs/verification/leptoskit-http.md) records the
accepted headless replay with rebuilt Rust 1.99.1 and 1.89.0 guests, six source
controls, four compiled ABI controls, and an unchanged independent public consumer.
The HTTP application guest requires exactly three renderer and eight plugin-v1
imports. The separate portable-core guest keeps its exact three-import policy.

The source-only socket extension uses `GuestWebSocket` and `SocketDriver` with the
same application-owned Events receiver and short host borrows. `GuestSubscriptionSocket`
connects that actual socket to the client `Session`; route events through
`Session::socket_mut().route` between polls. Incoming text and binary use the same
bounded native parser and guest queue. Existing OPEN/SEND/CLOSE method IDs and
OPENED/TEXT/ENDED events remain unchanged. Binary uses additive event kind 4.
Outbound commands remain text. A complete close Call observes the native actor
join; Drop cancels outstanding work and the host must still drain and join it.
The maintained interpreted fixture adds Helius pre-ack binary delivery and close.
This extension is uncompiled and unexecuted in the current checkpoint. The
historical HTTP receipt above does not verify this source pin or socket extension.

The unverified REST source extension adds `GuestRest<D>` and
`HttpDriver::install_rest`. The application binds provider destinations to named
host services. Each service owns a fixed `NativeRest<()>` root and caller policy.
Additive method1 carries a bounded version1 REST request; original method0 and
its wire remain unchanged. Guest bytes carry no endpoint or headers. REST shares
existing call ownership, Events routing, completion identities, retained capacity
through native join and awaited driver shutdown. The REST request cap includes its
complete binary frame, while the existing RPC cap still applies to its JSON body.
Actual Rust guest/provider journeys, host routing and joined shutdown replay are
required. Current source pins and stale locks require a coherent root-owned
upgrade before replay; the future framework target is public commit
`3c6383a17f786172d696e1e83d8026cba2085fa4`, tree
`be2aa83a1d9412dbf9f20896282a5ceb21327e1e`. These are continuation requirements,
not an execution or dependency compatibility claim.
