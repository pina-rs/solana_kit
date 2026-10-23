# Third party notices

Wallet Standard contract behavior and 37 exact icon data URIs originate in
`openbudgetfun/solana_kit` at commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`, package
`solana_kit_wallet_standard`. The package license is retained in `DART-LICENSE`.

The pinned `lib/src/logos.dart` attributes icons to official wallet adapter,
MetaMask, and Backpack sources under MIT, and Brave's Simple Icons source under
CC0. This unit preserves its icon bytes and names. It does not assert ownership
of wallet trademarks or platform integration with those wallets.

Regenerate the asset extraction with the private `logos` verification binary.
The declaration receipt checks every source file against its immutable catalog
SHA256. Independent upstream asset license provenance review remains pending.

Mobile Wallet Adapter behavior and oracle fixtures follow the same immutable
commit, package `solana_kit_mobile_wallet_adapter_protocol`. Its MIT license is
retained in `MWA-LICENSE`. Rust implementations use RustCrypto P-256, HKDF-SHA256,
and AES-GCM crates. Cryptographic algorithms are not copied or reimplemented.
The deterministic Dart signature oracle uses PointyCastle's RFC6979 signer.
The [protocol receipt](../../docs/parity/mobile-wallet-protocol.md) records the
bounded adaptation and remaining platform work.
