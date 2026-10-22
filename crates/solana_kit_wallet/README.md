# solana_kit_wallet

Portable Wallet Standard contracts for off-chain application code. Concrete wallet
backends implement typed authorization and signing capabilities. The crate does
not authorize users, start an executor, discover browser wallets, or integrate MWA.
It uses no_std plus alloc. On-chain programs use Pina or official Solana program
libraries. See the Wallet Standard parity receipt for remaining platform work.

Use `WalletFeatures::feature::<ConcreteBackend>(identifier)` to retrieve a typed
capability, then call its `Connect`, `SignMessage`, or other capability trait.
Backends own user authorization and platform cancellation. Their associated
futures preserve native Send or local Rc behavior without an executor dependency.

Keep each registry registration and subscription guard alive while it is needed.
Dropping a guard removes only its own registration or listener. The registry
compares wallet names and preserves identity for explicit removal. It invokes
callbacks outside shared state borrows. Registries use Rc and feature maps retain
local backends. Concrete native signing backends can move independently to Tokio.

Returned signatures are raw bytes. Verify them with a suitable cryptographic
implementation before trusting them. The signing contracts deliberately preserve
backend output and never claim cryptographic verification or real wallet success.
