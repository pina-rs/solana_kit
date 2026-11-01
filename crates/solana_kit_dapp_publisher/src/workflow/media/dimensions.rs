//! Image headers are inspected without decompression, host processes or pixel allocations.

use crate::workflow::Error;

const PNG_WIDTH: usize = 16;
const PNG_HEIGHT: usize = 20;
const GIF_WIDTH: usize = 6;
const GIF_HEIGHT: usize = 8;
const MIN_JPEG_SEGMENT: usize = 10;
const SOF_HEIGHT: usize = 5;
const SOF_WIDTH: usize = 7;

const MIN_WEBP_HEADER: usize = 30;
const WEBP_TYPE: std::ops::Range<usize> = 12..16;
const VP8X_WIDTH: std::ops::Range<usize> = 24..27;
const VP8X_HEIGHT: std::ops::Range<usize> = 27..30;
const VP8L_BITS: std::ops::Range<usize> = 21..25;

const VP8_WIDTH: usize = 26;
const VP8_HEIGHT: usize = 28;
const WEBP_DIMENSION_BITS: u32 = 14;
const WEBP_DIMENSION_MASK: u32 = 0x3fff;
const WEBP_SIGNATURE: std::ops::Range<usize> = 8..12;

/// Nonzero dimensions reported by one supported image header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MediaDimensions {
    /// Width in pixels; not a claim that the complete image is decodable.
    pub width: u32,
    /// Height in pixels; no pixel buffer is allocated.
    pub height: u32,
}

/// Reads PNG, JPEG SOF0–3, GIF or WebP VP8X/VP8L/VP8 dimensions.
///
/// This preserves the source's header-only contract. It does not authenticate
/// a MIME type or validate compressed contents. Empty/truncated/zero headers reject.
pub fn read_image_dimensions(bytes: &[u8]) -> Result<MediaDimensions, Error> {
    let (width, height) =
        header(bytes).ok_or(Error::Invalid("unsupported or truncated image header"))?;

    if width == 0 || height == 0 {
        return Err(Error::Invalid("zero image dimensions"));
    }

    Ok(MediaDimensions { width, height })
}

fn header(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some((be32(bytes, PNG_WIDTH)?, be32(bytes, PNG_HEIGHT)?));
    }

    if bytes.starts_with(b"\xff\xd8") {
        return jpeg(bytes);
    }

    if bytes.starts_with(b"GIF8") {
        return Some((le16(bytes, GIF_WIDTH)?, le16(bytes, GIF_HEIGHT)?));
    }

    if bytes.starts_with(b"RIFF") && bytes.get(WEBP_SIGNATURE) == Some(b"WEBP") {
        return webp(bytes);
    }

    None
}

fn be32(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_be_bytes(
        bytes
            .get(offset..offset.checked_add(size_of::<u32>())?)?
            .try_into()
            .ok()?,
    ))
}

fn be16(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from(u16::from_be_bytes(
        bytes
            .get(offset..offset.checked_add(size_of::<u16>())?)?
            .try_into()
            .ok()?,
    )))
}

fn le16(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from(u16::from_le_bytes(
        bytes
            .get(offset..offset.checked_add(size_of::<u16>())?)?
            .try_into()
            .ok()?,
    )))
}

/// Segment lengths advance monotonically; truncated and non-progressing segments cannot loop.
fn jpeg(bytes: &[u8]) -> Option<(u32, u32)> {
    let mut offset = 2usize;

    while bytes.len().saturating_sub(offset) >= MIN_JPEG_SEGMENT {
        if bytes[offset] != 0xff {
            offset += 1;
            continue;
        }

        let marker = bytes[offset + 1];

        if (0xc0..=0xc3).contains(&marker) {
            return Some((
                be16(bytes, offset + SOF_WIDTH)?,
                be16(bytes, offset + SOF_HEIGHT)?,
            ));
        }

        if (0xd0..=0xd9).contains(&marker) {
            offset += 2;
            continue;
        }

        let length = usize::try_from(be16(bytes, offset + 2)?).ok()?;

        if length < 2 {
            return None;
        }

        offset = offset.checked_add(2)?.checked_add(length)?;
    }

    None
}

fn webp(bytes: &[u8]) -> Option<(u32, u32)> {
    let data = bytes.get(..MIN_WEBP_HEADER)?;

    match data.get(WEBP_TYPE)? {
        b"VP8X" => Some((
            1 + le24(data.get(VP8X_WIDTH)?)?,
            1 + le24(data.get(VP8X_HEIGHT)?)?,
        )),
        b"VP8L" => {
            let bits = u32::from_le_bytes(data.get(VP8L_BITS)?.try_into().ok()?);
            Some((
                (bits & WEBP_DIMENSION_MASK) + 1,
                ((bits >> WEBP_DIMENSION_BITS) & WEBP_DIMENSION_MASK) + 1,
            ))
        }
        b"VP8 " => Some((
            le16(data, VP8_WIDTH)? & WEBP_DIMENSION_MASK,
            le16(data, VP8_HEIGHT)? & WEBP_DIMENSION_MASK,
        )),
        _ => None,
    }
}

fn le24(bytes: &[u8]) -> Option<u32> {
    let [low, middle, high]: [u8; 3] = bytes.try_into().ok()?;
    Some(u32::from_le_bytes([low, middle, high, 0]))
}
