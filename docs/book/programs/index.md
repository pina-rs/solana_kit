# Program clients

A program client is a crate that knows the wire format of one deployed Solana program. You use it from application code to build that program's instructions, check instructions you received, and decode its accounts. Every client uses Kit-owned types: `Address`, `AccountMeta`, `Instruction`, `EncodedAccount`, and the bounded codecs from `solana_kit_core`. That means the output goes straight into the message, signer, and RPC APIs described in the rest of this book.

<Callout kind="important" title="Off-chain only">

These crates are for off-chain application clients. Most support `no_std + alloc`, but that is for portability (browser, WASI, embedded WebAssembly). It does not mean they work on-chain. Inside on-chain programs, use Pina or the official Solana program libraries.

</Callout>

## What a client gives you

Most clients follow the same shape. Each part is optional per crate. The table later on this page shows what each crate has.

| Capability           | What it looks like                                                                                                                                                                                                                                |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Data codecs          | A plain struct per instruction or account (`TransferSol`, `Mint`), with a matching `…Codec` that implements the core `Encoder` and `Decoder` traits.                                                                                               |
| Instruction builders | `value.instruction(accounts, limits)` returns an `Instruction`. Generated clients also offer `instruction_with_program` for a custom deployment.                                                                                                   |
| Strict parsers       | `Type::parse(&instruction, limits)` and a crate-level `parse_instruction` check the program identity, the discriminator, the account count and privileges, and that no bytes are left over.                                                       |
| Account decoding     | `decode_*` functions take an already fetched `EncodedAccount` and return `Account<T>`. They check the owner and allocation and keep all metadata (lamports, rent epoch, space).                                                                    |
| PDAs and defaults    | `find_*` functions behind a `pda` (or `crypto`) feature derive program addresses with core's deterministic crypto. Some clients also fill fixed sysvars or declared defaults for you.                                                             |
| Errors               | A program error enum with `from_code(u32)`. Unknown codes come back as `Unknown(code)`, so no code is ever lost.                                                                                                                                  |

Nearly every operation takes an explicit `Limits` value from `solana_kit_core::codecs`. `Limits::new(max_bytes, max_items)` caps the bytes read and the number of items allocated. Malformed or hostile input fails with an error and never causes an unbounded allocation. See [Codecs](../core/codecs.md) for how the budgets work.

```rust
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_system::{
    ClientError, SystemInstructionData, TransferSol, TransferSolAccounts, parse_instruction,
};

fn round_trip() -> Result<(), ClientError> {
    let limits = Limits::new(1232, 32);
    let instruction = TransferSol { amount: 500 }.instruction(
        TransferSolAccounts {
            source: Address::from_bytes([1; 32]),
            destination: Address::from_bytes([2; 32]),
        },
        limits,
    )?;

    match parse_instruction(&instruction, limits)? {
        SystemInstructionData::TransferSol(transfer) => assert_eq!(transfer.amount, 500),
        other => panic!("unexpected System instruction: {other:?}"),
    }
    Ok(())
}
```

Clients never fetch accounts, sign, or send anything on their own. You fetch with [RPC](../core/rpc.md), pass the bytes to a `decode_*` function, and put the built instructions into a message (see [Messages and transactions](../core/transactions.md)). The exceptions are Token-2022's `client` feature and Bubblegum's `das` feature, which add typed fetch helpers that you drive with your own transport.

## Every program crate

"Portable" means the crate's contract tests also run in the shared suite on native Rust, Chrome, Wasmtime (WASI Preview 1 and 2), and the pinned LeptosKit interpreter. Validator execution and installed-app journeys are still pending for every crate in this table, so none of them has been tested against a live cluster.

| Crate                                                     | Program                               | Source           | Verification | Page                                                    |
| --------------------------------------------------------- | ------------------------------------- | ---------------- | ------------ | ------------------------------------------------------- |
| `solana_kit_system`                                       | System: 14 instructions, nonce state  | Generated + hand | Portable     | [System](system.md)                                     |
| `solana_kit_token`                                        | SPL Token: 28 instructions, 3 accounts | Generated        | Portable     | [Token and Token-2022](token.md)                        |
| `solana_kit_token_2022`                                   | Token-2022: 100 builders, extensions  | Generated + hand | Portable     | [Token and Token-2022](token.md)                        |
| `solana_kit_compute_budget`                               | Compute Budget: 5 layouts, helpers    | Generated + hand | Portable     | [Compute Budget, Memo, and ATA](small-programs.md)      |
| `solana_kit_memo`                                         | Memo v1, v3, v4                       | Handwritten      | Portable     | [Compute Budget, Memo, and ATA](small-programs.md)      |
| `solana_kit_associated_token_account`                     | Associated Token Account              | Handwritten      | Portable     | [Compute Budget, Memo, and ATA](small-programs.md)      |
| `solana_kit_stake`                                        | Stake: 17 instructions, activation math | Generated + hand | Portable     | [Stake, Loader, Config, and lookup tables](runtime-programs.md) |
| `solana_kit_loader`                                       | Upgradeable loader v3 and loader v4   | Generated + hand | Portable     | [Stake, Loader, Config, and lookup tables](runtime-programs.md) |
| `solana_kit_config`                                       | Config: Store and account layout      | Generated + hand | Portable     | [Stake, Loader, Config, and lookup tables](runtime-programs.md) |
| `solana_kit_address_lookup_table`                         | Address Lookup Table: 5 instructions  | Generated + hand | Portable     | [Stake, Loader, Config, and lookup tables](runtime-programs.md) |
| `solana_kit_mpl_core`                                     | MPL Core: 42 instructions, plugins    | Generated + hand | Portable     | [Metaplex](metaplex.md)                                 |
| `solana_kit_mpl_token_metadata`                           | Token Metadata: 58 operations         | Generated + hand | Portable     | [Metaplex](metaplex.md)                                 |
| `solana_kit_mpl_bubblegum`                                | Bubblegum: 36 instructions, cNFT helpers | Generated + hand | Portable     | [Metaplex](metaplex.md)                                 |
| `solana_kit_account_compression`                          | SPL Account Compression: 7 instructions | Generated + hand | Portable     | [Metaplex](metaplex.md)                                 |
| `solana_kit_anchor`                                       | Any Anchor program, from its IDL      | Runtime coder    | Portable     | [Anchor](anchor.md)                                     |
| `solana_kit_squads`                                       | Squads V4 multisig: 36 instructions   | Generated + hand | Portable     | [More program clients](more.md)                         |
| `solana_kit_sns`                                          | Solana Name Service (offline)         | Handwritten      | Native       | [More program clients](more.md)                         |
| `solana_kit_subscriptions`                                | Recurring-payments Subscriptions      | Generated + hand | Portable     | [More program clients](more.md)                         |
| `solana_kit_attestation_service`                          | Solana Attestation Service            | Generated + hand | Portable     | [More program clients](more.md)                         |

<Callout kind="warning" title="Not on crates.io">

Publishing is turned off for every program crate. Add them as git dependencies from the repository. See [Installation](../getting-started/installation.md) for the full setup.

</Callout>

```toml
[dependencies]
solana_kit_core = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_system = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_token = { git = "https://github.com/pina-rs/solana_kit" }
```

## Generated and handwritten clients

Most data codecs, builders, and parsers are generated. The private generator in [`tools/codama-renderer`](https://github.com/pina-rs/solana_kit/tree/main/tools/codama-renderer) reads each program's pinned Codama IDL and lowers it through one shared typed wire model and emitter. Each program has its own command, such as `system`, `token`, `token-2022-data`, `compute-budget`, `alt-config`, `stake-loader-data`, `mpl-core-generate`, `token-metadata-generate`, `bubblegum-generate`, `compression-generate`, `squads-generate`, `subscriptions-generate`, and `attestation-generate`. Generated files are committed and start with a `// Generated from …` header. Drift checks regenerate them and require byte-identical output, so do not edit them by hand.

The generator is not a general Codama renderer. It supports the node shapes these programs use and rejects anything else before it writes output. Behavior the IDL cannot express is handwritten next to the generated module, for example:

- System nonce accounts. The flat IDL cannot describe uninitialized allocated storage.
- Token-2022 account dispatch, extension validation, and pre-initialization helpers.
- Compute Budget's instruction-list helpers, Stake's activation and withdrawal math, and Loader's write-then-deploy plans.
- Memo and Associated Token Account, which are fully handwritten. The generator cannot yet model untagged remainder layouts or their account contracts.
- SNS, which has no instruction builders. It covers derivation and data parsing only.

`solana_kit_anchor` is the third kind of client. Instead of generated Rust, it parses an Anchor IDL at runtime and encodes and decodes values dynamically.

The reproducible input ledger is [`docs/parity/program-inputs.tsv`](https://github.com/pina-rs/solana_kit/tree/main/docs/parity/program-inputs.tsv). It records each program's repository, revision, input path, checksum, and license. The generator design notes are in [`docs/verification/generator-design.md`](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/generator-design.md).

## Conventions that differ from the Dart SDK

These clients port the Dart `solana_kit` program packages, but they choose strict, explicit behavior wherever the Dart code was permissive:

- Parsers check the whole invocation, not only the data bytes. A wrong program address, a missing signer, or trailing bytes is an error.
- No signer, payer, or funding amount is invented. Authorities and optional accounts are inputs you pass in.
- Generated clients that take raw `AccountMeta` lists with optional roles (MPL Core, Token Metadata, Squads) use a readonly program-ID sentinel for an omitted optional account. Clients with typed `…Accounts` structs use `Option<Address>`.
- Custom deployments are explicit, through `instruction_with_program`, `parse_with_program`, or a `program` argument.

Each crate's README and `THIRD_PARTY_NOTICES.md` list the exact differences. [Parity and status](../reference/parity.md) explains how claims are tracked.
