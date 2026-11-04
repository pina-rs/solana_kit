//! Unicode-safe text bounds shared by normalized projections and metadata generation.

/// Truncates to at most `maximum` UTF-16 code units without splitting a scalar.
///
/// The source applies a Dart `substring`, which can end between UTF-16 surrogate
/// halves. Rust cannot represent that value, so a character that would cross the
/// bound is omitted instead of manufacturing an unpaired surrogate.
pub(crate) fn truncate_utf16(value: &str, maximum: usize) -> String {
    let mut units = 0;

    value
        .chars()
        .take_while(|character| {
            units += character.len_utf16();
            units <= maximum
        })
        .collect()
}
