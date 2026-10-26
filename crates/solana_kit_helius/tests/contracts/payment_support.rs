//! Scripted Developer exchanges shared by signup and standalone checkout contracts.

use solana_kit_client::{Reply, TransportError};
use solana_kit_helius::service::*;
use std::{
    cell::RefCell,
    collections::VecDeque,
    future::{Ready, ready},
    rc::Rc,
};

type Trace = Rc<RefCell<Vec<ServiceRequest>>>;

pub(super) struct Replay {
    replies: RefCell<VecDeque<Reply>>,
    requests: Trace,
}

impl ServiceTransport<Service> for Replay {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: ServiceRequest) -> Self::Exchange<'_> {
        assert_eq!(request.service, Service::Developer);
        self.requests.borrow_mut().push(request);
        ready(Ok(self
            .replies
            .borrow_mut()
            .pop_front()
            .expect("scripted exchange")))
    }
}

pub(super) fn client(replies: Vec<Reply>) -> (ServiceClient<Replay>, Trace) {
    let requests = Rc::new(RefCell::new(Vec::new()));
    let transport = Replay {
        replies: RefCell::new(replies.into()),
        requests: Rc::clone(&requests),
    };
    (ServiceClient::new(transport, Budgets::default()), requests)
}

pub(super) fn reply(body: &str) -> Reply {
    Reply {
        status: 200,
        body: body.as_bytes().to_vec(),
    }
}

pub(super) fn body(request: &ServiceRequest) -> serde_json::Value {
    serde_json::from_slice(request.body.as_deref().expect("JSON request")).expect("object body")
}
