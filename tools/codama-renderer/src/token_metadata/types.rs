//! Complete named records and finite enums from the pinned MIT Dart package.
use super::schema::{number, record};
use crate::schema::Error;
use serde_json::{Value, json};

const RECORDS: &[(&str, &str)] = &[
    ("Creator", "address:address,verified:bool,share:u8"),
    (
        "Data",
        "name:text,symbol:text,uri:text,sellerFeeBasisPoints:u16,creators:?[]Creator",
    ),
    (
        "DataV2",
        "name:text,symbol:text,uri:text,sellerFeeBasisPoints:u16,creators:?[]Creator,collection:?Collection,uses:?Uses",
    ),
    (
        "AssetData",
        "name:text,symbol:text,uri:text,sellerFeeBasisPoints:u16,creators:?[]Creator,primarySaleHappened:bool,isMutable:bool,tokenStandard:TokenStandard,collection:?Collection,uses:?Uses,collectionDetails:?CollectionDetails,ruleSet:?address",
    ),
    ("Collection", "verified:bool,key:address"),
    ("Uses", "useMethod:UseMethod,remaining:u64,total:u64"),
    ("AuthorizationData", "payload:Payload"),
    ("Payload", "map:payloadMap"),
    ("SeedsVec", "seeds:[][]u8"),
    ("ProofInfo", "proof:[]fixed32"),
    (
        "Reservation",
        "address:address,spotsRemaining:u64,totalSpots:u64",
    ),
    (
        "ReservationV1",
        "address:address,spotsRemaining:u8,totalSpots:u8",
    ),
    ("SetCollectionSizeArgs", "size:u64"),
    ("MintNewEditionFromMasterEditionViaTokenArgs", "edition:u64"),
];

const ENUMS: &[(&str, &str)] = &[
    (
        "AuthorityType",
        "None,Metadata,Holder,MetadataDelegate,TokenDelegate",
    ),
    ("HolderDelegateRole", "PrintDelegate"),
    (
        "Key",
        "Uninitialized,EditionV1,MasterEditionV1,ReservationListV1,MetadataV1,ReservationListV2,MasterEditionV2,EditionMarker,UseAuthorityRecord,CollectionAuthorityRecord,TokenOwnedEscrow,TokenRecord,MetadataDelegate,EditionMarkerV2,HolderDelegate",
    ),
    (
        "MetadataDelegateRole",
        "AuthorityItem,Collection,Use,Data,ProgrammableConfig,DataItem,CollectionItem,ProgrammableConfigItem",
    ),
    ("MigrationType", "CollectionV1,ProgrammableV1"),
    (
        "PayloadKey",
        "Amount,Authority,AuthoritySeeds,Delegate,DelegateSeeds,Destination,DestinationSeeds,Holder,Source,SourceSeeds",
    ),
    (
        "RevokeArgs",
        "CollectionV1,SaleV1,TransferV1,DataV1,UtilityV1,StakingV1,StandardV1,LockedTransferV1,ProgrammableConfigV1,MigrationV1,AuthorityItemV1,DataItemV1,CollectionItemV1,ProgrammableConfigItemV1,PrintDelegateV1",
    ),
    (
        "TokenDelegateRole",
        "Sale,Transfer,Utility,Staking,Standard,LockedTransfer,Migration",
    ),
    (
        "TokenStandard",
        "NonFungible,FungibleAsset,Fungible,NonFungibleEdition,ProgrammableNonFungible,ProgrammableNonFungibleEdition",
    ),
    ("TokenState", "Unlocked,Locked,Listed"),
    ("UseMethod", "Burn,Multiple,Single"),
    ("VerificationArgs", "CreatorV1,CollectionV1"),
];

pub(super) fn definitions() -> Result<Vec<Value>, Error> {
    let mut result = RECORDS
        .iter()
        .map(|(name, fields)| {
            Ok(json!({"kind":"definedTypeNode","name":name,"type":record(fields)?}))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    result.extend(ENUMS.iter().map(|(name, variants)| {
        let variants = variants
            .split(',')
            .map(|variant| json!({"kind":"enumEmptyVariantTypeNode","name":variant}))
            .collect::<Vec<_>>();
        json!({"kind":"definedTypeNode","name":name,"type":{"kind":"enumTypeNode",
            "size":number("u8"),"variants":variants}})
    }));

    Ok(result)
}
