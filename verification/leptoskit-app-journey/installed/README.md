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
application's confirmation text and real rendered pixels before disconnecting.
Neither funding nor approval is automatically repeated.

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
Button actions use the public accessibility labels so Android's uppercase visual
titles do not change which control the journey selects.

CI invokes one shell launcher so the installed APK and Appium PID share a cleanup
owner. It uninstalls its app on every controller outcome, then the emulator action
stops the owned emulator. The controller's local staging directory is also new and
removed after the session; it never copies private wallet material to the host.

Both the run directory and application data directory must be absent initially.
The generated macOS host uses the bundle-scoped Application Support directory shown
above; iOS uses its installed sandbox directly. The controller removes the owned application data and ledger after closing
the native session and joining the validator. CI
retains only the native screenshot/accessibility report and process logs; it
never uploads ledger keypairs. The driver quits only the tested application.

The workflow pins Agave 4.2.0 archives and checks their SHA-256 digests before
execution. The native driver dependencies and framework revision are also pinned.
Compilation, Clippy and protocol tests do not establish installed OS acceptance;
the actual CI journey must pass for each platform. Subscription controls, cancellation
and app relaunch/persistence journeys remain separate acceptance work.

This is exclusively off-chain client verification. Use Pina or official Solana
program libraries for on-chain programs.
