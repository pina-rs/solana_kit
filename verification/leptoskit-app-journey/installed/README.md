# Installed generated wallet journey

This private controller drives an application produced by the public LeptosKit
generator through Appium's native macOS, iOS and Android drivers. It selects the ordinary composed
runtime using `prepare-wallet-rpc`. No generated platform host is replaced or
patched. This package is never published or reserved on crates.io.

The controller starts and joins its own validator with a fresh ledger and explicit
configuration. Health is accepted only while that child is alive and its RPC
identity matches the keypair in that new ledger. Signing authority comes from the
generated application's native wallet, created through a real button press.
There is no fixed payer seed, ambient Solana configuration, or borrowed RPC node.

The native journey reads the visible account from the actual accessibility
capture, funds it once on the owned validator, reads its balance through the SDK,
rejects a proposed transfer, and approves a second review showing the payer,
recipient, amount and fee notice. An independent JSON-RPC observer checks the
executed legacy System transfer, exact accounts and instruction bytes, network
fee and payer debit, recipient effect and unique signature. It then checks the
application's confirmation text and real rendered pixels. Before disconnecting,
the controller retains the payer's post-transfer balance from the verified
transaction metadata, requires the owned validator's balance to match, and
checks the app's **Read balance** text against the same value. The network fee
comes from that transaction; the controller does not assume a fixed fee.
Neither funding nor approval is automatically repeated.

The controller explicitly makes a private copy of that first wallet, then creates
and saves a different native wallet. Each later phase closes its OS automation
session and launches the generated application again with the same private app
data. The first relaunch selects **Load saved wallet**; the second selects
**Restore private copy**. Each must show its original, distinct public address,
read the independently observed balance, and approve a message whose signature
the guest verifies against that wallet's public key. Before signing with the
restored private copy, both the owned validator's balance and the app's **Read
balance** text must equal the retained first-session post-transfer balance.
Recovery cannot adopt a changed ledger balance as its new baseline. Signing must
leave each wallet's ledger balance unchanged. A restore implementation that
incorrectly loads the saved wallet therefore fails the address check.

The initial and recovered balance checks retain their own named checkpoints.
Identify each balance and signature capture by its checkpoint label rather than
by a fixed capture number.

The initial session and the saved-wallet relaunch also drive **Watch account
updates**. The guest opens the granted validator WebSocket and uses the public
SDK's typed `accountSubscribe` and `rootSubscribe` owners on that same socket.
A separate public probe account receives three single airdrop requests. The
controller independently confirms each request and checks its exact RPC balance;
the payer, transfer recipient and saved wallet receive none of those credits.

The first notification must update the native balance display. **Hold next
account update** retains the second genuine SDK-decoded notification without
committing it to that display. **Cancel account subscription** invalidates its
delivery generation, awaits a true `accountUnsubscribe` acknowledgement, and then
releases the held notification through the ordinary state reducer. The displayed
balance must remain at the first credit. Removing that reducer's generation
guard makes this step fail.

After cancellation, the third credit must change the independent RPC balance.
The controller then requires an actual root notification on the continuing
socket beyond that transaction's slot and checks the frozen display again. A
timer or an absence of notifications cannot satisfy this check. The named native
captures and `subscription-initial.json` or `subscription-relaunched.json` retain
the observed RPC balances, captured display text, transaction slot and root.
An unsupported root subscription or missing acknowledgement fails the journey.

**Close subscription socket** remains available while opening and throughout the
probe. The controller attempts it after either assertion success or failure.
The guest attempts both stream cancellations and a real socket close on every
result, preserving cleanup failures alongside the original error. The existing
native runtime still owns and joins the socket actor before application teardown.
After process relaunch, a new probe must start with a waiting balance; the same
positive update, cancellation race and late-credit checks run again.

Relaunch never reconnects a wallet automatically: both sessions first require the
ordinary selection controls and no connected account. Private wallet bytes remain
inside the application sandbox. The controller only retains public addresses,
native screenshots and accessibility captures. It never reads or copies either
private key file. All three native sessions must finish before the owned data and
validator are removed; failure in a phase stops later phases and retains its
failure capture.

Each successful phase explicitly terminates only its tested app and requires the
driver's not-running state before the next session. This uses the documented
[Mac2 application commands](https://appium.github.io/appium-mac2-driver/latest/reference/execute-methods/),
[XCUITest application commands](https://appium.github.io/appium-xcuitest-driver/latest/reference/execute-methods/)
and [UiAutomator2 application commands](https://appium.github.io/appium-uiautomator2-driver/latest/reference/execute-methods/).
Creating another automation session alone does not count as process relaunch.

Run the controller on a dedicated macOS test machine after building the ordinary
generated application and starting the pinned Mac2 driver:

```sh
cargo run --locked --manifest-path "$SDK/verification/leptoskit-app-journey/installed/Cargo.toml" -- \
  macos http://127.0.0.1:14731 "$PROJECT/.leptoskit/macos/App.app" \
  "$HOME/Library/Application Support/com.leptoskit.solanawalletapp" \
  "$RUN_DIRECTORY" "$VALIDATOR_BINARY"
```

The iOS CI journey creates its own simulator, builds the ordinary generated
application with `leptoskit native build ios-simulator`, and installs that bundle.
It selects Xcode 26.6 on the macOS 26 runner, verifies the test runtime library
required by the pinned XCTest agent, and chooses an available simulator runtime
matching that Xcode's simulator SDK.
It queries the installed application's container and passes its new
`Library/Application Support` directory to the controller. The job builds the
pinned driver's WebDriverAgent separately and passes its explicit derived-data
directory. Agent compilation therefore completes before its service-start
deadline begins. Launch uses the bundle
identifier and explicit simulator ID, preserving the configuration written before
launch. The iOS session explicitly starts and terminates only its tested app while
preserving that configuration, as described by the
[XCUITest lifecycle capabilities](https://appium.github.io/appium-xcuitest-driver/latest/reference/capabilities/).
The wallet, native UI actions and independent ledger checks are shared
with the macOS journey. CI shuts down and deletes only the simulator it created.
Before deletion it retains five minutes of unified logs scoped to the generated
app, including startup failures that precede the first wallet control.
The private runtime also records six fixed startup phases. Before removing owned
app data, the controller retains a bounded receipt containing only those phase
names. A missing receipt is explicit; other read failures remain errors.
The observer reads the mounted root and pending backend log without draining
render mutations. Each observation is recorded once, and a new factory call
resets the receipt. This helps distinguish factory, configuration, activation,
polling and rendering progress without retaining wallet or endpoint data.
No simulator or generated mobile build is created on the development machine.

The Android job builds the ordinary generated x86-64 debug APK and installs it on
a fresh API35 emulator. Its controller receives an explicit ADB binary and device
ID. Before launching the app, it copies only the public bootstrap configuration
into the new app's private files directory through `run-as`. The guest uses the
[emulator's host-loopback alias](https://developer.android.com/studio/run/emulator-networking-address),
while the independent observer keeps its own numeric host-loopback connection.
The controller reads the native account through its Android content description
and applies the same funding, rejection and executed-transfer checks.
Android button actions use public accessibility descriptions so uppercase visual
titles do not change which control the journey selects. Apple actions retain the
public text selector; on iOS this excludes inaccessible title children that can
share a button's name. The controller carries its selected platform into each
action.

CI invokes one shell launcher so the installed APK and Appium PID share a cleanup
owner. It uninstalls its app on every controller outcome, then the emulator action
stops the owned emulator. The controller's local staging directory is also new and
removed after the session; it never copies private wallet material to the host.

Before uninstalling, the launcher retains the owned emulator's main and crash
buffers at error level. Capture keeps at most 1,000 records with a deadline of
20 seconds and a further five seconds before forced termination. The workflow
uploads `installed-android.log`. A capture failure fails a successful journey and
preserves an earlier controller failure.

Both the run directory and application data directory must be absent initially.
The generated macOS host uses the bundle-scoped Application Support directory shown
above; iOS uses its installed sandbox directly. The controller removes the owned application data and ledger after closing
the native session and joining the validator. CI
retains only the native screenshot/accessibility report, public subscription
receipts and process logs; it
never uploads ledger keypairs. The driver quits only the tested application.

The workflow pins Agave 4.2.0 archives and checks their SHA-256 digests before
execution. The native driver dependencies and framework revision are also pinned.
Compilation, Clippy and protocol tests do not establish installed OS acceptance;
the actual CI journey must pass for each platform. The save/relaunch/restore
and subscription/cancellation phases above are implemented in source but require
their current-source CI execution before claiming installed acceptance. The new
journeys and generation-guard regression remain **UNEXECUTED** until their actual
compiler and macOS, Android and iOS runs are recorded. Browser subscription
acceptance remains separate work.

This is exclusively off-chain client verification. Use Pina or official Solana
program libraries for on-chain programs.
