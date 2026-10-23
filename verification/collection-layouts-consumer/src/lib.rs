//! Ordinary public collection contracts without repository flags.
extern crate alloc;

#[cfg(test)]
#[path = "../../../verification/collection-layouts-independent/absence.rs"]
mod absence;

#[cfg(test)]
#[path = "../../../verification/collection-layouts-independent/boolean.rs"]
mod boolean;

#[cfg(test)]
#[path = "../../../verification/collection-layouts-independent/budgets.rs"]
mod budgets;

#[cfg(test)]
#[path = "../../../verification/collection-layouts-independent/counts.rs"]
mod counts;

#[cfg(test)]
#[path = "../../../verification/collection-layouts-independent/markers.rs"]
mod markers;

#[cfg(test)]
#[path = "../../../verification/collection-layouts-independent/minimum.rs"]
mod minimum;

#[cfg(test)]
#[path = "../../../crates/solana_kit_core/tests/collection_layouts.rs"]
mod collection_layouts;

#[cfg(test)]
#[path = "../../../crates/solana_kit_core/tests/native_option_values.rs"]
mod native_option_values;

#[cfg(test)]
#[path = "../../../crates/solana_kit_core/tests/codec_framing.rs"]
mod codec_framing;

#[cfg(test)]
#[path = "../../../crates/solana_kit_core/tests/codec_framing_accounting.rs"]
mod codec_framing_accounting;

#[cfg(test)]
#[path = "../../../crates/solana_kit_core/tests/composite_codecs.rs"]
mod composite_codecs;

#[cfg(test)]
#[path = "../../../crates/solana_kit_core/tests/composite_accounting.rs"]
mod composite_accounting;
