# Privacy Rings

`solana_kit_privacy_rings` builds Helius Solana Privacy Rings instructions and typed indexer calls on Kit addresses, instructions, and the shared RPC client. It is a native port of the `zolana` v0.8.0 layouts. It does not wrap `zolana-client` or `solana-sdk`.

> [!NOTE]
> The crate is unpublished (`publish = false`). Tests check instruction bytes, account order, and recorded API examples. They make no live Helius calls and do not prove a note is unspent.

## Add the crate

```toml
[dependencies]
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_core = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_privacy_rings = { git = "https://github.com/pina-rs/solana_kit" }
```

The `crypto` feature adds viewing-key decryption and the shielded-pool PDA helpers. It uses the same `solana_kit_core/crypto` address derivation as the rest of the SDK.

## Instructions

`Deposit` shields SOL or up to five SPL mints into the shielded pool (tag 15). `ConfidentialTransfer` moves value between private notes. `Withdraw` unshields to a public system or token account. Transfer and withdraw are both the `transact` instruction (tag 16). The caller supplies the Groth16 proof and signs the fee payer and every owner account. This crate never signs and never sends.

`Cluster::indexer_url` and `local_indexer_url` return the documented path for one method. The transport adds the API key. Origins in this crate do not contain one.

## Indexer methods

Pass any of these to `Client::call`. The method name is the last path segment.

| Method | Response |
| --- | --- |
| `GetEncryptedUtxosByTags` | Matching encrypted outputs |
| `GetShieldedTransactionsByTags` | Shielded history for view tags |
| `GetShieldedTransactionsBySignature` | Events in one Solana transaction |
| `GetShieldedTransactionsByNullifiers` | Shielded history for nullifiers |
| `GetMerkleProofs` | State-tree inclusion proofs |
| `GetNonInclusionProofs` | Nullifier absence proofs |
| `GetUserRecords` | Registry viewing and nullifier keys |

`getNullifierQueueElements` appears in the API overview without a published schema. It is not implemented.

## Balances

`ViewingKey::open_output` decrypts a confidential or ring-confidential payload and reads a proofless deposit. The slot index is the output's position in that event. `private_balances` adds amounts for notes whose `(tree_id, leaf_index)` the caller has not listed as spent. Confidential asset ids and proofless mints stay separate. Nullifier derivation needs Poseidon and is left to the caller.
