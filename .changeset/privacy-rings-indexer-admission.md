---
solana_kit_privacy_rings: minor
---

# Admit indexer responses against their requests

History pages now require an explicit `paginationToken` member: a missing key is a schema error, and an explicit `null` still ends the walk. `Method::decode` binds each response to its request — `minContextSlot` fences the page's `context.slot`, the requested limit bounds transaction groups, signature events must name the requested signature, Merkle and non-inclusion proofs bind positionally to the requested leaves and tree, registry records bind count and owner, and nullifier-queue pages stay bounded, ordered from the inclusive `startSeq`, and carry 32-byte base58 values. `Page` carries the page's `context_slot`, `collect_pages` takes the requested floor and refuses regressing contexts and repeated tokens, and `UtxoPage` and `ShieldedPage` convert into it. `GetNullifierQueueElements::next_start_seq` returns the next checked cursor, and `collect_nullifier_queue` walks the queue with a fixed tree and limit, checked advancement, and monotonic context slots.
