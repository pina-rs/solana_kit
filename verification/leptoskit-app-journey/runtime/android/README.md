# Generated Android plugin sources

This directory stays free of Kotlin sources. `prepare-wallet-rpc` selects the
composed `solana-journey` runtime as the generated app's only Rust service, and
the runtime declares `dev.solanakit.wallet.MwaPlatform` as its Android
dispatcher. The generator still requires an `android/` directory beside the
runtime manifest; the actual plugin sources are packaged from the SDK by the
native-support crate in [`../android-support`](../android-support/README.md).

`prepare-wallet` (the wallet-only runtime) continues to compile its Android
plugin from `crates/solana_kit_leptoskit/android` directly.
