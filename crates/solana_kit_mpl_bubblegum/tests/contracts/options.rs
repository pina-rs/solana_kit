//! Independent option tags cover absent, false and present payloads separately.
use super::fixtures::{LIMITS, metadata};
use solana_kit_core::codecs::{Decoder, Encoder};
use solana_kit_mpl_bubblegum::*;

/// Exercise the full cross product: absent and false have different wire representations.
pub fn payload_options() {
    for asset in [None, Some(vec![0xa1, 0xb2])] {
        for schema in [None, Some(AssetDataSchema::Json)] {
            mint_option_case(asset.clone(), schema);
        }
    }

    for public in [None, Some(false), Some(true)] {
        let value = CreateTree {
            max_depth: 14,
            max_buffer_size: 64,
            public,
        };
        let bytes = CreateTreeCodec.encode(&value, LIMITS.max_bytes).unwrap();

        let expected = match public {
            None => vec![0],
            Some(false) => vec![1, 0],
            Some(true) => vec![1, 1],
        };

        assert_eq!(&bytes[16..], expected);
        assert_eq!(CreateTreeCodec.decode(&bytes, LIMITS).unwrap(), value);
    }

    let invalid = [
        vec![120, 121, 23, 146, 173, 110, 199, 205],
        metadata(true),
        vec![2, 0],
    ]
    .concat();
    assert!(MintV2Codec.decode(&invalid, LIMITS).is_err());
}

fn mint_option_case(asset: Option<Vec<u8>>, schema: Option<AssetDataSchema>) {
    let value = MintV2 {
        metadata_args: MetadataArgsV2Codec.decode(&metadata(true), LIMITS).unwrap(),
        asset_data: asset.clone(),
        asset_data_schema: schema.clone(),
    };
    let mut bytes = vec![120, 121, 23, 146, 173, 110, 199, 205];
    bytes.extend(metadata(true));

    match &asset {
        None => bytes.push(0),
        Some(data) => {
            bytes.extend([1, 2, 0, 0, 0]);
            bytes.extend(data);
        }
    }

    match schema {
        None => bytes.push(0),
        Some(AssetDataSchema::Json) => bytes.extend([1, 1]),
        _ => unreachable!(),
    }

    assert_eq!(MintV2Codec.encode(&value, LIMITS.max_bytes).unwrap(), bytes);
    assert_eq!(MintV2Codec.decode(&bytes, LIMITS).unwrap(), value);
}

#[test]
fn optional_mint_fields_and_tree_publicness_are_independent() {
    payload_options();
}
