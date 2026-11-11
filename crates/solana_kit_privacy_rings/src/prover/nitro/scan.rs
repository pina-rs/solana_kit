//! CBOR subset both the Go prover and this client accept.
//!
//! One leading COSE tag is allowed. Floats, indefinite strings, integers
//! outside `±(2^53-1)`, and any other tag are rejected before a document is
//! trusted.

use ciborium_ll::{Decoder, Header, simple};

use crate::Error;

const MAX_INTEGER: u64 = (1 << 53) - 1;
const MAX_DEPTH: usize = 16;

/// Walks one document iteratively. A tag is legal only as the first header,
/// and a finished value has to end exactly at the last byte.
pub(super) fn scan(cbor: &[u8], leading_tag: Option<u64>) -> Result<(), Error> {
    let mut rest = cbor;
    let mut open: Vec<Option<usize>> = Vec::new();
    let mut tag_allowed = leading_tag;

    loop {
        let mut decoder = Decoder::from(rest);
        let header = decoder.pull().map_err(|_| bad())?;
        let width = decoder.offset();
        rest = &rest[width..];
        let finished = accept(header, width, &mut tag_allowed, &mut rest, &mut open)?;
        tag_allowed = None;

        if finished && closes(&mut open) {
            if rest.is_empty() {
                return Ok(());
            }

            return Err(reason("trailing"));
        }
    }
}

fn accept(
    header: Header,
    width: usize,
    tag_allowed: &mut Option<u64>,
    rest: &mut &[u8],
    open: &mut Vec<Option<usize>>,
) -> Result<bool, Error> {
    match header {
        Header::Tag(tag) => tagged(tag, tag_allowed),
        Header::Float(_) => Err(reason("float")),
        Header::Simple(value) => simple_value(value, width),
        Header::Positive(value) => integer(value, false),
        Header::Negative(value) => integer(value, true),
        Header::Bytes(len) => borrowed(rest, len, false),
        Header::Text(len) => borrowed(rest, len, true),
        Header::Array(items) => entered(open, items, false),
        Header::Map(items) => entered(open, items, true),
        Header::Break => ended(open),
    }
}

fn tagged(tag: u64, allowed: &mut Option<u64>) -> Result<bool, Error> {
    if allowed.take() == Some(tag) {
        return Ok(false);
    }

    Err(reason("tag"))
}

fn simple_value(value: u8, width: usize) -> Result<bool, Error> {
    if width == 1 && matches!(value, simple::FALSE | simple::TRUE | simple::NULL) {
        return Ok(true);
    }

    Err(reason("simple"))
}

fn integer(value: u64, negative: bool) -> Result<bool, Error> {
    let in_range = if negative {
        value < MAX_INTEGER
    } else {
        value <= MAX_INTEGER
    };

    if in_range {
        return Ok(true);
    }

    Err(reason("integer"))
}

fn borrowed(rest: &mut &[u8], len: Option<usize>, text: bool) -> Result<bool, Error> {
    let Some(len) = len else {
        return Err(reason("indefinite string"));
    };
    let bytes = take(rest, len)?;

    if text {
        std::str::from_utf8(bytes).map_err(|_| reason("utf8"))?;
    }

    Ok(true)
}

fn entered(open: &mut Vec<Option<usize>>, items: Option<usize>, map: bool) -> Result<bool, Error> {
    if items == Some(0) {
        return Ok(true);
    }

    let items = match items {
        Some(pairs) if map => Some(pairs.checked_mul(2).ok_or_else(bad)?),
        other => other,
    };

    nest(open, items)?;
    Ok(false)
}

fn ended(open: &mut Vec<Option<usize>>) -> Result<bool, Error> {
    if open.last() == Some(&None) {
        open.pop();

        return Ok(true);
    }

    Err(bad())
}

fn take<'a>(rest: &mut &'a [u8], len: usize) -> Result<&'a [u8], Error> {
    let (taken, after) = rest.split_at_checked(len).ok_or_else(bad)?;
    *rest = after;
    Ok(taken)
}

fn nest(open: &mut Vec<Option<usize>>, items: Option<usize>) -> Result<(), Error> {
    open.push(items);

    if open.len() > MAX_DEPTH {
        return Err(reason("depth"));
    }

    Ok(())
}

fn closes(open: &mut Vec<Option<usize>>) -> bool {
    while let Some(last) = open.last_mut() {
        let Some(left) = last else {
            return false;
        };

        *left -= 1;

        if *left > 0 {
            return false;
        }

        open.pop();
    }

    true
}

fn bad() -> Error {
    reason("cbor")
}

fn reason(kind: &'static str) -> Error {
    Error::Attestation(kind)
}

#[cfg(test)]
mod tests {
    use super::scan;

    #[test]
    fn a_break_or_a_short_string_is_rejected() {
        assert!(scan(&[0xff], None).is_err());
        assert!(scan(&[0x45], None).is_err());
        assert!(
            scan(
                &[0xbb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
                None
            )
            .is_err()
        );
    }
}
