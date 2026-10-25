//! Every stable named enum tag is checked against its independent literal byte.
use super::fixtures::LIMITS;
use solana_kit_core::codecs::{Decoder, Encoder};
use solana_kit_mpl_bubblegum::*;

macro_rules! tags {
    ($codec:expr, $($value:expr),+ $(,)?) => {{
        let values = [$($value),+];

        for (tag, value) in values.iter().enumerate() {
            let bytes = [u8::try_from(tag).unwrap()];
            assert_eq!($codec.encode(value, 1).unwrap(), bytes);
            assert_eq!(&$codec.decode(&bytes, LIMITS).unwrap(), value);
        }

        assert!($codec.decode(&[u8::try_from(values.len()).unwrap()], LIMITS).is_err());
        assert!($codec.decode(&[], LIMITS).is_err());
        assert!($codec.decode(&[0, 0], LIMITS).is_err());
    }};
}

pub fn named_tags() {
    tags!(VersionCodec, Version::V1, Version::V2);
    tags!(
        TokenProgramVersionCodec,
        TokenProgramVersion::Original,
        TokenProgramVersion::Token2022
    );
    tags!(
        TokenStandardCodec,
        TokenStandard::NonFungible,
        TokenStandard::FungibleAsset,
        TokenStandard::Fungible,
        TokenStandard::NonFungibleEdition
    );
    tags!(
        UseMethodCodec,
        UseMethod::Burn,
        UseMethod::Multiple,
        UseMethod::Single
    );
    tags!(
        DecompressibleStateCodec,
        DecompressibleState::Enabled,
        DecompressibleState::Disabled
    );
    tags!(
        AssetDataSchemaCodec,
        AssetDataSchema::Binary,
        AssetDataSchema::Json,
        AssetDataSchema::MsgPack
    );
    tags!(
        BubblegumEventTypeCodec,
        BubblegumEventType::Uninitialized,
        BubblegumEventType::LeafSchemaEvent
    );
    instruction_tags();
}

fn instruction_tags() {
    tags!(
        InstructionNameCodec,
        InstructionName::Unknown,
        InstructionName::MintV1,
        InstructionName::Redeem,
        InstructionName::CancelRedeem,
        InstructionName::Transfer,
        InstructionName::Delegate,
        InstructionName::DecompressV1,
        InstructionName::Compress,
        InstructionName::Burn,
        InstructionName::CreateTree,
        InstructionName::VerifyCreator,
        InstructionName::UnverifyCreator,
        InstructionName::VerifyCollection,
        InstructionName::UnverifyCollection,
        InstructionName::SetAndVerifyCollection,
        InstructionName::MintToCollectionV1,
        InstructionName::SetDecompressibleState,
        InstructionName::UpdateMetadata,
        InstructionName::BurnV2,
        InstructionName::CollectV2,
        InstructionName::CreateTreeV2,
        InstructionName::DelegateAndFreezeV2,
        InstructionName::DelegateV2,
        InstructionName::FreezeV2,
        InstructionName::MintV2,
        InstructionName::SetCollectionV2,
        InstructionName::SetNonTransferableV2,
        InstructionName::ThawAndRevokeV2,
        InstructionName::ThawV2,
        InstructionName::TransferV2,
        InstructionName::UnverifyCreatorV2,
        InstructionName::UpdateAssetDataV2,
        InstructionName::UpdateMetadataV2,
        InstructionName::VerifyCreatorV2,
        InstructionName::CloseTreeV2
    );
}

#[test]
fn every_named_enum_tag() {
    named_tags();
}
