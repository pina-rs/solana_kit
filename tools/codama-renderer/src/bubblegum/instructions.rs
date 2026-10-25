//! Complete operation table. Mandatory roots, authority accounts and metadata are never invented.
use super::schema::{discriminator, fields, roles};
use crate::schema::{Error, identifier};
use serde_json::{Value, json};

const PROOF: &str = "root:fixed32,dataHash:fixed32,creatorHash:fixed32,nonce:u64,index:u32";
const V2: &str = "root:fixed32,dataHash:fixed32,creatorHash:fixed32,assetDataHash:?fixed32,flags:?u8,nonce:u64,index:u32";
const V2_COLLECTION: &str = "root:fixed32,dataHash:fixed32,creatorHash:fixed32,collectionHash:?fixed32,assetDataHash:?fixed32,flags:?u8,nonce:u64,index:u32";
const V2_METADATA: &str =
    "root:fixed32,assetDataHash:?fixed32,flags:?u8,nonce:u64,index:u32,message:MetadataArgsV2";
const PROGRAMS: &str = "logWrapper:r,compressionProgram:r,systemProgram:r";
const CORE_PROGRAMS: &str = "logWrapper:r,compressionProgram:r,mplCoreProgram:r,systemProgram:r";

struct Operation {
    name: &'static str,
    fields: String,
    roles: String,
    proof: bool,
}

fn operation(
    name: &'static str,
    fields: &str,
    roles: &str,
    programs: &str,
    proof: bool,
) -> Operation {
    let roles = if programs.is_empty() {
        roles.to_owned()
    } else {
        format!("{roles},{programs}")
    };

    Operation {
        name,
        fields: fields.to_owned(),
        roles,
        proof,
    }
}

fn operations() -> Vec<Operation> {
    let collection = "treeAuthority:r,leafOwner:r,leafDelegate:r,merkleTree:w,payer:s,treeDelegate:r,collectionAuthority:s,collectionAuthorityRecordPda:r,collectionMint:r,collectionMetadata:w,editionAccount:r,bubblegumSigner:r";
    let metadata_programs =
        "logWrapper:r,compressionProgram:r,tokenMetadataProgram:r,systemProgram:r";
    let core = "treeAuthority:w,payer:ws,authority:s?,leafOwner:r,leafDelegate:r?,merkleTree:w,coreCollection:r?";
    let delegated = "treeAuthority:w,payer:ws,leafOwner:s?,previousLeafDelegate:r?,newLeafDelegate:r,merkleTree:w";
    let creator = "treeAuthority:r,leafOwner:r,leafDelegate:r,merkleTree:w,payer:s,creator:s";
    let creator_v2 = "treeAuthority:w,payer:ws,creator:s?,leafOwner:r,leafDelegate:r?,merkleTree:w";
    let proof_metadata = format!("{PROOF},message:MetadataArgs");

    vec![
        operation(
            "burn",
            PROOF,
            "treeAuthority:r,leafOwner:e,leafDelegate:e,merkleTree:w",
            PROGRAMS,
            true,
        ),
        operation(
            "burnV2",
            V2,
            "treeAuthority:w,payer:ws,authority:s?,leafOwner:r,leafDelegate:r?,merkleTree:w,coreCollection:w?,mplCoreCpiSigner:r?",
            CORE_PROGRAMS,
            true,
        ),
        operation(
            "cancelRedeem",
            "root:fixed32",
            "treeAuthority:r,leafOwner:ws,merkleTree:w,voucher:w",
            PROGRAMS,
            true,
        ),
        operation(
            "closeTreeV2",
            "",
            "treeAuthority:w,authority:s,merkleTree:w,recipient:w,feeRecipient:w",
            "compressionProgram:r,logWrapper:r,systemProgram:r",
            false,
        ),
        operation("collectV2", "", "treeAuthority:w,destination:w", "", false),
        operation(
            "compress",
            "",
            "treeAuthority:r,leafOwner:s,leafDelegate:r,merkleTree:r,tokenAccount:w,mint:w,metadata:w,masterEdition:w,payer:ws",
            "logWrapper:r,compressionProgram:r,tokenProgram:r,tokenMetadataProgram:r,systemProgram:r",
            false,
        ),
        operation(
            "createTree",
            "maxDepth:u32,maxBufferSize:u32,public:?bool",
            "treeAuthority:w,merkleTree:w,payer:ws,treeCreator:s",
            PROGRAMS,
            false,
        ),
        operation(
            "createTreeV2",
            "maxDepth:u32,maxBufferSize:u32,public:?bool",
            "treeAuthority:w,merkleTree:w,payer:ws,treeCreator:s?",
            PROGRAMS,
            false,
        ),
        operation(
            "decompressV1",
            "metadataArgs:MetadataArgs",
            "voucher:w,leafOwner:ws,tokenAccount:w,mint:w,mintAuthority:w,metadata:w,masterEdition:w",
            "systemProgram:r,sysvarRent:r,tokenMetadataProgram:r,tokenProgram:r,associatedTokenProgram:r,logWrapper:r",
            false,
        ),
        operation(
            "delegate",
            PROOF,
            "treeAuthority:r,leafOwner:s,previousLeafDelegate:r,newLeafDelegate:r,merkleTree:w",
            PROGRAMS,
            true,
        ),
        operation(
            "delegateAndFreezeV2",
            V2_COLLECTION,
            delegated,
            PROGRAMS,
            true,
        ),
        operation("delegateV2", V2_COLLECTION, delegated, PROGRAMS, true),
        operation(
            "freezeV2",
            V2,
            "treeAuthority:w,payer:ws,authority:s?,leafOwner:r,leafDelegate:r,merkleTree:w,coreCollection:r?",
            PROGRAMS,
            true,
        ),
        operation(
            "mintToCollectionV1",
            "metadataArgs:MetadataArgs",
            "treeAuthority:w,leafOwner:r,leafDelegate:r,merkleTree:w,payer:s,treeDelegate:s,collectionAuthority:s,collectionAuthorityRecordPda:r,collectionMint:r,collectionMetadata:w,editionAccount:r,bubblegumSigner:r",
            metadata_programs,
            false,
        ),
        operation(
            "mintV1",
            "message:MetadataArgs",
            "treeAuthority:w,leafOwner:r,leafDelegate:r,merkleTree:w,payer:s,treeDelegate:s",
            PROGRAMS,
            false,
        ),
        operation(
            "mintV2",
            "metadataArgs:MetadataArgsV2,assetData:?bytes,assetDataSchema:?AssetDataSchema",
            "treeAuthority:w,payer:ws,treeDelegate:s?,collectionAuthority:s?,leafOwner:r,leafDelegate:r?,merkleTree:w,coreCollection:w?,mplCoreCpiSigner:r?",
            CORE_PROGRAMS,
            false,
        ),
        operation(
            "redeem",
            PROOF,
            "treeAuthority:r,leafOwner:ws,leafDelegate:r,merkleTree:w,voucher:w",
            PROGRAMS,
            true,
        ),
        operation(
            "setAndVerifyCollection",
            &format!("{proof_metadata},collection:address"),
            collection,
            metadata_programs,
            true,
        ),
        operation(
            "setCollectionV2",
            V2_METADATA,
            "treeAuthority:w,payer:ws,authority:s?,newCollectionAuthority:s?,leafOwner:r,leafDelegate:r?,merkleTree:w,coreCollection:w?,newCoreCollection:w?,mplCoreCpiSigner:r",
            CORE_PROGRAMS,
            true,
        ),
        operation(
            "setDecompressableState",
            "decompressableState:DecompressibleState",
            "treeAuthority:w,treeCreator:s",
            "",
            false,
        ),
        operation(
            "setDecompressibleState",
            "decompressableState:DecompressibleState",
            "treeAuthority:w,treeCreator:s",
            "",
            false,
        ),
        operation(
            "setNonTransferableV2",
            V2,
            "treeAuthority:w,payer:ws,authority:s?,leafOwner:r,leafDelegate:r?,merkleTree:w,coreCollection:r",
            PROGRAMS,
            true,
        ),
        operation(
            "setTreeDelegate",
            "",
            "treeAuthority:w,treeCreator:s,newTreeDelegate:r,merkleTree:r",
            "systemProgram:r",
            false,
        ),
        operation(
            "thawAndRevokeV2",
            V2_COLLECTION,
            "treeAuthority:w,payer:ws,leafDelegate:s?,leafOwner:r,merkleTree:w",
            PROGRAMS,
            true,
        ),
        operation(
            "thawV2",
            V2,
            "treeAuthority:w,payer:ws,authority:s?,leafOwner:r,leafDelegate:r,merkleTree:w,coreCollection:r?",
            PROGRAMS,
            true,
        ),
        operation(
            "transfer",
            PROOF,
            "treeAuthority:r,leafOwner:e,leafDelegate:e,newLeafOwner:r,merkleTree:w",
            PROGRAMS,
            true,
        ),
        operation(
            "transferV2",
            V2,
            "treeAuthority:w,payer:ws,authority:s?,leafOwner:r,leafDelegate:r?,newLeafOwner:r,merkleTree:w,coreCollection:r?",
            PROGRAMS,
            true,
        ),
        operation(
            "unverifyCollection",
            &proof_metadata,
            collection,
            metadata_programs,
            true,
        ),
        operation("unverifyCreator", &proof_metadata, creator, PROGRAMS, true),
        operation("unverifyCreatorV2", V2_METADATA, creator_v2, PROGRAMS, true),
        operation(
            "updateAssetDataV2",
            "root:fixed32,dataHash:fixed32,creatorHash:fixed32,previousAssetDataHash:?fixed32,flags:?u8,nonce:u64,index:u32,newAssetData:?bytes,newAssetDataSchema:?AssetDataSchema",
            core,
            PROGRAMS,
            true,
        ),
        operation(
            "updateMetadata",
            "root:fixed32,nonce:u64,index:u32,currentMetadata:MetadataArgs,updateArgs:UpdateArgs",
            "treeAuthority:r,authority:s,collectionMint:r?,collectionMetadata:r?,collectionAuthorityRecordPda:r?,leafOwner:r,leafDelegate:r,payer:s,merkleTree:w",
            metadata_programs,
            true,
        ),
        operation(
            "updateMetadataV2",
            "root:fixed32,assetDataHash:?fixed32,flags:?u8,nonce:u64,index:u32,currentMetadata:MetadataArgsV2,updateArgs:UpdateArgs",
            core,
            PROGRAMS,
            true,
        ),
        operation(
            "verifyCollection",
            &proof_metadata,
            collection,
            metadata_programs,
            true,
        ),
        operation("verifyCreator", &proof_metadata, creator, PROGRAMS, true),
        operation("verifyCreatorV2", V2_METADATA, creator_v2, PROGRAMS, true),
    ]
}

pub(super) fn instructions() -> Result<Vec<Value>, Error> {
    operations().into_iter().map(|operation| {
        let header = discriminator("global", &identifier(operation.name)?);
        let mut arguments = vec![json!({"kind":"instructionArgumentNode","name":"discriminator",
            "type":{"kind":"fixedSizeTypeNode","size":8,"type":{"kind":"bytesTypeNode"}},
            "defaultValue":{"kind":"bytesValueNode","encoding":"base16","data":header},
            "defaultValueStrategy":"omitted"})];
        arguments.extend(fields(&operation.fields, "instructionArgumentNode")?);
        let mut output = json!({"kind":"instructionNode","name":operation.name,
            "arguments":arguments,"accounts":roles(&operation.roles)?,
            "optionalAccountStrategy":"programId",
            "discriminators":[{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]});

        if operation.proof {
            output["remainingAccounts"] = json!([{"kind":"instructionRemainingAccountsNode",
                "value":{"kind":"argumentValueNode","name":"proof"},
                "isOptional":true,"isSigner":false,"isWritable":false}]);
        } else if ["mintV1", "mintV2", "mintToCollectionV1"].contains(&operation.name) {
            output["remainingAccounts"] = json!([{"kind":"instructionRemainingAccountsNode",
                "value":{"kind":"argumentValueNode","name":"creatorAuthorities"},
                "isOptional":true,"privilegePolicy":"callerProvided"}]);
        }

        Ok(output)
    }).collect()
}
