//! Every suspended RPC phase retains cancel-on-drop ownership without a spawned task.

use super::*;
use solana_kit_helius::transactions::create_smart_transaction;
use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

struct BlockingRpc {
    replies: RefCell<VecDeque<Step>>,
    requests: Requests,
    block_at: usize,
    dropped: Rc<Cell<usize>>,
}

enum Exchange {
    Ready(Ready<Result<Reply, TransportError>>),
    Pending(Pending),
}

struct Pending {
    request: Request,
    dropped: Rc<Cell<usize>>,
    observed: Requests,
}

impl Transport for BlockingRpc {
    type Exchange<'a> = Exchange;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let id = match self.requests.record(&request) {
            Ok(id) => id,

            Err(error) => return Exchange::Ready(ready(Err(error))),
        };

        if self.requests.frames.borrow().len() == self.block_at {
            return Exchange::Pending(Pending {
                request,
                dropped: Rc::clone(&self.dropped),
                observed: Rc::clone(&self.requests),
            });
        }

        let result = self
            .replies
            .borrow_mut()
            .pop_front()
            .ok_or_else(|| self.requests.fail("preceding queued reply missing"))
            .map(|reply| reply.reply(id));
        Exchange::Ready(ready(result))
    }
}

impl Future for Exchange {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        match self.get_mut() {
            Self::Ready(reply) => Pin::new(reply).poll(context),
            Self::Pending(pending) => Pin::new(pending).poll(context),
        }
    }
}

impl Future for Pending {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.request.body.is_empty() {
            return Poll::Ready(Err(self
                .observed
                .fail("pending smart RPC request body is empty")));
        }

        Poll::Pending
    }
}

impl Drop for Pending {
    fn drop(&mut self) {
        self.dropped.set(self.dropped.get() + 1);
    }
}

pub(super) fn owned_exchanges() -> Outcome {
    let methods = [
        "simulateTransaction",
        "getPriorityFeeEstimate",
        "getLatestBlockhash",
    ];

    for (index, method) in methods.into_iter().enumerate() {
        let block_at = index + 1;
        let (rpc, requests, dropped) = blocking_client(block_at);
        let lifetime = simulation_lifetime();
        let signers = [address(1)];
        let mut future = Box::pin(create_smart_transaction(
            &rpc,
            input(&lifetime, &signers)?,
            options(),
        ));

        require(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending(),
            "selected smart RPC phase did not remain pending",
        )?;
        requests.valid()?;
        require(
            requests.frames.borrow().len() == block_at,
            "suspended smart RPC count changed",
        )?;
        require(
            requests
                .frames
                .borrow()
                .get(index)
                .ok_or("suspended smart RPC observation missing")?["method"]
                == method,
            "suspended smart RPC phase changed",
        )?;
        require(dropped.get() == 0, "live smart exchange already cancelled")?;
        drop(future);
        require(
            dropped.get() == 1,
            "dropped smart exchange was not cancelled once",
        )?;
        require(
            requests.frames.borrow().len() == block_at,
            "dropping smart preparation admitted another RPC",
        )?;
    }

    Ok(())
}

fn blocking_client(block_at: usize) -> (Client<BlockingRpc>, Requests, Rc<Cell<usize>>) {
    let requests = Rc::new(Observed::default());
    let dropped = Rc::new(Cell::new(0));
    let rpc = Client::new(
        BlockingRpc {
            replies: RefCell::new(replies().into()),
            requests: Rc::clone(&requests),
            block_at,
            dropped: Rc::clone(&dropped),
        },
        Limits::default(),
    );
    (rpc, requests, dropped)
}
