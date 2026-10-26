# Recurring-payments Subscriptions client

Build and parse all eighteen pinned Subscriptions program instructions with Kit-owned
types, complete account/event codecs and all seventy-eight error contracts. This is
the recurring-payments Solana program, distinct from RPC subscriptions.

Default use is `no_std + alloc` with core only. Enable `pda` for all six real PDA
recipes and typed account resolvers for every instruction. Resolver `None` selects
an explicitly declared fixed/PDA default; supplied addresses always override it.
Optional trailing sponsors remain optional. Explicit program overrides derive every
PDA against the overridden identity.

Instruction values expose `instruction`, `instruction_with_program`, `parse` and
`parse_with_program`. The `pda` feature additionally provides each instruction's
`...Accounts` input and its `instruction`/`instruction_with_program` methods.
Already fetched account helpers preserve core owner/allocation/metadata validation;
call `MaybeEncodedAccount::require` for a required fetched account before decoding.
The empty EventAuthority signing PDA has no allocated program-state owner contract;
its snapshot helper requires the caller's explicit expected owner.
RPC ownership, commitment and transport stay with the caller.

Fixed UTF-8 fields occupy exactly 128 bytes and retain null padding on decode.
Overlong text and malformed UTF-8 fail explicitly. All seven event headers include
the original eight-byte event domain and one-byte event selector. Raw codecs
preserve source discriminator/version fields; typed fetched-account helpers enforce
the concrete data account's discriminator.

Checked schedule and amount helpers accept explicit snapshots and timestamps. They
construct no hidden clock, mutation, transfer or transaction retry. This crate is
exclusively for off-chain application code; use Pina or official program libraries
for on-chain programs. Deployed transitions and installed device journeys require
separate evidence. See `THIRD_PARTY_NOTICES.md` for exact source dispositions.
