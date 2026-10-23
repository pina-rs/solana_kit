//! Full accepted ordinary collection consumer bodies on every host.

mod absence;
mod boolean;
mod budgets;
mod counts;
mod markers;
mod minimum;

mod codec_framing;
mod codec_framing_accounting;
mod collection_layouts;
mod composite_accounting;
mod composite_codecs;
mod native_option_values;

mod registry;

pub(super) use registry::cases;
