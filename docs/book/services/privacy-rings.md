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

`ViewingKey::open_output` decrypts a confidential or ring-confidential payload and reads a proofless deposit. The slot index is the output's position in that event's full `outputSlots` list. `GetEncryptedUtxosByTags` can return only the matching outputs, so that result's order is not the slot index. Look the output's `tx_signature` up with `GetShieldedTransactionsBySignature`, find the matching `output_slot`, and use its position in that event's `output_slots`.

Confidential payloads use unauthenticated AES-CTR. `open_output` does not verify their Poseidon commitment. Check each decrypted note against a commitment you trust before passing it to `private_balances`. A proofless deposit is cleartext, and opening it does not check `ProoflessNote::owner`. Exclude deposits owned by someone else before aggregating.

`private_balances` adds amounts for notes whose `(tree_id, leaf_index)` the caller has not listed as spent. The same leaf counts once, and two different notes for one leaf are rejected. Confidential asset ids and proofless mints stay separate. Nullifier derivation needs Poseidon and is left to the caller.
