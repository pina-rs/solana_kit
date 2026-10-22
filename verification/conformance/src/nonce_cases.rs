//! Accepted isolated nonce groups appended after every existing case.

use crate::{CaseResult, case};
use solana_kit_durable_nonce_verification as nonce;

pub(crate) fn cases() -> [CaseResult; 4] {
    [
        case("nonce-construction", nonce::construction()),
        case("nonce-wire", nonce::wire()),
        case("nonce-negatives", nonce::negatives()),
        case("nonce-signatures", nonce::signatures()),
    ]
}
