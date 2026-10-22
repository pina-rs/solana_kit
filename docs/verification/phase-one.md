# Phase 1 experiment evidence

The five narrow portability experiments in the [delivery plan](../planning/plan.md)
have executed evidence. This reference records their boundaries. It does not
complete the ecosystem, production adapters, or installed application journeys.
All proofs concern off-chain application clients. On-chain programs use Pina or
official Solana program libraries.

## Experiment matrix

| Experiment                       | Executed evidence                                                                                                                                                                                                                                                                                                            | Boundary of the result                                                                                                                                                              |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Identical deterministic fixtures | The accepted 59-case suite executes native Rust, real Chrome, Wasmtime Preview 1 and Preview 2, and actual LeptosKit `WasmHost`. Every host observes all 59 canonical passes and only the corrupt-hash failure in the 58/59 control. [Numeric receipt](../parity/numeric-string-codecs.md), [runtime receipt](runtimes.md).  | Headless interpreted execution establishes no desktop/mobile application or network support.                                                                                        |
| Generic native and local futures | One generic private client executes in multi-thread `tokio::spawn` and with `Rc` retained across Pending. The public RPC client preserves concrete associated futures, with native spawn and portable Pending/wake/correlation checks. [Boundary receipt](boundaries.md), [public RPC receipt](../parity/rpc-foundation.md). | Memory transports and fixture TCP cancellation establish no production HTTP adapter. A compile-negative control rejects local futures in Tokio.                                     |
| Official CreateAccount reuse     | The real standalone constructor executes canonical and corrupt modes in native Rust, Chrome, both WASI previews, and actual interpreted LeptosKit. All 52 bytes, program ID, account order, and privileges are checked. [Constructor receipt](boundaries.md).                                                                | The selected 22-package normal/build closure enables `std`. Public no_std System data codecs use the pinned IDL and retain the official constructor as a private oracle.            |
| Async guest capabilities         | The actual pinned LeptosKit guest bridge executes entropy Pending/wake, injected timer deadlines, quiet cancellation, late reply rejection, and generation-aware replacement. Missing bridge, wake, cancellation, and corrupt entropy controls fail. [LeptosKit receipt](leptoskit-async.md).                                | Deterministic fixture entropy and injected time prove the bridge contract. They do not prove OS entropy, real-time scheduling, native I/O shutdown, or guest destructors on unload. |
| Ordinary downstream consumption  | `cargo xtask consumer` packages core, extracts it outside the repository, removes Rust flags and compiler wrappers, and executes public API tests. The RPC source consumer also executes without patches or repository flags. [Runtime receipt](runtimes.md), [RPC receipt](../parity/rpc-foundation.md).                    | The client and System depend on unpublished core. A clean source consumer is not a successful registry publication rehearsal.                                                       |

## Named host capabilities

Separate accepted [native HTTP](native-http.md) and
[Window Fetch HTTP](browser-http-integration.md) adapters execute actual network
requests, bounds, deadlines, credential isolation, and Drop cancellation with
independent peer observations. Their receipts define the accepted scope.
Browser workers, WebSockets, installed applications, and complete host support
remain separate gates.

The [WASI HTTP experiment](wasi-contract.md) executes outbound HTTP, bounded bodies,
status errors, deadlines, pending-response Drop with peer EOF, retained-response
control, entropy, and monotonic timer wake. It pins Wasmtime 50.0.0 and WASI
0.2.12. Its HTTP grant provides no destination allowlist or socket availability.

The separate [WASI socket experiment](wasi-sockets.md) executes 12 assertion-bearing
journeys through TCP, Rustls 0.23.46, and Tungstenite 0.29.0. They include validating
TLS, WebSocket exchange and close, wrong CA/name, plaintext, upgrade rejection,
frame bounds, quiet resource Drop, retained-owner control, process unload, and
denied capabilities. `inherit-network=y` grants all TCP destinations. Process
termination does not prove embedded Store replacement or async socket wakeup.

The core and async LeptosKit workspaces pin upstream revision
`fecf8628d326ff7b03f295523e6bb584c3fdf88d` and execute wasmi 2.0.0 through the real
host. The synchronous core guest allows three rendering imports. The separate
async guest allows those three plus eight plugin-v1 imports with exact signatures.
Neither guest assumes browser globals. Application code owns event routing,
generation tokens, and fixture cleanup.

WASI and LeptosKit network adapters are outside the accepted snapshot documented
here. WASI denial and early HTTP responses during upload need corrected error
precedence and independent verification. Actual headless LeptosKit HTTP work is
under separate review and integration. Neither candidate replaces the limits of
the private experiments above.

## Workspace and release gates

The [style gate](style.md) requires Monostyle 0.3.26 and both scores at least 90
for the repository and every scored file, including generated code. Git supplies
tracked and nonignored new files explicitly. Every authored Rust path must appear
in the scored report, even under a cache-like directory. Unsupported languages
remain reported as skipped, with no invented score.

[Monochange configuration](../../monochange.toml) assigns core, client, native,
and browser to the `sdk` group and System to an independent package. The
[verification workflow](../../.github/workflows/plan.yml) runs `monochange check`,
`monochange step validate`, discovery, and release preview. The initial real core
package preview and later core/client/System dry runs establish version ownership.
They do not establish final release readiness. Component publishing is disabled.
Only the dependency-free `solana_kit` 0.0.0 name reservation was published.

## Reproduction and remaining work

`cargo xtask all` runs the shared portable suite and its gates. The linked private
experiment receipts contain their own build/run commands, pins, artifact hashes,
and expected failures. Missing tools or guest artifacts fail explicitly.
Historical receipt hashes identify their actual local builds. Rebuilding in a
different directory can change a debug artifact hash without changing its source.

Accepted native and Window Fetch HTTP evidence does not complete WASI or
LeptosKit networking, destination policy, or installed-host resource accounting.
Full RPC,
subscriptions, wallets, other program clients, and generated desktop/Android/iOS
journeys remain required by the later phases. These experiment results cannot
substitute for those acceptance gates.
