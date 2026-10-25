//! Complete MIT Dart named values and nine account layouts, in canonical wire order.
use super::schema::{constant_header, discriminator, record};
use crate::schema::Error;
use serde_json::{Value, json};

const ACCOUNTS: &[(&str, &str)] = &[
    (
        "Batch",
        concat!(
            "multisig:address,creator:address,index:u64,bump:u8,",
            "vaultIndex:u8,vaultBump:u8,size:u32,executedTransactionIndex:u32"
        ),
    ),
    (
        "VaultBatchTransaction",
        "bump:u8,ephemeralSignerBumps:bytes,message:VaultTransactionMessage",
    ),
    (
        "ConfigTransaction",
        concat!(
            "multisig:address,creator:address,index:u64,",
            "bump:u8,actions:[]ConfigAction"
        ),
    ),
    (
        "Multisig",
        concat!(
            "createKey:address,configAuthority:address,threshold:u16,timeLock:u32,",
            "transactionIndex:u64,staleTransactionIndex:u64,rentCollector:?address,",
            "bump:u8,members:[]Member"
        ),
    ),
    (
        "ProgramConfig",
        concat!(
            "authority:address,multisigCreationFee:u64,",
            "treasury:address,reserved:fixed64"
        ),
    ),
    (
        "Proposal",
        concat!(
            "multisig:address,transactionIndex:u64,status:ProposalStatus,bump:u8,",
            "approved:[]address,rejected:[]address,cancelled:[]address"
        ),
    ),
    (
        "SpendingLimit",
        concat!(
            "multisig:address,createKey:address,vaultIndex:u8,mint:address,",
            "amount:u64,period:Period,remainingAmount:u64,lastReset:i64,bump:u8,",
            "members:[]address,destinations:[]address"
        ),
    ),
    (
        "TransactionBuffer",
        concat!(
            "multisig:address,creator:address,bufferIndex:u8,vaultIndex:u8,",
            "finalBufferHash:fixed32,finalBufferSize:u16,buffer:bytes"
        ),
    ),
    (
        "VaultTransaction",
        concat!(
            "multisig:address,creator:address,index:u64,bump:u8,",
            "vaultIndex:u8,vaultBump:u8,ephemeralSignerBumps:bytes,message:VaultTransactionMessage"
        ),
    ),
];

const RECORDS: &[(&str, &str)] = &[
    ("Member", "key:address,permissions:Permissions"),
    ("Permissions", "mask:u8"),
    (
        "VaultTransactionMessage",
        concat!(
            "numSigners:u8,numWritableSigners:u8,",
            "numWritableNonSigners:u8,accountKeys:[]address,",
            "instructions:[]MultisigCompiledInstruction,",
            "addressTableLookups:[]MultisigMessageAddressTableLookup"
        ),
    ),
    (
        "MultisigCompiledInstruction",
        "programIdIndex:u8,accountIndexes:bytes,data:bytes",
    ),
    (
        "MultisigMessageAddressTableLookup",
        "accountKey:address,writableIndexes:bytes,readonlyIndexes:bytes",
    ),
    ("ProposalVoteArgs", "memo:?text"),
    (
        "VaultTransactionCreateArgs",
        "vaultIndex:u8,ephemeralSigners:u8,transactionMessage:bytes,memo:?text",
    ),
];

pub(super) fn accounts() -> Result<Vec<Value>, Error> {
    ACCOUNTS
        .iter()
        .map(|(name, fields)| {
            Ok(json!({"kind":"accountNode","name":name,
        "data":record(fields)?,"discriminators":constant_header(&discriminator("account",name))}))
        })
        .collect()
}

pub(super) fn definitions() -> Result<Vec<Value>, Error> {
    let mut output = RECORDS
        .iter()
        .map(|(name, fields)| {
            Ok(json!({"kind":"definedTypeNode",
        "name":name,"type":record(fields)?}))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    output.push(enumeration(
        "Vote",
        &[("Approve", ""), ("Reject", ""), ("Cancel", "")],
    )?);
    output.push(enumeration(
        "Period",
        &[("OneTime", ""), ("Day", ""), ("Week", ""), ("Month", "")],
    )?);
    output.push(enumeration(
        "ProposalStatus",
        &[
            ("Draft", "timestamp:i64"),
            ("Active", "timestamp:i64"),
            ("Rejected", "timestamp:i64"),
            ("Approved", "timestamp:i64"),
            ("Executing", ""),
            ("Executed", "timestamp:i64"),
            ("Cancelled", "timestamp:i64"),
        ],
    )?);
    output.push(enumeration(
        "ConfigAction",
        &[
            ("AddMember", "newMember:Member"),
            ("RemoveMember", "oldMember:address"),
            ("ChangeThreshold", "newThreshold:u16"),
            ("SetTimeLock", "newTimeLock:u32"),
            (
                "AddSpendingLimit",
                concat!(
                    "createKey:address,vaultIndex:u8,mint:address,amount:u64,",
                    "period:Period,members:[]address,destinations:[]address"
                ),
            ),
            ("RemoveSpendingLimit", "spendingLimit:address"),
            ("SetRentCollector", "newRentCollector:?address"),
        ],
    )?);

    Ok(output)
}

fn enumeration(name: &str, variants: &[(&str, &str)]) -> Result<Value, Error> {
    let variants = variants
        .iter()
        .map(|(name, fields)| {
            if fields.is_empty() {
                return Ok(json!({"kind":"enumEmptyVariantTypeNode","name":name}));
            }

            Ok(json!({"kind":"enumStructVariantTypeNode","name":name,"struct":record(fields)?}))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(
        json!({"kind":"definedTypeNode","name":name,"type":{"kind":"enumTypeNode",
        "size":super::schema::number("u8"),"variants":variants}}),
    )
}
