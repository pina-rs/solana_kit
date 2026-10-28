//! Fixed pinned Jupiter contracts execute in native, browser, WASI and production wasmi.

mod fixture;
mod models;
mod ownership;
mod swap;

pub(crate) use swap::cases as swap_cases;

use crate::{CaseResult, case, require};
use alloc::{rc::Rc, string::ToString};
use fixture::{Peer, State, finish};
use solana_kit_client::rest::{self, Budgets};
use solana_kit_core::Address;
use solana_kit_jupiter::{Category, Interval, TokenLimit};

const SEARCH_LIMIT: u64 = 7;
const TARGET_BYTES: usize = 256;

pub(crate) fn cases() -> [CaseResult; 5] {
    [
        case("jupiter-price-token-routes-and-local-futures", routes()),
        case("jupiter-exact-price-token-wire-models", models::precise()),
        case(
            "jupiter-malformed-known-fields-and-mint-keys",
            models::invalid(),
        ),
        case(
            "jupiter-http-budgets-and-single-exchange",
            ownership::failures(),
        ),
        case(
            "jupiter-input-admission-and-owned-cancellation",
            ownership::admission(),
        ),
    ]
}

#[test]
fn jupiter_public_contracts_pass() {
    for case in cases().into_iter().chain(swap::cases()) {
        assert!(case.outcome.is_ok(), "{}: {:?}", case.name, case.outcome);
    }
}

fn routes() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let prices = Peer::new(state.clone(), b"{}").client(Budgets::default());
    let mint = Address::from_bytes([0; 32]);
    finish(prices.get_prices(&[mint, mint]))?.map_err(|_| "Jupiter price request failed")?;
    let tokens = Peer::new(state.clone(), b"[]").client(Budgets::default());
    finish(tokens.search_tokens("SOL +/雪", Some(SEARCH_LIMIT)))?
        .map_err(|_| "Jupiter search failed")?;
    finish(tokens.tagged_tokens("verified"))?.map_err(|_| "Jupiter tag request failed")?;
    finish(tokens.category_tokens(
        Category::TopTraded,
        Interval::default(),
        TokenLimit::new(100),
    ))?
    .map_err(|_| "Jupiter category request failed")?;

    finish(tokens.recent_tokens())?.map_err(|_| "Jupiter recent request failed")?;
    check_requests(&state, mint)
}

fn check_requests(state: &State, mint: Address) -> Result<(), &'static str> {
    let requests = state.requests.borrow();
    let expected = [
        ("/price/v3", "ids", alloc::format!("{mint},{mint}")),
        ("/tokens/v2/search", "query", "SOL +/雪".to_string()),
        ("/tokens/v2/tag", "query", "verified".to_string()),
        ("/tokens/v2/toptraded/24h", "limit", "100".to_string()),
        ("/tokens/v2/recent", "", "".to_string()),
    ];
    require(
        requests.len() == expected.len(),
        "Jupiter route count differs",
    )?;

    for (request, (path, key, value)) in requests.iter().zip(expected) {
        let actual = alloc::format!("/{}", request.path_segments.join("/"));
        require(actual == path, "Jupiter path differs")?;
        require(
            request.method == rest::Method::Get,
            "Jupiter method differs",
        )?;
        require(request.body.as_deref().is_none(), "Jupiter GET has a body")?;

        if !key.is_empty() {
            require(
                request.query.first() == Some(&(key.to_string(), value)),
                "Jupiter query differs",
            )?;
        }
    }

    require(
        requests[1].query[1] == ("limit".into(), "7".into()),
        "search limit was lost",
    )?;
    let encoded = rest::request_target(
        "/",
        &requests[1].path_segments,
        &requests[1].query,
        TARGET_BYTES,
    )
    .map_err(|_| "Jupiter target encoding failed")?;
    require(
        encoded == "/tokens/v2/search?query=SOL%20%2B%2F%E9%9B%AA&limit=7",
        "query escaped its boundary",
    )
}
