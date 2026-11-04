# Current installed macOS wallet execution

On 2026-11-04, the generated macOS wallet completed three real process sessions through the existing
computer-use accessibility and screenshot connection. No system security setting changed. This is
functional native UI acceptance, not a successful Mac2/XCTest controller run.

The public generator/framework pin was `7437097c52aa68a3401bd378764e051ed4c9165a`. The ordinary generated
bundle identity was `com.leptoskit.solanakitwalletmacos20261104`. Its composed wallet runtime was built
in this session from SDK source `65c502f9044d320ec809fc0b45670b7860a8b4e3`. Later SDK additions are not
retroactively attributed to that binary. The exercised wallet, signer, native, subscription and
confirmation implementations did not change before this run; the later optional parsed-account and
fixed-point additions have separate execution receipts.

The native executable SHA-256 was
`a803efa91a06a82d477d0f6fc8ee47a42d6d82ee0081998935b3a927329a2e49`; its embedded `app.wasm` was
`44b77f227264617350839d5571a792a25a444aa7bf7af925837c8d81773fd323`.

## Owned ledger and actual UI behavior

A local Rust companion reused the installed controller's `Validator`, numeric-loopback `Rpc`, port
reservation and independent transaction-effect verifier. It created fresh application data and an
owned Agave 4.2.0 ledger, challenged the live node identity, and wrote the explicit HTTP/WebSocket
bootstrap grants. It accepted public addresses and bounded observation commands on stdin. All app
launches, clicks, approval/rejection, screenshots and quits used computer use. The companion never
read wallet private files or submitted a signed wallet transaction itself.

The initial process created a fresh native wallet and made its private copy inside the app. One
2,000,000,000-lamport airdrop funded the visible public address. **Read balance** displayed that exact
amount. The first transfer review was rejected; the independent observer required unchanged payer
and recipient balances and no recipient transaction.

A separate review approved a 1,000,000-lamport System transfer. The independent observer required one
successful signature, the exact payer/recipient/System account list, the literal transfer bytes and
matching before/after balances. The payer retained 1,998,995,000 lamports, the recipient received
1,000,000, and the fee was 5,000. The app displayed the same confirmed signature and post-transfer
balance. The transaction signature was
`22JkjFaN5mvuK1fyuVFsevUMFaYmyfVabzbt9C8iSpXHNhApNuFqPG5WgH2Lo5hsPxVuw4d2LWuuhP4Tp5Uye7C4`.

The subscription probe used a separate account and six single airdrop requests across two sessions.
Each session rendered the first actual account update, held the second, acknowledged cancellation,
and ignored the released held callback. A third credit changed the ledger while the cancelled display
remained frozen. **Observe later socket activity** fetched a later confirmed balance through the SDK
and observed a strictly later slot on the continuing socket.

| Session               | Frozen display | Independent late balance | Late credit slot | SDK HTTP context | Later socket slot |
| --------------------- | -------------: | -----------------------: | ---------------: | ---------------: | ----------------: |
| Initial               |      1,000,000 |                3,000,000 |              201 |              208 |               210 |
| Saved-wallet relaunch |      4,000,000 |                6,000,000 |              473 |              483 |               485 |

Both sessions displayed **Held update ignored after cancellation**, then **Same-socket slot activity
observed**, and finally **Subscription socket closed**. Screenshots showed the actual rendered
balances and activity values. Neither subscription session changed either wallet's ledger balance.

## Persistence, relaunch and cleanup

The first wallet was `7t7SC1QdnQ71Pye85KPXjEd1jaccwomdtBN4Vyw2ZaAx`. After disconnecting, the app created
and explicitly saved a distinct second wallet, `8hzyZCcaySre4JiyPUsAsELYfEeKeQ7ggvEk1W9DvgZS`.

The initial app process, PID 88337, quit and was confirmed absent before relaunch. **Load saved wallet**
in the new process recovered the second address and its independently observed zero balance. A
reviewed message signature passed the guest's signature verification. The subscription sequence above
then repeated. That process, PID 90848, also quit and was confirmed absent.

The third process selected **Restore private copy** and recovered the first address rather than the
saved second address. Its **Read balance** remained exactly 1,998,995,000 lamports, matching the
retained first-session baseline and the independent ledger read. Another reviewed message signature
verified; both wallets' balances remained unchanged. Explicit disconnect returned the selection
controls. The third process, PID 92000, quit and was confirmed absent before cleanup.

The companion exited 0 after closing/reaping its validator and removing its own ledger and fresh
application data. No validator or tested app process remained. Its retained startup receipt contained
all six expected phases from `factory-entered` through `render-pending`.

Public observations and the startup receipt remain locally in
`/tmp/solana-kit-wallet-macos-cua-run-20261104-2`. The observations JSONL SHA-256 is
`e60b14e5668f68c7a6507db835ebb173b80db82fe89d8b05a00f10746e758e64`. The temporary Rust companion source is
retained at `/tmp/solana-kit-wallet-cua-controller.RgAHoQ/src/main.rs`, SHA-256
`4aba24d54029b34559f183d720d8c7d2dca76b1896a5c7d98875b5c673886967`. Its completed 403 MiB Cargo target
was cleaned. No wallet private file or ledger keypair was retained as evidence.

For an automated reproduction on a machine with working Mac2 authorization, use the maintained
[installed controller](../../verification/leptoskit-app-journey/installed/README.md). This run instead
used its ledger verification alongside the same visible UI steps through computer use. It establishes
those observed wallet behaviors, not Mac2 availability, Android acceptance, a rebuilt latest-commit
binary, physical-device support or production-network behavior.
