# LeptosKit HTTP for Solana Kit

This is exclusively an off-chain application client adapter. For Solana on-chain
programs use Pina or the official Solana program libraries.

Enable `guest` only for interpreted LeptosKit Wasm, never browser Wasm. Construct
`GuestHttp::new("balance-service")`, then use the ordinary public Kit `Client`.
The application owns its local executor, exports `leptoskit_poll`, and routes its
single upstream `Events` receiver. The adapter never takes that receiver.

Enable `host` in the installed native application. `HttpHost` owns an actual pinned
`WasmHost` and all HTTP tasks. Install named services using preconfigured
`NativeHttp` values and explicit grants before starting the guest. Service names
select fixed host destinations, TLS roots, authentication and deadlines. Guest
bytes contain no endpoint or headers. Independent host ceilings bound concurrent
work, request bytes and response bytes.

After every guest entry point, call `drain` and route its returned unrelated
requests/cancellations. Await `next_completion` in the application's event loop.
This wakes the actual upstream registered guest wakers and application poll export.
When `Routed::guest_woken` is true, drain again before awaiting native work.
Keep other event sources in the same application-owned `select!` loop. Do not
replace the owned host's plugin bridge. `shutdown().await` aborts and joins native
work before unloading the guest. Drop aborts work as a last resort; only awaited
shutdown proves that accounting and native resources have terminated.

The bridge uses pinned unpublished LeptosKit source at
`fecf8628d326ff7b03f295523e6bb584c3fdf88d`. Registry publication requires an
available compatible upstream release. Component publishing remains disabled.
Headless interpreted execution does not establish generated desktop, Android or
iOS application integration. Those installed application loops must wire this
owner and guest executor explicitly.
