# Installed external Android wallet execution

On 2026-11-04, the generated Android wallet journey completed against an
independently built, pinned Solana Mobile Mobile Wallet Adapter reference wallet
installed on the same fresh arm64 API 35 emulator. The three-process
controller—one ordinary generated application process, one genuine process
relaunch, and the external wallet application—ran three consecutive full
invocations with exit 0. This is an installed reference-wallet acceptance on an
owned emulator, not a production-wallet, physical-device, or CI claim.

The SDK source starts at upstream `028b8b86` (the merged
[remote reflector transport](https://github.com/pina-rs/solana_kit/pull/165)),
which includes the native transport refactor, `MwaConfig.join_timeout` and its
preserved defaults, deadline validation, and the refreshed generated-runtime
lockfile. The generated application used the public generator/runtime framework
pin `7437097c52aa68a3401bd378764e051ed4c9165a` and the ordinary
`prepare-wallet-rpc` composition. No generated native host was replaced.

## External wallet prerequisite

The wallet is the official reference/test wallet from
[`solana-mobile/mobile-wallet-adapter`](https://github.com/solana-mobile/mobile-wallet-adapter)
at revision `100e023048b7ffd3a67af2bf334f4bda3b9d8d7c`. Its `v1` debug flavor was
built unchanged except for a generated, disposable test seed supplied through
the wallet's documented `local.properties` key:

- Package: `com.solana.mobilewalletadapter.fakewallet`
- Version name: `1.0`; version code: `1`; `minSdk` 23; `targetSdk` 37
- APK SHA-256:
  `f558946164124e887096db1d6265926242ea78e3db5420d2d89eb33fafb95bb3`
- The APK contains only the disposable reference-test key. The
  `local.properties` file was deleted immediately after the build. No personal
  account, seed phrase, production wallet, or funds were used, and the APK
  itself was removed after the run.

The reference wallet is a test application. It does not establish compatibility
with any production wallet, does not implement production transaction review
or key custody, and does not publish or retain a recovery phrase. The fixed key
exists only to keep one public account across the authorization and relaunch
sessions; it grants no authority beyond the disposable local validator.

## Generated application

The project was created with the public CLI:

```sh
leptoskit new "$PROJECT" --name solana_kit_mwa_wallet_20261104 \
  --platform native --native-target android
cargo run --locked --manifest-path "$SDK/Cargo.toml" --package solana_kit_xtask \
  -- prepare-wallet-rpc "$PROJECT"
cd "$PROJECT"
leptoskit native build android
```

- Identity: `com.leptoskit.solanakitmwawallet20261104`
- Activity: `com.leptoskit.solanakitmwawallet20261104.MainActivity`
- Debug APK: `minSdk` 26, `compileSdk`/`targetSdk` 35, versionCode 1,
  versionName 0.1.0, `arm64-v8a` only
- Rust 1.99.1 (`58cf8f9c3`) built both the guest and controller. The
  `aarch64-linux-android` standard library was installed for that toolchain.
  `CARGO`, `RUSTC` and `RUSTDOC` named the concrete toolchain binaries;
  `CARGO_INCREMENTAL=0` and `CARGO_BUILD_JOBS=2` were set.
- Android SDK platform 35, build-tools 36.0.0, NDK 28.2.13676358, Gradle 9.8.1,
  Android Gradle Plugin 9.0.1, and Java 17.0.19 produced the APK.
- Appium 3.8.0 with UiAutomator2 driver 8.7.0 came from the pinned framework's
  `setup/native-ui` dependencies and ran on `127.0.0.1:14734` with
  `ANDROID_HOME`/`ANDROID_SDK_ROOT` explicitly exported.
- The controller started the explicit Agave 4.2.0
  `solana-test-validator` with fresh numeric loopback ports and removed its
  fresh ledger on close. Only the installed app received the explicit
  `10.0.2.2` emulator-gateway grants; the independent observer kept its own
  numeric `127.0.0.1` connection.

| Artifact                         | SHA-256                                                            |
| -------------------------------- | ------------------------------------------------------------------ |
| Debug APK                        | `2c081eb99455abb779ec8c66f4a0de88a8981d7cf63d9f6171ffba0b4cad478d` |
| Generated guest `app.wasm`       | `ba22efc8ef7f78902b227f48748fe960d99d813519b45415d4d6f2a07b70118b` |
| Stripped JNI `lib` (`arm64-v8a`) | `185f3edf9416d87f5744a990a748a5febe340748fe8fa3259ceafed6ee0ba600` |
| Installed controller executable  | `8d2c6fb0ab13e0ca1db8c0099458864eb9dcfbde47f398fbec024e20c8ce785b` |

## Composed runtime and platform handover

`prepare-wallet-rpc` selects the private `solana-journey` runtime, which also
installs the SDK wallet service. The generator accepts Android sources only
from the selected runtime and its native-support dependencies, so the runtime
now declares the existing `dev.solanakit.wallet.MwaPlatform` dispatcher and the
`solana-wallet` view-intent query, and an empty support package republishes the
exact SDK Kotlin host sources. A unit test fails if those sources differ from
`crates/solana_kit_leptoskit/android`. The original `prepare-wallet` path keeps
its direct SDK plugin packaging and was not changed.

The generated manifest therefore contains the required package-visibility
query, and the generated registrant constructs the real platform dispatcher.
Without that declaration Android reports no wallet, which was the first
installed-path defect exposed by this work.

## Executed behavior

The `android-mwa` controller drives the generated app's ordinary external wallet
path and a second, independently installed wallet UI:

- **Fresh authorization.** The app launches the wallet association, the wallet
  displays the exact dapp identity and cluster, and the person authorizes the
  fixed reference account. The app displays
  `6vB4RERPfT2MbbpWwbmfLMCUjQwUEV84kNjgRspyPKs1`.
- **Visible public account and funding.** The controller reads that public
  address from the app, funds it once with a 2,000,000,000-lamport owned-validator
  airdrop, and the app's **Read external wallet balance** action displays
  `Balance: 2000000000 lamports` through the SDK's typed RPC client.
- **Verified message signature.** The app requests a message review, the wallet
  displays `SIGN MESSAGE(S)`, the person authorizes, and the app verifies the
  returned Ed25519 signature against the displayed message and the authorized
  account. The controller independently verifies the same public receipt with
  `ed25519-dalek`.
- **Rejected transfer.** The app freezes a 1,000,000-lamport System transfer to
  `7d3y2WdzxE7CfsWjkGy3WndkvZcj1EHMkzKJiFPiDecH`; the wallet displays
  `SIGN TRANSACTION(S)` and the person declines. The app shows
  `Transfer rejected; nothing submitted`; the independent observer requires an
  empty recipient history and an unchanged recipient and payer.
- **Approved transfer.** A second review of the exact same transfer is approved
  in the wallet. The SDK—not the wallet—submits the signed bytes once to the
  owned validator. The independent observer requires one signature, exactly one
  System instruction, the exact payer/recipient/program account list, the
  amount, a positive network fee, the exact payer debit, and the recipient
  credit. Run 3 produced signature
  `4DC8xqJFDVhfyCGZibbN7MuwxF6FkLQfMpKaG5vaqqZcwx2jAvVQWJG6paVB8MGAXz6PEAdMd9R28sDj23uPnMi5`;
  the payer moved from 2,000,000,000 to 1,998,995,000 lamports (1,000,000
  transferred plus the 5,000-lamport fee) and the recipient reached 1,000,000.
- **Subscription ownership.** The app opens the granted validator WebSocket and
  uses the SDK's typed account and slot owners on one actual socket. The
  controller independently credits the public probe account
  `7Z8ftDAzMvoyXnGEJye8DurzgQQXLAbYCaeeesM7UKHa`, holds a genuine update,
  acknowledges `accountUnsubscribe`, releases the held callback, and requires
  the displayed balance to stay frozen. A later confirmed SDK HTTP balance and
  a strictly later socket slot must cross that context before both streams and
  the socket close. Run 3 recorded 1,000,000/2,000,000/3,000,000 lamports, late
  transaction slot 119, SDK context slot 119, and same-socket slot 121. The
  wallet's ledger balance was unchanged by the subscription.
- **Pending cancellation.** The app starts a wallet signature request and shows
  its real `Cancel request` control. Cancellation disposes the pending call and
  the wallet lease; the app returns to `Choose a wallet`. The controller
  requires the approved post-transfer recipient and payer balances to remain
  unchanged.
- **Fresh authorization after cancellation.** A new association authorizes the
  same public account again, proving the cancelled session left no stale
  authority.
- **Explicit disconnect.** The app sends the MWA close, awaits the native owner,
  and returns to selection. The wallet activity finishes.
- **Genuine process relaunch.** The controller terminates only the tested app
  and requires the driver's not-running state. A new Appium session starts a new
  process with the same app data. It does not reconnect automatically; it
  requires a new wallet authorization, restores the same address, reads the
  retained 1,998,995,000-lamport balance, signs and verifies a new review
  message, and disconnects again.

The two sessions retained 18 and 6 named checkpoints respectively, each with a
real PNG and accessibility capture. The wallet UI was driven only through the
same Appium session and the documented Android task switch; no `adb`-driven
tap, text entry, or intent dispatch was used.

## The external-wallet sequencing defect

The installed reference wallet exposed a real native framing defect. A wallet's
encrypted `HELLO_RSP` session properties frame is sequence **1**, and its first
request reply is sequence **2**. The native client created its receive counter
at 0 when properties were present, so a genuine wallet's first frame failed
ordering and the pending authorization closed with `MwaError::Protocol`. The
existing loopback peers used 0, which hid the mismatch.

`NativeAssociation::establish` now starts the receive counter at 1 for both the
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
prior local-wallet persistence. The native MWA suite passed 27 library tests
plus its reflector, remote, browser and integration groups:

```sh
cargo test --locked -p solana_kit_native --features mwa --offline
cargo test --locked -p solana_kit_wallet --offline
```

The runtime crate passed 11 tests and the support package's source-equality
test passed; strict Clippy passed for the native MWA features, the wallet, the
runtime and the installed controller; `cargo doc --no-deps -p
solana_kit_native --features mwa` passed with `RUSTDOCFLAGS=-D warnings`. The
repository gates `cargo fmt --all --check`, `dprint check`, `monochange check`
and `cargo xtask style` passed. Style reported 2,275 files, every category at or
above 90, and overall 97.9.

The retained run directory is
`/tmp/solana-kit-rw03-20261104/acceptance/run3`. It contains both sessions'
PNG/XML captures, `index.html`, `subscription-mwa-initial.json`,
`mwa-initial-{receipt,diagnostics,startup}.txt`,
`mwa-relaunch-{receipt,diagnostics,startup}.txt` and `wallet.json`. The two
earlier full invocations are retained under `acceptance/run` and
`acceptance/run2`; run 2 has the same retained receipt shape. The controller log
is `acceptance/controller-6.log`, the Appium log is
`/tmp/solana-kit-rw03-20261104/appium.log`, and the evidence hashes are
`evidence/artifacts-final.sha256`.

## Remaining limits

This is installed acceptance on one emulator with a test-only reference wallet.
It does not establish production-wallet compatibility, physical-device
behavior, the Solana Mobile Seeker, iOS MWA (unavailable on that platform),
production RPC or funds, Code Push, or a whole-SDK claim. The wallet's fixed
disposable key and the harness's 15-second switch window are explicit test
affordances, not production behavior. No CI workflow was dispatched and no
artifact was published.
