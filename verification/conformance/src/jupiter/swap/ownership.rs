//! Swap futures retain adapter ownership through success, failure and cancellation.

use super::super::fixture::{Peer, State, finish};
use super::routes::order;
use crate::require;
use alloc::{format, rc::Rc};
use core::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};
use solana_kit_client::{TransportFailureKind, rest::Budgets};
use solana_kit_jupiter::{Error, OrderResponse};

const UNAVAILABLE: u16 = 503;

pub(super) fn failures() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let mut peer = Peer::new(state.clone(), b"sensitive HTTP body");
    peer.status = UNAVAILABLE;
    let client = peer.client(Budgets::default());
    let order = order();
    let accepted = accepted();
    let errors = [
        finish(client.get_order(&order))?.err(),
        finish(client.build_swap(&order))?.err(),
        finish(client.execute_order(&accepted, "signed"))?.err(),
    ];

    for error in errors {
        let error = error.ok_or("swap HTTP failure disappeared")?;
        require(
            !format!("{error:?} {error}").contains("sensitive"),
            "swap error diagnostics exposed HTTP body",
        )?;
        let unchanged = matches!(
            error,
            Error::Http { status: UNAVAILABLE, body } if body == b"sensitive HTTP body"
        );
        require(unchanged, "swap HTTP status or exact body changed")?;
    }

    require(
        state.requests.borrow().len() == 3 && state.active.get() == 0 && state.dropped.get() == 3,
        "failed swap retried or leaked exchange",
    )?;
    input_budgets()?;
    super::transport::failures()?;
    response_budgets()
}

fn input_budgets() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let client = Peer::new(state.clone(), b"{}").client(Budgets {
        bytes: 32,
        ..Budgets::default()
    });
    let order = order();
    require(
        finish(client.get_order(&order))?.is_err() && finish(client.build_swap(&order))?.is_err(),
        "oversized swap query reached adapter",
    )?;
    require(
        finish(client.execute_order(&accepted(), "signed"))?.is_err(),
        "oversized execution body reached adapter",
    )?;
    require(
        state.requests.borrow().is_empty(),
        "request budget failed after exchange began",
    )?;
    let client = Peer::new(state.clone(), b"{}").client(Budgets {
        bytes: 128,
        ..Budgets::default()
    });
    let escaped = OrderResponse {
        request_id: Some("\u{0001}".repeat(40)),
        ..OrderResponse::default()
    };
    require(
        finish(client.execute_order(&escaped, "signed"))?.is_err(),
        "escaped execution body exceeded budget without rejection",
    )?;
    require(
        state.requests.borrow().is_empty(),
        "escaped execution body reached adapter",
    )?;
    many_query_parameters(&state)
}

fn many_query_parameters(state: &Rc<State>) -> Result<(), &'static str> {
    let client = Peer::new(state.clone(), b"{}").client(Budgets {
        bytes: 256,
        ..Budgets::default()
    });
    let mut order = order();

    for index in 0..150 {
        order
            .extra_query_parameters
            .insert(format!("extra{index}"), "value".into());
    }

    for result in [
        finish(client.get_order(&order))?.map(|_| ()),
        finish(client.build_swap(&order))?.map(|_| ()),
    ] {
        require(
            matches!(result, Err(Error::Transport(error)) if error.kind == TransportFailureKind::BodyLimit),
            "many query parameters did not report the target budget",
        )?;
    }

    require(
        state.requests.borrow().is_empty(),
        "oversized query reached adapter",
    )
}

fn response_budgets() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let oversized = b"{\"future\":\"oversized response\"}";
    let client = Peer::new(state.clone(), oversized).client(Budgets {
        bytes: 1024,
        string_bytes: 2,
        ..Budgets::default()
    });
    require(
        finish(client.get_order(&order()))?.is_err(),
        "order string budget ignored",
    )?;
    let client = Peer::new(state.clone(), b"[]").client(Budgets::default());
    require(
        finish(client.build_swap(&order()))?.is_err(),
        "build array response accepted",
    )?;
    require(
        finish(client.execute_order(&accepted(), "signed"))?.is_err(),
        "execute array response accepted",
    )?;
    require(
        state.requests.borrow().len() == 3 && state.active.get() == 0,
        "malformed reply retried or leaked",
    )
}

pub(super) fn cancellation() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let mut peer = Peer::new(state.clone(), b"{}");
    peer.pending = true;
    let client = peer.client(Budgets::default());
    let order = order();
    let accepted = accepted();
    drop(client.get_order(&order));
    require(
        state.requests.borrow().is_empty(),
        "unpolled swap future performed I/O",
    )?;

    drop_pending(client.get_order(&order), &state)?;
    drop_pending(client.build_swap(&order), &state)?;
    drop_pending(client.execute_order(&accepted, "signed"), &state)?;
    require(
        state.requests.borrow().len() == 3 && state.dropped.get() == 3,
        "cancelled swap retried or retained work",
    )
}

fn drop_pending(future: impl Future, state: &State) -> Result<(), &'static str> {
    let before = state.dropped.get();

    {
        let mut call = pin!(future);
        let mut context = Context::from_waker(Waker::noop());
        require(
            matches!(call.as_mut().poll(&mut context), Poll::Pending),
            "pending swap peer completed",
        )?;
        require(
            state.active.get() == 1,
            "swap exchange did not retain ownership",
        )?;
    }

    require(
        state.active.get() == 0 && state.dropped.get() == before + 1,
        "dropping swap future did not cancel owned exchange",
    )
}

fn accepted() -> OrderResponse {
    OrderResponse {
        request_id: Some("accepted".into()),
        ..OrderResponse::default()
    }
}
