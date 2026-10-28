//! Adapter failures keep their category and explicit source chain behind redacted diagnostics.

use super::super::fixture::finish;
use super::routes::order;
use crate::require;
use alloc::{format, rc::Rc};
use core::{
    cell::Cell,
    error::Error as _,
    fmt,
    future::{Ready, ready},
};
use solana_kit_client::{Reply, TransportError, TransportFailureKind, rest};
use solana_kit_jupiter::{Client, Error, OrderResponse, Service};

pub(super) fn failures() -> Result<(), &'static str> {
    let calls = Rc::new(Cell::new(0));
    let client = Client::new(Failing(calls.clone()), rest::Budgets::default());
    let order = order();
    let accepted = OrderResponse {
        request_id: Some("accepted".into()),
        ..OrderResponse::default()
    };
    let errors = [
        finish(client.get_order(&order))?.err(),
        finish(client.build_swap(&order))?.err(),
        finish(client.execute_order(&accepted, "signed"))?.err(),
    ];

    for error in errors {
        let error = error.ok_or("adapter failure disappeared")?;
        require(
            matches!(&error, Error::Transport(failure) if failure.kind == TransportFailureKind::Denied),
            "adapter failure category changed",
        )?;
        require(
            !format!("{error:?} {error}").contains("sensitive"),
            "ordinary adapter diagnostics exposed source",
        )?;
        let cause = error
            .source()
            .and_then(|cause| cause.source())
            .and_then(|cause| cause.source())
            .and_then(|cause| cause.source())
            .ok_or("original adapter cause disappeared")?;
        require(
            format!("{cause}") == "sensitive adapter cause",
            "original adapter cause changed",
        )?;
    }

    require(calls.get() == 3, "adapter failure was retried")
}

struct Failing(Rc<Cell<usize>>);

impl rest::Transport<Service> for Failing {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, _: rest::Request<Service>) -> Self::Exchange<'_> {
        self.0.set(self.0.get() + 1);
        ready(Err(TransportError::with_source(
            TransportFailureKind::Denied,
            Sensitive,
        )))
    }
}

#[derive(Debug)]
struct Sensitive;

impl fmt::Display for Sensitive {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("sensitive adapter cause")
    }
}

impl core::error::Error for Sensitive {}
