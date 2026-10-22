//! Accepted literal and strict Token groups appended after every earlier case.

pub(crate) fn cases() -> [crate::CaseResult; 2] {
    [
        crate::case("token-literal-oracles", super::token_oracle::run()),
        crate::case("token-strict-errors", super::token_strict::run()),
    ]
}
