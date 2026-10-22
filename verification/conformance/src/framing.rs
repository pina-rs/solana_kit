//! Isolated off-chain framing cases awaiting central runtime integration.

use solana_kit_core::codecs::{
    ByteSizePrefix, Bytes, Decoder, Encoder, Error, FixedSize, FrameConsumption, Limits, Number,
    Offset, OffsetScope, Reverse, Sentinel,
};

/// Checks independent literal prefix, fixed field and reversal bytes.
pub fn transforms() -> Result<(), &'static str> {
    let codec = ByteSizePrefix {
        item: Reverse {
            item: Number::<u16>::LE,
            size: 2,
        },
        prefix: Number::<u64>::LE,
        consumption: FrameConsumption::Exact,
    };
    let bytes = [2, 0, 0, 0, 0, 0, 0, 0, 1, 2];

    if codec.encode(&258, 10).map_err(|_| "framing encode")? != bytes {
        return Err("framing bytes");
    }

    if codec
        .decode(&bytes, Limits::new(10, 2))
        .map_err(|_| "framing decode")?
        != 258
    {
        return Err("framing value");
    }

    let field = FixedSize {
        item: Bytes,
        size: 2,
        decoded_size: None,
        allow_truncation: true,
        max_encoded_size: 3,
    };

    if field
        .encode(&[1, 2, 3][..], 2)
        .map_err(|_| "fixed encode")?
        != [1, 2]
    {
        return Err("fixed truncation");
    }

    Ok(())
}

/// Checks rewind, overlapping delimiters and exhausted scratch claims.
pub fn boundaries() -> Result<(), &'static str> {
    let codec = Offset {
        item: Number::<u8>::LE,
        pre: |scope: OffsetScope<'_>| Ok(scope.wrap(-1)),
        post: |scope: OffsetScope<'_>| Ok(scope.original),
    };

    if codec
        .decode_prefix(&[1, 2, 3], 1, Limits::new(3, 0))
        .map_err(|_| "offset read")?
        != (3, 1)
    {
        return Err("offset rewind");
    }

    let sentinel = Sentinel::dart(Bytes, &[1, 1], 2).map_err(|_| "delimiter setup")?;

    if sentinel
        .decode_prefix(&[1, 1, 1], 0, Limits::new(3, 0))
        .map_err(|_| "delimiter read")?
        != (alloc::vec![], 2)
    {
        return Err("delimiter overlap");
    }

    let reversed = Reverse {
        item: Number::<u16>::LE,
        size: 2,
    };

    if reversed.decode(&[1, 2], Limits::new(2, 1)) != Err(Error::LimitExceeded) {
        return Err("scratch budget");
    }

    Ok(())
}
