# Metaplex

Four crates cover the Metaplex NFT stack and the compression program it relies on. `solana_kit_mpl_core` handles MPL Core assets and collections. `solana_kit_mpl_token_metadata` handles classic Token Metadata. `solana_kit_mpl_bubblegum` handles Bubblegum compressed NFTs, and `solana_kit_account_compression` handles the SPL Account Compression program underneath them. All four are generated from pinned source models, are `no_std + alloc`, and depend on `solana_kit_core` by default.

## Shared pattern

These builders take an ordered slice of `AccountMeta`, like SPL Token. Every instruction value has `instruction`, `instruction_with_program`, `parse`, and `parse_with_program`. The builder checks the account count and each account's role. To leave out an optional account, pass the program address with a readonly role (the program-ID sentinel). Account decoders take an already fetched `EncodedAccount` and keep the core owner and allocation checks. Each one has a `_with_program` variant for custom deployments. PDA helpers are behind the `pda` feature.

```toml
[dependencies]
solana_kit_mpl_core = { git = "https://github.com/pina-rs/solana_kit", features = ["pda"] }
solana_kit_mpl_token_metadata = { git = "https://github.com/pina-rs/solana_kit", features = ["pda"] }
```

## MPL Core

The crate models all 42 instructions, the six account types, plugins and external plugin adapters, and 57 errors (`MplCoreError`) from the pinned Dart client.

```rust
use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::{Error, Limits}};
use solana_kit_mpl_core::{CreateV1, DataState, PROGRAM};

fn create_asset(asset: Address, payer: Address) -> Result<(), Error> {
    let limits = Limits::new(4096, 64);
    let signer = |address| AccountMeta { address, role: AccountRole::WritableSigner };
    let omitted = AccountMeta { address: PROGRAM, role: AccountRole::Readonly };
    let system_program = AccountMeta { address: Address::from_bytes([0; 32]), role: AccountRole::Readonly };

    let accounts = [
        signer(asset),  // asset
        omitted,        // collection
        omitted,        // authority
        signer(payer),  // payer
        omitted,        // owner
        omitted,        // update_authority
        system_program, // system_program
        omitted,        // log_wrapper
    ];
    let create = CreateV1 {
        data_state: DataState::AccountState,
        name: "Asset".into(),
        uri: "https://example.com/asset.json".into(),
        plugins: None,
    };
    let instruction = create.instruction(&accounts, limits)?;
    assert_eq!(CreateV1::parse(&instruction, limits)?, create);
    Ok(())
}
```

Account decoders are `decode_asset_v1`, `decode_collection_v1`, `decode_group_v1`, `decode_hashed_asset_v1`, `decode_plugin_header_v1`, and `decode_plugin_registry_v1`. Each returns `Account<AccountData<T>>`, where `base` is the decoded account and `extension_bytes` keeps the trailing plugin bytes uninterpreted. To read plugins, call `inspect_asset_plugins` or `inspect_collection_plugins`, which validate the full plugin registry. Unknown typed discriminators are rejected.

With `pda`, you get `find_asset_signer`, `find_preconfigured_program`, `find_preconfigured`, `find_oracle_account`, and `derive_extra_account`.

## Token Metadata

The crate models all 58 operations, 14 accounts, 45 named types, and 203 errors (`MplTokenMetadataError`). Optional accounts in modern instructions use the program-ID sentinel. In legacy instructions, you leave optional trailing accounts off the end.

```rust
use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::Limits};
use solana_kit_mpl_token_metadata::{SignMetadata, find_metadata};

fn sign_metadata(mint: Address, creator: Address) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = find_metadata(mint)?.address();
    let accounts = [
        AccountMeta { address: metadata, role: AccountRole::Writable },
        AccountMeta { address: creator, role: AccountRole::ReadonlySigner },
    ];
    let instruction = SignMetadata {}.instruction(&accounts, Limits::new(1024, 16))?;
    assert_eq!(instruction.data, [7]);
    Ok(())
}
```

### Decoding metadata accounts

Token Metadata accounts often have more bytes than their encoded value. You pick how to handle the extra bytes with an `AllocationPolicy`:

| Policy         | Trailing bytes                                                          |
| -------------- | ----------------------------------------------------------------------- |
| `Exact`        | Rejected.                                                               |
| `ZeroPadding`  | Accepted only if all zero, and kept byte for byte.                      |
| `PreserveTail` | Kept as they are, without interpretation. Use this for `Metadata`, whose last byte can be a nonzero fee flag. |

`decode_metadata(account, policy, limits)` returns `Account<Allocated<Metadata>>`. Other decoders include `decode_edition`, `decode_master_edition_v1`, `decode_master_edition_v2`, and `decode_reservation_list_v1`. `AllocatedCodec` wraps the same codecs for typed RPC account helpers. Missing fields from an older, shorter account version are an error, never a made-up default.

Strings keep their exact bytes, including the NUL padding ("puffing") the program writes. `unpuffed` gives you a trimmed view for display.

### PDAs

With `pda`, the ten pinned recipes are available, each with a `_with_program` variant: `find_metadata`, `find_master_edition`, `find_edition_marker`, `find_edition_marker_v2`, `find_token_record`, `find_use_authority_record`, `find_collection_authority_record`, `find_holder_delegate_record`, `find_metadata_delegate_record`, and `find_program_as_burner`. `find_edition_marker` uses `edition / 248` (rounded down) as a decimal seed, as the program does.

## Bubblegum

The crate models 36 Bubblegum instructions and the `TreeConfig` and `Voucher` accounts (`decode_tree_config`, `decode_voucher`). It also has several optional features:

| Feature   | Adds                                                                                                            |
| --------- | --------------------------------------------------------------------------------------------------------------- |
| `pda`     | `find_tree_authority`, `find_leaf_asset_id`, `find_voucher`, `find_bubblegum_signer`, `find_mpl_core_cpi_signer`, `LeafAccounts::derive`, and mint builders. |
| `hashing` | Keccak-256 leaf and metadata hashing (`hash_leaf_v1`, `hash_metadata`, and others), `MerkleTree`, and `verify_proof`. |
| `tree`    | `TreeAccounts` with `create_v1` and `create_v2`, using Account Compression's tree sizing.                        |
| `das`     | `DasApi`, `get_asset_with_proof`, `bind_asset_proof`, and `HeliusDasClient` over your own `solana_kit_client` transport. |

`LeafAccounts` builds leaf operations: `burn`, `transfer`, and `delegate`. For `burn` and `transfer` you choose who signs with `LeafAuthority`, the owner or the delegate. `delegate` always signs as the owner; a delegate cannot re-delegate. For all three you supply the proof nodes, which are appended as readonly, non-signer accounts.

```rust
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_mpl_bubblegum::{LeafAccounts, LeafAuthority, Node, Transfer};

fn transfer_cnft(
    tree: Address,
    owner: Address,
    recipient: Address,
    leaf: Transfer,
    proof: &[Node],
) -> Result<(), Box<dyn std::error::Error>> {
    let accounts = LeafAccounts::derive(tree, owner, None)?;
    let instruction = accounts.transfer(&leaf, recipient, LeafAuthority::Owner, proof, Limits::new(1232, 64))?;
    let _ = instruction;
    Ok(())
}
```

`Transfer` holds the `root`, `data_hash`, `creator_hash`, `nonce`, and `index` of the leaf. Usually you get these from a DAS provider. `AssetProof::remaining_accounts(depth, canopy_depth, limits)` trims the part of the proof that is already stored in the tree's canopy.

<Callout kind="warning" title="DAS data is not authenticated">

`bind_asset_proof` checks that the asset and proof belong to the identities you asked for. With `hashing`, it also checks that the proof path leads to the root the provider returned. It does not prove that root is the tree's current on-chain root. The program checks that when the transaction runs.

</Callout>

Where the Dart client's layouts were incompatible with the program, the Rust model corrects them and does not emit missing required values. For example, V1 metadata updates require a root, nonce, index, current metadata, and update arguments. The full list is in the crate's `THIRD_PARTY_NOTICES.md`.

## Account Compression

`solana_kit_account_compression` builds and parses all seven SPL Account Compression instructions: `InitEmptyMerkleTree`, `ReplaceLeaf`, `TransferAuthority`, `VerifyLeaf`, `Append`, `InsertOrAppend`, and `CloseEmptyTree`. It also has the ten header and event codecs, and `SplAccountCompressionError`. `modification_accounts(tree, authority)` returns the standard three accounts, using `NOOP_PROGRAM` as the log wrapper.

```rust
use solana_kit_account_compression::{Append, account_size, is_valid_depth_buffer_pair, modification_accounts};
use solana_kit_core::{Address, codecs::{Error, Limits}};

fn append_leaf(tree: Address, authority: Address) -> Result<(), Error> {
    let accounts = modification_accounts(tree, authority);
    let instruction = Append { leaf: vec![7; 32] }.instruction(&accounts, Limits::new(256, 64))?;
    let _ = instruction;

    assert!(is_valid_depth_buffer_pair(14, 64));
    let bytes = account_size(14, 64, 0)?; // depth, buffer size, canopy depth
    let _ = bytes;
    Ok(())
}
```

`account_size`, `canopy_size`, `canopy_depth`, and `VALID_DEPTH_BUFFER_PAIRS` do the tree allocation math with overflow checks. Building an instruction does not check that a tree exists or what state it is in.

## Status

The public contract bodies of all four crates, with the `pda`, `hashing`, `tree`, and `das` features enabled, run on native Rust, Chrome, both WASI previews, and the LeptosKit interpreter. DAS runs against deterministic in-memory replies. Still pending:

- Validator transitions, any real DAS provider, and installed-app journeys.
- Independent semantic review of the Token Metadata model. All 1,896 of its canonical source IDs are pending review.

Publishing is disabled.
