//! Borsh records retain source field order and distinguish leaf tags from hash versions.
use super::schema::{constant_header, discriminator, number, record};
use crate::schema::Error;
use serde_json::{Value, json};

const RECORDS: &[(&str, &str)] = &[
    ("Creator", "address:address,verified:bool,share:u8"),
    ("Collection", "verified:bool,key:address"),
    ("Uses", "useMethod:UseMethod,remaining:u64,total:u64"),
    (
        "MetadataArgs",
        "name:text,symbol:text,uri:text,sellerFeeBasisPoints:u16,primarySaleHappened:bool,isMutable:bool,editionNonce:?u8,tokenStandard:?TokenStandard,collection:?Collection,uses:?Uses,tokenProgramVersion:TokenProgramVersion,creators:[]Creator",
    ),
    (
        "MetadataArgsV2",
        "name:text,symbol:text,uri:text,sellerFeeBasisPoints:u16,primarySaleHappened:bool,isMutable:bool,tokenStandard:?TokenStandard,creators:[]Creator,collection:?address",
    ),
    (
        "UpdateArgs",
        "name:?text,symbol:?text,uri:?text,creators:?[]Creator,sellerFeeBasisPoints:?u16,primarySaleHappened:?bool,isMutable:?bool",
    ),
    (
        "LeafSchemaEvent",
        "eventType:BubblegumEventType,version:Version,schema:LeafSchema,leafHash:fixed32",
    ),
];

const ENUMS: &[(&str, &[&str])] = &[
    ("Version", &["V1", "V2"]),
    ("TokenProgramVersion", &["Original", "Token2022"]),
    (
        "TokenStandard",
        &[
            "NonFungible",
            "FungibleAsset",
            "Fungible",
            "NonFungibleEdition",
        ],
    ),
    ("UseMethod", &["Burn", "Multiple", "Single"]),
    ("DecompressibleState", &["Enabled", "Disabled"]),
    ("AssetDataSchema", &["Binary", "Json", "MsgPack"]),
    ("BubblegumEventType", &["Uninitialized", "LeafSchemaEvent"]),
    (
        "InstructionName",
        &[
            "Unknown",
            "MintV1",
            "Redeem",
            "CancelRedeem",
            "Transfer",
            "Delegate",
            "DecompressV1",
            "Compress",
            "Burn",
            "CreateTree",
            "VerifyCreator",
            "UnverifyCreator",
            "VerifyCollection",
            "UnverifyCollection",
            "SetAndVerifyCollection",
            "MintToCollectionV1",
            "SetDecompressibleState",
            "UpdateMetadata",
            "BurnV2",
            "CollectV2",
            "CreateTreeV2",
            "DelegateAndFreezeV2",
            "DelegateV2",
            "FreezeV2",
            "MintV2",
            "SetCollectionV2",
            "SetNonTransferableV2",
            "ThawAndRevokeV2",
            "ThawV2",
            "TransferV2",
            "UnverifyCreatorV2",
            "UpdateAssetDataV2",
            "UpdateMetadataV2",
            "VerifyCreatorV2",
            "CloseTreeV2",
        ],
    ),
];

pub(super) fn definitions() -> Result<Vec<Value>, Error> {
    let mut output = RECORDS
        .iter()
        .map(|(name, fields)| {
            Ok(json!({"kind":"definedTypeNode","name":name,"type":record(fields)?}))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    for (name, variants) in ENUMS {
        let variants = variants
            .iter()
            .map(|name| json!({"kind":"enumEmptyVariantTypeNode","name":name}))
            .collect::<Vec<_>>();
        output.push(json!({"kind":"definedTypeNode","name":name,
            "type":{"kind":"enumTypeNode","size":number("u8"),"variants":variants}}));
    }

    let common =
        "id:address,owner:address,delegate:address,nonce:u64,dataHash:fixed32,creatorHash:fixed32";
    let v2 = format!("{common},collectionHash:fixed32,assetDataHash:fixed32,flags:u8");
    output.push(json!({"kind":"definedTypeNode","name":"LeafSchema",
        "type":{"kind":"enumTypeNode","size":number("u8"),"variants":[
            {"kind":"enumStructVariantTypeNode","name":"V1","struct":record(common)?},
            {"kind":"enumStructVariantTypeNode","name":"V2","struct":record(&v2)?}]}}));

    Ok(output)
}

pub(super) fn accounts() -> Result<Vec<Value>, Error> {
    [
        ("TreeConfig", "treeCreator:address,treeDelegate:address,totalMintCapacity:u64,numMinted:u64,isPublic:bool,isDecompressible:DecompressibleState,version:Version"),
        ("Voucher", "leafSchema:LeafSchema,index:u32,merkleTree:address"),
    ].iter().map(|(name, fields)| {
        Ok(json!({"kind":"accountNode","name":name,"data":record(fields)?,
            "discriminators":constant_header(&discriminator("account", name))}))
    }).collect()
}
