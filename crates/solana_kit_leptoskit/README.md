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

The bridge uses pinned unpublished LeptosKit source at
`706334eaa3e011d1ef986159911b0d80c80d3e9f`. Registry publication requires an
available compatible upstream release. Component publishing remains disabled.
Headless interpreted execution does not establish generated desktop, Android or
iOS application integration. Those installed application loops must wire this
owner and guest executor explicitly.

The [actual-host receipt](../../docs/verification/leptoskit-http.md) records the
accepted headless replay with rebuilt Rust 1.99.1 and 1.89.0 guests, six source
controls, four compiled ABI controls, and an unchanged independent public consumer.
The HTTP application guest requires exactly three renderer and eight plugin-v1
imports. The separate portable-core guest keeps its exact three-import policy.
