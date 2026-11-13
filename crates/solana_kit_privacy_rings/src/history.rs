//! Paging and nullifier collection for indexer history.
//!
//! A caller that stops at the first page can miss a spend. `collect_pages`
//! follows `paginationToken` and refuses a truncated history, a page that
//! moves to an older snapshot, or a token that repeats. Nullifier strings on
//! those pages are the spend set a balance can subtract.
//! `collect_nullifier_queue` walks the nullifier queue the same bounded way.

use std::collections::BTreeSet;

use crate::rpc::{GetNullifierQueueElements, NullifierQueue};
use crate::{Error, ShieldedTransaction};

/// One indexer page, the snapshot slot the indexer claimed for it, and the
/// token that fetches the next one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page<T> {
    /// Highest slot the indexer claimed to have persisted for this page. The
    /// walk refuses pages below the requested floor or below an earlier page,
    /// but it cannot tell whether the claim is true.
    pub context_slot: u64,
    /// Items on this page.
    pub items: Vec<T>,
    /// Continuation token. `None` or empty ends the walk.
    pub pagination_token: Option<String>,
}

/// Walk state: the floor the next page must meet and the tokens already followed.
struct PageWalk {
    /// Slot the next page must at least claim.
    floor: Option<u64>,
    /// Token that fetches the next page.
    token: Option<String>,
    /// Tokens this walk already followed, for cycle rejection.
    seen: BTreeSet<String>,
}

impl PageWalk {
    fn new(min_context_slot: Option<u64>) -> Self {
        Self {
            floor: min_context_slot,
            token: None,
            seen: BTreeSet::new(),
        }
    }

    /// Rejects a page older than the requested floor or its predecessor.
    ///
    /// # Errors
    /// Returns `Limit` when the page moves below the current watermark.
    fn admit<T>(&mut self, page: &Page<T>) -> Result<(), Error> {
        if let Some(required) = self.floor
            && page.context_slot < required
        {
            return Err(Error::Limit("page slot"));
        }

        self.floor = Some(page.context_slot);

        Ok(())
    }

    /// Follows the page's continuation and reports whether the walk ended.
    ///
    /// # Errors
    /// Rejects a token this walk already followed: it would fetch the same
    /// page again instead of advancing.
    fn follow(&mut self, token: Option<String>) -> Result<bool, Error> {
        let Some(next) = continuation(token) else {
            return Ok(true);
        };

        if !self.seen.insert(next.clone()) {
            return Err(Error::Limit("page cycle"));
        }

        self.token = Some(next);

        Ok(false)
    }
}

/// Concatenates pages until the token runs out.
///
/// `next_page` receives the previous token, or `None` for the first page.
/// Every page must claim a snapshot at least as fresh as `min_context_slot`
/// and never older than the page before it. The walk stops with an error when
/// `max_pages` is exhausted with a token remaining, when a token repeats, or
/// when a page moves backwards, so a caller cannot treat a stale or partial
/// history as complete. The slots are the provider's own claims: this checks
/// declared contexts, not that the indexer indexed truthfully, and the
/// concatenation is not an atomic multi-page snapshot.
///
/// # Errors
/// Rejects `max_pages == 0`, a fetch error, a stale or regressing page, a
/// repeated token, or a history that did not end.
pub fn collect_pages<T>(
    mut next_page: impl FnMut(Option<&str>) -> Result<Page<T>, Error>,
    max_pages: usize,
    min_context_slot: Option<u64>,
) -> Result<Vec<T>, Error> {
    if max_pages == 0 {
        return Err(Error::Limit("page"));
    }

    let mut walk = PageWalk::new(min_context_slot);
    let mut items = Vec::new();

    for _page in 0..max_pages {
        let page = next_page(walk.token.as_deref())?;
        walk.admit(&page)?;
        items.extend(page.items);

        if walk.follow(page.pagination_token)? {
            return Ok(items);
        }
    }

    Err(Error::Limit("page"))
}

/// Collects the full nullifier-queue tail for one tree with a bounded walk.
///
/// `fetch` performs one `getNullifierQueueElements` exchange; through
/// [`crate::Client`](solana_kit_client::Client) each page is already admitted
/// against the request it answers. The walk keeps the request's tree and page
/// limit fixed, validates every page with
/// [`GetNullifierQueueElements::next_start_seq`], advances the inclusive
/// cursor with checked arithmetic, and stops when a page is shorter than the
/// limit. Every page must claim a context slot at least `min_context_slot`
/// (when given) and never lower than the previous page's. The aggregate
/// carries the last page's claimed context; the indexer serves pages
/// independently, so this is not an atomic snapshot.
///
/// # Errors
/// Rejects an invalid request, `max_pages == 0`, a fetch error, a page that
/// violates its request, a stale or regressing context, a cursor overflow, or
/// a budget exhausted while the tail still looked full.
pub fn collect_nullifier_queue(
    initial_request: GetNullifierQueueElements,
    mut fetch: impl FnMut(&GetNullifierQueueElements) -> Result<NullifierQueue, Error>,
    max_pages: usize,
    min_context_slot: Option<u64>,
) -> Result<NullifierQueue, Error> {
    initial_request.validate()?;

    if max_pages == 0 {
        return Err(Error::Limit("page"));
    }

    let mut request = initial_request;
    let mut elements = Vec::new();
    let mut floor = min_context_slot;

    for _page in 0..max_pages {
        let page = fetch(&request)?;

        if let Some(required) = floor
            && page.context.slot < required
        {
            return Err(Error::Limit("page slot"));
        }

        floor = Some(page.context.slot);

        let next_seq = request.next_start_seq(&page)?;
        elements.extend(page.elements);

        let Some(next_seq) = next_seq else {
            return Ok(NullifierQueue {
                context: page.context,
                elements,
            });
        };

        request = GetNullifierQueueElements {
            tree_account: request.tree_account,
            start_seq: next_seq,
            limit: request.limit,
        };
    }

    Err(Error::Limit("page"))
}

/// Decodes base58 nullifiers published on shielded events.
///
/// # Errors
/// Rejects a nullifier that is not 32 decoded bytes.
pub fn spent_nullifiers(events: &[ShieldedTransaction]) -> Result<BTreeSet<[u8; 32]>, Error> {
    let mut spent = BTreeSet::new();

    for event in events {
        for text in &event.nullifiers {
            spent.insert(decode_nullifier(text)?);
        }
    }

    Ok(spent)
}

fn continuation(token: Option<String>) -> Option<String> {
    match token {
        Some(value) if !value.is_empty() => Some(value),
        _ => None,
    }
}

/// Decodes a base58 string into the 32 bytes of a nullifier hash.
pub(crate) fn decode_nullifier(text: &str) -> Result<[u8; 32], Error> {
    let bytes = bs58::decode(text)
        .into_vec()
        .map_err(|_| Error::Limit("nullifier"))?;
    bytes.try_into().map_err(|_| Error::Limit("nullifier"))
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;
    use crate::rpc::{IndexerContext, NullifierQueueElement, RingsMessage};

    const STALE_PAGE_SLOT: u64 = 499;
    const REQUESTED_FLOOR: u64 = 500;
    const SHARED_SLOT: u64 = 5;
    const LATER_SLOT: u64 = 6;
    const QUEUE_HEAD: u64 = 4;
    const QUEUE_LIMIT: u64 = 2;
    const GAP_SEQUENCE: u64 = 9;
    const AFTER_GAP: u64 = 10;
    const QUEUED_SLOT: u64 = 10;

    fn event(nullifiers: Vec<String>) -> ShieldedTransaction {
        ShieldedTransaction {
            slot: 1,
            tx_signature: "sig".to_owned(),
            event_index: None,
            tx_viewing_pk: None,
            salt: None,
            output_slots: Vec::new(),
            messages: Vec::<RingsMessage>::new(),
            nullifiers,
            proofless: false,
            ring_config: None,
            ring_program_id: None,
        }
    }

    fn page(context_slot: u64, items: Vec<i32>, token: Option<&str>) -> Page<i32> {
        Page {
            context_slot,
            items,
            pagination_token: token.map(str::to_owned),
        }
    }

    /// A real 32-byte base58 nullifier hash.
    fn nullifier(byte: u8) -> String {
        bs58::encode([byte; 32]).into_string()
    }

    fn element(seq: u64, value: &str) -> NullifierQueueElement {
        NullifierQueueElement {
            seq,
            value: value.to_owned(),
        }
    }

    fn queue(slot: u64, elements: Vec<NullifierQueueElement>) -> NullifierQueue {
        NullifierQueue {
            context: IndexerContext {
                block_time: 1,
                slot,
            },
            elements,
        }
    }

    fn queue_request(start_seq: u64) -> GetNullifierQueueElements {
        GetNullifierQueueElements {
            tree_account: "tree".to_owned(),
            start_seq,
            limit: QUEUE_LIMIT,
        }
    }

    #[test]
    fn pages_stop_at_the_end_and_refuse_a_truncated_walk() {
        let step = Cell::new(0);
        let fetch = |_: Option<&str>| {
            let current = step.get() + 1;
            step.set(current);

            match current {
                1 => Err(Error::Limit("rpc")),
                2 => Ok(page(1, vec![1], Some("next"))),
                3 => Ok(page(2, vec![2], Some(""))),
                _ => Ok(page(3, Vec::new(), None)),
            }
        };

        assert!(matches!(
            collect_pages(&fetch, 2, None),
            Err(Error::Limit("rpc"))
        ));
        assert_eq!(collect_pages(&fetch, 4, None).expect("pages"), vec![1, 2]);
        assert!(format!("{:?}", page(1, vec![1], Some("next"))).contains("next"));
        assert!(matches!(
            collect_pages(&fetch, 1, None),
            Err(Error::Limit("page"))
        ));
        assert!(matches!(
            collect_pages(&fetch, 0, None),
            Err(Error::Limit("page"))
        ));
    }

    #[test]
    fn pages_enforce_the_requested_floor_and_monotonic_contexts() {
        let stale = page(STALE_PAGE_SLOT, vec![1], Some(""));

        assert!(matches!(
            collect_pages(|_| Ok(stale.clone()), 2, Some(REQUESTED_FLOOR)),
            Err(Error::Limit("page slot"))
        ));
        assert!(collect_pages(|_| Ok(stale.clone()), 2, Some(STALE_PAGE_SLOT)).is_ok());

        let step = Cell::new(0);
        let regressing = |token: Option<&str>| {
            let current = step.get() + 1;
            step.set(current);

            match (current, token) {
                (1, None) => Ok(page(REQUESTED_FLOOR, vec![1], Some("next"))),
                _ => Ok(page(STALE_PAGE_SLOT, vec![2], Some(""))),
            }
        };

        assert!(matches!(
            collect_pages(regressing, 3, Some(REQUESTED_FLOOR)),
            Err(Error::Limit("page slot"))
        ));

        let equal = |token: Option<&str>| match token {
            None => Ok(page(SHARED_SLOT, vec![1], Some("next"))),
            Some(_) => Ok(page(SHARED_SLOT, vec![2], None)),
        };

        assert_eq!(
            collect_pages(equal, 3, Some(SHARED_SLOT)).expect("equal"),
            vec![1, 2]
        );

        let rising = |token: Option<&str>| match token {
            None => Ok(page(SHARED_SLOT, vec![1], Some("next"))),
            Some(_) => Ok(page(LATER_SLOT, vec![2], None)),
        };

        assert_eq!(collect_pages(rising, 3, None).expect("rising"), vec![1, 2]);
    }

    #[test]
    fn repeated_tokens_are_a_cycle_not_completeness() {
        let looped = |_: Option<&str>| Ok::<Page<i32>, Error>(page(1, vec![1], Some("same")));

        assert!(matches!(
            collect_pages(looped, 4, None),
            Err(Error::Limit("page cycle"))
        ));
    }

    #[test]
    fn the_queue_cursor_ends_short_pages_and_advances_full_ones() {
        let request = queue_request(QUEUE_HEAD);
        let leaf_a = nullifier(1);
        let leaf_b = nullifier(2);

        assert_eq!(
            request
                .next_start_seq(&queue(QUEUED_SLOT, Vec::new()))
                .expect("empty"),
            None
        );

        let short = queue(QUEUED_SLOT, vec![element(QUEUE_HEAD, &leaf_a)]);
        assert_eq!(request.next_start_seq(&short).expect("short"), None);

        // A full page advances past its last sequence, pruned gaps and all.
        let gapped = queue(
            QUEUED_SLOT,
            vec![element(QUEUE_HEAD, &leaf_a), element(GAP_SEQUENCE, &leaf_b)],
        );
        assert_eq!(
            request.next_start_seq(&gapped).expect("full"),
            Some(AFTER_GAP)
        );

        // A full page at the top of the sequence space cannot advance.
        let saturated = queue(
            QUEUED_SLOT,
            vec![element(u64::MAX - 1, &leaf_a), element(u64::MAX, &leaf_b)],
        );
        assert!(matches!(
            request.next_start_seq(&saturated),
            Err(Error::Limit("queue sequence bound"))
        ));
    }

    #[test]
    fn the_queue_walk_aggregates_full_pages_until_a_short_tail() {
        let leaf_a = nullifier(1);
        let leaf_b = nullifier(2);
        let leaf_c = nullifier(3);
        let step = Cell::new(0);

        let walk = collect_nullifier_queue(
            queue_request(QUEUE_HEAD),
            |request| {
                let current = step.get() + 1;
                step.set(current);

                match current {
                    1 => {
                        assert_eq!(request.start_seq, QUEUE_HEAD);
                        Ok(queue(
                            QUEUED_SLOT,
                            vec![element(QUEUE_HEAD, &leaf_a), element(GAP_SEQUENCE, &leaf_b)],
                        ))
                    }
                    2 => {
                        assert_eq!(request.start_seq, AFTER_GAP);
                        Ok(queue(QUEUED_SLOT, vec![element(AFTER_GAP, &leaf_c)]))
                    }
                    _ => panic!("a short page ends the walk"),
                }
            },
            4,
            Some(QUEUED_SLOT),
        )
        .expect("queue");

        assert_eq!(walk.elements.len(), 3);
        assert_eq!(walk.elements[2].seq, AFTER_GAP);
        // The aggregate carries the last page's claimed context.
        assert_eq!(walk.context.slot, QUEUED_SLOT);
    }

    #[test]
    fn the_queue_walk_refuses_a_truncated_live_tail() {
        let leaf_a = nullifier(1);
        let leaf_b = nullifier(2);
        let full = |request: &GetNullifierQueueElements| {
            let head = request.start_seq;
            Ok(queue(
                QUEUED_SLOT,
                vec![element(head, &leaf_a), element(head + 1, &leaf_b)],
            ))
        };

        assert!(matches!(
            collect_nullifier_queue(queue_request(QUEUE_HEAD), full, 4, Some(QUEUED_SLOT)),
            Err(Error::Limit("page"))
        ));
    }

    #[test]
    fn the_queue_walk_fetches_nothing_for_a_zero_budget() {
        assert!(matches!(
            collect_nullifier_queue(
                queue_request(QUEUE_HEAD),
                |_: &GetNullifierQueueElements| -> Result<NullifierQueue, Error> {
                    panic!("no IO for a zero budget")
                },
                0,
                None,
            ),
            Err(Error::Limit("page"))
        ));
    }

    #[test]
    fn the_queue_walk_refuses_an_invalid_request_before_fetch() {
        assert!(matches!(
            collect_nullifier_queue(
                GetNullifierQueueElements {
                    tree_account: String::new(),
                    start_seq: 0,
                    limit: QUEUE_LIMIT,
                },
                |_: &GetNullifierQueueElements| -> Result<NullifierQueue, Error> {
                    panic!("no IO for an invalid request")
                },
                4,
                None,
            ),
            Err(Error::Limit("nullifier queue"))
        ));
    }

    #[test]
    fn the_queue_walk_refuses_a_stale_first_page() {
        let leaf_a = nullifier(1);
        let stale = |_: &GetNullifierQueueElements| {
            Ok(queue(STALE_PAGE_SLOT, vec![element(QUEUE_HEAD, &leaf_a)]))
        };

        assert!(matches!(
            collect_nullifier_queue(queue_request(QUEUE_HEAD), stale, 4, Some(REQUESTED_FLOOR)),
            Err(Error::Limit("page slot"))
        ));
    }

    #[test]
    fn the_queue_walk_refuses_a_regressing_context() {
        let leaf_a = nullifier(1);
        let leaf_b = nullifier(2);
        let step = Cell::new(0);

        let regressing = |_: &GetNullifierQueueElements| {
            let current = step.get() + 1;
            step.set(current);

            match current {
                1 => Ok(queue(
                    REQUESTED_FLOOR,
                    vec![element(QUEUE_HEAD, &leaf_a), element(LATER_SLOT, &leaf_b)],
                )),
                _ => Ok(queue(STALE_PAGE_SLOT, vec![element(LATER_SLOT, &leaf_a)])),
            }
        };

        assert!(matches!(
            collect_nullifier_queue(
                queue_request(QUEUE_HEAD),
                regressing,
                4,
                Some(REQUESTED_FLOOR)
            ),
            Err(Error::Limit("page slot"))
        ));
    }

    #[test]
    fn the_queue_walk_refuses_a_saturated_cursor() {
        let leaf_a = nullifier(1);
        let leaf_b = nullifier(2);
        let saturated = |_: &GetNullifierQueueElements| {
            Ok(queue(
                QUEUED_SLOT,
                vec![element(u64::MAX - 1, &leaf_a), element(u64::MAX, &leaf_b)],
            ))
        };

        assert!(matches!(
            collect_nullifier_queue(queue_request(QUEUE_HEAD), saturated, 4, Some(QUEUED_SLOT)),
            Err(Error::Limit("queue sequence bound"))
        ));
    }

    #[test]
    fn the_queue_walk_propagates_a_fetch_failure() {
        let failing = |_: &GetNullifierQueueElements| Err(Error::Limit("rpc"));

        assert!(matches!(
            collect_nullifier_queue(queue_request(QUEUE_HEAD), failing, 4, None),
            Err(Error::Limit("rpc"))
        ));
    }

    #[test]
    fn nullifiers_decode_and_reject_a_short_value() {
        let encoded = bs58::encode([9u8; 32]).into_string();
        let found = spent_nullifiers(&[event(vec![encoded])]).unwrap();
        assert_eq!(found.len(), 1);
        assert!(found.contains(&[9u8; 32]));
        assert!(matches!(
            spent_nullifiers(&[event(vec!["0".to_owned()])]),
            Err(Error::Limit("nullifier"))
        ));
        let short = bs58::encode([1u8, 2, 3]).into_string();
        assert!(matches!(
            spent_nullifiers(&[event(vec![short])]),
            Err(Error::Limit("nullifier"))
        ));
    }
}
