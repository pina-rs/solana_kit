//! Real Hermes public GET futures execute identically in every core-suite host.
mod fixture;
mod ownership;
use crate::{CaseResult, case, require};
use alloc::rc::Rc;
use fixture::{LISTING, Manual, Peer, State, UPDATE, client, finish};
use solana_kit_pyth::hermes::{AssetType, ConfigError, Encoding, Error, Policy, UpdateOptions};

pub(crate) fn cases() -> [CaseResult; 4] {
    [
        case("pyth-hermes-GET-routes-and-ordered-options", routes()),
        case("pyth-hermes-finite-retries-and-last-error", retries()),
        case(
            "pyth-hermes-pending-deadlines-and-cancellation",
            ownership::run(),
        ),
        case("pyth-hermes-policy-and-request-admission", admission()),
    ]
}

/// All three routes preserve decoded query values and option order from Dart.
fn routes() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let clock = Manual::default();
    let listing = client(Peer::new(state.clone(), LISTING), clock.clone());
    finish(
        listing.get_price_feeds(Some("btc +/雪"), Some(AssetType::Crypto)),
        &clock,
    )?
    .map_err(|_| "Hermes listing failed")?;
    let updates = client(Peer::new(state.clone(), UPDATE), clock.clone());
    finish(
        updates.get_latest_price_updates(&["a", "a"], UpdateOptions::default()),
        &clock,
    )?
    .map_err(|_| "Hermes latest failed")?;
    finish(
        updates.get_price_updates_at_timestamp(
            -123,
            &["a"],
            UpdateOptions {
                encoding: Encoding::Base64,
                parsed: true,
                ignore_invalid_price_ids: true,
            },
        ),
        &clock,
    )?
    .map_err(|_| "Hermes historical failed")?;

    check_routes(&state.requests.borrow())
}

/// Compare complete requests separately from driving the client futures.
fn check_routes(
    requests: &[solana_kit_client::rest::Request<solana_kit_pyth::hermes::Service>],
) -> Result<(), &'static str> {
    require(requests.len() == 3, "Hermes GET count differs")?;
    require(
        requests[0].path_segments == ["v2", "price_feeds"],
        "Hermes listing route differs",
    )?;
    compare(
        &requests[0].query,
        &[("query", "btc +/雪"), ("asset_type", "crypto")],
    )?;
    require(
        requests[1].path_segments == ["v2", "updates", "price", "latest"],
        "Hermes latest route differs",
    )?;
    compare(
        &requests[1].query,
        &[
            ("ids[]", "a"),
            ("ids[]", "a"),
            ("encoding", "hex"),
            ("parsed", "false"),
        ],
    )?;
    require(
        requests[2].path_segments == ["v2", "updates", "price", "-123"],
        "Hermes historical route differs",
    )?;
    compare(
        &requests[2].query,
        &[
            ("ids[]", "a"),
            ("encoding", "base64"),
            ("parsed", "true"),
            ("ignore_invalid_price_ids", "true"),
        ],
    )?;

    for request in requests.iter() {
        require(
            request.method == solana_kit_client::rest::Method::Get
                && request.body.as_deref().is_none(),
            "Hermes request method/body differs",
        )?;
    }

    Ok(())
}

fn compare(
    actual: &[(alloc::string::String, alloc::string::String)],
    expected: &[(&str, &str)],
) -> Result<(), &'static str> {
    require(
        actual
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .eq(expected.iter().copied()),
        "Hermes ordered query differs",
    )
}

/// Finite retries keep the final bounded body and the exact 100/200/400ms schedule.
fn retries() -> Result<(), &'static str> {
    const TOTAL_BACKOFF_MS: u64 = 700;
    const UNAVAILABLE: u16 = 503;
    let state = Rc::new(State::default());
    let clock = Manual::default();
    let mut peer = Peer::new(state.clone(), b"last");
    peer.failures.set(usize::MAX);
    peer.status = UNAVAILABLE;
    let client = client(peer, clock.clone());
    let result = finish(client.get_price_feeds(None, None), &clock)?;

    require(
        matches!(result, Err(Error::Http { status:UNAVAILABLE, body }) if body==b"last"),
        "Hermes final failure was lost",
    )?;
    require(
        (
            state.requests.borrow().len(),
            clock.0.get(),
            state.active.get(),
            state.dropped.get(),
        ) == (4, TOTAL_BACKOFF_MS, 0, 4),
        "Hermes finite backoff or ownership differs",
    )
}

/// Invalid timing and oversized input fail before constructing an exchange.
fn admission() -> Result<(), &'static str> {
    use solana_kit_client::{rest::Budgets, subscriptions::ClockError};
    require(
        Policy::new(0, 0, 0) == Err(ConfigError::ZeroTimeout),
        "zero Hermes deadline accepted",
    )?;
    require(
        Policy::new(1, 2, u64::MAX) == Err(ConfigError::BackoffRange),
        "Hermes backoff overflow accepted",
    )?;

    let state = Rc::new(State::default());
    let clock = Manual::default();
    let bounded = solana_kit_pyth::hermes::Client::new(
        Peer::new(state.clone(), LISTING),
        clock.clone(),
        Policy::default(),
        Budgets {
            items: 0,
            ..Budgets::default()
        },
    );
    require(
        finish(
            bounded.get_latest_price_updates(&["a"], UpdateOptions::default()),
            &clock,
        )?
        .is_err(),
        "Hermes request budget bypassed",
    )?;

    clock.0.set(u64::MAX);
    let client = client(Peer::new(state.clone(), LISTING), clock.clone());
    let result = finish(client.get_price_feeds(None, None), &clock)?;
    require(
        matches!(result, Err(Error::Clock(ClockError::DeadlineRange))),
        "Hermes clock overflow was lost",
    )?;
    require(
        state.requests.borrow().is_empty(),
        "invalid Hermes request had effects",
    )
}
