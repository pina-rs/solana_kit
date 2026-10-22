# Version-one client envelope and validator proof

This verifies the owned public version-one client API alongside a private
independent wire oracle. It is exclusively for off-chain application code; use
Pina or official Solana program libraries for on-chain programs. The private
validator/oracle dependencies stay outside application normal/build graphs.

## Authorities

- Dart source: `0138bb956d2d81ccc0c56edc602401c0df2f2b46`, especially
  `solana_kit_transaction_messages/lib/src/codecs/message_codec.dart`,
  `v1_transaction_config.dart`, `compile_transaction_message.dart`, and
  `solana_kit_transactions/lib/src/codecs/transaction_codec.dart`.
- Dart's literal fixtures identify upstream `@solana/kit` 8.3.0. The three fixture
  envelopes (416, 3216, 248 bytes) are retained with the original MIT notice.
  The public fixture seed metadata is omitted. No private key file is committed.
- Official `solana-message` 4.4.1, repository commit
  `378addd5ec3da13f8382d73e191c985bbd5983e6`, `message/src/versions/v1/`.
  The private Cargo lock pins its complete oracle dependency closure.
- [SIMD-0385](https://github.com/solana-foundation/solana-improvement-documents/blob/27ff5a704391d121869dd3bc8cfa8127483329d0/proposals/0385-transaction-v1.md)
  and [SIMD-0296](https://github.com/solana-foundation/solana-improvement-documents/blob/27ff5a704391d121869dd3bc8cfa8127483329d0/proposals/0296-larger-transactions.md).

## Exact client wire contract

The signed message begins `0x81`, three header bytes, little-endian u32 config
mask, 32-byte lifetime token, u8 instruction count, u8 address count, addresses,
config values, all four-byte instruction headers, then all instruction payloads.
Each instruction header is program index, account-index count, little-endian u16
data length. Each payload is account indexes followed by data. The envelope ends
with exactly header-signature-count ordered 64-byte signatures, without a count
prefix. Signers approve every preceding message byte. No trailing data or padding
is permitted; no compact-u16 length appears in this format.

The inclusive 4096-byte cap includes signatures. Bounds are 12 signatures,
64 distinct addresses, 64 instructions and 255 account positions per instruction.
The fee payer is address zero and writable. A program cannot be address zero;
all indexes must reference the static address list. No ALT section exists.

Mask bits 0 and 1 are an atomic priority-fee u64 pair, followed by compute limit,
loaded-account byte limit and heap size u32 values in bits 2, 3 and 4 order.
Unknown bits and half-set priority pairs fail explicitly. Unset compute and loaded
limits mean zero; unset heap means 32 KiB. A supplied heap must be a KiB multiple
in 32–256 KiB. Priority fees are total lamports, not micro-lamports per CU.

The lifetime token preserves recent-blockhash/nonce wire semantics. Durable nonce
recognition still requires the first System advance-nonce instruction: the
cached official transaction 4.1.6 `uses_durable_nonce` inspects that first
instruction through the version-generic message interface. This experiment only
executes blockhash lifetimes; it does not prove durable-nonce creation, expiry,
advance, rollback or confirmation behavior.

## Executed local validator result

Installed `solana-test-validator` 4.2.0 reports source `ac82b5d4`, feature-set
`21b0d33a` (RPC decimal 565236538). All probe requests use loopback. Independent
inspection of this installed validator observed the RPC listener on `*:20899`
despite `--bind-address 127.0.0.1`; its CLI has no RPC-bind option. The bind flag
does not establish loopback-only RPC exposure. Each owned validator was stopped
after its isolated probe. No remote cluster,
funds, secrets or custom on-chain programs are used. Genesis funds a public
fixture payer; deterministic signatures use its published fixture seed.

| Probe                                                      | Observed result                                                                                                  |
| ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Feature enabled                                            | `txv1aq4pp281K9um3tnPgkfX8UqtFT6wcVW3hNezGLL` account active at slot 0                                           |
| Legacy 215-byte control                                    | Simulated, submitted, confirmed and read back; transfer 10,000,000 lamports, fee 5000                            |
| v1 228-byte transfer                                       | Simulated, submitted, confirmed and read back as version 1 with inline config                                    |
| v1 1864-byte transfer plus arbitrary ComputeBudget payload | Simulated, submitted, confirmed and read back; 300 CU; invalid ComputeBudget payload executes as specified no-op |
| v1 4096-byte envelope                                      | Simulated, submitted, confirmed and read back, including signature verification                                  |
| 4097-byte envelope                                         | RPC error -32602 explicitly states 4097 exceeds maximum 4096                                                     |
| Corrupt v1 signature                                       | Simulation returns `SignatureFailure`                                                                            |
| Feature disabled in a fresh genesis                        | Legacy control still confirms; v1 returns `UnsupportedVersion`, zero CU                                          |

`getVersion` identifies the node and feature-set hash; it is not a transaction
version advertisement. Check the actual feature/capability and handle explicit
unsupported-version errors. This local result does not assert any remote cluster's
activation state.

## Reproduction and limits

Run `cargo +1.99.1 test --manifest-path verification/v1-wire/Cargo.toml --locked`.
Five wire tests and a specific RPC-error negative control compare an independently assembled 138-byte literal envelope, all
three Dart fixtures and boundary cases against official Rust deserialization,
validation and exact serialization. They reject every truncated fixture prefix,
trailing bytes, wrong versions/masks, duplicates, invalid signer partitions and
packet overflow; exercise inclusive 12/64/64 maxima and all 16 valid low-bit masks.

Start a separate installed 4.2.0 test validator with isolated temporary ledger,
`--bind-address 127.0.0.1 --rpc-port 19899 --gossip-port 19898 --faucet-port 19902
--dynamic-port-range 20000-20100 --mint
9C6hybhQ6Aycep9jaUnP6uL9ZYvDjUp1aSkFWPUFJtpj`. Run the private binary with
`cargo +1.99.1 run --manifest-path verification/v1-wire/Cargo.toml --locked`.
For the negative control stop that process, use a fresh ledger and add
`--deactivate-feature txv1aq4pp281K9um3tnPgkfX8UqtFT6wcVW3hNezGLL`, then pass
`-- --expect-disabled`. Stop only the validator process you started.

The probe is a bounded local experiment using `curl` as its loopback HTTP driver,
not a public RPC transport. It does not establish browser/WASI/LeptosKit support,
wallet approval, multi-signer runtime behavior, QUIC ingress, mobile/device
journeys, durable nonce lifecycle, or complete Phase 3 parity. Its codec rejects unsupported versions;
callers cannot silently treat v1 as v0 or apply the old 1232-byte packet cap.

One source discrepancy must remain visible: the pinned Dart raw v1 config decoder
only reads known bits and does not reject unknown mask bits, whereas official
Rust 4.4.1 explicitly rejects them to preserve signed bytes. The public SDK and private oracle
follow the stricter official rule. The SIMD document still carries review/status
placeholder metadata; installed validator behavior is evidenced separately.

The validator binary constructs v1 messages through `Message::compile_v1` and
signs/encodes through `Transaction`; expected literal/official fixtures remain
independent. Its oversized-packet check requires code -32602 and the explicit
4097/max4096 size text. Unit controls reject internal errors, invalid base64 and
wrong-size errors as evidence for that boundary.
