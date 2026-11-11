//! Paging and nullifier collection for indexer history.
//!
//! A caller that stops at the first page can miss a spend. `collect_pages`
//! follows `paginationToken` and refuses a truncated history. Nullifier
//! strings on those pages are the spend set a balance can subtract.

use std::collections::BTreeSet;

use crate::Error;
use crate::rpc::ShieldedTransaction;

/// One indexer page and the token that fetches the next one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page<T> {
    /// Items on this page.
    pub items: Vec<T>,
    /// Continuation token. `None` or empty ends the walk.
    pub pagination_token: Option<String>,
}

/// Concatenates pages until the token runs out.
///
/// `next_page` receives the previous token, or `None` for the first page.
/// The walk stops with an error when `max_pages` is exhausted and a token
/// remains, so a caller cannot treat a partial history as complete.
///
/// # Errors
/// Rejects `max_pages == 0`, a fetch error, or a history that did not end.
pub fn collect_pages<T>(
    mut next_page: impl FnMut(Option<&str>) -> Result<Page<T>, Error>,
    max_pages: usize,
) -> Result<Vec<T>, Error> {
    if max_pages == 0 {
        return Err(Error::Limit("page"));
    }

    let mut items = Vec::new();
    let mut token = None;

    for _page in 0..max_pages {
        let page = next_page(token.as_deref())?;
        items.extend(page.items);
        token = continuation(page.pagination_token);

        if token.is_none() {
            return Ok(items);
        }
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

fn decode_nullifier(text: &str) -> Result<[u8; 32], Error> {
    let bytes = bs58::decode(text)
        .into_vec()
        .map_err(|_| Error::Limit("nullifier"))?;
    bytes.try_into().map_err(|_| Error::Limit("nullifier"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::RingsMessage;

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

    #[test]
    fn pages_stop_at_the_end_and_refuse_a_truncated_walk() {
        let pages = [
            Page {
                items: vec![1],
                pagination_token: Some("next".to_owned()),
            },
            Page {
                items: vec![2],
                pagination_token: Some(String::new()),
            },
        ];
        let step = std::cell::Cell::new(0);
        let fetch = |_: Option<&str>| {
            let current = step.get() + 1;
            step.set(current);

            match current {
                1 => Err(Error::Limit("rpc")),
                2 => Ok(pages[0].clone()),
                3 => Ok(pages[1].clone()),
                _ => Ok(pages[0].clone()),
            }
        };
        assert!(matches!(collect_pages(&fetch, 2), Err(Error::Limit("rpc"))));
        assert_eq!(collect_pages(&fetch, 4).expect("pages"), vec![1, 2]);
        assert!(format!("{:?}", pages[0]).contains("next"));
        assert!(matches!(
            collect_pages(&fetch, 1),
            Err(Error::Limit("page"))
        ));
        assert!(matches!(
            collect_pages(&fetch, 0),
            Err(Error::Limit("page"))
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
