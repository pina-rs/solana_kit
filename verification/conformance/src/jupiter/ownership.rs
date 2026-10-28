//! Adapter ownership, failure and input bounds remain observable on every host.

use super::fixture::{Peer, State, finish};
use crate::require;
use alloc::{format, rc::Rc};
use core::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};
use solana_kit_client::rest::{Budgets, JsonError};
use solana_kit_core::Address;
use solana_kit_jupiter::{Error, TokenLimit, decode_tokens};

const UNAVAILABLE: u16 = 503;
const ABOVE_CATEGORY_LIMIT: u8 = 101;

pub(super) fn failures() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let mut peer = Peer::new(state.clone(), b"sensitive server body");
    peer.status = UNAVAILABLE;
    let client = peer.client(Budgets::default());
    let error = finish(client.recent_tokens())?
        .err()
        .ok_or("HTTP failure disappeared")?;
    require(
        !format!("{error:?} {error}").contains("sensitive"),
        "HTTP diagnostics leaked body",
    )?;
    require(
        matches!(error, Error::Http { status:UNAVAILABLE, body } if body==b"sensitive server body"),
        "HTTP body/status changed",
    )?;
    require(
        state.requests.borrow().len() == 1 && state.active.get() == 0,
        "HTTP failure retried or leaked",
    )?;

    response_budgets()
}

fn response_budgets() -> Result<(), &'static str> {
    let body = br#"[{"name":"large"}]"#;
    let limits = [
        Budgets {
            bytes: 1,
            ..Budgets::default()
        },
        Budgets {
            depth: 1,
            ..Budgets::default()
        },
        Budgets {
            items: 1,
            ..Budgets::default()
        },
        Budgets {
            string_bytes: 1,
            ..Budgets::default()
        },
    ];

    for budget in limits {
        require(
            decode_tokens(body, budget).is_err(),
            "Jupiter response budget was ignored",
        )?;
    }

    let state = Rc::new(State::default());
    let client = Peer::new(state.clone(), body).client(Budgets {
        bytes: 30,
        ..Budgets::default()
    });
    let oversized = Peer::new(state.clone(), &[b'a'; 31]).client(Budgets {
        bytes: 30,
        ..Budgets::default()
    });
    require(
        finish(client.recent_tokens())?.is_ok(),
        "bounded token reply failed",
    )?;
    require(
        matches!(
            finish(oversized.recent_tokens())?,
            Err(Error::Json(JsonError::Bytes))
        ),
        "unbounded adapter reply accepted",
    )
}

pub(super) fn admission() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let client = Peer::new(state.clone(), b"[]").client(Budgets::default());
    let mints = [Address::from_bytes([0; 32]); 51];
    require(
        matches!(finish(client.get_prices(&mints))?, Err(Error::TooManyMints)),
        "51mints accepted",
    )?;
    require(
        state.requests.borrow().is_empty(),
        "invalid batch reached transport",
    )?;
    require(
        TokenLimit::new(0).is_none()
            && TokenLimit::new(ABOVE_CATEGORY_LIMIT).is_none()
            && TokenLimit::new(100).is_some(),
        "category limit domain differs",
    )?;
    let small = Peer::new(state.clone(), b"[]").client(Budgets {
        bytes: 8,
        ..Budgets::default()
    });

    require(
        finish(small.search_tokens("oversized query", None))?.is_err(),
        "query bytes were unbounded",
    )?;
    require(
        state.requests.borrow().is_empty(),
        "oversized query reached transport",
    )?;
    boundaries()?;
    cancellation(state)
}

fn boundaries() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let prices = Peer::new(state.clone(), b"{}").client(Budgets::default());
    let maximum = [Address::from_bytes([0; 32]); solana_kit_jupiter::MAX_PRICE_MINTS];
    require(
        finish(prices.get_prices(&maximum))?.is_ok(),
        "maximum valid price batch rejected",
    )?;
    require(
        finish(prices.get_prices(&[]))?.is_ok(),
        "empty reference price query rejected",
    )?;
    let tokens = Peer::new(state.clone(), b"[]").client(Budgets {
        bytes: 32,
        ..Budgets::default()
    });
    require(
        finish(tokens.search_tokens("雪雪雪雪", None))?.is_err(),
        "percent-expanded query exceeded budget",
    )?;
    require(
        state.requests.borrow().len() == 2,
        "oversized encoded query reached transport",
    )
}

fn cancellation(state: Rc<State>) -> Result<(), &'static str> {
    let mut peer = Peer::new(state.clone(), b"[]");
    peer.pending = true;
    let client = peer.client(Budgets::default());

    {
        let mut call = pin!(client.recent_tokens());
        let mut context = Context::from_waker(Waker::noop());
        require(
            matches!(call.as_mut().poll(&mut context), Poll::Pending),
            "pending peer completed",
        )?;
        require(state.active.get() == 1, "pending peer not owned")?;
    }

    require(
        state.active.get() == 0 && state.dropped.get() == 1,
        "cancelled Jupiter exchange was not dropped",
    )
}
