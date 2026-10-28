//! Requests assert the reference wire contract independently of response decoding.

use super::super::fixture::{Peer, State, finish};
use crate::require;
use alloc::{format, rc::Rc, string::ToString, vec};
use solana_kit_client::rest::{self, Budgets};
use solana_kit_core::Address;
use solana_kit_jupiter::{Error, Integer, OrderRequest, OrderResponse, SwapMode};

const INITIAL_SLIPPAGE_BPS: i64 = 25;
const REFERRAL_FEE_BPS: i64 = 9;
const PLATFORM_FEE_BPS: i64 = 7;
const TARGET_BYTES: usize = 2048;
const LARGE_AMOUNT: &str = "1844674407370955161600001";

pub(super) fn order() -> OrderRequest {
    OrderRequest::new(
        Address::from_bytes([1; 32]),
        Address::from_bytes([2; 32]),
        Integer::from(u64::MAX),
    )
}

pub(super) fn requests() -> Result<(), &'static str> {
    let mut request = order();
    request.amount = LARGE_AMOUNT
        .parse()
        .map_err(|_| "exact request amount rejected")?;
    let defaults = request.to_query_parameters();
    require(
        defaults.len() == 3 && defaults[2].1 == LARGE_AMOUNT,
        "swap defaults or integer amount changed",
    )?;
    request.swap_mode = Some(SwapMode::ExactIn);
    require(
        request
            .to_query_parameters()
            .contains(&("swapMode".into(), "ExactIn".into())),
        "ExactIn mode changed",
    )?;
    request.slippage_bps = Some(INITIAL_SLIPPAGE_BPS);
    request.swap_mode = Some(SwapMode::ExactOut);
    request.referral_account = Some(request.input_mint);
    request.referral_fee = Some(REFERRAL_FEE_BPS);
    request.restrict_intermediate_tokens = Some(false);

    request.platform_fee_bps = Some(PLATFORM_FEE_BPS);
    request.exclude_direct_router_routes = Some(true);
    request.payer = Some(request.output_mint);
    request.taker = Some(request.input_mint);
    request
        .extra_query_parameters
        .insert("slippageBps".into(), "31".into());
    request
        .extra_query_parameters
        .insert("extra".into(), "a +/雪".into());

    let state = Rc::new(State::default());
    let client = Peer::new(state.clone(), b"{}").client(Budgets::default());
    finish(client.get_order(&request))?.map_err(|_| "order route failed")?;
    finish(client.build_swap(&request))?.map_err(|_| "build route failed")?;
    check_routes(&state, &request)?;
    many_query_overrides()?;
    shadowed_large_amount()
}

fn shadowed_large_amount() -> Result<(), &'static str> {
    let mut request = order();
    request.amount = "9"
        .repeat(2 * 1024 * 1024)
        .parse()
        .map_err(|_| "large amount fixture rejected")?;
    request
        .extra_query_parameters
        .insert("amount".into(), "1".into());
    let state = Rc::new(State::default());
    let client = Peer::new(state.clone(), b"{}").client(Budgets {
        bytes: 256,
        ..Budgets::default()
    });
    finish(client.get_order(&request))?.map_err(|_| "shadowed order amount rejected")?;
    finish(client.build_swap(&request))?.map_err(|_| "shadowed build amount rejected")?;
    let requests = state.requests.borrow();
    require(
        requests.len() == 2,
        "shadowed amount changed exchange count",
    )?;

    for (request, route) in requests.iter().zip(["order", "build"]) {
        require(
            request.path_segments == ["swap", "v2", route]
                && request.method == rest::Method::Get
                && request.query.len() == 3
                && request.query[2] == ("amount".into(), "1".into()),
            "shadowed amount changed emitted query or route",
        )?;
    }

    Ok(())
}

fn many_query_overrides() -> Result<(), &'static str> {
    let mut request = order();
    request.slippage_bps = Some(INITIAL_SLIPPAGE_BPS);
    request
        .extra_query_parameters
        .insert("amount".into(), "2".into());
    request
        .extra_query_parameters
        .insert("slippageBps".into(), "3".into());
    request
        .extra_query_parameters
        .insert("taker".into(), "custom".into());
    request
        .extra_query_parameters
        .insert("z".into(), "last".into());
    request
        .extra_query_parameters
        .insert("a".into(), "first".into());

    for index in 0..32 {
        request
            .extra_query_parameters
            .insert(format!("extra{index:02}"), "value".into());
    }

    let expected = request.to_query_parameters();
    let state = Rc::new(State::default());
    let client = Peer::new(state.clone(), b"{}").client(Budgets::default());
    finish(client.get_order(&request))?.map_err(|_| "many query parameters rejected")?;
    let requests = state.requests.borrow();
    require(
        requests[0].query == expected
            && expected[2] == ("amount".into(), "2".into())
            && expected[3] == ("slippageBps".into(), "3".into())
            && expected[4] == ("a".into(), "first".into())
            && expected.last() == Some(&("z".into(), "last".into())),
        "many query parameters changed sorted or override order",
    )
}

fn check_routes(state: &State, request: &OrderRequest) -> Result<(), &'static str> {
    let requests = state.requests.borrow();
    let expected = vec![
        ("inputMint".into(), request.input_mint.to_string()),
        ("outputMint".into(), request.output_mint.to_string()),
        ("amount".into(), LARGE_AMOUNT.into()),
        ("slippageBps".into(), "31".into()),
        ("swapMode".into(), "ExactOut".into()),
        ("referralAccount".into(), request.input_mint.to_string()),
        ("referralFee".into(), "9".into()),
        ("restrictIntermediateTokens".into(), "false".into()),
        ("platformFeeBps".into(), "7".into()),
        ("excludeDirectRouterRoutes".into(), "true".into()),
        ("payer".into(), request.output_mint.to_string()),
        ("taker".into(), request.input_mint.to_string()),
        ("extra".into(), "a +/雪".into()),
    ];
    require(requests.len() == 2, "swap route count changed")?;

    for (request, route) in requests.iter().zip(["order", "build"]) {
        require(
            request.path_segments == ["swap", "v2", route],
            "swap path changed",
        )?;
        require(
            request.method == rest::Method::Get && request.body.as_deref().is_none(),
            "swap read body or method changed",
        )?;
        require(
            request.query == expected,
            "swap query values or override order changed",
        )?;
        let target =
            rest::request_target("/", &request.path_segments, &request.query, TARGET_BYTES)
                .map_err(|_| "swap query encoding failed")?;
        require(
            target.ends_with("extra=a%20%2B%2F%E9%9B%AA"),
            "extra parameter escaped its query boundary",
        )?;
    }

    Ok(())
}

pub(super) fn execute() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let client = Peer::new(state.clone(), b"{}").client(Budgets::default());
    let mut order = OrderResponse::default();

    for identifier in [None, Some(""), Some(" \t\n")] {
        order.request_id = identifier.map(str::to_string);
        require(
            matches!(
                finish(client.execute_order(&order, "signed"))?,
                Err(Error::MissingRequestId)
            ),
            "blank request identifier accepted",
        )?;
    }

    require(
        state.requests.borrow().is_empty(),
        "invalid execute request reached adapter",
    )?;
    order.request_id = Some(" opaque-id ".into());
    order.last_valid_block_height = Some(
        "1844674407370955161600001"
            .parse()
            .map_err(|_| "integer fixture failed")?,
    );
    finish(client.execute_order(&order, "signed+/=="))?.map_err(|_| "execute failed")?;
    order.last_valid_block_height = None;
    finish(client.execute_order(&order, "signed"))?.map_err(|_| "execute without expiry failed")?;
    check_execution(&state)
}

fn check_execution(state: &State) -> Result<(), &'static str> {
    let requests = state.requests.borrow();
    let expected = br#"{"requestId":" opaque-id ","signedTransaction":"signed+/==","lastValidBlockHeight":"1844674407370955161600001"}"#;
    require(
        requests[0].body.as_deref() == Some(expected.as_slice()),
        "execute exact string wire body changed",
    )?;
    require(
        requests[1].body.as_deref()
            == Some(br#"{"requestId":" opaque-id ","signedTransaction":"signed"}"#.as_slice()),
        "absent expiry serialized on execute",
    )?;

    for request in requests.iter() {
        require(
            request.method == rest::Method::Post
                && request.path_segments == ["swap", "v2", "execute"],
            "execute route changed",
        )?;
        require(
            request.query.is_empty() && request.body.content_type() == Some("application/json"),
            "execute query or media type changed",
        )?;
    }

    Ok(())
}
