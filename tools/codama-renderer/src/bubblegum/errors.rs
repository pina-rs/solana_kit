//! Exact MIT Dart names/messages. Numeric codes remain explicit and contiguous.
use serde_json::{Value, json};

const ERRORS: &[(&str, &str)] = &[
    ("AssetOwnerMismatch", "Asset owner does not match"),
    ("PublicKeyMismatch", "Public key mismatch"),
    ("HashingMismatch", "Hashing mismatch within leaf schema"),
    ("UnsupportedSchemaVersion", "Unsupported schema version"),
    ("CreatorShareSum", "Creator shares must sum to 100"),
    (
        "DuplicateCreatorAddress",
        "No duplicate creator addresses in metadata",
    ),
    ("CreatorNotVerified", "Creator did not verify the metadata"),
    ("CreatorNotFound", "Creator not found in creator Vec"),
    ("NoCreators", "No creators in creator Vec"),
    (
        "CreatorHashMismatch",
        "User-provided creator Vec must result in same user-provided creator hash",
    ),
    (
        "DataHashMismatch",
        "User-provided metadata must result in same user-provided data hash",
    ),
    ("CreatorsTooLong", "Creators list too long"),
    ("NameTooLong", "Name in metadata is too long"),
    ("SymbolTooLong", "Symbol in metadata is too long"),
    ("UriTooLong", "URI in metadata is too long"),
    (
        "BasisPointsExceedMax",
        "Basis points in metadata cannot exceed 10000",
    ),
    (
        "TreeCreatorOrDelegateMustSign",
        "Tree creator or tree delegate must sign",
    ),
    (
        "InsufficientMintApprovals",
        "Not enough unapproved mints left",
    ),
    ("NumericalOverflow", "Numerical overflow error"),
    ("IncorrectOwner", "Incorrect account owner"),
    (
        "CannotVerifyCollection",
        "Cannot verify collection in this instruction",
    ),
    ("CollectionNotFound", "Collection not found on metadata"),
    (
        "CollectionAlreadyVerified",
        "Collection item is already verified",
    ),
    (
        "CollectionAlreadyUnverified",
        "Collection item is already unverified",
    ),
    (
        "IncorrectMetadataUpdateAuthority",
        "Incorrect leaf metadata update authority",
    ),
    (
        "OwnerOrDelegateMismatch",
        "This transaction must be signed by either the leaf owner or leaf delegate",
    ),
    (
        "CollectionMustBeSized",
        "Collection must be a sized collection",
    ),
    (
        "MetadataMintMismatch",
        "Metadata mint does not match collection mint",
    ),
    ("InvalidCollectionAuthority", "Invalid collection authority"),
    (
        "InvalidDelegateRecordPda",
        "Invalid delegate record PDA derivation",
    ),
    (
        "EditionMismatch",
        "Edition account doesn't match collection",
    ),
    (
        "CollectionMustBeMasterEdition",
        "Collection must be a unique master edition v2",
    ),
    (
        "ExternalErrorConversion",
        "Could not convert external error to BubblegumError",
    ),
    (
        "DecompressionDisabled",
        "Decompression is disabled for this tree",
    ),
    ("MissingCollectionMint", "Missing collection mint account"),
    (
        "MissingCollectionMetadata",
        "Missing collection metadata account",
    ),
    ("CollectionMismatch", "Collection mismatch"),
    ("MetadataNotMutable", "Metadata not mutable"),
    (
        "PrimarySaleCanOnlyBeFlipped",
        "Can only update primary sale to true",
    ),
    (
        "CreatorDidNotUnverify",
        "Creator did not unverify the metadata",
    ),
    (
        "OnlyNonFungibleSupported",
        "Only NonFungible standard is supported",
    ),
    (
        "CanopySizeTooSmall",
        "Canopy size should be set bigger for this tree",
    ),
    ("InvalidLogWrapper", "Invalid log wrapper program"),
    ("InvalidCompressionProgram", "Invalid compression program"),
    (
        "LeafMustBeDelegated",
        "Leaf must be delegated to someone other than the leaf owner",
    ),
    ("AssetFrozen", "Asset is frozen"),
    ("AssetNonTransferable", "Asset is non-transferable"),
    ("InvalidAuthority", "Invalid authority"),
    ("CollectionFrozen", "Collection is frozen"),
    (
        "CoreCollectionsMustHavePlugin",
        "Core collections must have the Bubblegum V2 plugin on them",
    ),
    ("FeatureNotAvailable", "Feature not currently available"),
    ("MissingCollectionAccount", "Missing collection account"),
    ("AssetDataTooLong", "Asset data length too long"),
    ("AlreadyInCollection", "Item is already in the collection"),
    (
        "AlreadyNotInCollection",
        "Item is already not in a collection",
    ),
    (
        "MissingMplCoreCpiSigner",
        "Missing mpl-core CPI signer account",
    ),
    ("AssetNotFrozen", "Asset is not frozen"),
    (
        "CollectionMustHaveRoyaltiesPlugin",
        "Core collections must have the Royalties plugin to inherit seller fee basis points",
    ),
    (
        "InheritedSellerFeeCannotHaveLeafCreators",
        "Inherited seller fee basis points cannot be used with leaf-level creators",
    ),
    (
        "CannotRemoveFromCollectionWithInheritedSellerFee",
        "Cannot remove from collection while seller fee basis points are inherited",
    ),
];

pub(super) fn errors() -> Vec<Value> {
    ERRORS.iter().enumerate().map(|(offset, (name, message))| {
        json!({"kind":"errorNode","name":name,"code":6000 + offset,"message":message})
    }).collect()
}
