# Reference findings and reuse

Source inspection was performed on 2026-10-22 against the immutable revisions in
[references.toml](references.toml). Reference tests, browser builds, WASI hosts,
and mobile apps were not executed during this audit. Checked-in tests identify
useful verification scenarios; they are not fresh passing results.

## Dart Solana Kit

The reference contains 70 top-level Dart packages and one TypeScript Codama
renderer. Its own upstream compatibility marker is `@solana/kit` 8.5.0; that is
the comparison baseline, not a compatibility claim for this Rust project.

Addresses flow into ordered instruction account metadata, then compiled messages,
ordered signature slots, and immutable transaction bytes. Signing retains
blockhash/nonce lifetime metadata. Sending checks the node's reported signature;
confirmation races observed success against lifetime expiry and cancels the
losing work. Resource estimation lives where RPC and transaction compilation
are both available. These are useful ownership boundaries to preserve.

The [message model](https://github.com/openbudgetfun/solana_kit/blob/0138bb956d2d81ccc0c56edc602401c0df2f2b46/packages/solana_kit_transaction_messages/lib/src/transaction_message.dart)
already supports legacy, v0, and v1. The
[v1 transaction fixtures](https://github.com/openbudgetfun/solana_kit/tree/0138bb956d2d81ccc0c56edc602401c0df2f2b46/packages/solana_kit_transactions/test/fixtures)
and [upstream parity tests](https://github.com/openbudgetfun/solana_kit/blob/0138bb956d2d81ccc0c56edc602401c0df2f2b46/packages/solana_kit/test/upstream_parity_test.dart)
provide byte-level oracles. Rust CI must fail if required fixtures are missing;
the reference harness can skip absent fixtures.

The [RPC catalog](https://github.com/openbudgetfun/solana_kit/blob/0138bb956d2d81ccc0c56edc602401c0df2f2b46/packages/solana_kit_rpc_api/lib/src/solana_rpc_api.dart)
contains 54 methods, while the
[convenience extension](https://github.com/openbudgetfun/solana_kit/blob/0138bb956d2d81ccc0c56edc602401c0df2f2b46/packages/solana_kit_rpc/lib/src/rpc_methods.dart)
exposes 14 distinct methods plus four parsed-value helpers. The Rust port should
give the entire catalog typed requests and responses. Exact integers and
missing/null values need explicit Rust representations.

There are [nine subscription families](https://github.com/openbudgetfun/solana_kit/blob/0138bb956d2d81ccc0c56edc602401c0df2f2b46/packages/solana_kit_rpc_subscriptions_api/lib/src/solana_rpc_subscriptions_api.dart),
including separately marked unstable methods. The `solana_kit_subscriptions`
package is instead a recurring-payments on-chain program. Preserve that name
distinction in Rust.

Reuse IDLs, pinned program revisions/artifacts, corrected layouts, discriminators,
seed vectors, codec boundaries, and generator security cases. The
[renderer](https://github.com/openbudgetfun/solana_kit/blob/0138bb956d2d81ccc0c56edc602401c0df2f2b46/packages/codama-renderers-dart/src/visitors/renderVisitor.ts)
resolves output before replacing files. Its exact instruction consumption,
account capacity, optional-account position, and invalid-IDL rules matter more
than its Dart templates. Not every program is generated; ATA has handwritten
behavior. [Generation inputs and corrections](https://github.com/openbudgetfun/solana_kit/blob/0138bb956d2d81ccc0c56edc602401c0df2f2b46/scripts/generate_program_packages.mjs)
must be retained explicitly.

Platform boundaries need adaptation. Keys import `dart:io` for filesystem work;
three wallet packages require Flutter. Filesystem persistence, browser discovery,
mobile host interactions, and LeptosKit UI must have separate Rust implementations.
Dart's `Future` wrappers around deterministic PDA work need not become Rust async
APIs. Rust `Option`, enums, integer types, and standard futures/streams replace
language-specific wrappers.

## Wasm Solana

The existing client selects gloo browser transports or reqwest native transports,
with browser abort-on-drop and lazy WebSockets. Its public account/message/
transaction types and full decoder forks remain tied to a much broader Solana
dependency graph. Reuse isolated behavior and fixtures; give the new SDK its own
public protocol types.

Useful regression scenarios include registration before subscribe send,
notification-before-ack races, quiet-stream cancellation, terminal handshake
failure, commitment ordering, account-data decode failures, decompression limits,
and [v1 wire comparison](https://github.com/pina-rs/wasm_solana/blob/6f43f957a069902195fe55bff89b5531211e170a/crates/wasm_client_solana/tests/v1_wire_format.rs).

Avoid copying environment selection through additive `js`/`ssr` flags or requiring
all browser futures to be `Send`. The
[provider](https://github.com/pina-rs/wasm_solana/blob/6f43f957a069902195fe55bff89b5531211e170a/crates/wasm_client_solana/src/providers/websocket_provider.rs)
holds an unread root fork of its
[shared stream](https://github.com/pina-rs/wasm_solana/blob/6f43f957a069902195fe55bff89b5531211e170a/crates/wasm_client_solana/src/providers/forked_stream.rs).
Static inspection suggests that ownership can retain socket history until the
root is dropped. This has not been reproduced here. Use bounded dispatch queues
and test memory/lifecycle behavior in the new implementation.

Keep validator/oracle work in separate test workspaces. Existing example version
pins demonstrate why transitive Solana upgrades need downstream tests, not merely
successful workspace compilation.

## LeptosKit

The [native builder](https://github.com/ifiokjr/leptoskit/blob/fecf8628d326ff7b03f295523e6bb584c3fdf88d/crates/leptoskit_cli/src/native/mod.rs)
produces plain `wasm32-unknown-unknown` guests. The
[host](https://github.com/ifiokjr/leptoskit/blob/fecf8628d326ff7b03f295523e6bb584c3fdf88d/crates/leptoskit_wasm/src/lib.rs)
interprets them in wasmi with custom renderer/plugin imports, rather than a
JavaScript environment or a WASI component host. Guest futures run in a local
pool and wake through host callbacks.

The current
[HTTP](https://github.com/ifiokjr/leptoskit/blob/fecf8628d326ff7b03f295523e6bb584c3fdf88d/crates/leptoskit_plugin_http/src/lib.rs)
and [WebSocket](https://github.com/ifiokjr/leptoskit/blob/fecf8628d326ff7b03f295523e6bb584c3fdf88d/crates/leptoskit_plugin_websocket/src/lib.rs)
plugins expose direct native/browser clients and explicitly lack guest bridge
handlers. The SDK needs a real cancellation-aware host integration before guest
networking works. Existing native/browser tests do not establish Android/iOS
networking support.

[Guest entropy](https://github.com/ifiokjr/leptoskit/blob/fecf8628d326ff7b03f295523e6bb584c3fdf88d/crates/leptoskit_plugin_crypto/src/lib.rs)
is asynchronous. Await seed bytes, then derive keys with audited deterministic
Rust crypto. Inject an actual monotonic sleep provider. Do not select browser
timers for every Wasm target or assume `getrandom/wasm_js` can work in a guest.

The [bridge](https://github.com/ifiokjr/leptoskit/blob/fecf8628d326ff7b03f295523e6bb584c3fdf88d/crates/leptoskit_bridge/src/guest.rs)
has a single guest event receiver. Socket events must use application-owned
routing or cancellable per-socket receive calls. An SDK adapter cannot acquire a
second global receiver and consume other plugins' events.

LeptosKit remains an optional adapter dependency. No framework implementation was
copied into this repository. Integrating its installed hosts is a separate,
explicit delivery unit with its own runtime evidence.

## Reuse policy

| Material                                                     | Treatment                                                                         |
| ------------------------------------------------------------ | --------------------------------------------------------------------------------- |
| Ed25519, SHA-256, curve operations, zeroization              | Reuse vetted portable Rust crates; isolate entropy                                |
| Small official address/instruction/System Program interfaces | Prefer direct bounded reuse behind Kit-owned conversions after target/graph proof |
| Official client/validator/decoder stacks                     | Keep out of library normal/build dependencies                                     |
| IDLs, wire vectors, program pins                             | Reuse with immutable provenance and applicable notices                            |
| Existing transport implementations                           | Adapt useful isolated behavior after lifecycle/target review                      |
| Dart templates, Flutter widgets, dynamic JSON plumbing       | Implement idiomatic Rust/LeptosKit equivalents                                    |
| Existing Rust Codama renderer                                | Evaluate before building a focused Kit renderer                                   |

The account-creation example deserves an early feasibility experiment:
`solana-system-interface` 3.3.0 exposes `alloc` and `bincode` features and declares
Rust 1.89 support. `cargo info` confirmed those manifest facts during this audit.
Compilation and runtime portability have **not** been established. Test the
actual constructor and full selected graph before admitting it; the dependency's
name alone cannot establish compatibility. Preserve licenses for any copied code.
