# Current installed wallet execution

On 2026-11-04, the complete installed iOS wallet controller passed twice with
fresh application data and a separately owned Agave 4.2.0 validator for each run.
Both invocations exited 0. This is local execution evidence; no CI result is
required or inferred.

The installed controller's 14 native tests passed. The composed runtime's 11
native tests and six generated-wallet protocol tests also passed, with
`GENERATED_WALLET_WASM` set to the actual generated iOS guest. Invoking that
optional suite without its required artifact initially failed at its explicit
configuration guard; supplying the built guest resolved that setup failure.

SDK source: `65c502f9044d320ec809fc0b45670b7860a8b4e3` (the generated
application was built from its identical implementation before the fixture PR
was squash-merged). Public generator and application runtime framework:
`7437097c52aa68a3401bd378764e051ed4c9165a`. The test device was an arm64
iPhone Air simulator running iOS 26.5, with Xcode 27.1's simulator SDK and Rust
1.99.1. Appium 3.8.0 used XCUITest driver 12.19.1 and its separately built
WebDriverAgent.

## Executed behavior

Each invocation ran all three native sessions through the ordinary generated
host and composed `solana-journey` runtime:

- Create a wallet using native entropy and retain a private copy inside its app.
- Fund its visible public address on the owned validator and read its balance
  through the SDK.
- Reject a transfer without ledger effects, then approve a separate review.
  Independently verify transaction accounts, instruction bytes, signature,
  recipient credit, network fee and exact payer debit.
- Open account and slot subscriptions on one actual socket. Hold an account
  update, acknowledge unsubscribe, release the held callback, and require the
  displayed balance to remain frozen.
- Observe a later confirmed HTTP balance and then a strictly later slot on the
  continuing socket. Close subscriptions and socket.
- Save a different wallet, explicitly terminate the app, relaunch and load it.
  Verify its address, balance and a message signature, then repeat the socket
  cancellation probe.
- Explicitly terminate and relaunch again, restore the first wallet's private
  copy, and verify its original address, retained post-transfer balance and
  message signature.
- Close native sessions, stop and join the owned validator, and remove only the
  application data and ledger created by that invocation.

The controller retained these public subscription observations:

| Run | Session               | Frozen balance | Later RPC balance | SDK HTTP context | Later socket slot |
| --- | --------------------- | -------------: | ----------------: | ---------------: | ----------------: |
| 1   | Initial               |      1,000,000 |         3,000,000 |               65 |                67 |
| 1   | Saved-wallet relaunch |      4,000,000 |         6,000,000 |              101 |               103 |
| 2   | Initial               |      1,000,000 |         3,000,000 |               34 |                36 |
| 2   | Saved-wallet relaunch |      4,000,000 |         6,000,000 |               70 |                72 |

Balances are lamports in the separate public probe account. Every SDK balance
matched the independently observed RPC balance. Each HTTP context covered its
late transaction slot, and each socket slot exceeded that context. The frozen
display retained the accepted first credit despite held and later credits.

## Artifacts and reproduction

Local run directories are `/tmp/solana-kit-wallet-ios-run-20261104-1` and
`/tmp/solana-kit-wallet-ios-run-20261104-2`. They retain named native checkpoints,
PNG/XML captures, `subscription-initial.json`, `subscription-relaunched.json`
and startup receipts. These directories are local artifacts, not checked-in
fixtures. No ledger keypairs or wallet private files were retained as evidence.

The installed application's SHA-256 values were:

```text
33f3451e4aef492c7db60177b835878af693b3eab6c6dcfd71e81eaa120f0f06  App
6e16d06dbd26956e0db95a3b56eed8457d61308bcd2ba41c988458689b867568  app.lkb
```

Generate with a unique name before preparation, for example
`leptoskit new "$PROJECT" --name solana_kit_wallet_ios_20261104 --platform native --native-target ios`.
Run `cargo xtask prepare-wallet-rpc "$PROJECT"`, then the pinned CLI's
`native build ios-simulator`. Install that ordinary bundle on the selected
simulator, build the pinned driver's WebDriverAgent with Xcode's compiler, and
use the explicit installed-controller arguments documented in its
[README](../../verification/leptoskit-app-journey/installed/README.md).
Select a fresh run directory and an absent Application Support directory in
that app's installed sandbox for every invocation.

## Remaining limits

The macOS application subsequently completed the same three-process functional journey through the
existing computer-use connection, including independent ledger checks and both subscription races;
see the [macOS execution receipt](installed-wallet-macos-current.md). Mac2's XCTest runner still timed
out while enabling automation, and developer-tool authorization was unchanged. The successful manual
native journey is separate from that unavailable automated driver route.
The maintained Android controller subsequently completed the three-process journey twice on a freshly
created arm64 API 35 emulator, including both subscription races and all ledger oracles; see the
[Android execution receipt](installed-wallet-android-current.md). Physical devices, production
networks, external wallet Intents, and complete platform acceptance beyond this wallet controller
remain outside these receipts.
