//! Real shared-client admission and Surfpool error annotation failure paths.

use super::{
    fixture::{Recording, framed, poll_ready},
    require,
};
use alloc::{format, rc::Rc, vec::Vec};
use core::cell::{Cell, RefCell};
use solana_kit_client::{Client, Error, Limits, Presence};
use solana_kit_surfpool::{Surfnet, cheatcodes::*};

const HTTP_OK: u16 = 200;
const HTTP_UNAVAILABLE: u16 = 503;
const PROVIDER_ERROR_CODE: i64 = -32099;
const OVERSIZED_RESULT_BYTES: usize = 256;

pub fn run() -> Result<(), &'static str> {
    require(
        matches!(
            framed(
                GetSurfnetInfo,
                HTTP_OK,
                r#"{"jsonrpc":"2.0","id":2,"result":{}}"#,
                Limits::default()
            )?,
            Err(Error::Correlation)
        ),
        "correlation mismatch accepted",
    )?;
    require(
        matches!(
            framed(
                GetSurfnetInfo,
                HTTP_OK,
                r#"{"jsonrpc":"2.0","id":1,"result":{},"error":{}}"#,
                Limits::default()
            )?,
            Err(Error::Envelope)
        ),
        "ambiguous envelope accepted",
    )?;
    require(
        matches!(
            framed(
                PauseClock,
                HTTP_OK,
                r#"{"jsonrpc":"2.0","id":1,"result":{}}"#,
                Limits::default()
            )?,
            Err(Error::Schema(_))
        ),
        "missing epoch fields accepted",
    )?;
    require(
        matches!(
            framed(GetSurfnetInfo, HTTP_OK, "not-json", Limits::default())?,
            Err(Error::Json(_))
        ),
        "invalid JSON accepted",
    )?;
    budgets()?;
    annotated_error()?;
    super::privacy::run()
}

fn budgets() -> Result<(), &'static str> {
    let small = Limits {
        bytes: 8,
        depth: 32,
    };
    require(
        matches!(
            framed(GetSurfnetInfo, HTTP_OK, "{}", small)?,
            Err(Error::BodyLimit)
        ),
        "request budget was bypassed",
    )?;
    let nested = r#"{"jsonrpc":"2.0","id":1,"result":[[[[[0]]]]]}"#;
    require(
        matches!(
            framed(
                GetSurfnetInfo,
                HTTP_OK,
                nested,
                Limits {
                    bytes: 1024,
                    depth: 3
                }
            )?,
            Err(Error::DepthLimit)
        ),
        "response nesting budget was bypassed",
    )?;
    let large = format!(
        r#"{{"jsonrpc":"2.0","id":1,"result":"{}"}}"#,
        "x".repeat(OVERSIZED_RESULT_BYTES)
    );
    require(
        matches!(
            framed(
                GetSurfnetInfo,
                HTTP_OK,
                &large,
                Limits {
                    bytes: 128,
                    depth: 32
                }
            )?,
            Err(Error::BodyLimit)
        ),
        "response byte budget was bypassed",
    )
}

fn annotated_error() -> Result<(), &'static str> {
    let body = r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32099,"message":"provider original","data":{"quantity":18446744073709551615,"secret":"preserved"}}}"#;
    let surfnet = make_surfnet(HTTP_OK, body);
    let error = poll_ready(surfnet.execute(GetSurfnetInfo))?
        .err()
        .ok_or("remote error unexpectedly succeeded")?;
    require(
        error.method == "surfnet_getSurfnetInfo"
            && error.rpc_code() == Some(PROVIDER_ERROR_CODE)
            && error.message() == Some("provider original"),
        "Surfpool error fields changed",
    )?;

    let data = match &error.remote().ok_or("remote cause missing")?.data {
        Presence::Present(data) => data.get(),
        Presence::Missing => return Err("remote data was discarded"),
    };

    require(
        data == r#"{"quantity":18446744073709551615,"secret":"preserved"}"#,
        "remote data was altered or redacted",
    )?;
    let status = make_surfnet(HTTP_UNAVAILABLE, "unavailable");
    let error = poll_ready(status.execute(GetSurfnetInfo))?
        .err()
        .ok_or("HTTP error unexpectedly succeeded")?;

    require(
        error.status_code() == Some(HTTP_UNAVAILABLE) && error.rpc_code().is_none(),
        "HTTP failure lost its typed status",
    )
}

pub(super) fn make_surfnet(status: u16, body: &str) -> Surfnet<Recording> {
    Surfnet::new(Client::new(
        Recording {
            requests: Rc::new(RefCell::new(Vec::new())),
            body: body.into(),
            status,
            budget: Rc::new(Cell::new(0)),
        },
        Limits::default(),
    ))
}
