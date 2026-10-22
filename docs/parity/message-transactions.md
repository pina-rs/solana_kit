# Message and transaction contracts

This SDK is exclusively for off-chain client application code. Its portable
`no_std + alloc` core does not promise support for on-chain programs; use Pina
or official Solana program libraries for those.

The legacy/v0 slice preserves immutable signed message bytes and ordered signer
slots. Local construction requires explicit blockhash expiry metadata. External
wire decoding cannot recover that metadata and must retain an unresolved token.
Incoming messages have a separate protocol validation path: local compiler
restrictions must not reject otherwise valid received messages.

The pinned Dart source is commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. Its compiler sorts addresses by
base58 text within privilege groups, places the payer first, retains duplicate
instruction positions, and keeps invoked programs readonly and static.

The adjacent TSV contains all phase 3 source symbols, copied from the canonical
capability inventory. Pending rows are deliberate follow-ups, including v1's
message-first envelope, durable nonces, introspection, off-chain messages and
modifying, partial and sending signers. A legacy/v0 vertical slice does not
complete phase 3. No implementation or runtime proof is claimed by a pending row.
