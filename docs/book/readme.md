<Hero start="getting-started/installation.md">

# solana_kit

A portable Rust Solana client for native applications, browser WebAssembly, WASI, and <Highlight kind="loop">embedded</Highlight> WebAssembly.

</Hero>

solana_kit is for <Highlight kind="marker">off-chain</Highlight> application code: wallets, services, indexers, and apps that read accounts, build transactions, and talk to RPC nodes. One `no_std + alloc` core owns addresses, codecs, messages, and transactions. Typed RPC sits on top of it without choosing an executor, and a small adapter crate per runtime owns the network.

> [!CAUTION]
> Do not use solana_kit in on-chain programs. Use Pina or the official Solana program libraries there. `no_std + alloc` makes the clients portable; it does not make them suitable for the chain.

## Where it runs

<Platforms>

<Platform name="Native" crate="solana_kit_native">

Bounded Tokio HTTP/1, TLS, and WebSockets with deadlines, cancellation, and authentication.

</Platform>

<Platform name="Browser" crate="solana_kit_browser">

Window Fetch and WebSocket on `wasm32-unknown-unknown`, with local futures.

</Platform>

<Platform name="WASI" crate="solana_kit_wasi">

Caller-owned WASI Preview2 HTTP and sockets on `wasm32-wasip2`.

</Platform>

<Platform name="LeptosKit" crate="solana_kit_leptoskit">

Guest and host transports for LeptosKit apps, with optional wallet controls.

</Platform>

</Platforms>

## Start here

New to the SDK? Read these in order, from an empty project to a confirmed transaction:

<Steps>

### Install the crates

[Installation](getting-started/installation.md) covers dependencies, features, targets, and the toolchain.

### Make a first call

[Quick start](getting-started/quick-start.md) reads a balance and the latest blockhash over native HTTP.

### Read, build, and send

[Addresses and keys](getting-started/addresses-and-keys.md), [Fetch an account](getting-started/fetch-an-account.md), [Build a transaction](getting-started/build-a-transaction.md), then [Sign and send](getting-started/sign-and-send.md).

</Steps>

## Find your way

<Cards>

<Card title="Core concepts" href="core/architecture.md">

How codecs, accounts, transactions, signers, RPC, and subscriptions fit together.

</Card>

<Card title="Runtimes" href="runtimes/index.md">

Pick the transport for native, browser, WASI, or LeptosKit, and connect a wallet.

</Card>

<Card title="Program clients" href="programs/index.md">

Typed builders and parsers for System, Token, Metaplex, Anchor, and more.

</Card>

<Card title="Services" href="services/helius.md">

Helius, Jupiter, Pyth, Surfpool, and the dApp Store publisher.

</Card>

<Card title="Crates" href="reference/crates.md">

Every crate in the workspace, what it is for, and how far along it is.

</Card>

<Card title="Parity and status" href="reference/parity.md">

What is implemented, partial, or pending, and where the ledgers live.

</Card>

</Cards>

## Honest status

solana_kit is young. Each capability is recorded as implemented, partial, or pending in ledgers that live in the repository, and these docs say plainly when something is partial. When a page and the source disagree, the source wins. [Parity and status](reference/parity.md) explains how to read the ledgers.
