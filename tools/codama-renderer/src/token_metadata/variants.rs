//! Plain Borsh variants retain every field, including authorization payloads.
use super::schema::{number, record, wire};
use crate::schema::Error;
use serde_json::{Value, json};

const ENUMS: &[(&str, &[(&str, &str)])] = &[
    ("BurnArgs", &[("V1", "amount:u64")]),
    (
        "CollectionDetails",
        &[("V1", "size:u64"), ("V2", "padding:fixed8")],
    ),
    (
        "CollectionDetailsToggle",
        &[("None", ""), ("Clear", ""), ("Set", "@CollectionDetails")],
    ),
    (
        "CollectionToggle",
        &[("None", ""), ("Clear", ""), ("Set", "@Collection")],
    ),
    (
        "CreateArgs",
        &[(
            "V1",
            "assetData:AssetData,decimals:?u8,printSupply:?PrintSupply",
        )],
    ),
    (
        "EscrowAuthority",
        &[("TokenOwner", ""), ("Creator", "@address")],
    ),
    (
        "LockArgs",
        &[("V1", "authorizationData:?AuthorizationData")],
    ),
    (
        "UnlockArgs",
        &[("V1", "authorizationData:?AuthorizationData")],
    ),
    ("UseArgs", &[("V1", "authorizationData:?AuthorizationData")]),
    (
        "MintArgs",
        &[("V1", "amount:u64,authorizationData:?AuthorizationData")],
    ),
    (
        "TransferArgs",
        &[("V1", "amount:u64,authorizationData:?AuthorizationData")],
    ),
    (
        "PayloadType",
        &[
            ("Pubkey", "@address"),
            ("Seeds", "@SeedsVec"),
            ("MerkleProof", "@ProofInfo"),
            ("Number", "@u64"),
        ],
    ),
    ("PrintArgs", &[("V1", "edition:u64"), ("V2", "edition:u64")]),
    (
        "PrintSupply",
        &[("Zero", ""), ("Limited", "@u64"), ("Unlimited", "")],
    ),
    ("ProgrammableConfig", &[("V1", "ruleSet:?address")]),
    (
        "RuleSetToggle",
        &[("None", ""), ("Clear", ""), ("Set", "@address")],
    ),
    (
        "UsesToggle",
        &[("None", ""), ("Clear", ""), ("Set", "@Uses")],
    ),
    (
        "DelegateArgs",
        &[
            ("CollectionV1", "authorizationData:?AuthorizationData"),
            ("SaleV1", "amount:u64,authorizationData:?AuthorizationData"),
            (
                "TransferV1",
                "amount:u64,authorizationData:?AuthorizationData",
            ),
            ("DataV1", "authorizationData:?AuthorizationData"),
            (
                "UtilityV1",
                "amount:u64,authorizationData:?AuthorizationData",
            ),
            (
                "StakingV1",
                "amount:u64,authorizationData:?AuthorizationData",
            ),
            ("StandardV1", "amount:u64"),
            (
                "LockedTransferV1",
                "amount:u64,lockedAddress:address,authorizationData:?AuthorizationData",
            ),
            (
                "ProgrammableConfigV1",
                "authorizationData:?AuthorizationData",
            ),
            ("AuthorityItemV1", "authorizationData:?AuthorizationData"),
            ("DataItemV1", "authorizationData:?AuthorizationData"),
            ("CollectionItemV1", "authorizationData:?AuthorizationData"),
            (
                "ProgrammableConfigItemV1",
                "authorizationData:?AuthorizationData",
            ),
            ("PrintDelegateV1", "authorizationData:?AuthorizationData"),
        ],
    ),
    (
        "UpdateArgs",
        &[
            (
                "V1",
                "newUpdateAuthority:?address,data:?Data,primarySaleHappened:?bool,isMutable:?bool,collection:CollectionToggle,collectionDetails:CollectionDetailsToggle,uses:UsesToggle,ruleSet:RuleSetToggle,authorizationData:?AuthorizationData",
            ),
            (
                "AsUpdateAuthorityV2",
                "newUpdateAuthority:?address,data:?Data,primarySaleHappened:?bool,isMutable:?bool,collection:CollectionToggle,collectionDetails:CollectionDetailsToggle,uses:UsesToggle,ruleSet:RuleSetToggle,tokenStandard:?TokenStandard,authorizationData:?AuthorizationData",
            ),
            (
                "AsAuthorityItemDelegateV2",
                "newUpdateAuthority:?address,primarySaleHappened:?bool,isMutable:?bool,tokenStandard:?TokenStandard,authorizationData:?AuthorizationData",
            ),
            (
                "AsCollectionDelegateV2",
                "collection:CollectionToggle,authorizationData:?AuthorizationData",
            ),
            (
                "AsDataDelegateV2",
                "data:?Data,authorizationData:?AuthorizationData",
            ),
            (
                "AsProgrammableConfigDelegateV2",
                "ruleSet:RuleSetToggle,authorizationData:?AuthorizationData",
            ),
            (
                "AsDataItemDelegateV2",
                "data:?Data,authorizationData:?AuthorizationData",
            ),
            (
                "AsCollectionItemDelegateV2",
                "collection:CollectionToggle,authorizationData:?AuthorizationData",
            ),
            (
                "AsProgrammableConfigItemDelegateV2",
                "ruleSet:RuleSetToggle,authorizationData:?AuthorizationData",
            ),
        ],
    ),
];

pub(super) fn definitions() -> Result<Vec<Value>, Error> {
    ENUMS
        .iter()
        .map(|(name, variants)| {
            let variants = variants
                .iter()
                .map(|(name, payload)| variant(name, payload))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(
                json!({"kind":"definedTypeNode","name":name,"type":{"kind":"enumTypeNode",
            "size":number("u8"),"variants":variants}}),
            )
        })
        .collect()
}

fn variant(name: &str, payload: &str) -> Result<Value, Error> {
    if payload.is_empty() {
        return Ok(json!({"kind":"enumEmptyVariantTypeNode","name":name}));
    }

    if let Some(ty) = payload.strip_prefix('@') {
        return Ok(json!({"kind":"enumTupleVariantTypeNode","name":name,
            "tuple":{"kind":"tupleTypeNode","items":[wire(ty)?]}}));
    }

    Ok(json!({"kind":"enumStructVariantTypeNode","name":name,"struct":record(payload)?}))
}
