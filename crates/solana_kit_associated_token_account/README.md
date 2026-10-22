# Solana Kit Associated Token Account

A handwritten, `no_std + alloc` off-chain client for all three ATA constructors,
exact positional metadata and full canonical parsing. The optional `crypto`
feature derives associated addresses through the existing Kit core PDA code.
On-chain programs should use Pina or official Solana program libraries.

The implementation follows handwritten Dart source
`0138bb956d2d81ccc0c56edc602401c0df2f2b46` and official ATA interface source
`0b867b5340cd001e5980d8ca7928effc4e10015c`. It is not generated. The finite
generator remains unchanged and cannot yet model these account contracts.

`AssociatedTokenInstruction` holds typed create, idempotent create and nested
recovery inputs. Builders preserve all account positions and duplicates. Parsing
checks the canonical ATA identity, exact byte length, tags, account count and
roles. The Dart low-level parsers only decode data and permit caller-supplied
mutable discriminators. Rust canonical parsing deliberately rejects those inputs.
System and token account identities remain caller-owned, as in the Dart source.
PDA seeds accept arbitrary token identities, including both exported Token and
Token-2022 constants, and never impose an owner-on-curve restriction.

`Limits` bounds instruction data bytes and account positions. PDA seed sizes
are fixed by the three typed addresses. No Token or Token-2022 account codecs,
validator acceptance, account-existence checks, transaction signing or complete
package parity is claimed. Publication remains disabled.
