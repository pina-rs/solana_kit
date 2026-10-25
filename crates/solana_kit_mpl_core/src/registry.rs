//! Checked absolute registry offsets and disjoint plugin/data ranges.
use crate::generated::*;
use alloc::vec::Vec;
use core::ops::Range;
use solana_kit_core::codecs::{Decoder, Encoder, Error, Limits, Reader};

/// One known typed plugin and its exact account byte range.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginEntry<T> {
    /// Decoded plugin or external adapter.
    pub value: T,
    /// Absolute bytes consumed by its codec.
    pub range: Range<usize>,
}

/// Complete registry with validated nonoverlapping references into one account.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginRegistry {
    /// Registry descriptors and authorities in source order.
    pub registry: PluginRegistryV1,
    /// Internal plugins in descriptor order.
    pub plugins: Vec<PluginEntry<Plugin>>,
    /// External adapters in descriptor order.
    pub external_plugins: Vec<PluginEntry<ExternalPluginAdapter>>,
    /// Optional external data spans in descriptor order.
    pub external_data: Vec<Option<Range<usize>>>,
}

/// Reads an asset base and validates its complete extension registry.
/// # Errors
/// Rejects malformed bases, offsets, overlaps, discriminators and caller budgets.
pub fn inspect_asset_plugins(bytes: &[u8], limits: Limits) -> Result<PluginRegistry, Error> {
    let mut reader = Reader::new(bytes, limits)?;
    AssetV1Codec.read(&mut reader)?;
    inspect(reader)
}

/// Reads a collection base and validates its complete extension registry.
/// # Errors
/// Rejects malformed bases, offsets, overlaps, discriminators and caller budgets.
pub fn inspect_collection_plugins(bytes: &[u8], limits: Limits) -> Result<PluginRegistry, Error> {
    let mut reader = Reader::new(bytes, limits)?;
    CollectionV1Codec.read(&mut reader)?;
    inspect(reader)
}

fn inspect(mut reader: Reader<'_>) -> Result<PluginRegistry, Error> {
    let header = PluginHeaderV1Codec.read(&mut reader)?;
    let start = reader.position();
    let offset = usize::try_from(header.plugin_registry_offset).map_err(|_| Error::Overflow)?;

    if offset < start {
        return Err(Error::InvalidValue);
    }

    reader.seek(offset)?;
    let registry = PluginRegistryV1Codec.read(&mut reader)?;
    reader.finish()?;
    let internal = registry.registry.len();
    let external = registry.external_registry.len();
    let spans = internal
        .checked_add(external.checked_mul(2).ok_or(Error::Overflow)?)
        .ok_or(Error::Overflow)?;
    reader.claim(spans.checked_mul(2).ok_or(Error::Overflow)?)?;
    let mut ranges = Vec::new();

    ranges
        .try_reserve_exact(spans)
        .map_err(|_| Error::AllocationFailed)?;
    let mut result = PluginRegistry {
        registry,
        plugins: Vec::new(),
        external_plugins: Vec::new(),
        external_data: Vec::new(),
    };
    inspect_internal(&mut reader, &mut result, &mut ranges, start..offset)?;
    inspect_external(&mut reader, &mut result, &mut ranges, start..offset)?;
    Ok(result)
}

/// Internal descriptors share the same absolute bounds and overlap ledger as
/// external adapters; their original tag and allocation checks remain intact.
fn inspect_internal(
    reader: &mut Reader<'_>,
    result: &mut PluginRegistry,
    ranges: &mut Vec<Range<usize>>,
    bounds: Range<usize>,
) -> Result<(), Error> {
    result
        .plugins
        .try_reserve_exact(result.registry.registry.len())
        .map_err(|_| Error::AllocationFailed)?;

    for record in &result.registry.registry {
        let entry = read_entry(
            reader,
            record.offset,
            &PluginCodec,
            bounds.start,
            bounds.end,
        )?;
        check_tag(
            reader.bytes(),
            &entry.range,
            &PluginTypeCodec,
            &record.plugin_type,
        )?;
        insert(ranges, entry.range.clone())?;
        result.plugins.push(entry);
    }

    Ok(())
}

/// External payloads and optional data spans must remain disjoint from every
/// earlier internal or external entry, using one shared range ledger.
fn inspect_external(
    reader: &mut Reader<'_>,
    result: &mut PluginRegistry,
    ranges: &mut Vec<Range<usize>>,
    bounds: Range<usize>,
) -> Result<(), Error> {
    result
        .external_plugins
        .try_reserve_exact(result.registry.external_registry.len())
        .map_err(|_| Error::AllocationFailed)?;
    result
        .external_data
        .try_reserve_exact(result.registry.external_registry.len())
        .map_err(|_| Error::AllocationFailed)?;

    for record in &result.registry.external_registry {
        let entry = read_entry(
            reader,
            record.offset,
            &ExternalPluginAdapterCodec,
            bounds.start,
            bounds.end,
        )?;
        check_tag(
            reader.bytes(),
            &entry.range,
            &ExternalPluginAdapterTypeCodec,
            &record.plugin_type,
        )?;
        insert(ranges, entry.range.clone())?;
        let data = data_range(
            record.data_offset,
            record.data_len,
            bounds.start,
            bounds.end,
        )?;

        if let Some(range) = &data {
            insert(ranges, range.clone())?;
        }

        result.external_plugins.push(entry);
        result.external_data.push(data);
    }

    Ok(())
}

fn read_entry<T>(
    reader: &mut Reader<'_>,
    offset: u64,
    codec: &impl Decoder<T>,
    minimum: usize,
    maximum: usize,
) -> Result<PluginEntry<T>, Error> {
    let start = usize::try_from(offset).map_err(|_| Error::Overflow)?;

    if start < minimum || start >= maximum {
        return Err(Error::InvalidValue);
    }

    reader.seek(start)?;
    let value = codec.read(reader)?;
    let end = reader.position();

    if end > maximum {
        return Err(Error::InvalidValue);
    }

    Ok(PluginEntry {
        value,
        range: start..end,
    })
}

fn check_tag<T>(
    bytes: &[u8],
    range: &Range<usize>,
    codec: &impl Encoder<T>,
    value: &T,
) -> Result<(), Error> {
    let expected = codec.encode(value, 1)?;

    if bytes.get(range.start) != expected.first() {
        return Err(Error::InvalidDiscriminator);
    }

    Ok(())
}

fn data_range(
    offset: Option<u64>,
    length: Option<u64>,
    minimum: usize,
    maximum: usize,
) -> Result<Option<Range<usize>>, Error> {
    match (offset, length) {
        (None, None) => Ok(None),
        (Some(offset), Some(length)) => {
            let start = usize::try_from(offset).map_err(|_| Error::Overflow)?;
            let length = usize::try_from(length).map_err(|_| Error::Overflow)?;
            let end = start.checked_add(length).ok_or(Error::Overflow)?;

            if start < minimum || end > maximum {
                return Err(Error::InvalidValue);
            }

            Ok(Some(start..end))
        }
        _ => Err(Error::InvalidValue),
    }
}

fn insert(ranges: &mut Vec<Range<usize>>, range: Range<usize>) -> Result<(), Error> {
    if ranges
        .iter()
        .any(|other| range.start < other.end && other.start < range.end)
    {
        return Err(Error::InvalidValue);
    }

    ranges.push(range);
    Ok(())
}
