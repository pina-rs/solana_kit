# Composed runtime Android support

The composed `solana-journey` runtime also installs the SDK wallet service, so
its generated Android app needs the SDK's existing MWA platform plugin compiled
into the same application. The generator accepts Android sources only from the
selected runtime package and its declared native-support dependencies.

This private package supplies those sources under
[`android/`](android/) as byte-for-byte copies of
`crates/solana_kit_leptoskit/android/{MwaPlatform,AssociationUri,ForegroundActivity}.kt`.
The Rust crate is empty; only the native source copy is meaningful. The
`support_sources_match_the_sdk` test fails when the SDK files and these copies
differ, so the package cannot drift into a second implementation.

`prepare-wallet-rpc` selects `solana-journey`; the runtime declares the
`dev.solanakit.wallet.MwaPlatform` dispatcher and the `solana-wallet` view
intent query. `prepare-wallet` (the wallet-only runtime) keeps its original
single-package Android plugin path and does not use this support package.
