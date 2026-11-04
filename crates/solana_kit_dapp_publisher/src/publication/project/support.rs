//! Shared presence and text helpers for normalized projection modules.

use crate::{portal::Optional, text::truncate_utf16};
use serde_json::Number;
use solana_kit_client::Presence;
use std::{string::String, vec::Vec};

const SHORT_DESCRIPTION_UTF16_UNITS: usize = 50;

pub(super) fn value<T>(field: &Optional<T>) -> Option<&T> {
    match field {
        Presence::Present(Some(value)) => Some(value),
        _ => None,
    }
}

pub(super) fn text(field: &Optional<String>) -> Option<&str> {
    value(field).map(String::as_str)
}

pub(super) fn first_present<'a>(values: &[Option<&'a str>]) -> Option<&'a str> {
    values.iter().copied().flatten().next()
}

pub(super) fn first_non_empty<'a>(values: &[Option<&'a str>]) -> Option<&'a str> {
    values
        .iter()
        .copied()
        .flatten()
        .find(|value| !value.is_empty())
}

pub(super) fn is_non_empty(value: Option<&str>) -> bool {
    value.is_some_and(|value| !value.is_empty())
}

pub(super) fn string_or_empty(field: &Optional<String>) -> String {
    text(field).unwrap_or_default().to_owned()
}

pub(super) fn optional_text<T>(
    record: Option<&T>,
    field: fn(&T) -> &Optional<String>,
) -> Option<String> {
    record
        .and_then(|record| text(field(record)))
        .map(str::to_owned)
}

pub(super) fn required_text<T>(record: Option<&T>, field: fn(&T) -> &Optional<String>) -> String {
    optional_text(record, field).unwrap_or_default()
}

pub(super) fn list<T>(record: Option<&T>, field: fn(&T) -> &Optional<Vec<String>>) -> Vec<String> {
    record
        .and_then(|record| value(field(record)))
        .cloned()
        .unwrap_or_default()
}

pub(super) fn optional_number<T>(
    record: Option<&T>,
    field: fn(&T) -> &Optional<Number>,
) -> Option<Number> {
    record.and_then(|record| value(field(record))).cloned()
}

pub(super) fn number_text(value: &Number) -> String {
    if value.as_f64().is_some_and(|number| number == 0.0) {
        String::new()
    } else {
        value.to_string()
    }
}

pub(super) fn bounded(description: &str) -> String {
    truncate_utf16(description, SHORT_DESCRIPTION_UTF16_UNITS)
}

pub(super) fn file_name_from_url(url: &str) -> Option<String> {
    if url.is_empty()
        || url
            .bytes()
            .any(|byte| byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return None;
    }

    let without_fragment = url.split('#').next().unwrap_or_default();
    let without_query = without_fragment.split('?').next().unwrap_or_default();
    let path = without_query
        .split_once("://")
        .map_or(without_query, |(_, rest)| {
            rest.split_once('/').map_or("", |(_, path)| path)
        });

    path.rsplit('/')
        .find(|segment| !segment.is_empty())
        .map(str::to_owned)
}
