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

The optional `plans` feature adds the pinned handwritten helper layer:
`create_mint_plan` builds an indivisible System `CreateAccount` then
`InitializeMint2` pair; `mint_to_ata_plan` and `transfer_to_ata_plan` build a
divisible ATA `CreateIdempotent` then `MintToChecked` or `TransferChecked` pair.
`mint_to_ata_derived_plan` and `transfer_to_ata_derived_plan` synchronously
replace the source's async ATA derivation. Configuration structs preserve the
source's optional token, System and ATA program overrides and apply each
configured identity to instruction metadata as well as the program address.
The mint and transfer authorities are required readonly signers, matching the
handwritten source helpers. Low-level builders remain available for explicit
multisignature account assembly.

The create-mint default uses the source's fixed 82-byte allocation and
`1_461_600` lamport rent-exempt value. That value is not a live rent lookup;
callers must pass `mint_account_lamports` when the target cluster requires a
different amount. The helpers select no signer, blockhash, executor, network or
transaction, and they add no balance, owner, delegate or rent validation.
The default dependency closure remains Token to core to bs58. Enabling `plans`
adds System and associated-token-account dependencies plus the existing ATA
crypto derivation.

Raw UTF8 keeps a leading BOM; the source Dart decoder removes it. Encoding bytes
match. Batch constructors retain the source's empty account-list contract;
arbitrary batch account assembly remains pending.

Use Pina or official Solana program libraries for on-chain programs.
