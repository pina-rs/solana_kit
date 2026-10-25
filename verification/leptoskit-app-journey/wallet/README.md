# Generated wallet application source

This is an adaptation of a real public LeptosKit generator workspace. No app was
generated, compiled or installed in source unit479. The Rust preparation source
changes a generated app's own source and dependencies; it retains the generated
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
`solana-wallet-platform`. Public3c/e446 rejects this hybrid metadata; root's
framework480 correction and its verified public pin are required before generation.
The SDK and generated guest must share that exact framework dependency identity.

Later admitted commands use the authenticated LeptosKit executable:

```sh
leptoskit new "$PROJECT" --name solana_wallet --platform macos --sdk-path "$FRAMEWORK"
"$RUSTC" --edition 2024 verification/leptoskit-app-journey/wallet/prepare.rs -o "$PREPARE"
"$PREPARE" "$SDK" "$FRAMEWORK" "$FRAMEWORK_HEAD" "$PROJECT"
cd "$PROJECT"
leptoskit build --platform macos
leptoskit build --platform android
leptoskit build --platform ios
```

The `new` arguments are traced to the public CLI source. Commands remain
prospective until the selected authenticated executable runs. The preparer checks
the exact clean framework HEAD and patches both registry and Git framework bridge
dependencies to that same checkout, preserving shared Rust type identity.
Root owns compiler/resource admission, producer identity,
registry resolution, generated source custody, native signing and installed runs.
There is no new fixture package or compiler cache in this source unit.

The fixture supplies a wallet-selection and approval entry point. Full D1–D9,
A1–A9 and I1–I9 still require real RPC funding, transactions, confirmation,
subscriptions, rejection/cancellation and replacement/unload evidence through the
existing client transport owners. This source is exclusively an off-chain client;
on-chain applications use Pina or official Solana program libraries.
