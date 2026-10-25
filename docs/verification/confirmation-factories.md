# Application-owned confirmation observations

The optional client `confirmation-factories` feature supplies read-only RPC
reconciliation, a whole-owner `confirm_signature`, a transient shared-owner
`confirm_signature_shared`, guarded `submit_and_confirm`, and a
`ConfirmationWatch` for application-owned shared sessions. These are exclusively
off-chain client contracts. On-chain programs use Pina or official Solana libraries.

Lifetime invalidation triggers a second signature-status lookup. A block-height
observation supplies its `absoluteSlot`; an authenticated nonce account supplies
its response context slot. The final status context must reach that slot before
an absent status can establish expiry. Older final contexts leave confirmation
pending. The caller retries safe reads under its injected deadline and cancellation.
No unsupported `minContextSlot` field is added to `getSignatureStatuses`.

A watch contains a reducer and its typed signature registration. It borrows the
shared Session only during each poll. Applications flush registration before
HTTP reconciliation, schedule reads at `next_read`, poll their own deadline and
cancellation, then call `poll_cleanup` with a bounded cleanup deadline. Cleanup
removes this registration. The application owns shutdown of the shared socket.
Dropping a watch cannot promise remote acknowledgement.

The callable shared operations accept `SessionAccess`: a synchronous callback
over the application's `RefCell<Session<_>>`, `Mutex<Session<_>>`, or its own
implementation. No guard survives an await. Drive concurrent operations together
from the application's one notification-poll owner. The native future retains
`Send` when its concrete transports, access and clock permit it; local browser
futures retain their `Rc` values. The shared operation unsubscribes only its own
watch and never closes the application socket. A failed bounded cleanup reports
its error separately; the application must close or replace that uncertain owner.

`submit_and_confirm` uses the original submission's exact encoding, signature,
packet-size and explicit lifetime validation. It flushes subscription registration
before sending the frozen bytes exactly once. The original transport error and
source remain available if acceptance is uncertain. Cancellation or deadline
during submission can also be uncertain. No submission is replayed as a safe read.
The original reducer and the post-invalidation freshness barrier remain shared.

Every injected sleep returns `Result<(), ClockError>`. Successful clock expiry
still means timeout. Construction and poll failures retain their typed cause as
`FactoryFailure::Session(SessionError::Clock(_))`. Cleanup reports its own clock
failure separately and leaves application socket shutdown explicit. Shared
acknowledgement timers skip expired wake ticks and latch completed sleeps.
No clock failure resubmits signed bytes. The composed source has not executed.

The separate `verification/confirmation-factories` workspace runs eleven identical
deterministic race and ownership bodies. The original seven cover height and nonce confirmation
races, stale final context, genuine fresh absent-status expiry, typed execution
failure, two concurrent watches retaining another subscription, and preliminary
receipt notifications. Four additional bodies cover pending local ownership,
submission success and uncertainty, pre-I/O guards and bounded cleanup failures.
The separate optional-feature normal/build policy leaves the
default no-feature client guard unchanged.

The native `factory_network` test composes actual loopback HTTP with two callable
confirmation operations and all nine typed subscription families on one WebSocket.
Unrelated typed values survive both confirmations, then explicit unsubscribe and
socket close complete. The private browser network runner exercises Window Fetch,
the maintained WebSocket poll adapter and a genuinely pending Promise while an
`Rc` stays alive across awaits. Each fixture owner is joined explicitly.

Complete source parity, resource/simulation factories, slot-monotonic stores,
the full transport failure matrix and installed application confirmation remain
separate acceptance requirements. Runtime receipts must name the actual compiler
and host; these tests make no validator, P1 networking or installed-device claim.
