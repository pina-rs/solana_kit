//! Account keys are literal framing; allocation padding is owned by the public decoder.
use super::schema::{constant_header, record};
use crate::schema::Error;
use serde_json::{Value, json};

const MASTER_EDITION_V2_KEY: u8 = 6;
const EDITION_MARKER_KEY: u8 = 7;
const USE_AUTHORITY_RECORD_KEY: u8 = 8;
const EDITION_MARKER_V2_KEY: u8 = 13;

const ACCOUNTS: &[(&str, u8, &str)] = &[
    ("Edition", 1, "parent:address,edition:u64"),
    (
        "MasterEditionV1",
        2,
        "supply:u64,maxSupply:?u64,printingMint:address,oneTimePrintingAuthorizationMint:address",
    ),
    (
        "ReservationListV1",
        3,
        "masterEdition:address,supplySnapshot:?u64,reservations:[]ReservationV1",
    ),
    (
        "Metadata",
        4,
        "updateAuthority:address,mint:address,data:Data,primarySaleHappened:bool,isMutable:bool,editionNonce:?u8,tokenStandard:?TokenStandard,collection:?Collection,uses:?Uses,collectionDetails:?CollectionDetails,programmableConfig:?ProgrammableConfig",
    ),
    (
        "ReservationListV2",
        5,
        "masterEdition:address,supplySnapshot:?u64,reservations:[]Reservation,totalReservationSpots:u64,currentReservationSpots:u64",
    ),
    (
        "MasterEditionV2",
        MASTER_EDITION_V2_KEY,
        "supply:u64,maxSupply:?u64",
    ),
    ("EditionMarker", EDITION_MARKER_KEY, "ledger:fixed31"),
    (
        "UseAuthorityRecord",
        USE_AUTHORITY_RECORD_KEY,
        "allowedUses:u64,bump:u8",
    ),
    (
        "CollectionAuthorityRecord",
        9,
        "bump:u8,newUpdateAuthority:?address",
    ),
    (
        "TokenOwnedEscrow",
        10,
        "baseToken:address,authority:EscrowAuthority,bump:u8",
    ),
    (
        "TokenRecord",
        11,
        "bump:u8,state:TokenState,ruleSetRevision:?u64,delegate:?address,delegateRole:?TokenDelegateRole,lockedTransfer:?address",
    ),
    (
        "MetadataDelegateRecord",
        12,
        "bump:u8,mint:address,delegate:address,updateAuthority:address",
    ),
    ("EditionMarkerV2", EDITION_MARKER_V2_KEY, "ledger:[]u8"),
    (
        "HolderDelegateRecord",
        14,
        "bump:u8,mint:address,delegate:address,updateAuthority:address",
    ),
];

pub(super) fn accounts() -> Result<Vec<Value>, Error> {
    ACCOUNTS
        .iter()
        .map(|(name, key, fields)| {
            Ok(
                json!({"kind":"accountNode","name":name,"data":record(fields)?,
            "discriminators":constant_header(&format!("{key:02x}"))}),
            )
        })
        .collect()
}
