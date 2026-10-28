//! Portable swap contracts use the real client and Kit conversion boundaries.

mod build;
mod conversion;
mod malformed;
mod models;
mod ownership;
mod routes;
mod transport;
mod wire;

use crate::{CaseResult, case};

pub(crate) fn cases() -> [CaseResult; 10] {
    [
        case(
            "jupiter-swap-query-defaults-overrides-and-routes",
            routes::requests(),
        ),
        case(
            "jupiter-execute-request-id-and-string-expiry",
            routes::execute(),
        ),
        case(
            "jupiter-order-execution-precision-and-raw-response",
            models::responses(),
        ),
        case(
            "jupiter-build-optionality-and-ordered-lookup-fallback",
            build::responses(),
        ),
        case(
            "jupiter-swap-malformed-fields-and-json-admission",
            malformed::responses(),
        ),
        case(
            "jupiter-instruction-conversion-and-positional-roles",
            conversion::instructions(),
        ),
        case(
            "jupiter-lookup-and-blockhash-kit-conversion",
            conversion::metadata(),
        ),
        case(
            "jupiter-swap-transaction-wire-decoding",
            wire::transactions(),
        ),
        case(
            "jupiter-swap-single-exchange-error-and-budgets",
            ownership::failures(),
        ),
        case(
            "jupiter-swap-owned-future-cancellation",
            ownership::cancellation(),
        ),
    ]
}
