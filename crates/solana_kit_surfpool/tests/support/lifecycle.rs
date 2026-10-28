//! A non-Send Rc future proves lazy execution and active-exchange cancellation.

use super::require;
use solana_kit_client::{Client, Limits, Reply, Request, Transport, TransportError};
use solana_kit_surfpool::{Surfnet, cheatcodes::GetSurfnetInfo};
use std::{
    cell::Cell,
    future::Future,
    pin::{Pin, pin},
    rc::Rc,
    task::{Context, Poll, Waker},
};

struct LocalTransport {
    starts: Rc<Cell<usize>>,
    drops: Rc<Cell<usize>>,
}

struct LocalExchange(Rc<Cell<usize>>);

impl Future for LocalExchange {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Drop for LocalExchange {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

impl Transport for LocalTransport {
    type Exchange<'a> = LocalExchange;

    fn exchange(&self, _: Request) -> Self::Exchange<'_> {
        self.starts.set(self.starts.get() + 1);
        LocalExchange(Rc::clone(&self.drops))
    }
}

pub fn run() -> Result<(), &'static str> {
    let starts = Rc::new(Cell::new(0));
    let drops = Rc::new(Cell::new(0));
    let transport = LocalTransport {
        starts: Rc::clone(&starts),
        drops: Rc::clone(&drops),
    };
    let surfnet = Surfnet::new(Client::new(transport, Limits::default()));
    let unpolled = surfnet.execute(GetSurfnetInfo);
    require(
        starts.get() == 0,
        "constructing a Surfpool future performed I/O",
    )?;
    drop(unpolled);
    require(drops.get() == 0, "unpolled operation created an exchange")?;

    {
        let mut pending = pin!(surfnet.execute(GetSurfnetInfo));
        require(
            pending
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending(),
            "local future did not remain pending",
        )?;
        require(
            starts.get() == 1 && drops.get() == 0,
            "exchange was duplicated or dropped early",
        )?;
    }

    require(
        starts.get() == 1 && drops.get() == 1,
        "dropping the caller future did not cancel exactly one exchange",
    )
}
