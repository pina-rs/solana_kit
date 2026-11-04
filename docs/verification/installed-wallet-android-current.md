# Current installed Android wallet execution

On 2026-11-04, the maintained installed wallet controller completed the full
three-process wallet journey against an ordinary generated Android application.
Two independent invocations each completed all three native sessions on a newly
created arm64 API 35 emulator; the second recorded exit 0 and the first left an
empty error log. This is local emulator evidence, not a CI result or a
physical-device claim.

The SDK source was `8b288f7c90c3bc1fb5d5f0871895224d0465613e` (the merged
[portable qualification checkpoint](https://github.com/pina-rs/solana_kit/pull/159)).
The generated application used public generator/runtime framework
`7437097c52aa68a3401bd378764e051ed4c9165a`, and `prepare-wallet-rpc` selected the
composed runtime through its ordinary Cargo metadata. No generated native host
was replaced or edited, and no production SDK source changed.

## Build and environment

- Emulator `solana-kit-wallet-acceptance`: Android 15 (API 35), arm64-v8a,
  1080×2400, explicitly addressed as `emulator-5554`.
- The project was generated with `leptoskit new "$PROJECT" --name
  solana_kit_wallet_android_20261104 --platform native --native-target android`.
  The resulting ordinary identity was
  `com.leptoskit.solanakitwalletandroid20261104` with activity
  `com.leptoskit.solanakitwalletandroid20261104.MainActivity`. The installed
  artifact was a debug APK with `minSdk` 26, `compileSdk`/`targetSdk` 35,
  versionCode 1 and versionName 0.1.0, containing only `arm64-v8a`.
- Rust 1.99.1 (`58cf8f9c3`) built the application and controller. The
  `aarch64-linux-android` standard library was installed for that toolchain.
  `CARGO_INCREMENTAL=0` and `CARGO_BUILD_JOBS=4` were set, and `CARGO`, `RUSTC`
  and `RUSTDOC` named the concrete toolchain binaries.
- Android SDK: platform 35, build-tools 36.0.0, NDK 28.2.13676358 explicitly
  selected with `ANDROID_NDK_HOME`. The installed 29.0.13599879-beta2 preview
  was not used. Gradle 9.8.1, Android Gradle Plugin 9.0.1 and Java 17.0.19
  produced the APK.
- Appium 3.8.0 with UiAutomator2 driver 8.7.0 came from the pinned framework's
  `setup/native-ui` dependencies. It ran on `127.0.0.1:14733` with
  `APPIUM_HOME` unset and `ANDROID_HOME`/`ANDROID_SDK_ROOT` exported to that
  owned server. The controller received the explicit device and
  `$ANDROID_HOME/platform-tools/adb`.
- The controller started the explicit Agave 4.2.0
  `solana-test-validator`, acquired its own numeric loopback ports, and removed
  its fresh ledger on close. Only the newly installed debug app received the
  `10.0.2.2` emulator-gateway grants through `run-as`; the independent observer
  kept its numeric `127.0.0.1` connection.

The ordinary build command `leptoskit native build android` exited 0 after a
successful Gradle `assembleDebug`.

The earlier missing-AVD error came from different discovery roots: the command-line tools used
`~/.config/.android/avd`, while the Device panel's emulator lookup used `~/.android/avd`. Creating this
new AVD with explicit `ANDROID_AVD_HOME="$HOME/.android/avd"` and the already cached
`system-images;android-35;default;arm64-v8a` image made it available to the Device panel. The attached
Seeker was not used.

| Artifact                         | SHA-256                                                            |
| -------------------------------- | ------------------------------------------------------------------ |
| Debug APK                        | `5cbc5a9d8281322b29c9d79e07fdbf82b5c8bffe33089ac2f31d6a4b28a797f7` |
| Generated guest `app.wasm`       | `7c27980fbd91a8fac62f0a736e0a221a223b0ba1b59a817a30843f2a605da607` |
| Stripped JNI `lib` (`arm64-v8a`) | `bcd4bfddb331adca1091fbf9ca6df2e2ff738b7ca3731eb79bddabf85fe44b7b` |
| Installed controller executable  | `9c9200789e684cced84d9af546f8d149e34db5e6d14b9bc811ae94a16de1c6e3` |

## Executed behavior

Both invocations ran all three native sessions through the ordinary generated
Android host and composed `solana-journey` runtime:

- Create a wallet from native entropy, retain a private copy inside the app, and
  fund its visible public address once with a 2,000,000,000-lamport owned-validator
  airdrop. **Read balance** through the SDK displayed the funded amount.
- Reject a reviewed transfer; the independent observer required unchanged payer
  and recipient balances and no recipient transaction.
- Review and approve a separate 1,000,000-lamport System transfer. The observer
  verified the exact payer/recipient/System account list, System transfer
  instruction bytes, single signature, recipient credit, network fee, and exact
  payer debit before the app's confirmation text was accepted.
- Open account and slot subscriptions on one actual socket, hold a genuine
  account update, acknowledge `accountUnsubscribe`, release the held callback,
  and require the displayed balance to stay frozen. Then observe a later
  confirmed SDK HTTP balance and a strictly later slot on the continuing socket
  before closing both streams and the socket.
- Save a different wallet, terminate the app, relaunch it, and load that saved
  wallet. Verify its address, zero balance and a reviewed message signature,
  then repeat the subscription race.
- Terminate and relaunch again, restore the first wallet's private copy, and
  require its original address, retained post-transfer balance and another
  verified message signature.
- Explicitly disconnect every session, close the native sessions, stop and join
  the owned validator, and remove only its ledger and app data.

The second invocation's public transfer was signature
`2QANfYNeGm1wXdPLVuyLKfowUDagjmgB98NqjwVShWSRDYmGoXqLHozwiAQ3cHYh3iTBF13yqzyNKXTRRidKU89e`
from `4QhZ7hddrJ2kd1MwYnmMLs2XtnZqXM2ZMiK1TxqBfVwE` to
`7d3y2WdzxE7CfsWjkGy3WndkvZcj1EHMkzKJiFPiDecH`. The recipient received exactly
1,000,000 lamports, the network fee was 5,000, and the payer's retained balance
was 1,998,995,000. The first invocation produced the same balances with signature
`guSuVdzxNdp595u1pgVxr62oqF9rESUr4rFyHdNcrfcA2xCmyAzvZkHhPjxQe5ytA5L6zocSbs9S4WPCwjkFr9t`.

The second invocation's first wallet was `4QhZ7hddrJ2kd1MwYnmMLs2XtnZqXM2ZMiK1TxqBfVwE`;
its explicitly saved different wallet was
`D1fdEu1MNkuXMqAQuH9YKmrgFUtYxF7Y5LUiQB7AJj62`. **Load saved wallet** after the
first genuine process termination returned the saved address with a zero balance
and a verified signature. **Restore private copy** after the second termination
returned the original address rather than the saved one, and its **Read balance**
remained 1,998,995,000 lamports, equal to the independently observed ledger
balance retained from the initial transfer.

The public subscription probe account is the fixture's fixed public address
`7Z8ftDAzMvoyXnGEJye8DurzgQQXLAbYCaeeesM7UKHa`, freshly created on each owned
ledger. Each invocation's initial and relaunch sessions recorded these actual
observations:

| Run | Session               | Frozen display |      Held | Later RPC balance | Late tx slot | SDK HTTP context | Later socket slot |
| --- | --------------------- | -------------: | --------: | ----------------: | -----------: | ---------------: | ----------------: |
| 1   | Initial               |      1,000,000 | 2,000,000 |         3,000,000 |           43 |               43 |                45 |
| 1   | Saved-wallet relaunch |      4,000,000 | 5,000,000 |         6,000,000 |           91 |               91 |                93 |
| 2   | Initial               |      1,000,000 | 2,000,000 |         3,000,000 |           50 |               50 |                52 |
| 2   | Saved-wallet relaunch |      4,000,000 | 5,000,000 |         6,000,000 |          106 |              106 |               108 |

Balances are lamports. Every SDK balance matched the independent RPC balance,
its HTTP context covered the late transaction's slot, and the same-socket slot
was strictly later than that context. The cancelled display retained the
accepted first credit despite the held and late credits, and no subscription
session changed its wallet's ledger balance.

## Controller and evidence

`cargo test --locked --manifest-path verification/leptoskit-app-journey/installed/Cargo.toml
--all-targets` passed all 14 tests. `cargo clippy` with `--all-targets -D warnings`
passed. The repository's `cargo fmt --all --check`, `dprint check`,
`monochange check` and `cargo xtask style` gates passed; style reported 2,252
files with every category at or above 90 and overall 97.9.

The retained run directories are `/tmp/solana-kit-wallet-android-run-20261104`
and `/tmp/solana-kit-wallet-android-run2-20261104`. Each contains the three
sessions' actual PNG and XML captures, `index.html`, and
`subscription-initial.json`/`subscription-relaunched.json`. Each invocation
retained 23 named checkpoints and 23 native captures (46 total): 12 in the
initial session, eight in the saved-wallet relaunch and three in the restore
session. No ledger keypair or wallet private file is retained.

The bounded `files/journey-diagnostics.txt` receipt was 194 bytes and passed the
existing launcher whitelist; its SHA-256 is
`d19be291068ccf60d2d458b72a5937ae0f0be90bc13ed50af9cd90124f50631e`.
Logcat restricted to the app's UID contained no error records (0 bytes). The
Appium log recorded six created sessions and no failed session request. Local
evidence remains in `/tmp/solana-kit-wallet-android-evidence-20261104`.

The generated project (2.9 GiB, including its 1.8 GiB target) and the installed
controller target (469 MiB) were measured and removed after the runs. The
repository's shared `target` was left intact because it holds retained
runtime/style evidence referenced by other receipts. The tested app was
terminated, then uninstalled; the owned validator and Appium server were stopped
and reaped, and port 14733 is free. The emulator remains open in the Device
panel, and no other device or app was touched.

The first setup attempt failed before any UI action because the owned Appium
server had been started without `ANDROID_HOME`; the driver rejected session
creation. Restarting that server with the SDK variables and reinstalling a fresh
copy of the unique app resolved it. No assertion was changed or skipped. A
second full invocation then recorded exit 0.

## Remaining limits

This is installed acceptance of one ordinary generated Android app against an
owned emulator and loopback validator. It does not establish physical-device
behavior, the Solana Mobile Seeker, external wallet Intents, production networks,
Code Push, or a whole-SDK/platform claim. No CI workflow was dispatched, and no
artifact was published. The Android startup receipt remains the explicit
`receipt-missing-in-controller-data` marker: the controller stages its bounded
startup receipt in host data while the guest writes it inside app-private data,
so the retained marker records an unavailable receipt rather than inventing
phases. That pre-existing controller boundary was not changed in this run.
