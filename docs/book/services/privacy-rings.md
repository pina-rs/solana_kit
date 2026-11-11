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

`Cluster::indexer_url` and `local_indexer_url` return the documented path for one method. `Cluster::prover_url` and `local_prover_url` are the prover bases. The transport adds the API key as a header. These strings do not contain one.

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

## Prover

`PrivacyProverClient` posts a witness you already built to `POST /prove/{circuit}` with `X-Sync: true`. A `429` is sent again with `X-Async: true`, and a `jobId` is polled at `/prove/{circuit}/status`. The gnark proof comes back with proof `A` negated and compressed into the 32-byte `a`, 128-byte `b`, and 32-byte `c` that `TransactBody::with_proof` stores.

The client is generic over `solana_kit_client::rest::Transport<ProverDestination>`. The transport sets `X-Sync` or `X-Async` from `ProverDestination::delivery` and sends an API key as a header. The client rejects a URL that already contains `api-key`, and it does not write the key into a URL or an error.

A prover on `127.0.0.1`, `localhost`, `*.localhost`, or `::1` accepts the JSON body directly. Any other host requires `allow_unauthenticated_remote_prover(true)` and an `https` URL. The TypeScript client encrypts that witness to an attested TEE session. This crate does not implement that session, so the flag sends the witness in the clear.

The pinned proving-key SHA-256 must match `provingKeySha256` in the response. Poseidon witness assembly is not included. A proof is only as good as the witness you pass in.

### Running a prover locally

Helius publishes the prover and the circuit artifacts:

- The prover binary is the `prover-linux-x64` asset on the `cli-v0.10.0` release of `helius-labs/privacy-rings` (version `2.0.7`). Its SHA-256 is in that release's `SHA256SUMS`.
- The gnark circuits and the witness assembler are in that repository, under `prover/server` and `sdk-libs/client/src/prover`.
- Proving keys are public at `https://d3gbdb0egjwcw9.cloudfront.net/` plus the `prefix` in `prover/server/prover/provingkeys/proving-keys.lock`. `transfer_confidential_1_2.key` is 9,268,304 bytes and matches pin `c0b5c1a1081136911a6ad5496fb989ed26d85b09e983a4d5048cff96e0833d85`. The full set is several gigabytes, and one batch key is about 74 GB.

This repository has no devenv or Nix setup, so the prover stays an external binary:

```bash
curl -fsSL -o prover \
  https://github.com/helius-labs/privacy-rings/releases/download/cli-v0.10.0/prover-linux-x64-cli-v0.10.0
chmod +x prover
mkdir -p proving-keys
curl -fsSL -o proving-keys/transfer_confidential_1_2.key \
  https://d3gbdb0egjwcw9.cloudfront.net/proving-keys/78751f5d1c591777/transfer_confidential_1_2.key
./prover start \
  --prover-address 127.0.0.1:3001 \
  --metrics-address 127.0.0.1:9997 \
  --keys-dir proving-keys \
  --serve transfer_confidential_1_2 \
  --server-only
```

Point `PRIVACY_RINGS_LOCAL_PROVER_URL` at `http://127.0.0.1:3001` and run the ignored test. Loopback does not need the unauthenticated opt-in. A valid proof still needs a witness from the upstream assembler. An empty witness gets a `400` from this binary, which shows the client reached it.

The hosted mainnet and devnet provers answer `401` without a key. Set `HELIUS_PRIVACY_RINGS_LIVE=1` and `HELIUS_API_KEY` for the ignored live test. The test sends the key as an `x-api-key` header and never puts it in the URL.

Blueshift's XARK stack is a separate Arkworks Groth16 toolchain. It does not prove these gnark circuits and it does not publish these keys.

## Balances

`ViewingKey::open_output` decrypts a confidential or ring-confidential payload and reads a proofless deposit. The slot index is the output's position in that event's full `outputSlots` list. `GetEncryptedUtxosByTags` can return only the matching outputs, so that result's order is not the slot index. Look the output's `tx_signature` up with `GetShieldedTransactionsBySignature`, find the matching `output_slot`, and use its position in that event's `output_slots`.

Confidential payloads use unauthenticated AES-CTR. `open_output` does not verify their Poseidon commitment. Check each decrypted note against a commitment you trust before passing it to `private_balances`. A proofless deposit is cleartext, and opening it does not check `ProoflessNote::owner`. Exclude deposits owned by someone else before aggregating.

`private_balances` adds amounts for notes whose `(tree_id, leaf_index)` the caller has not listed as spent. The same leaf counts once, and two different notes for one leaf are rejected. Confidential asset ids and proofless mints stay separate. Nullifier derivation needs Poseidon and is left to the caller.
