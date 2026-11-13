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
and Merkle paths, sized for the pinned circuit shapes. `indexed_proof_request`
builds the unresolved indexed body from the same witness. The `tee` feature
verifies an AWS Nitro attestation document and seals a prover request with
HPKE. `PrivacyProverClient::attest` runs the attestation round in one call: it
draws a fresh 32-byte nonce from the caller's `rand_core` generator for every
attempt, fetches `GET /attestation?nonce=`, and verifies the quote against the
caller's `NitroPolicy` using a `now_secs` callback sampled after the fetch
completes. `max_age_secs` and
`max_future_secs` (seconds) bound the signed document timestamp on both sides
of `now_secs`, and `VerifiedNitro::timestamp_ms` exposes that timestamp for
any later reuse decision. There is no attestation cache, and an entropy
failure from the generator returns an error before any request.
`fetch_unverified_attestation` returns the raw `GET /attestation?nonce=` body
for callers that pair their own fresh CSPRNG nonce with `verify_attestation`;
a raw fetch alone does not prevent a replay. Groth16 proving stays with the
prover. The built `transfer-2x2` witness matches the published fixture.

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
a header. These strings never contain one, and the client refuses any base
URL that carries a query, so a key can never ride in the origin. A remote
prover must be `https` and the caller must set
`allow_unauthenticated_remote_prover`; loopback does not.

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
AES-CTR, and opening a proofless deposit does not check its owner; keep
`open_output` results untrusted until they pass admission.

With `poseidon`, `verify_note` checks an expected owner and recomputes the
commitment with the pinned UTXO domain. For proofless notes it requires an
explicit trusted mint-to-asset-field resolver so the commitment uses the
registry asset field while the note keeps its displayed mint. It admits only
data-free confidential notes (no ring program id and no application data) and
data-free proofless notes whose trailing wire fields are the canonical empty
option tags; ring-, data-, or extension-bearing shapes fail closed.
`private_balances` sums only `VerifiedNote` values by verified commitment, so
one commitment counts once across leaf relabels and a spent commitment stays
excluded. It does not derive nullifiers and matching an indexer-supplied
commitment is not independent chain inclusion: callers must obtain trustworthy
commitment and tree context.

Program addresses match the v0.8.0 constants and the
[addresses guide](https://www.helius.dev/docs/privacy/addresses). RPC fields
match the [Privacy Rings API reference](https://www.helius.dev/docs/api-reference/privacy)
(OpenAPI 0.51.2). Decode admits each response against its request: every
history page serializes an explicit `paginationToken` (`null` ends the
history, and a missing key is a schema error), `minContextSlot` rejects a page
whose `context.slot` sits below the floor, the requested limit bounds
transaction groups, signature events must name the requested signature, proofs
bind positionally to the requested leaves and tree, registry records bind
count and owner, and queue pages stay ordered from the inclusive `startSeq`
with 32-byte base58 values. `UtxoPage` and `ShieldedPage` convert into `Page`,
and `collect_pages` walks them under a slot floor, a nondecreasing watermark,
and token-cycle rejection. For the queue, `next_start_seq` returns the next
checked cursor and `collect_nullifier_queue` walks full pages with a fixed
tree and limit. These checks bind what the provider declares — they do not
prove the indexer indexed honestly, and a multi-page walk is not an atomic
snapshot. `getNullifierQueueElements` pages the nullifier queue by tree,
`startSeq`, and a limit from 1 to 1000.
