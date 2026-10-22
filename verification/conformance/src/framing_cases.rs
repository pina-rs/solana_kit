//! Stable registration for the unchanged accepted framing assertions.

/// Framing follows the preserved first 83 case groups.
pub(crate) fn cases() -> [crate::CaseResult; 2] {
    [
        crate::case("codec-framing-transforms", crate::framing::transforms()),
        crate::case("codec-framing-boundaries", crate::framing::boundaries()),
    ]
}
