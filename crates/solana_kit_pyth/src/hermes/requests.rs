//! Ordered decoded query pairs retain repeated feed IDs without selecting a root.
use super::{AssetType, Encoding};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use solana_kit_client::rest::{Budgets, JsonError};

/// Options shared by latest and historical price-update reads.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UpdateOptions {
    /// Returned binary chunk encoding; hexadecimal by default.
    pub encoding: Encoding,
    /// Request parsed records; false by default.
    pub parsed: bool,
    /// Skip unknown IDs instead of rejecting them; omitted when false.
    pub ignore_invalid_price_ids: bool,
}

/// Bounded listing parameters in the same order as the pinned reference.
pub(super) fn listing(
    query: Option<&str>,
    asset: Option<AssetType>,
    limits: Budgets,
) -> Result<Vec<(String, String)>, JsonError> {
    let mut output = Query::new(limits);

    if let Some(query) = query {
        output.push("query", query)?;
    }

    if let Some(asset) = asset {
        output.push("asset_type", asset.wire_name())?;
    }

    Ok(output.pairs)
}

/// IDs precede options and remain verbatim, including duplicates and 0x prefixes.
pub(super) fn updates(
    ids: &[&str],
    options: UpdateOptions,
    limits: Budgets,
) -> Result<Vec<(String, String)>, JsonError> {
    let mut output = Query::new(limits);

    for id in ids {
        output.push("ids[]", id)?;
    }

    output.push("encoding", options.encoding.wire_name())?;
    output.push("parsed", if options.parsed { "true" } else { "false" })?;

    if options.ignore_invalid_price_ids {
        output.push("ignore_invalid_price_ids", "true")?;
    }

    Ok(output.pairs)
}

/// Admission precedes every caller-controlled string allocation.
struct Query {
    pairs: Vec<(String, String)>,
    bytes: usize,
    limits: Budgets,
}

impl Query {
    fn new(limits: Budgets) -> Self {
        Self {
            pairs: Vec::new(),
            bytes: 0,
            limits,
        }
    }

    fn push(&mut self, key: &str, value: &str) -> Result<(), JsonError> {
        if self.pairs.len() >= self.limits.items {
            return Err(JsonError::Items);
        }

        if key.len().max(value.len()) > self.limits.string_bytes {
            return Err(JsonError::String);
        }

        self.bytes = self
            .bytes
            .checked_add(key.len())
            .and_then(|size| size.checked_add(value.len()))
            .filter(|size| *size <= self.limits.bytes)
            .ok_or(JsonError::Bytes)?;
        self.pairs.push((key.into(), value.into()));

        Ok(())
    }
}

/// Protocol route, with bounded components cloned only for an admitted attempt.
pub(super) struct Route {
    pub path: Vec<String>,
    pub query: Vec<(String, String)>,
}

impl Route {
    pub fn new(parts: &[&str], query: Vec<(String, String)>) -> Self {
        Self {
            path: parts.iter().map(ToString::to_string).collect(),
            query,
        }
    }
}
