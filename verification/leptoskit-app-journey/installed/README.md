# Installed generated wallet journey

This private controller drives an application produced by the public LeptosKit
generator through Appium's native macOS driver. It selects the ordinary composed
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
  http://127.0.0.1:14731 "$PROJECT/.leptoskit/macos/App.app" \
  "$HOME/Library/Application Support/com.leptoskit.solanawalletapp" \
  "$RUN_DIRECTORY" "$VALIDATOR_BINARY"
```

Both the run directory and application data directory must be absent initially.
The generated host uses the bundle-scoped Application Support directory shown
above. The controller removes the owned application data and ledger after closing
the native session and joining the validator. CI
retains only the native screenshot/accessibility report and process logs; it
never uploads ledger keypairs. The driver quits only the tested application.

The workflow pins Agave 4.2.0 archives and checks their SHA-256 digests before
execution. The native driver dependencies and framework revision are also pinned.
Compilation, Clippy and protocol tests do not establish installed OS acceptance;
the actual CI journey must pass. Android, iOS, subscription controls, cancellation
and app relaunch/persistence journeys remain separate acceptance work.

This is exclusively off-chain client verification. Use Pina or official Solana
program libraries for on-chain programs.
