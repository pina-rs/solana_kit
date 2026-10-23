//! Pinned code definitions and message templates.
use super::{CodeDefinition, ErrorCode};
mod group_0;
mod group_1;
mod group_10;
mod group_11;
mod group_12;
mod group_13;
mod group_14;
mod group_15;
mod group_2;

mod group_3;
mod group_4;
mod group_5;
mod group_6;
mod group_7;
mod group_8;
mod group_9;
const GROUPS: &[&[CodeDefinition]] = &[
    group_0::DEFINITIONS,
    group_1::DEFINITIONS,
    group_2::DEFINITIONS,
    group_3::DEFINITIONS,
    group_4::DEFINITIONS,
    group_5::DEFINITIONS,
    group_6::DEFINITIONS,
    group_7::DEFINITIONS,
    group_8::DEFINITIONS,
    group_9::DEFINITIONS,
    group_10::DEFINITIONS,
    group_11::DEFINITIONS,
    group_12::DEFINITIONS,
    group_13::DEFINITIONS,
    group_14::DEFINITIONS,
    group_15::DEFINITIONS,
];
pub(super) fn definitions() -> impl Iterator<Item = &'static CodeDefinition> {
    GROUPS.iter().flat_map(|group| group.iter())
}
