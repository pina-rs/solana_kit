//! Real value-bearing errors stay available only through explicit cause access.

use super::{failures::make_surfnet, fixture::poll_ready, require};
use solana_kit_client::{
    Client, Error, Limits, Presence, Reply, Request, Transport, TransportError,
    TransportFailureKind,
};
use solana_kit_surfpool::{
    Surfnet, cheatcodes::GetSurfnetInfo, deploy::DeployError, errors::SurfpoolRpcError,
};
use std::{error::Error as StdError, fmt, format, future::Ready, future::ready, string::ToString};

const HTTP_OK: u16 = 200;
const HTTP_UNAVAILABLE: u16 = 503;
const PROVIDER_SECRET: &str = "provider-secret";
const TRANSPORT_SECRET: &str = "transport-secret";
const CUSTOM_METHOD: &str = "surfnet_custom-method-secret";
const SURFNET_INFO: &str = "surfnet_getSurfnetInfo";

pub fn run() -> Result<(), &'static str> {
    provider_error()?;
    http_error()?;
    malformed_envelope()?;
    malformed_result()?;
    transport_error()
}

fn http_error() -> Result<(), &'static str> {
    let surfnet = make_surfnet(HTTP_UNAVAILABLE, PROVIDER_SECRET);
    let error = poll_ready(surfnet.execute(GetSurfnetInfo))?
        .err()
        .ok_or("HTTP failure unexpectedly succeeded")?;
    require(
        error.status_code() == Some(HTTP_UNAVAILABLE)
            && matches!(&error.cause, Error::Status(HTTP_UNAVAILABLE)),
        "HTTP failure lost its typed status",
    )?;
    redacted(&error, SURFNET_INFO)
}

fn provider_error() -> Result<(), &'static str> {
    let body = r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32099,"message":"provider-secret","data":{"secret":"provider-secret"}}}"#;
    let surfnet = make_surfnet(HTTP_OK, body);
    let error = poll_ready(surfnet.execute(GetSurfnetInfo))?
        .err()
        .ok_or("provider error unexpectedly succeeded")?;
    let remote = error.remote().ok_or("provider cause missing")?;
    let retained_data =
        matches!(&remote.data, Presence::Present(raw) if raw.get().contains(PROVIDER_SECRET));

    require(
        error.message() == Some(PROVIDER_SECRET) && retained_data,
        "explicit provider message or data changed",
    )?;
    redacted(&error, SURFNET_INFO)?;
    redacted(&DeployError::Rpc(error), SURFNET_INFO)
}

fn malformed_envelope() -> Result<(), &'static str> {
    let body = r#"{"jsonrpc":"2.0","id":"provider-secret","result":{}}"#;
    let surfnet = make_surfnet(HTTP_OK, body);
    let error = poll_ready(surfnet.execute(GetSurfnetInfo))?
        .err()
        .ok_or("string envelope id unexpectedly accepted")?;
    require(
        matches!(&error.cause, Error::Json(cause) if cause.to_string().contains(PROVIDER_SECRET)),
        "envelope fixture did not produce the value-bearing JSON cause",
    )?;
    redacted(&error, SURFNET_INFO)
}

fn malformed_result() -> Result<(), &'static str> {
    let body = r#"{"jsonrpc":"2.0","id":1,"result":"provider-secret"}"#;
    let surfnet = make_surfnet(HTTP_OK, body);
    let error = poll_ready(surfnet.minimum_balance(0))?
        .err()
        .ok_or("string rent value unexpectedly accepted")?;
    require(
        matches!(&error.cause, Error::Schema(cause) if cause.to_string().contains(PROVIDER_SECRET)),
        "typed fixture did not produce the value-bearing schema cause",
    )?;
    redacted(&error, "getMinimumBalanceForRentExemption")
}

fn transport_error() -> Result<(), &'static str> {
    let surfnet = Surfnet::new(Client::new(FailingTransport, Limits::default()));
    let error = poll_ready(surfnet.execute(GetSurfnetInfo))?
        .err()
        .ok_or("failing adapter unexpectedly succeeded")?;
    require(
        matches!(&error.cause, Error::Transport(cause) if cause.kind == TransportFailureKind::Io),
        "transport category changed",
    )?;
    let cause = error.cause.source().ok_or("adapter error discarded")?;
    let source = cause.source().ok_or("adapter source discarded")?;
    require(
        source.to_string() == TRANSPORT_SECRET
            && source
                .source()
                .is_some_and(|nested| nested.to_string() == PROVIDER_SECRET)
            && format!("{:?}", error.cause).contains(TRANSPORT_SECRET),
        "explicit nested adapter cause changed",
    )?;
    redacted(&error, SURFNET_INFO)?;

    let custom = SurfpoolRpcError {
        method: CUSTOM_METHOD,
        cause: error.cause,
    };
    require(
        custom.method == CUSTOM_METHOD,
        "original custom name changed",
    )?;
    redacted(&custom, "<custom>")
}

/// Check each formatted error reached by an ordinary recursive source logger.
fn redacted(error: &dyn StdError, method: &str) -> Result<(), &'static str> {
    let debug = format!("{error:?}");
    let display = format!("{error}");
    require(
        debug.contains(method) && display.contains(method),
        "approved diagnostic method context missing",
    )?;
    let mut current = Some(error);

    while let Some(cause) = current {
        let rendered = format!("{cause:?} {cause} {cause:#?}");

        for secret in [PROVIDER_SECRET, TRANSPORT_SECRET, CUSTOM_METHOD] {
            require(
                !rendered.contains(secret),
                "implicit error diagnostic leaked",
            )?;
        }

        current = cause.source();
    }

    Ok(())
}

struct FailingTransport;

impl Transport for FailingTransport {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, _request: Request) -> Self::Exchange<'_> {
        ready(Err(TransportError::with_source(
            TransportFailureKind::Io,
            AdapterCause(TRANSPORT_SECRET),
        )))
    }
}

#[derive(Debug)]
struct AdapterCause(&'static str);

impl fmt::Display for AdapterCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl StdError for AdapterCause {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&ProviderCause)
    }
}

#[derive(Debug)]
struct ProviderCause;

impl fmt::Display for ProviderCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(PROVIDER_SECRET)
    }
}

impl StdError for ProviderCause {}
