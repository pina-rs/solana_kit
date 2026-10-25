# MPL Token Metadata client

This crate builds and parses MPL Token Metadata instructions, decodes fetched
account snapshots and derives deterministic program addresses for off-chain
applications. It uses Kit-owned `Address`, `Instruction`, account and codec types.
On-chain programs should use Pina or official Solana program libraries.

The source draft models all58 operations,14 accounts,45 named types and203 errors
from the pinned MIT Dart package. The maintained `token-metadata-generate OUTPUT`
command lowers those contracts through the shared typed renderer. The generated
module retains explicit account-key framing and uses the same Kit-owned codecs
as the handwritten account and instruction helpers.

Builders take ordered core account metadata and explicit caller limits.
`instruction` selects the canonical program. `instruction_with_program` and the
matching parsers use an explicit program identity. Modern optional accounts use
readonly program-ID sentinels. Legacy optional tails are omitted as a prefix of
their declared optional group. Arbitrary proof or remaining accounts are not
invented for source operations that declare none.

Account keys become fixed codec framing rather than mutable struct fields.
`AllocatedCodec` composes with the existing typed Client account helpers.
`decode_metadata` and the other named snapshot decoders preserve core owner and
allocation checks. The caller explicitly chooses exact consumption, retained zero
padding or uninterpreted future tails. Missing fields of a shorter historical
version are errors; the parser does not silently manufacture optional values.
Strings retain their exact UTF-8 bytes, including puffed NULs. `unpuffed` gives a
presentation view without changing the wire value.

The primary metadata allocation can contain a nonzero final fee flag. Choose
`PreserveTail` to retain that byte. `ZeroPadding` deliberately rejects it.
No shorter historical account-version compatibility is inferred from this model.

The optional `pda` feature enables all ten pinned PDA recipes. Explicit-program
variants change both the PDA program and the program bytes inside its seeds.
Edition markers use decimal bytes of `floor(edition /248)`. Authorization payloads
use ordered Rust pairs to retain wire order. `authorization_payload` rejects
duplicate keys. Its encoded order is caller controlled.

```rust
use solana_kit_core::{Address, AccountMeta, AccountRole, codecs::Limits};
use solana_kit_mpl_token_metadata::SignMetadata;

let accounts = [
    AccountMeta { address: Address::from_bytes([7;32]), role: AccountRole::Writable },
    AccountMeta { address: Address::from_bytes([9;32]), role: AccountRole::ReadonlySigner },
];
let instruction = SignMetadata {}.instruction(&accounts, Limits::new(1024,16))?;
assert_eq!(instruction.data, [7]);
# Ok::<(), solana_kit_core::codecs::Error>(())
```

Transports, runtimes, authorization-rule state, lookup/account resolution, signing
and submission belong to the caller. Legacy and deprecated operations retain the
pinned source wire contracts; source presence does not promise validator support.
All1896 canonical source IDs remain pending independent semantic review and actual
behavioral verification. Cargo and Monochange implementation publishing are disabled.
