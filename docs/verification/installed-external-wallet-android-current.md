# Installed external Android wallet execution

On 2026-11-04, the generated Android wallet journey completed against an
independently built, pinned Solana Mobile Mobile Wallet Adapter reference wallet
installed on the same fresh arm64 API 35 emulator. The three-process
controller—one ordinary generated application process, one genuine process
relaunch, and the external wallet application—first ran three consecutive full
invocations whose cancellation still raced ahead of the wallet dispatch, and
then one stronger final invocation that cancelled an actual pending wallet
prompt. The final invocation exited 0 and is the current acceptance. This is an
installed reference-wallet acceptance on an owned emulator, not a
production-wallet, physical-device, or CI claim.

The SDK source for the final run is branch `test/android-external-wallet-final`
at `da4aaef2`, which carries the four reviewed implementation commits, the
parent's acceptance fixes, and the fixture cancellation repair described below.
The generated application used the public generator/runtime framework pin
`7437097c52aa68a3401bd378764e051ed4c9165a` and the ordinary
`prepare-wallet-rpc` composition. No generated native host was replaced.

## External wallet prerequisite

The wallet is the official reference/test wallet from
[`solana-mobile/mobile-wallet-adapter`](https://github.com/solana-mobile/mobile-wallet-adapter)
at revision `100e023048b7ffd3a67af2bf334f4bda3b9d8d7c`, rebuilt from the
retained official source for the final run. Its `v1` debug flavor was built
unchanged except for a newly generated, disposable test seed supplied through
the wallet's documented `local.properties` key:

- Package: `com.solana.mobilewalletadapter.fakewallet`
- Version name: `1.0`; version code: `1`; `minSdk` 23; `targetSdk` 37
- APK SHA-256:
  `949e79063b0858467e331d99389336b068a8e86f5b0ddc582e9e0f770b3b0e40`
- The historical three invocations used a different disposable seed and APK
  (`f558946164124e887096db1d6265926242ea78e3db5420d2d89eb33fafb95bb3`), so their
  account and signatures are not comparable with the final run.
- The APK contains only the disposable reference-test key. The
  `local.properties` file was deleted immediately after the build. No personal
  account, seed phrase, production wallet, or funds were used, and the APK
  itself is removed after the run.

The reference wallet is a test application. It does not establish compatibility
with any production wallet, does not implement production transaction review
or key custody, and does not publish or retain a recovery phrase. The fixed key
exists only to keep one public account across the authorization and relaunch
sessions; it grants no authority beyond the disposable local validator.

## Generated application

The final project was created with the public CLI:

```sh
leptoskit new "$PROJECT" --name solana_kit_mwa_pendingcancel_20261104 \
  --platform native --native-target android
cargo run --locked --manifest-path "$SDK/Cargo.toml" --package solana_kit_xtask \
  -- prepare-wallet-rpc "$PROJECT"
cd "$PROJECT"
leptoskit native build android
```

- Identity: `com.leptoskit.solanakitmwapendingcancel20261104`
- Activity: `com.leptoskit.solanakitmwapendingcancel20261104.MainActivity`
- Debug APK: `minSdk` 26, `compileSdk`/`targetSdk` 35, versionCode 1,
  versionName 0.1.0, `arm64-v8a` only
- Rust 1.99.1 (`58cf8f9c3`) built both the guest and controller. The
  `aarch64-linux-android` standard library was installed for that toolchain.
  `CARGO`, `RUSTC` and `RUSTDOC` named the concrete toolchain binaries;
  `CARGO_INCREMENTAL=0` and `CARGO_BUILD_JOBS=2` were set.
- Android SDK platform 35, build-tools 36.0.0, NDK 28.2.13676358, Gradle 9.8.1
  (wallet build 9.7.0), Android Gradle Plugin 9.0.1, and the Android Studio JBR
  Java 21 produced the APKs. The Gradle, Cargo and npm caches were used
  offline.
- Appium 3.8.0 with UiAutomator2 driver 8.7.0 came from the pinned framework's
  `setup/native-ui` dependencies and ran on `127.0.0.1:14734` with
  `ANDROID_HOME`/`ANDROID_SDK_ROOT` and `APPIUM_HOME` pointing at that pinned
  directory.
- The controller started the explicit Agave `solana-test-validator` at the
  owned install path (active release 4.3.2 at run time; the historical
  invocations used 4.2.0 at the same path) with fresh numeric loopback ports
  and removed its fresh ledger on close. Only the installed app received the
  explicit `10.0.2.2` emulator-gateway grants; the independent observer kept
  its own numeric `127.0.0.1` connection.

Final run artifacts:

| Artifact                         | SHA-256                                                            |
| -------------------------------- | ------------------------------------------------------------------ |
| Debug APK (installed build)      | `25483247c358cdcbb7fe8fc7591b9afae35c83a5b51750e8976ae2d51adb18d5` |
| Generated guest `app.wasm`       | `44b53c2fd6cd3fb7ca8665655b4f896c69e713aa71f4e0469dc6623c64b3f44c` |
| Stripped JNI `lib` (`arm64-v8a`) | `7118607b523c33f61b6523fdad6b9d23affef6ca5ecc012df8f352bd76f31f42` |
| Installed controller executable  | `29eaed158665af40232136a920889819afe569cb9518766c067d7f7a9df3de95` |
| Reference wallet APK             | `949e79063b0858467e331d99389336b068a8e86f5b0ddc582e9e0f770b3b0e40` |

The historical three invocations used the earlier identity
`com.leptoskit.solanakitmwawallet20261104` with APK
`2c081eb99455abb779ec8c66f4a0de88a8981d7cf63d9f6171ffba0b4cad478d`, guest wasm
`ba22efc8ef7f78902b227f48748fe960d99d813519b45415d4d6f2a07b70118b`, JNI lib
`185f3edf9416d87f5744a990a748a5febe340748fe8fa3259ceafed6ee0ba600` and
controller `8d2c6fb0ab13e0ca1db8c0099458864eb9dcfbde47f398fbec024e20c8ce785b`.

## Composed runtime and platform handover

`prepare-wallet-rpc` selects the private `solana-journey` runtime, which also
installs the SDK wallet service. The generator accepts Android sources only
from the selected runtime and its native-support dependencies, so the runtime
declares the existing `dev.solanakit.wallet.MwaPlatform` dispatcher and the
`solana-wallet` view-intent query, and an empty support package republishes the
exact SDK Kotlin host sources. A unit test fails if those sources differ from
`crates/solana_kit_leptoskit/android`. The original `prepare-wallet` path keeps
its direct SDK plugin packaging and was not changed.

The generated manifest therefore contains the required package-visibility
query, and the generated registrant constructs the real platform dispatcher.
Without that declaration Android reports no wallet, which was the first
installed-path defect exposed by this work.

## Executed behavior (final invocation)

The `android-mwa` controller drives the generated app's ordinary external wallet
path and a second, independently installed wallet UI:

- **Fresh authorization.** The app launches the wallet association, the wallet
  displays the exact dapp identity and cluster, and the person authorizes the
  fixed reference account. The app displays
  `8DddNPKNSpgLJmsZo6AqDvFegp2XaeN3v4odEx4R8vmA`.
- **Visible public account and funding.** The controller reads that public
  address from the app, funds it once with a 2,000,000,000-lamport owned-validator
  airdrop, and the app's **Read external wallet balance** action displays
  `Balance: 2000000000 lamports` through the SDK's typed RPC client.
- **Verified message signature.** The app requests a message review, the wallet
  displays `SIGN MESSAGE(S)`, the person authorizes, and the app verifies the
  returned Ed25519 signature against the displayed message and the authorized
  account. The controller independently verifies the same public receipt with
  `ed25519-dalek`. Receipt signature for generation 1:
  `2ke9yEhYmGqP2L6DoHof2ymxg4B6tGFmZ2hfAGtbCfzkgTKMP4x6HxKkaF9muFaWBFECCybhpaXmYaQsTenaDUgV`.
- **Rejected transfer.** The app freezes a 1,000,000-lamport System transfer to
  `7d3y2WdzxE7CfsWjkGy3WndkvZcj1EHMkzKJiFPiDecH`; the wallet displays
  `SIGN TRANSACTION(S)` and the person declines. The app shows
  `Transfer rejected; nothing submitted`; the independent observer requires an
  empty recipient history and unchanged recipient **and payer** balances.
- **Approved transfer.** A second review of the exact same transfer is approved
  in the wallet. The SDK—not the wallet—submits the signed bytes once to the
  owned validator. The independent observer requires one signature, exactly one
  System instruction, the exact payer/recipient/program account list, the
  amount, a positive network fee, the exact payer debit, and the recipient
  credit. The final run produced signature
  `hWYviY2uvVuoSpfnTujUg2WJ58Uof1wmB3U4PHG3FPiz92zmXDR5vANjYzbGfDoJMNiYsBb18vpoJ6CWzV891EY`;
  the payer moved from 2,000,000,000 to 1,998,995,000 lamports (1,000,000
  transferred plus the 5,000-lamport fee) and the recipient reached 1,000,000.
- **Subscription ownership.** The app opens the granted validator WebSocket and
  uses the SDK's typed account and slot owners on one actual socket. The
  controller independently credits the public probe account
  `7Z8ftDAzMvoyXnGEJye8DurzgQQXLAbYCaeeesM7UKHa`, holds a genuine update,
  acknowledges `accountUnsubscribe`, releases the held callback, and requires
  the displayed balance to stay frozen. A later confirmed SDK HTTP balance and
  a strictly later socket slot must cross that context before both streams and
  the socket close. The final run recorded 1,000,000/2,000,000/3,000,000
  lamports, late transaction slot 126, SDK context slot 126, and same-socket
  slot 128. The wallet's ledger balance was unchanged by the subscription.
- **Cancellation of the actual pending prompt.** The controller taps
  **Request wallet signature**, waits for the app's `Cancel request` control,
  brings the wallet task forward, and requires the wallet's `SIGN MESSAGE(S)`
  prompt with the new request visible before anything is cancelled. The
  checkpoint capture shows that prompt (one payload, `fakewallet account 0`,
  AUTHORIZE/DECLINE) while the request is pending. The controller then returns
  to the app, taps `Cancel request`, requires the app back at `Choose a
  wallet`, and requires the post-transfer recipient (1,000,000) and payer
  (1,998,995,000) balances to remain unchanged. The wallet's pending activity
  finishes when the cancelled session closes. This is the proof the three
  historical invocations did not provide: they cancelled during the fixture's
  15-second switch window, before the wallet call was polled.
- **Fresh authorization after cancellation.** A new association authorizes the
  same public account again, proving the cancelled session left no stale
  authority.
- **Explicit disconnect.** The app sends the MWA close, awaits the native owner,
  and returns to selection. The wallet activity finishes.
- **Genuine process relaunch.** The controller terminates only the tested app
  and requires the driver's not-running state. A new Appium session starts a new
  process with the same app data. It does not reconnect automatically; it
  requires a new wallet authorization, restores the same address, reads the
  retained 1,998,995,000-lamport balance, signs and verifies a new review round
  (generation 1 again in the new process, so the deterministic Ed25519 signature
  over the same message bytes matches the first session's; the receipt cannot
  distinguish the signature bytes, and the checkpoint proves a fresh signing
  round through a new wallet session), and disconnects again.

The two sessions retained 18 and 6 named checkpoints respectively, each with a
real PNG and accessibility capture. The wallet UI was driven only through the
same Appium session and the documented Android task switch; no `adb`-driven
tap, text entry, or intent dispatch was used.

## The fixture cancellation defect

The stronger cancellation exposed a real defect in the verification fixture
itself. Cancelling a pending request drops its in-flight wallet call, and the
wallet service then ends the whole lease: the dropped call invalidates the
session exactly like the native `MwaError` path, and the wallet activity
finishes. The fixture nevertheless called `wallet.close()` again on that
already-unloaded lease, which the service correctly refuses as unavailable, so
the app displayed the redacted `wallet session request failed` status instead
of `Choose a wallet` and the controller lost its expected terminal state. The
historical invocations could not see this because their cancellation always
landed during the 15-second pause, when no request was in flight and the lease
still existed.

The fixture's interaction loop now distinguishes the two endings: a pending
request that was cancelled ends on the lease that the dropped call already
unloaded, while an explicit cancel or disconnect from the idle panel still
closes the native owner and awaits its completion. No SDK or wallet semantics
changed; the service refusal of a second close remains a typed error.

## The external-wallet sequencing defect

The installed reference wallet exposed a real native framing defect. A wallet's
encrypted `HELLO_RSP` session properties frame is sequence **1**, and its first
request reply is sequence **2**. The native client created its receive counter
at 0 when properties were present, so a genuine wallet's first frame failed
ordering and the pending authorization closed with `MwaError::Protocol`. The
existing loopback peers used 0, which hid the mismatch.

The shared native `establish_wallet` path now starts the receive counter at 1 for both the
local and reflector transports. The local and reflector test peers encode
properties at 1 and replies at 2, and the wallet crate's encrypted memory
session test uses the same real counters. The fix is covered by a changeset and
by the native MWA suite; the shared conformance suite and its independent
library vectors were not modified.

## Harness affordances

The desktop-driven acceptance must hand the foreground to the wallet application
and back. Android saves a stopped fragment manager, so a request that arrives
while the wallet is stopped is delivered but its navigation is dropped. The
fixture therefore shows its pending panel and waits 15 seconds before sending a
wallet request; the controller uses the documented app-switch task view to
resume the wallet before that request arrives, then reactivates the generated
app after approval. This delay and task switch belong only to the installed
acceptance harness. A production dapp normally keeps its wallet foreground while
the request is in flight, and the delay is not part of the SDK or the wallet.

## Verification

The installed controller passed:

```sh
cargo test --locked --manifest-path verification/leptoskit-app-journey/installed/Cargo.toml \
  --all-targets
cargo clippy --locked --manifest-path verification/leptoskit-app-journey/installed/Cargo.toml \
  --all-targets -- -D warnings
```

That suite contains 16 tests, including Android package validation, wallet
metadata parsing, termination acknowledgements, startup receipt bounds, and
prior local-wallet persistence. The runtime crate passed 11 tests, the Android
support package passed its source-equality test, and the native MWA library
suite passed 26 focused tests:

```sh
cargo test --locked --manifest-path verification/leptoskit-app-journey/runtime/Cargo.toml --all-targets
cargo test --locked --manifest-path verification/leptoskit-app-journey/runtime/Cargo.toml \
  -p solana_kit_app_journey_android_support
cargo test --locked -p solana_kit_native --features mwa --lib mwa --offline
```

`cargo fmt --all --check` plus the installed and runtime manifest `fmt` checks,
`dprint check`, `monochange check` and `cargo xtask style` passed. Style
reported 2,282 files, every category at or above 90, and overall 97.9. The
feature-gated `generated-wallet` host suite
(`cargo test ... --features generated-wallet-tests --test generated_wallet`)
currently fails its `failed_private_copy` case in this environment on both the
pristine reviewed sources and the final sources, so that failure predates and
is unrelated to the fixture repair; that suite is not part of the scoped gates
above.

## Retained evidence

The retained final run directory is
`/private/tmp/solana-kit-final-20261104/acceptance-final/run`. It contains both
sessions' PNG/XML captures with `index.html`,
`subscription-mwa-initial.json`, `mwa-initial-{receipt,diagnostics,startup}.txt`,
`mwa-relaunch-{receipt,diagnostics,startup}.txt` and `wallet.json`. The
checkpoint `External wallet signing prompt pending before cancellation` retains
the wallet's live `SIGN MESSAGE(S)` capture, and the following checkpoint
retains the app back at `Choose a wallet`; the initial diagnostics end with
`action cancel`, `guest result ok` and the later explicit disconnect's
`lease ended`. The two earlier weaker invocations remain under
`/tmp/solana-kit-rw03-20261104/acceptance/{run,run2,run3}` with their own
receipts; their cancellation checkpoint does not prove pending-prompt
cancellation. The final controller log is
`/private/tmp/solana-kit-final-20261104/controller-final.log`, the Appium log
is `/private/tmp/solana-kit-final-20261104/appium.log`, the build and style
logs are siblings, and the artifact hashes are
`/private/tmp/solana-kit-final-20261104/evidence/artifacts-final.sha256` with
the two prompt-state captures in the same directory.

## Remaining limits

This is installed acceptance on one emulator with a test-only reference wallet.
It does not establish production-wallet compatibility, physical-device
behavior, the Solana Mobile Seeker, iOS MWA (unavailable on that platform),
production RPC or funds, Code Push, or a whole-SDK claim. The wallet's fixed
disposable key and the harness's 15-second switch window are explicit test
affordances, not production behavior. The relaunch's deterministic signature is
byte-identical to the first session's because the fixture signs the same
generation-1 message with the same fixed key; the checkpoint proves the fresh
session, not fresh bytes. No CI workflow was dispatched and no artifact was
published.
