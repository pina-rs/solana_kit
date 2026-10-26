//! Exact optionality, decimals and binary preconfirmation vectors.

use solana_kit_client::methods::Method;
use solana_kit_helius::{Decimal, Field, das::GetAssetRequest, preconf};

const COMPLETE_FRAME_BYTES: usize = 152;
const TRUNCATED_FRAME_BYTES: usize = COMPLETE_FRAME_BYTES - 1;
const TRANSACTION_OFFSET: usize = 18;

/// Missing, null and false remain observably distinct; decimals never pass through f64.
pub fn presence_and_decimal_precision() {
    let request = GetAssetRequest {
        id: "a".into(),
        options: None,
    };
    assert_eq!(
        request.decode(r#"{"id":"a"}"#).expect("missing").burnt,
        Field::Missing
    );
    assert_eq!(
        request
            .decode(r#"{"id":"a","burnt":null}"#)
            .expect("null")
            .burnt,
        Field::Null
    );
    assert_eq!(
        request
            .decode(r#"{"id":"a","burnt":false}"#)
            .expect("false")
            .burnt,
        Field::Value(false)
    );
    let decimal: Decimal = "9007199254740993.125".parse().expect("exact decimal");
    assert_eq!(
        serde_json::to_string(&decimal).expect("decimal serialization"),
        "9007199254740993.125"
    );
    assert!("\"1.0\"".parse::<Decimal>().is_err());
    assert!(r#"{"$serde_json::private::Number":"1.0"}"#.parse::<Decimal>().is_err());
}

/// Literal little-endian header fields and a minimal structural transaction are preserved.
pub fn preconfirmation_binary_contracts() {
    let mut frame = literal_frame();
    let decoded =
        preconf::decode_frame(&frame, COMPLETE_FRAME_BYTES).expect("complete literal frame");
    assert_eq!(decoded.slot, u64::MAX);
    assert_eq!(decoded.transaction_index, 9_007_199_254_740_993);
    assert_eq!(decoded.status, preconf::Status::Success);
    assert_eq!(decoded.transaction_bytes, frame[TRANSACTION_OFFSET..]);
    assert_eq!(
        preconf::decode_frame(&frame, TRUNCATED_FRAME_BYTES),
        Err(preconf::Error::BodyLimit)
    );

    status_mapping(&mut frame);
    invalid_frames(&mut frame);
}

fn literal_frame() -> Vec<u8> {
    let mut frame = vec![0; COMPLETE_FRAME_BYTES];
    frame[0] = 1;
    frame[1..9].copy_from_slice(&u64::MAX.to_le_bytes());
    frame[9..17].copy_from_slice(&9_007_199_254_740_993_u64.to_le_bytes());
    frame[17] = 1;
    frame[18] = 1;
    frame[83] = 1;
    frame[86] = 1;

    frame
}

fn status_mapping(frame: &mut [u8]) {
    for status in [0, 2, 255] {
        frame[17] = status;
        let expected = if status == 0 {
            preconf::Status::Failed
        } else {
            preconf::Status::Unknown
        };

        assert_eq!(
            preconf::decode_frame(frame, COMPLETE_FRAME_BYTES)
                .expect("status mapping")
                .status,
            expected
        );
    }
}

fn invalid_frames(frame: &mut [u8]) {
    frame[0] = 2;
    assert_eq!(
        preconf::decode_frame(frame, COMPLETE_FRAME_BYTES),
        Err(preconf::Error::Version(2))
    );
    assert_eq!(
        preconf::decode_frame(&frame[..TRANSACTION_OFFSET], COMPLETE_FRAME_BYTES),
        Err(preconf::Error::TooShort)
    );
    frame[0] = 1;
    assert!(matches!(
        preconf::decode_frame(&frame[..TRUNCATED_FRAME_BYTES], COMPLETE_FRAME_BYTES),
        Err(preconf::Error::Transaction(_))
    ));
}
