# solana_kit_token

Generated off-chain SPL Token data codecs and explicit account-meta builders.
The source input is pinned at `31646dce3df0a03d468a14913f01b9fd3c3adad0`.
Every instruction accepts ordered account metas, checked against the IDL roles.
Caller-selected authorities can remain nonsigners when remaining signers apply.
Custom program identities are explicit through `instruction_with_program` and
`parse_with_program`. Caller-supplied defaults are always explicit addresses.
`SyncNative` permits its trailing rent account to be omitted.

Data decoding is strict and bounded. Mint, Token and Multisig codecs consume
82, 165 and 355 bytes respectively. Fixed COption absent payload bytes are ignored
on decode and zeroed on encoding. Unknown program error codes remain lossless.

The original Dart builders force authority signers and do not expose remaining
multisigners. This client follows the pinned IDL's explicit caller-selected roles.
Dart's mutable discriminator/data-only parsers, fetch helpers and wrapper contracts
are not claimed as equivalent. The additional ATA owner stays in its separate
accepted crate. Validator and portable runtime execution remain separate gates.

Raw UTF8 keeps a leading BOM; the source Dart decoder removes it. Encoding bytes
match. Batch constructors retain the source's empty account-list contract;
arbitrary batch account assembly remains pending.

Use Pina or official Solana program libraries for on-chain programs.
