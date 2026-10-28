//! A non-Send Rc future proves lazy execution and active-exchange cancellation.

use super::require;
use alloc::rc::Rc;
use core::{
    cell::Cell,
    future::Future,
    num::NonZeroUsize,
    pin::{Pin, pin},
    task::{Context, Poll, Waker},
};
use solana_kit_client::{Client, Limits, Reply, Request, Transport, TransportError};
use solana_kit_core::Address;
use solana_kit_surfpool::{Surfnet, cheatcodes::GetSurfnetInfo, deploy::DeployOptions};

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
    )?;
    deployment_cancellation()
}

fn deployment_cancellation() -> Result<(), &'static str> {
    let starts = Rc::new(Cell::new(0));
    let drops = Rc::new(Cell::new(0));
    let surfnet = Surfnet::new(Client::new(
        LocalTransport {
            starts: Rc::clone(&starts),
            drops: Rc::clone(&drops),
        },
        Limits::default(),
    ));
    let bytes = [0, 1, 2];
    let mut options = DeployOptions::new(Address::from_bytes([0; 32]), &bytes);
    options.chunk_bytes = NonZeroUsize::new(1).ok_or("invalid cancellation chunk")?;
    options.idl = Some(serde_json::Map::new());

    drop(surfnet.deploy(options.clone()));
    require(
        starts.get() == 0 && drops.get() == 0,
        "unpolled deployment created an exchange",
    )?;

    {
        let mut pending = pin!(surfnet.deploy(options));
        require(
            pending
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending(),
            "deployment did not retain the pending chunk",
        )?;
        require(
            starts.get() == 1 && drops.get() == 0,
            "deployment advanced beyond its pending chunk",
        )?;
    }

    require(
        starts.get() == 1 && drops.get() == 1,
        "cancelled deployment continued chunks or IDL registration",
    )
}
