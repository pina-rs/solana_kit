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

| Method                                | Response                                              |
| ------------------------------------- | ----------------------------------------------------- |
| `GetEncryptedUtxosByTags`             | Matching encrypted outputs                            |
| `GetShieldedTransactionsByTags`       | Shielded history for view tags                        |
| `GetShieldedTransactionsBySignature`  | Events in one Solana transaction                      |
| `GetShieldedTransactionsByNullifiers` | Shielded history for nullifiers                       |
| `GetMerkleProofs`                     | State-tree inclusion proofs                           |
| `GetNonInclusionProofs`               | Nullifier absence proofs                              |
| `GetUserRecords`                      | Registry viewing and nullifier keys                   |
| `GetNullifierQueueElements`           | Nullifier queue page from `startSeq`, limit 1 to 1000 |

Decode admits each response against its request before it is typed. A history
page must serialize an explicit `paginationToken` (`null` ends the history; a
missing key is a schema error), `minContextSlot` rejects a page whose
`context.slot` sits below the requested floor, and the requested limit bounds
transaction groups, not nested outputs or events. `GetShieldedTransactionsBySignature` events must name the requested signature, `GetMerkleProofs` and
`GetNonInclusionProofs` return one proof per requested leaf in request order
inside the requested tree, `GetUserRecords` returns one record per owner
(`null` when absent), and `GetNullifierQueueElements` pages are ordered oldest
first from the inclusive `startSeq` with 32-byte base58 values.

`UtxoPage` and `ShieldedPage` convert into `Page`, which carries the page's
claimed `context_slot`. `collect_pages` follows tokens under a requested
floor, refuses a page older than its predecessor or a token that repeats, and
errors when the page budget runs out with a token still present.
`collect_nullifier_queue` does the same for the queue with a checked
`start_seq` cursor. These checks bind what the provider declares; they do not
prove the indexer indexed honestly, and a multi-page walk is not an atomic
snapshot — the aggregate carries the last page's claimed context.

## Prover

`PrivacyProverClient` posts a witness you already built to `POST /prove/{circuit}` with `X-Sync: true`. A `429` is sent again with `X-Async: true`, and a `jobId` is polled at `/prove/{circuit}/status`. The gnark proof comes back with proof `A` negated and compressed into the 32-byte `a`, 128-byte `b`, and 32-byte `c` that `TransactBody::with_proof` stores.

The client is generic over `solana_kit_client::rest::Transport<ProverDestination>`. The transport sets `X-Sync` or `X-Async` from `ProverDestination::delivery` and sends an API key as a header. The client rejects any base URL that carries a query — an `api-key` cannot ride in the origin, not even before the first path segment — and it does not write the key into a URL or an error.

A prover on `127.0.0.1`, `localhost`, `*.localhost`, or `::1` accepts the JSON body directly. Any other host requires `allow_unauthenticated_remote_prover(true)` and an `https` URL. `report_data`, `gpu_nonce`, and `request_aad` match the published TEE vectors. The `tee` feature checks a Nitro COSE_Sign1 attestation against the AWS root and the caller's PCR policy, then seals the request with HPKE. Each seal draws a fresh ephemeral key from the caller's `rand_core` generator; keep that generator freshly seeded and never deliberately reset it, and the AES-256-GCM key and nonce do not repeat across requests — an entropy failure from the generator returns an error instead of a seal. An all-zero X25519 shared secret is rejected on both the seal and open paths. The shared HTTP transport has no custom headers, so the caller attaches `Spr-Tee` and `Spr-Tee-Enc` to the sealed bytes. `prove_indexed` posts to `/prove/{circuit}/indexed` and polls a queued job at the plain key route `/prove/{circuit}/status`, the route the indexed reply names; the prover registers no path that carries both segments. `PrivacyProverClient::attest` runs the attestation round in one call: it draws a fresh 32-byte nonce from the caller's `rand_core` generator for every attempt (an entropy failure returns before any request), fetches `GET /attestation?nonce=`, and verifies the Nitro document against the caller's `NitroPolicy` at the caller's `now_secs` — the exact nonce match pins the quote to this challenge, and the policy's `max_age_secs` and `max_future_secs` (seconds) bound the signed document timestamp on both sides of the caller's clock while the AWS certificate windows stay a separate check. `VerifiedNitro::timestamp_ms` exposes the signed millisecond timestamp; the client keeps no attestation cache and no reuse lifetime, so deciding how long a verified session may be reused is the caller's job. `fetch_unverified_attestation` returns the raw body for low-level callers that draw their own CSPRNG nonce per attempt and verify it immediately with `verify_attestation`; a raw fetch alone does not prevent a replay.

The pinned proving-key SHA-256 must match `provingKeySha256` in the response. With `poseidon`, `confidential_transfer_witness` builds the gnark witness from notes and Merkle paths for `transfer-confidential`, `transfer-confidential-eddsa`, and `transfer-eddsa-ring`, sizing the public signer vector at the pinned shape width `min(n, 64 - 4 - n) + 1` and refusing rail and shape pairs no pinned key serves. Those three published fixtures rebuild byte for byte. `indexed_proof_request` takes the same witness and builds the unresolved body the indexed prover accepts: the prepared object drops the tree slots, the public-input hash, and every input path the indexer fills, the `publicInputs` prefix omits the tree-slot chain the server resolves from the supplied trees and inserts itself, and trees and lookups are validated against the pinned envelope before transport. The merge circuit is a separate prover program. The Dart client does not assemble it, and this crate does not stub it.

### Running a prover locally

Helius publishes the prover and the circuit artifacts:

- The prover binary is `prover-linux-x64-cli-v0.10.0` on the `cli-v0.10.0` release of `helius-labs/privacy-rings` (release commit `357dd92c99e9294a158118982918cf735998e21c`). Its SHA-256 is `4be11e2d877e5312bb64f298fb50eb69bb46b26059e8d523678338c0e10d11b2` (the release's `SHA256SUMS` asset has SHA-256 `c60bebdff467f413e79a915bffd68bef15d6ca804afcd844f466a3a39727675e`).
- The gnark circuits are in that repository, under `prover/server`. This crate's witness builder follows `sdk-libs/client` at tag `v0.8.0`.
- Proving keys are public at `https://d3gbdb0egjwcw9.cloudfront.net/` plus the `prefix` (`proving-keys/78751f5d1c591777`) in `prover/server/prover/provingkeys/proving-keys.lock`. `transfer_confidential_1_2.key` is 9,268,304 bytes and matches pin `c0b5c1a1081136911a6ad5496fb989ed26d85b09e983a4d5048cff96e0833d85`. `transfer_confidential_2_2.key` is 16,407,649 bytes and matches pin `51380b1264737ebd4d31f40ebf6874db039078ce998d4fa187c049ace99798bb`. The full set is several gigabytes, and one batch key is about 74 GB.

This repository has no devenv or Nix setup, so the prover stays an external binary:

```bash
set -euo pipefail

curl -fsSL -o prover \
  https://github.com/helius-labs/privacy-rings/releases/download/cli-v0.10.0/prover-linux-x64-cli-v0.10.0
echo "4be11e2d877e5312bb64f298fb50eb69bb46b26059e8d523678338c0e10d11b2  prover" | sha256sum -c -
chmod +x prover
mkdir -p proving-keys
curl -fsSL -o proving-keys/transfer_confidential_1_2.key \
  https://d3gbdb0egjwcw9.cloudfront.net/proving-keys/78751f5d1c591777/transfer_confidential_1_2.key
echo "c0b5c1a1081136911a6ad5496fb989ed26d85b09e983a4d5048cff96e0833d85  proving-keys/transfer_confidential_1_2.key" | sha256sum -c -
# The published 2x2 fixture also needs the second key:
curl -fsSL -o proving-keys/transfer_confidential_2_2.key \
  https://d3gbdb0egjwcw9.cloudfront.net/proving-keys/78751f5d1c591777/transfer_confidential_2_2.key
echo "51380b1264737ebd4d31f40ebf6874db039078ce998d4fa187c049ace99798bb  proving-keys/transfer_confidential_2_2.key" | sha256sum -c -
./prover start \
  --prover-address 127.0.0.1:3001 \
  --metrics-address 127.0.0.1:9998 \
  --keys-dir proving-keys \
  --serve transfer_confidential_1_2 \
  --serve transfer_confidential_2_2 \
  --server-only
```

Point `PRIVACY_RINGS_LOCAL_PROVER_URL` at `http://127.0.0.1:3001` and run the ignored tests. Loopback does not need the unauthenticated opt-in. An empty witness gets a `400` from this binary, which shows the client reached it. `local_prover_proves_the_published_witness` posts `tests/fixtures/transfer-2x2.json` to a prover that serves `transfer_confidential_2_2`, so that key must be present. `confidential_transfer_witness` rebuilds that fixture, so the proof is the witness this crate builds. No Helius API key is needed for this path. The repository's `.github/workflows/privacy-rings.yml` downloads the pinned binary and both keys, verifies their SHA-256 pins, starts a loopback prover, and runs both ignored tests; this documentation does not claim a green result until that workflow reports one.

The hosted mainnet and devnet provers answer `401` without a key. The ignored live test reads `HELIUS_PRIVACY_RINGS_LIVE=1` and `HELIUS_API_KEY`. CI does not set either variable, and this crate's tests do not either. The test sends the key as an `x-api-key` header and never puts it in the URL.

Blueshift's XARK stack is a separate Arkworks Groth16 toolchain. It does not prove these gnark circuits and it does not publish these keys.

## Balances

`ViewingKey::open_output` decrypts a confidential or ring-confidential payload and reads a proofless deposit. The slot index is the output's position in that event's full `outputSlots` list. `GetEncryptedUtxosByTags` can return only the matching outputs, so that result's order is not the slot index. Look the output's `tx_signature` up with `GetShieldedTransactionsBySignature`, find the matching `output_slot`, and use its position in that event's `output_slots`.

Confidential payloads use unauthenticated AES-CTR. `open_output` does not verify their Poseidon commitment and remains an inspection helper only. With both `crypto` and `poseidon`, `verify_note` checks an expected owner and recomputes the commitment using the pinned UTXO domain. For proofless notes it requires an explicit trusted mint-to-asset-field resolver so the commitment uses the registry asset field while the note keeps its displayed mint. It admits only data-free confidential notes (no ring program id and no application data) and data-free proofless notes whose trailing wire fields are the canonical empty option tags; ring-, data-, or extension-bearing shapes fail closed. A proofless deposit is cleartext, and opening it does not check `ProoflessNote::owner`; pass the expected owner to `verify_note`.

`private_balances` adds amounts for verified notes whose commitment the caller has not listed as spent. Commitments, not indexer leaf labels, identify notes, so one commitment counts once across leaf relabels and a spent commitment stays excluded. Confidential asset ids and proofless mints stay separate. Matching an indexer-supplied commitment is not independent chain inclusion: callers must obtain trustworthy commitment and tree context. With the `poseidon` feature, `nullifier` derives a spend tag. `subtract_nullified` drops notes whose tag is in `spent_nullifiers`. `retain_non_inclusion` keeps a note when that tag has a non-inclusion proof. A note with no derived tag stays, because the history does not name its leaf.

Balance admission preserves raw `OpenedNote` and `TrackedNote` inspection:
proofless `records` contains every byte after the 104-byte prefix, including
unsupported or malformed tails. Only exactly six zero `None` tags enter balances;
ring and application notes remain inspection values. The expected owner hash,
published commitment, tree context, and exact mint-to-asset-field mapping must
come from trusted sources. A matching hash is not a Merkle inclusion proof.

Migrate balance inputs from `TrackedNote` to `VerifiedNote` through `verify_note`.
Key both `subtract_nullified` and `retain_non_inclusion` nullifier maps by verified
commitment instead of `(tree_id, leaf_index)`. Unknown nullifiers remain in the
result; these helpers cannot establish that such notes are unspent. Validate the
complete snapshot with `private_balances` before discarding spent notes: conflicting
leaf claims or commitment representations reject even when marked spent. Identical
commitments count once across different leaves. Confidential asset ids and displayed
proofless mints remain separate balance keys; representing one commitment through
both rails or different displayed mints is an error in either order.

## Verification and coverage

`.github/workflows/privacy-rings.yml` runs isolated, non-unified checks for
this crate: feature sets `""`, `crypto`, `poseidon`, `crypto,poseidon`, `tee`,
and `tee,poseidon` (test, clippy with `-D warnings`, and docs with
`RUSTDOCFLAGS=-D warnings`); the declared MSRV 1.89 library check with all
features; all-feature tests on `i686-unknown-linux-gnu`; all-feature library
builds for `wasm32-unknown-unknown`, `wasm32-wasip1`, and `wasm32-wasip2`
(compilation only); and the checksum-pinned local prover integration with its
two ignored tests.

These checks exercise crate contracts. They do not prove hosted indexer
honesty, on-chain inclusion, that a note is unspent, or browser/WASI runtime
behavior. Hosted live tests stay ignored.
