//! Shared native/portable assertions report mismatches without unwinding a guest.

pub type Outcome = Result<(), &'static str>;

pub fn require(condition: bool, explanation: &'static str) -> Outcome {
    condition.then_some(()).ok_or(explanation)
}
