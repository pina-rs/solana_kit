//! All 36 MIT Dart instruction shapes, ordered role policies and complete payloads.
use super::schema::{discriminator, fields, roles};
use crate::schema::{Error, identifier};
use serde_json::{Value, json};

const CONFIG: &str = "multisig:w,configAuthority:s,rentPayer:ws?,systemProgram:r?";
const CREATE: &str = "multisig:w,transaction:w,creator:s,rentPayer:ws,systemProgram:r";
const BUFFER: &str = "multisig:r,transactionBuffer:w,creator:s";
const VOTE: &str = "multisig:r,member:ws,proposal:w";
const CLOSE: &str = "multisig:r,proposal:w,transaction:w,rentCollector:w,systemProgram:r";
const HEADER_BYTES: usize = 8;

/// Name, ordered accounts and payload are source facts rather than emitted Rust.
pub(super) const INSTRUCTIONS: &[(&str, &str, &str)] = &[
    (
        "programConfigInit",
        "programConfig:w,initializer:ws,systemProgram:r",
        "authority:address,multisigCreationFee:u64,treasury:address",
    ),
    (
        "programConfigSetAuthority",
        "programConfig:w,authority:s",
        "newAuthority:address",
    ),
    (
        "programConfigSetMultisigCreationFee",
        "programConfig:w,authority:s",
        "newMultisigCreationFee:u64",
    ),
    (
        "programConfigSetTreasury",
        "programConfig:w,authority:s",
        "newTreasury:address",
    ),
    ("multisigCreate", "null:r", ""),
    (
        "multisigCreateV2",
        "programConfig:r,treasury:w,multisig:w,createKey:s,creator:ws,systemProgram:r",
        concat!(
            "configAuthority:?address,threshold:u16,members:[]Member,",
            "timeLock:u32,rentCollector:?address,memo:?text"
        ),
    ),
    ("multisigAddMember", CONFIG, "newMember:Member,memo:?text"),
    (
        "multisigRemoveMember",
        CONFIG,
        "oldMember:address,memo:?text",
    ),
    ("multisigSetTimeLock", CONFIG, "timeLock:u32,memo:?text"),
    (
        "multisigChangeThreshold",
        CONFIG,
        "newThreshold:u16,memo:?text",
    ),
    (
        "multisigSetConfigAuthority",
        CONFIG,
        "newConfigAuthority:address,memo:?text",
    ),
    (
        "multisigSetRentCollector",
        CONFIG,
        "rentCollector:?address,memo:?text",
    ),
    (
        "multisigAddSpendingLimit",
        "multisig:r,configAuthority:s,spendingLimit:w,rentPayer:ws,systemProgram:r",
        concat!(
            "createKey:address,vaultIndex:u8,mint:address,amount:u64,",
            "period:Period,members:[]address,destinations:[]address,memo:?text"
        ),
    ),
    (
        "multisigRemoveSpendingLimit",
        "multisig:r,configAuthority:s,spendingLimit:w,rentCollector:w",
        "memo:?text",
    ),
    (
        "configTransactionCreate",
        CREATE,
        "actions:[]ConfigAction,memo:?text",
    ),
    (
        "configTransactionExecute",
        "multisig:w,member:s,proposal:w,transaction:r,rentPayer:ws?,systemProgram:r?",
        "",
    ),
    (
        "vaultTransactionCreate",
        CREATE,
        "args:VaultTransactionCreateArgs",
    ),
    (
        "transactionBufferCreate",
        "multisig:r,transactionBuffer:w,creator:s,rentPayer:ws,systemProgram:r",
        "bufferIndex:u8,vaultIndex:u8,finalBufferHash:fixed32,finalBufferSize:u16,buffer:bytes",
    ),
    ("transactionBufferClose", BUFFER, ""),
    ("transactionBufferExtend", BUFFER, "buffer:bytes"),
    (
        "vaultTransactionCreateFromBuffer",
        concat!(
            "vaultTransactionCreateMultisig:w,vaultTransactionCreateTransaction:w,",
            "vaultTransactionCreateCreator:s,vaultTransactionCreateRentPayer:ws,",
            "vaultTransactionCreateSystemProgram:r,transactionBuffer:w,creator:ws"
        ),
        "args:VaultTransactionCreateArgs",
    ),
    (
        "vaultTransactionExecute",
        "multisig:r,proposal:w,transaction:r,member:s",
        "",
    ),
    (
        "batchCreate",
        "multisig:w,batch:w,creator:s,rentPayer:ws,systemProgram:r",
        "vaultIndex:u8,memo:?text",
    ),
    (
        "batchAddTransaction",
        "multisig:r,proposal:r,batch:w,transaction:w,member:s,rentPayer:ws,systemProgram:r",
        "ephemeralSigners:u8,transactionMessage:bytes",
    ),
    (
        "batchExecuteTransaction",
        "multisig:r,member:s,proposal:w,batch:w,transaction:r",
        "",
    ),
    (
        "proposalCreate",
        "multisig:r,proposal:w,creator:s,rentPayer:ws,systemProgram:r",
        "transactionIndex:u64,draft:bool",
    ),
    ("proposalActivate", VOTE, ""),
    ("proposalApprove", VOTE, "args:ProposalVoteArgs"),
    ("proposalReject", VOTE, "args:ProposalVoteArgs"),
    ("proposalCancel", VOTE, "args:ProposalVoteArgs"),
    (
        "proposalCancelV2",
        "multisig:r,member:ws,proposal:w,systemProgram:r",
        "args:ProposalVoteArgs",
    ),
    (
        "spendingLimitUse",
        concat!(
            "multisig:r,member:s,spendingLimit:w,vault:w,destination:w,systemProgram:r?,",
            "mint:r?,vaultTokenAccount:w?,destinationTokenAccount:w?,tokenProgram:r?"
        ),
        "amount:u64,decimals:u8,memo:?text",
    ),
    ("configTransactionAccountsClose", CLOSE, ""),
    ("vaultTransactionAccountsClose", CLOSE, ""),
    (
        "vaultBatchTransactionAccountClose",
        "multisig:r,proposal:r,batch:w,transaction:w,rentCollector:w,systemProgram:r",
        "",
    ),
    (
        "batchAccountsClose",
        "multisig:r,proposal:w,batch:w,rentCollector:w,systemProgram:r",
        "",
    ),
];

pub(super) fn instructions() -> Result<Vec<Value>, Error> {
    INSTRUCTIONS.iter().map(|(name, accounts, payload)| {
        let mut arguments = vec![json!({"kind":"instructionArgumentNode","name":"discriminator",
            "type":{
                "kind":"fixedSizeTypeNode","size":HEADER_BYTES,
                "type":{"kind":"bytesTypeNode"}
            },
            "defaultValue":{"kind":"bytesValueNode","encoding":"base16",
                "data":discriminator("global", &identifier(name)?)
            },"defaultValueStrategy":"omitted"})];
        arguments.extend(fields(payload,"instructionArgumentNode")?);
        let mut output = json!({"kind":"instructionNode","name":name,"accounts":roles(accounts)?,
            "arguments":arguments,"optionalAccountStrategy":"programId",
            "discriminators":[{
                "kind":"fieldDiscriminatorNode","name":"discriminator","offset":0
            }]});

        if [
            "configTransactionExecute", "vaultTransactionExecute", "batchExecuteTransaction"
        ].contains(name) {
            output["remainingAccounts"] = json!([{"kind":"instructionRemainingAccountsNode",
                "value":{"kind":"argumentValueNode","name":"remaining"},"isOptional":true,
                "privilegePolicy":"callerProvided"}]);
        }

        Ok(output)
    }).collect()
}
