//! Isolated compile and native execution entry for off-chain framing cases.
#![no_std]

extern crate alloc;

#[path = "../../conformance/src/framing.rs"]
pub mod cases;

#[cfg(test)]
mod tests {
    #[test]
    fn portable_cases_execute() {
        super::cases::transforms().unwrap();
        super::cases::boundaries().unwrap();
    }
}
