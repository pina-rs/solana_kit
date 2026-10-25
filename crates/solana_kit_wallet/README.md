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

## Mobile Wallet Adapter

Enable the optional `mwa` feature for portable association URIs, P-256 association
signing and agreement, ES256 JWS, HELLO handshake messages, HKDF-SHA256 session
keys, AES-128-GCM frames, ordered sessions, JSON-RPC contracts, and SIWS messages.
The default Wallet Standard dependency graph remains unchanged.
The `mwa` feature also supports `no_std + alloc`. URI validation uses the
ASCII-only `iri-string` URI type with all dependency features disabled. Reflector
authorities retain exact spelling, u16 ports and canonical IPv6 syntax; userinfo,
paths, queries, fragments and malformed percent escapes are rejected.

The caller supplies validated private scalars and a fresh, unpredictable 12-byte
IV for every encryption under a session key, across both directions. Obtain
these inputs from the application's entropy provider. The library never reads
OS entropy or browser globals. Private keys and session keys have no `Debug`,
`Display`, or secret export contract. Owned private and session keys erase their
storage when dropped.

Use `Session` for ordered framing. Stateless `encrypt_message` and
`decrypt_message` support wire adapters and fixtures but do not reject replays.
Each session owns separate send and receive counters and rejects gaps, replays,
and counter exhaustion. Failed authentication leaves receive state unchanged.
`AgreementKey::accept_hello` consumes the ephemeral key. `Session::close` consumes
the session. The caller owns transport closure and cancellation.

`Request::new` maps all eight wallet methods, including the pinned legacy chain
and authorization behavior. `Response::from_json` checks the response version,
expected request ID, and exclusive result or error outcome. `MobileWallet` is a
concrete-future transport contract. No task or connection starts automatically.
`SignInResult::verify` checks exact expected SIWS bytes, signer, and Ed25519
signature. Applications still own nonce issuance, expiration policy, and replay
storage.

The [protocol receipt](../../docs/parity/mobile-wallet-protocol.md) records the
source pin, independent Dart vectors, and compile-only target evidence. Platform
launching, Android intents, browser discovery, reflector networking, installed
wallet availability, and wallet UI remain separate work. A guest cannot launch
mobile applications by constructing an association URI.

Wallet implementations return owned chain, account and feature snapshots.
`WalletFeatures` clones share immutable concrete feature owners through `Rc`;
wallet changes replace a snapshot rather than invalidating a borrowed slice.
Bind `let features = wallet.features();` before borrowing a concrete feature from
that snapshot. A snapshot describes authorization at its observation time, so
platform controllers must revalidate it after asynchronous work or wallet events.
The optional browser adapter supplies actual Window discovery and controller
ownership; the portable registry itself remains passive.
