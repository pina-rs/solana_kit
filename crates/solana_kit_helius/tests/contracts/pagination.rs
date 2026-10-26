//! All-pages convenience calls use the actual Kit Client and finite literal pages.

use solana_kit_client::{Client, Limits, Reply, Request, Transport, TransportError};
use solana_kit_helius::rpc_v2::*;
use std::{
    cell::RefCell,
    collections::VecDeque,
    future::{Ready, ready},
    rc::Rc,
};

struct Pages {
    replies: RefCell<VecDeque<&'static str>>,
    requests: Rc<RefCell<Vec<serde_json::Value>>>,
}

impl Transport for Pages {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let request: serde_json::Value =
            serde_json::from_slice(&request.body).expect("Kit RPC frame");
        let id = request["id"].as_u64().expect("request ID");
        self.requests.borrow_mut().push(request);
        let result = self
            .replies
            .borrow_mut()
            .pop_front()
            .expect("bounded one-page exchange");
        ready(Ok(Reply {
            status: 200,
            body: format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#).into_bytes(),
        }))
    }
}

fn request() -> GetProgramAccountsV2Request {
    GetProgramAccountsV2Request {
        program_address: "p".into(),
        config: Some(GetProgramAccountsV2Config {
            limit: Some(1),
            ..Default::default()
        }),
    }
}

fn pages_client(replies: &[&'static str]) -> (Client<Pages>, Rc<RefCell<Vec<serde_json::Value>>>) {
    let requests = Rc::new(RefCell::new(Vec::new()));
    (
        Client::new(
            Pages {
                replies: RefCell::new(replies.iter().copied().collect()),
                requests: Rc::clone(&requests),
            },
            Limits::default(),
        ),
        requests,
    )
}

/// Request IDs and cursors advance; terminal pages stop and budget failures stay errors.
pub fn records_and_limits() {
    let first = r#"{"accounts":[{"pubkey":"a","account":{"lamports":1,"owner":"p","data":["","base64"],"executable":false,"rentEpoch":0}}],"paginationKey":"next"}"#;
    let second = r#"{"accounts":[{"pubkey":"b","account":{"lamports":2,"owner":"p","data":["","base64"],"executable":false,"rentEpoch":0}}],"paginationKey":null}"#;
    let (client, requests) = pages_client(&[first, second]);
    let records = futures::executor::block_on(all_program_accounts(
        &client,
        request(),
        Pagination {
            max_pages: 2,
            max_items: 2,
        },
    ))
    .expect("two pages");
    assert_eq!(
        records
            .iter()
            .map(|item| item.pubkey.as_str())
            .collect::<Vec<_>>(),
        ["a", "b"]
    );
    let requests = requests.borrow();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0]["method"], "getProgramAccountsV2");

    assert_eq!(requests[0]["id"], 1);
    assert_eq!(requests[1]["id"], 2);
    assert_eq!(requests[0]["params"][0], "p");
    assert!(requests[0]["params"][1].get("paginationKey").is_none());
    assert_eq!(requests[1]["params"][1]["paginationKey"], "next");
    drop(requests);

    bounded_failures(first);
    empty_intermediate_page(second);
}

fn empty_intermediate_page(last: &'static str) {
    let (client, requests) = pages_client(&[r#"{"accounts":[],"paginationKey":"next"}"#, last]);
    let records = futures::executor::block_on(all_program_accounts(
        &client,
        request(),
        Pagination {
            max_pages: 2,
            max_items: 1,
        },
    ))
    .expect("an empty filtered page can continue");
    assert_eq!(records.len(), 1);
    assert_eq!(requests.borrow().len(), 2);
}

fn bounded_failures(first: &'static str) {
    let (client, requests) = pages_client(&[first]);
    assert!(matches!(
        futures::executor::block_on(all_program_accounts(
            &client,
            request(),
            Pagination {
                max_pages: 1,
                max_items: 2,
            }
        )),
        Err(PaginationError::Pages)
    ));
    assert_eq!(requests.borrow().len(), 1);
    let (client, requests) = pages_client(&[first, first]);
    assert!(matches!(
        futures::executor::block_on(all_program_accounts(
            &client,
            request(),
            Pagination {
                max_pages: 3,
                max_items: 3,
            }
        )),
        Err(PaginationError::RepeatedCursor)
    ));
    assert_eq!(requests.borrow().len(), 2);
    let (client, requests) = pages_client(&[first]);
    assert!(matches!(
        futures::executor::block_on(all_program_accounts(
            &client,
            request(),
            Pagination {
                max_pages: 2,
                max_items: 0,
            }
        )),
        Err(PaginationError::Items)
    ));

    assert_eq!(requests.borrow().len(), 1);
}
