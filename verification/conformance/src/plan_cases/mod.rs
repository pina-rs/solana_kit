//! Full host-neutral planning contracts; Tokio spawn proof stays native.

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

mod asynchronous;
mod client;
mod core;
mod diagnostics;
mod execution;
mod independent_async;
mod packing;

mod polling;
mod registry;
mod resources;

pub(super) use registry::cases;
