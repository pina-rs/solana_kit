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
`7437097c52aa68a3401bd378764e051ed4c9165a` for the required guest Call identity API.
The manifest and workspace lockfiles select this revision.
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
required for each claimed runtime. The selected framework revision is public commit
`7437097c52aa68a3401bd378764e051ed4c9165a`, tree
`f4048de8e271deaf882de45e476fba554f937890`. A source revision identifies the
dependency; the individual transport receipts establish execution coverage.

## Optional wallet controls and Android platform

Enable `wallet-ui` for an accessible wallet picker/status view and bounded action
channel. Native buttons provide the platform's keyboard semantics. The view uses
opaque choice identities, actual account metadata and a 44-pixel minimum control
height. Applications own focus restoration and must verify it on each platform.

Browser bundles can enable `wallet-browser-ui` and construct `BrowserWalletView`
from real `WalletDiscovery`. Poll its short-borrow `poll_update` on the existing
local executor and render each returned element. The owner observes registry and
controller changes while wallet prompts remain pending; cancel invalidates local
authority and disposal releases subscriptions. This browser feature requires a
Window. It is not a wasmi wallet backend.

For Android interpreter bundles, enable `wallet-platform` in the guest.
`GuestWalletPlatform` exposes
actual availability, validated local-v1 association Intent launch and bounded
host entropy. An already launched external Activity cannot be undone by local
cancellation. The native plugin and immutable bridge reply own temporary entropy
buffers; erasing the guest's copy does not promise bridge allocator erasure.

The Android platform plugin is unavailable on desktop and iOS. Select the local
wallet explicitly there; no MWA capability is advertised.

Enable `wallet-host` in the native host and `wallet-session` in the interpreted
guest. Runtime selection `solana-wallet` installs
`wallet_service::from_app_dirs` and Android's `MwaPlatform`; these own the distinct
bridge names `solana-wallet-session` and `solana-wallet-platform`. This hybrid
registration uses the public framework support in
[LeptosKit](https://github.com/ifiokjr/leptoskit/pull/181), including
[Android scheme visibility](https://github.com/ifiokjr/leptoskit/pull/186).
The SDK and generated application select the same Git revision. Headless tests
and generated source checks do not establish installed application support.

`GuestMobileWallet::connect_android` retains a real pending bridge lease while it
creates an association, waits for its correlated URI event, launches the Android
wallet and connects the existing `NativeMobileWallet`. The application supplies
its single Events receiver and forwards unrelated events intact. Dropping the
lease cancels native state even between operations or during Android handover.
No second WebSocket or MWA protocol implementation is added.

`GuestLocalWallet::create` selects real native entropy and creates an in-memory
key. `load`, `save` and `backup` require explicit application actions. The guest
never receives private bytes. `sign_with_approval` waits for the actual approval
future before submitting signing work. `local_signing_approval` binds real native
Approve/Reject button actions to a non-reused review identity. The generated
fixture executes only a matching approval, then independently verifies the exact
signature. Native buttons, bounded queues, busy/cancel and disconnect states are
ordinary source; keyboard, focus and installed accessibility remain unverified.

Persistence uses a canonical host-selected app sandbox root and a private `0700`
child directory. Guest input never selects a file path. Loads reject leaf
symlinks with O_NOFOLLOW, nonregular/oversized files and permissive modes. Saves
reuse the private `0600`, atomic no-overwrite writer. The host must keep its sandbox
root trusted; this is not a hostile same-user directory-race defense. Desktop
Unix, Android and iOS use this path. Other systems fail explicitly rather than
writing an insecure file. The separate private copy remains inside the app; it
is not an off-device recovery backup and can be lost on uninstall. No automatic
export or silent overwrite occurs.

`WalletService` owns one finite command mailbox, one native runtime thread and
one session/key authority. Cancellation advances its generation, destroys active
private state and rejects stale delivery. `AppService::close` stops admission
and returns an owned future that joins the worker before Ready, including errors.
Use the framework's ClosingApp scheduler for replacement/unload. Drop signals
cancellation; it cannot itself await join. Original native causes remain in the
bounded `WalletFailures` journal available to custom native owners. Bridge errors
remain the framework's fixed codes; no secret or peer text enters UI diagnostics.

For a composite app, one AppService owns raw Bridge draining and a retained
`ServiceWork` queue. Each turn appends fresh work after retained work, then passes
that fixed batch through `HttpDriver::route`, `SocketDriver::route` and
`WalletService::route`. Their leftovers have visited every SDK owner and can
return to the framework in order. Drain requests created by guest wakes into
the next turn. Bound the loop to 32 turns, retain its unprocessed next batch and
wake the app before yielding. Poll native completion waits after admissions so
new jobs register their wakers before the app waits.

Each route method consumes its supplied batch. Choose the composite loop or
wallet-only `AppService::poll` as the draining owner for the service lifetime.
The standalone poll owns its own bounded drain loop and retained overflow.
The app's one ServiceClose must await every driver's shutdown before ClosingApp
finishes, including when one shutdown reports an error.

The [generated-app adaptation](../../verification/leptoskit-app-journey/wallet/README.md)
contains actual action callers and prospective generator/build commands. All new
code, source tests and native registrations require compilation and execution.
Generated desktop/Android/iOS D1–D9/A1–A9/I1–I9 journeys remain required. All APIs
are off-chain client code; on-chain programs use Pina or official Solana libraries.
