# Solana Kit Privacy Rings

Helius Solana Privacy Rings for off-chain Kit applications. Instructions use
`solana_kit_core::Instruction` and `Address`. Indexer methods use
`solana_kit_client::Client`. The owner signs with `solana_kit_signers`. This
crate has no HTTP stack, no API key, and no signer.

The wire format is a native port of Helius `zolana` tag `v0.8.0`
(`6660d9839714`). It does not depend on `zolana-client` or `solana-sdk`.
Serialization follows `wincode` 0.5 field order, checked against golden bytes
from that serializer. X25519, HKDF-SHA256, and AES-256-CTR sit behind the
`crypto` feature. The `poseidon` feature adds Circom BN254 commitments,
`owner_hash`, nullifiers, the blinding-seed derivations, and
`confidential_transfer_witness`, which builds the gnark witness from notes
and Merkle paths. The `tee` feature verifies an AWS Nitro attestation
document and seals a prover request with HPKE. Groth16 proving stays with
the prover. The built `transfer-2x2` witness matches the published fixture.

```rust
use solana_kit_core::Address;
use solana_kit_privacy_rings::{AssetDeposit, Deposit, DepositAsset, Error};

fn shield(tree: Address, depositor: Address) -> Result<(), Error> {
    let instruction = Deposit {
        tree,
        depositor,
        deposits: vec![AssetDeposit {
            asset: DepositAsset::Sol,
            view_tag: depositor.to_bytes(),
            owner: [4; 32],
            amount: 1,
            memo: None,
        }],
    }
    .instruction()?;
    assert_eq!(instruction.data[0], solana_kit_privacy_rings::DEPOSIT_TAG);
    Ok(())
}
```

`Deposit::instruction` shields SOL or SPL. `ConfidentialTransfer::instruction`
spends private notes with no public amount. `Withdraw::instruction` adds one
SOL or SPL withdrawal leg. Both spends are the shielded-pool `transact`
instruction (tag 16).

Indexer calls take a URL from `Cluster::indexer_url` or `local_indexer_url`.
The path ends in the method name. `PrivacyProverClient` posts a witness to
`Cluster::prover_url` or `local_prover_url`. The transport adds the API key as
a header. These strings never contain one. A remote prover must be `https` and
the caller must set `allow_unauthenticated_remote_prover`; loopback does not.

```rust
use solana_kit_client::methods::Method;
use solana_kit_privacy_rings::{Cluster, GetEncryptedUtxosByTags, HistoryFilters, SlotBounds};

fn utxo_query() -> Result<String, solana_kit_privacy_rings::Error> {
    let request = GetEncryptedUtxosByTags {
        tags: vec!["5Vt1Mnq2b9ynV1EMXS564i5ZDuNudxm8ZG7yCXfH55M".to_owned()],
        filters: Some(HistoryFilters {
            slot: Some(SlotBounds {
                gte: Some(467025494),
                lte: Some(467025494),
                ..SlotBounds::default()
            }),
        }),
        limit: Some(1),
        min_context_slot: None,
        pagination_token: None,
        ring_program_id: None,
    };
    request.validate()?;
    Cluster::Mainnet.indexer_url(GetEncryptedUtxosByTags::NAME)
}
```

With `crypto`, `ViewingKey` opens confidential and proofless indexer payloads.
The decryption slot is the output's position in the full event. A tag query
can omit outputs, so resolve that position through
`GetShieldedTransactionsBySignature`. Confidential ciphertext is unauthenticated
AES-CTR, and opening a proofless deposit does not check its owner.
`private_balances` sums distinct unspent leaves. It does not verify commitments
or owners, and it does not derive nullifiers.

Program addresses match the v0.8.0 constants and the
[addresses guide](https://www.helius.dev/docs/privacy/addresses). RPC fields
match the [Privacy Rings API reference](https://www.helius.dev/docs/api-reference/privacy)
(OpenAPI 0.51.2). `getNullifierQueueElements` pages the nullifier queue by
tree, `startSeq`, and a limit from 1 to 1000.
