# Generated wallet application source

This fixture adapts a real public LeptosKit generator workspace. The Rust preparer
changes the generated app's source and ordinary Cargo dependencies; it retains the generated
macOS, Android and iOS hosts, ABI entry points, service scheduler and registrants.
It does not supply a handwritten host mislabeled as generated.

The app owns one local executor and one bridge event receiver. Its real buttons
create or load a wallet, review a signature, approve or reject the matching review,
save privately, create a separate private copy, cancel and disconnect. Local keys
use host entropy. There is no fixed test seed. A signature is independently
verified against the exact displayed message before success is shown. Android
also calls the real availability/Intent path, binary association, authorization
and message signing. Other platforms offer the local wallet and no MWA capability.

Both persistence actions are explicit and never overwrite an existing file.
The private copy stays in the app sandbox; it is not an off-device recovery backup.
Uninstalling an app can remove both files. No secret bytes appear in UI or receipts.
Native errors stay in the service's bounded original-cause journal; guest errors
stay in the app's redacted observer slot. Unrelated bridge events are retained in
a bounded application queue, not consumed by a second Events receiver.

The selected runtime is `solana-wallet`. It installs Rust
`wallet_service::from_app_dirs` and Android `dev.solanakit.wallet.MwaPlatform`.
Their actual bridge names are distinct: `solana-wallet-session` and
`solana-wallet-platform`. The SDK and generated application select framework commit
`25b1861ddc729a8f3b70117cd0c48e06659c9697`, which supports hybrid registration and
Android action/scheme visibility queries. Normal Git dependencies give every
framework package the same Rust source identity. Cargo patch and replace tables
are rejected.

`prepare-wallet-rpc` instead selects the private [composed runtime](../runtime/README.md)
for local-wallet HTTP, WebSocket and timer journeys. It retains the generated
hosts and existing approval controls. The original runtime remains the fixture
for Android external wallet availability and Intent dispatch.

Use the CLI built from that verified framework source:

```sh
leptoskit new "$PROJECT" --name solana_wallet_app --platform native --native-target macos
cargo run --locked --manifest-path "$SDK/Cargo.toml" --package solana_kit_xtask \
  -- prepare-wallet "$PROJECT"
cd "$PROJECT"
leptoskit build --platform macos
leptoskit build --platform android
leptoskit build --platform ios
```

The preparer checks the SDK's framework revision and the actual generated member
layout before writing any files. It retains target-specific host dependency
sections and fails on unknown template shapes or existing source overrides.
Repeated preparation preserves every generated file byte for byte. Source
preparation does not establish guest compilation or an installed application run.

The fixture supplies a wallet-selection and approval entry point. Full D1–D9,
A1–A9 and I1–I9 still require real RPC funding, transactions, confirmation,
subscriptions, rejection/cancellation and replacement/unload evidence through the
existing client transport owners. This source is exclusively an off-chain client;
on-chain applications use Pina or official Solana program libraries.
