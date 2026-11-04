# Wallets

Wallet support is split into layers so that each runtime pulls in only what it needs. `solana_kit_wallet` defines portable Wallet Standard contracts: wallet metadata, typed capabilities and a passive registry. It has no platform code. Each runtime crate then adds a real backend. The browser crate discovers Window wallets, the native crate speaks Mobile Wallet Adapter to an Android wallet, and the LeptosKit crate adds UI and Android handover. Signing always happens in the real wallet. The SDK never fakes an authorization or a signature.

## Pick the layer you need

| You want to                                                              | Use                               | Feature                                            |
| ------------------------------------------------------------------------ | --------------------------------- | -------------------------------------------------- |
| Write code that is generic over any wallet backend                       | `solana_kit_wallet`               | default                                            |
| Speak the Mobile Wallet Adapter protocol (keys, framing, JSON-RPC, SIWS) | `solana_kit_wallet::mwa`          | `mwa`                                              |
| Connect to browser extension wallets from a web page                     | `solana_kit_browser::wallet`      | `wallet`                                           |
| Open an MWA session to an Android wallet from native code                | `solana_kit_native::mwa`          | `mwa`                                              |
| Show a wallet picker in a LeptosKit app                                  | `solana_kit_leptoskit::wallet_ui` | `wallet-ui`, `wallet-browser-ui`                   |
| Hand over to an Android wallet from a LeptosKit guest                    | `solana_kit_leptoskit`            | `wallet-platform`, `wallet-session`, `wallet-host` |

## Wallet Standard contracts

`solana_kit_wallet` is `no_std` plus `alloc`. Its main types:

- **`Wallet`** is the trait a backend implements: `version`, `name`, `icon`, `chains`, `features` and `accounts`. `version`, `name` and `icon` return borrowed references; `chains`, `features` and `accounts` return owned snapshots, so they stay valid if the backend's metadata changes.
- **`WalletFeatures`** is a typed map from a feature identifier to a concrete backend type. `features.feature::<T>(identifier)` returns `Some(&T)` only when the type matches.
- **Capability traits** each have their own associated future type: `Connect`, `Disconnect`, `Events`, `SignTransaction`, `SignAndSendTransaction`, `SignAndSendAllTransactions`, `SignMessage`, `SignIn` and `SignOffchainMessage`.
- **`WalletRegistry`** is a caller-owned registry. `register` returns a `Registration` guard. Dropping the guard removes only that entry.

Identifiers are constants: `CONNECT` (`standard:connect`), `DISCONNECT`, `EVENTS`, `SIGN_TRANSACTION` (`solana:signTransaction`), `SIGN_AND_SEND_TRANSACTION`, `SIGN_AND_SEND_ALL_TRANSACTIONS`, `SIGN_MESSAGE`, `SIGN_IN` and `SIGN_OFFCHAIN_MESSAGE`. `SOLANA_CHAINS` lists `solana:mainnet`, `solana:devnet`, `solana:testnet` and `solana:localnet`.

Because each capability names its own future type, a backend chooses its own threading. A browser backend returns local futures that hold an `Rc`. A native backend can return `Send` futures and run on Tokio. No executor is required.

This backend is adapted from the crate's test fixtures:

```rust
use solana_kit_wallet::{
    CONNECT, Connect, ConnectInput, ConnectOutput, WalletError, WalletFeature, WalletFeatures,
};
use std::future::{Ready, ready};

struct MyBackend;

impl WalletFeature for MyBackend {
    fn version(&self) -> &str {
        "1.0.0"
    }
}

impl Connect for MyBackend {
    type Error = WalletError;
    type Future<'a> = Ready<Result<ConnectOutput, WalletError>>;

    fn connect(&self, _input: ConnectInput) -> Self::Future<'_> {
        // A real backend prompts the user here, unless `_input.silent` is true.
        ready(Ok(ConnectOutput { accounts: Vec::new() }))
    }
}

let mut features = WalletFeatures::new();
features.insert(CONNECT, MyBackend)?;

// Bind the snapshot before borrowing a feature from it.
if let Some(connect) = features.feature::<MyBackend>(CONNECT) {
    let accounts = connect.connect(ConnectInput::default()).await?.accounts;
}
```

<Callout kind="important" title="Returned signatures are raw bytes">

The contracts pass wallet output through unchanged. They do not verify Ed25519 signatures or check that a signed transaction matches the one you sent. Verify signatures before you trust them. The browser signer below does check them.

</Callout>

## Browser wallets

Enable `solana_kit_browser`'s `wallet` feature. It implements the Wallet Standard `wallet-standard:register-wallet` and `wallet-standard:app-ready` events in a browser Window. This flow is adapted from the crate's browser tests:

```rust
use solana_kit_browser::wallet::{Connection, WalletController, WalletDiscovery, WalletFailure};
use solana_kit_signers::MessageSigner;
use solana_kit_wallet::WalletRegistry;
use std::rc::Rc;

// Keep discovery alive for as long as the page talks to wallets.
let discovery = Rc::new(WalletDiscovery::start(WalletRegistry::new())?);
let Some(wallet) = discovery.wallets().into_iter().next() else {
    // No wallet extension registered: show a "no wallet found" state instead.
    return Ok(());
};

let controller = WalletController::new(discovery.clone(), "solana:mainnet".into())?;
controller.connect(wallet, false)?.await?;
assert!(matches!(controller.state().connection, Connection::Connected { .. }));

let signer = controller.signer()?;
let signed = MessageSigner::<WalletFailure>::sign_message(&signer, b"hello").await?;
signed.validate(signer.address(), b"hello")?;
```

What the controller guarantees:

- Before prompting, `connect` requires the wallet to support the chain plus `standard:connect` and `standard:events`.
- `controller.signer()` implements `MessageSigner`, `TransactionPartialSigner`, `TransactionModifyingSigner` and `TransactionSendingSigner`. Exact-message and partial signing reject modified bytes. The modifying signer checks the signature over the message the wallet returned.
- Account changes, wallet removal, `disconnect`, `dispose` and replacement invalidate old signers. A stale signer fails with `WalletErrorCode::Disconnected`.
- To sign with a wallet and a local key together, use `solana_kit_signers::local::Composition::<WalletFailure>::with_failure`. See [Signers](../core/signers.md).

<Callout kind="caution" title="Cancellation is local">

Dropping a connect or signing future stops your app from acting on the result. It cannot close a prompt that is already open or undo a transaction the wallet already submitted. Wallet Standard has no abort method.

</Callout>

Discovery closes when its last owner is dropped. `WalletDiscovery::close` needs `&mut self`, so you can only call it for early teardown while you own the discovery directly, not through the `Rc` in the example above. `discovery.take_failures()` returns registration, event and cleanup errors. A submitted signature from `TransactionSendingSigner` is not proof of execution. Confirm it through [RPC](../core/rpc.md).

## Mobile Wallet Adapter

Mobile Wallet Adapter (MWA) is how Android apps talk to an installed wallet app. It has two parts in this SDK.

### The protocol: `solana_kit_wallet` with `mwa`

The `mwa` feature implements the portable protocol layer: association URIs, P-256 association keys and ECDH agreement, ES256 JWS, the HELLO handshake, HKDF-SHA256 session keys, AES-128-GCM frames, an ordered `Session` that rejects replays and gaps, JSON-RPC requests for all eight methods (`Method::Authorize`, `Reauthorize`, `Deauthorize`, `GetCapabilities`, `SignTransactions`, `SignMessages`, `SignAndSendTransactions`, `CloneAuthorization`) and Sign-In With Solana (`SignInResult::verify`). It is `no_std` plus `alloc` and never reads OS entropy. You supply the private scalars and a fresh IV.

The protocol pins the Dart package `solana_kit_mobile_wallet_adapter_protocol`. It negotiates `v1` and `legacy`.

### The native transport: `solana_kit_native` with `mwa`

The native `mwa` feature opens the local WebSocket session to the wallet:

```rust
use solana_kit_native::keys::NativeEntropy;
use solana_kit_native::mwa::{MwaConfig, NativeAssociation};
use solana_kit_wallet::mwa::Method;

// Only after an explicit user action:
let association = NativeAssociation::generate(NativeEntropy::default()).await?;
let uri = association.uri(None)?;
// ...launch `uri` with an Android Intent (your platform code)...
let mut wallet = association.connect(MwaConfig::default()).await?;

let response = wallet.call(Method::GetCapabilities, Default::default()).await?;
wallet.close().await?;
```

`MwaConfig::approval_timeout` defaults to five minutes. It bounds authorization and signing, including the time the user spends approving. A wallet rejection comes back as a protocol response, not a transport error.

This transport does not launch Android activities. On desktop and iOS no MWA wallet exists, so select a different backend there. An association URI built inside a Wasm guest cannot launch anything by itself.

### Android from a LeptosKit app

In a LeptosKit app, enable `wallet-session` (which includes `wallet-platform`) in the guest and `wallet-host` in the native host. `GuestMobileWallet::connect_android(&mut events, route)` checks availability, creates the association on the host, launches the Android wallet through `MwaPlatform` and connects the native session. You pass your single `Events` receiver and a `route` callback for unrelated events. Dropping the returned value cancels the native session, even mid-handover.

The runtime manifest selects `solana-wallet`. It registers `wallet_service::from_app_dirs` and the Android type `dev.solanakit.wallet.MwaPlatform`, under the bridge names `solana-wallet-session` and `solana-wallet-platform`.

## Local wallets in LeptosKit

`GuestLocalWallet::create` generates a key from native host entropy and keeps it in the host. The guest never receives private bytes. `load`, `save` and `backup` happen only when your app calls them. `sign_with_approval` waits for a real approval before it signs. Pair it with `wallet_ui::local_signing_approval`, which binds native Approve and Reject buttons to one review.

Saved keys live in a private `0700` directory under the app's sandbox, written as `0600` files that never overwrite existing ones. This works on desktop Unix, Android and iOS. Other systems fail explicitly. The private copy stays on the device. It is not an off-device backup, and uninstalling the app can delete it.

## Wallet UI

The `wallet-ui` feature renders a wallet picker and status panel with LeptosKit native elements:

```rust
use solana_kit_leptoskit::wallet_ui::{panel_channel, wallet_panel};
use solana_kit_wallet::ui::{PanelAction, PanelState, WalletPanel};

let (commands, mut events) = panel_channel(64)?;

let model = WalletPanel {
    state: PanelState::Disconnected,
    wallets: Vec::new(),
    accounts: Vec::new(),
    picker_open: false,
    error: None,
};
let element = wallet_panel(&model, &commands);

// On your local executor:
while let Some(action) = events.next_action().await? {
    match action {
        PanelAction::Connect(key) => { /* resolve `key` and connect */ }
        PanelAction::Disconnect => { /* disconnect */ }
        _ => {}
    }
    // Rebuild `model` from your controller and render again.
}
```

The panel holds only view data. It never owns keys, futures or a runtime. Buttons use native keyboard semantics and a minimum control height of 44 pixels. You are responsible for restoring focus, and you need to check it on each platform.

For a browser bundle, `wallet-browser-ui` adds `BrowserWalletView::new(discovery, chain)`. It drives a browser `WalletController` for you. Call `poll_update` on your local executor and render each update.

## Status

| Capability                             | Status                                                                                                                                                                                                                |
| -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Wallet Standard contracts and registry | Tests run in CI on native and in the portable conformance suite (native, Chrome, Wasmtime, LeptosKit). Declaration coverage in the parity receipt is still partial.                                                   |
| Browser `wallet` feature               | Tests run in CI in Chrome against a fixture wallet, not against real extension wallets                                                                                                                                |
| MWA protocol (`mwa` feature)           | Runs in the portable suite on every host. All 179 MWA declaration rows are still marked partial.                                                                                                                      |
| Native MWA transport                   | Unit tests against local loopback peers; no installed Android wallet is exercised                                                                                                                                     |
| LeptosKit local wallet and approval UI | The `generated-wallet` CI workflow drives a generated app on macOS, an iOS simulator and an Android emulator: create, approve, transfer, save and reload. Keyboard, focus and installed accessibility are unverified. |
| LeptosKit Android MWA handover         | Source and generated-app code exist; an installed journey against a real Android wallet is not recorded                                                                                                               |

The receipts are [Wallet Standard parity](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/wallet-standard.md), [MWA protocol](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/mobile-wallet-protocol.md), [wallet runtime](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/wallet-runtime.md) and [MWA runtime](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/mwa-runtime.md).
