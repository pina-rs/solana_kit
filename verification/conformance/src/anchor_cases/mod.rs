//! Complete host-neutral Anchor contracts with reportable assertion failures.

macro_rules! assert {
    ($condition:expr $(, $($message:tt)*)?) => {
        crate::require($condition, stringify!($condition))?
    };
}

macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        crate::require($left == $right, stringify!($left == $right))?
    };
}

pub(super) use {assert, assert_eq};

mod composition;
mod consumer;
mod errors_hash;
mod events;
mod idl;
mod registry;
mod review;
mod support;

mod wire;

pub(super) use registry::cases;
