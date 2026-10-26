//! Public REST clients run through explicit local mock exchanges.

use solana_kit_client::{Reply, TransportError};
use solana_kit_helius::{service::*, webhooks::*};
use std::{
    cell::{Cell, RefCell},
    future::{Future, Ready, ready},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

const WEBHOOK: &[u8] = br#"{"webhookID":"id","wallet":"w","webhookURL":"https://example.invalid","transactionTypes":[],"accountAddresses":[],"webhookType":"enhanced","authHeader":""}"#;

pub(super) struct Mock {
    requests: Rc<RefCell<Vec<ServiceRequest>>>,
    reply: RefCell<Option<Reply>>,
}

impl Mock {
    pub(super) fn new(status: u16, body: &[u8]) -> (Self, Rc<RefCell<Vec<ServiceRequest>>>) {
        let requests = Rc::new(RefCell::new(Vec::new()));
        (
            Self {
                requests: Rc::clone(&requests),
                reply: RefCell::new(Some(Reply {
                    status,
                    body: body.to_vec(),
                })),
            },
            requests,
        )
    }
}

impl ServiceTransport<Service> for Mock {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: ServiceRequest) -> Self::Exchange<'_> {
        self.requests.borrow_mut().push(request);
        ready(Ok(self.reply.borrow_mut().take().expect("one exchange")))
    }
}

pub(super) fn exchange<O: Operation>(operation: O, body: &[u8]) -> ServiceRequest {
    let (transport, requests) = Mock::new(200, body);
    let client = ServiceClient::new(transport, Budgets::default());
    futures::executor::block_on(client.call(operation)).expect("typed service success");
    assert_eq!(requests.borrow().len(), 1);
    requests.borrow_mut().pop().expect("owned request")
}

/// Historical selectors and opaque path/query values survive the actual REST request boundary.
pub fn wallet_query_order() {
    super::wallet::historical_points();
}
/// A failed HTTP response is never interpreted as a successful empty result.
pub fn http_errors_retain_bytes_without_debug_leak() {
    let body = b"error includes secret-token";
    let (transport, requests) = Mock::new(429, body);
    let client = ServiceClient::new(transport, Budgets::default());
    let error =
        futures::executor::block_on(client.call(GetAllWebhooks {})).expect_err("HTTP failure");
    assert!(!format!("{error:?}").contains("secret-token"));
    assert!(!error.to_string().contains("secret-token"));

    match error {
        Error::Http {
            status,
            body: actual,
        } => {
            assert_eq!(status, 429);
            assert_eq!(actual, body);
        }
        other => panic!("wrong failure {other:?}"),
    }

    assert_eq!(requests.borrow().len(), 1);
}

fn failure(body: &[u8], budgets: Budgets) -> Error {
    let (transport, _) = Mock::new(200, body);
    let client = ServiceClient::new(transport, budgets);
    futures::executor::block_on(client.call(GetAllWebhooks {})).expect_err("invalid response")
}

/// Duplicate unknown keys fail admission and oversized requests never reach the adapter.
pub fn shared_rest_admission() {
    const DUPLICATE_EXTENSION: &[u8] = br#"[{"webhookID":"id","wallet":"w","webhookURL":"https://example.invalid","transactionTypes":[],"accountAddresses":[],"webhookType":"enhanced","authHeader":"","extra":1,"extra":2}]"#;
    assert!(matches!(
        failure(DUPLICATE_EXTENSION, Budgets::default()),
        Error::Schema(_)
    ));

    let create: CreateWebhookRequest = serde_json::from_str(
        r#"{"webhookURL":"https://example.invalid","transactionTypes":[],"accountAddresses":[],"webhookType":"enhanced","authHeader":""}"#,
    ).expect("valid request");
    let (transport, requests) = Mock::new(200, WEBHOOK);
    let client = ServiceClient::new(
        transport,
        Budgets {
            bytes: 1,
            ..Budgets::default()
        },
    );
    let result = futures::executor::block_on(client.call(create));

    assert!(matches!(result, Err(Error::Bytes)));
    assert!(requests.borrow().is_empty());
}

/// Byte, depth, item, string and typed-schema failures remain distinct.
pub fn budgets_precede_typed_success() {
    assert!(matches!(
        failure(
            b"[]",
            Budgets {
                bytes: 1,
                ..Budgets::default()
            }
        ),
        Error::Bytes
    ));
    assert!(matches!(
        failure(
            b"[[[]]]",
            Budgets {
                depth: 1,
                ..Budgets::default()
            }
        ),
        Error::Depth
    ));
    assert!(matches!(
        failure(
            b"[0,1]",
            Budgets {
                items: 2,
                ..Budgets::default()
            }
        ),
        Error::Items
    ));
    assert!(matches!(
        failure(
            br#"["abcd"]"#,
            Budgets {
                string_bytes: 3,
                ..Budgets::default()
            }
        ),
        Error::String
    ));
    assert!(matches!(
        failure(b"[{}]", Budgets::default()),
        Error::Schema(_)
    ));
    assert!(matches!(
        failure(b"[", Budgets::default()),
        Error::Schema(_)
    ));
}

struct Pending {
    polls: Rc<Cell<usize>>,
    drops: Rc<Cell<usize>>,
}

struct Exchange {
    polls: Rc<Cell<usize>>,
    drops: Rc<Cell<usize>>,
    _request: ServiceRequest,
}

impl Future for Exchange {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        self.polls.set(self.polls.get() + 1);
        Poll::Pending
    }
}

impl Drop for Exchange {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

impl ServiceTransport<Service> for Pending {
    type Exchange<'a> = Exchange;

    fn exchange(&self, request: ServiceRequest) -> Self::Exchange<'_> {
        Exchange {
            polls: Rc::clone(&self.polls),
            drops: Rc::clone(&self.drops),
            _request: request,
        }
    }
}

/// A genuinely local Rc-containing exchange is polled once and synchronously dropped.
pub fn dropped_exchange_releases_owned_request() {
    let polls = Rc::new(Cell::new(0));
    let drops = Rc::new(Cell::new(0));
    let client = ServiceClient::new(
        Pending {
            polls: Rc::clone(&polls),
            drops: Rc::clone(&drops),
        },
        Budgets::default(),
    );
    let mut call = Box::pin(client.call(GetAllWebhooks {}));
    let mut context = Context::from_waker(futures::task::noop_waker_ref());
    assert!(call.as_mut().poll(&mut context).is_pending());
    assert_eq!(polls.get(), 1);
    assert_eq!(drops.get(), 0);

    drop(call);
    assert_eq!(drops.get(), 1);
}
