//! Literal requests and typed results for accounts, logs and chain progress.

use super::*;
use alloc::{string::ToString, vec};
use solana_kit_client::methods::{
    self,
    accounts::{AccountData, AccountEncoding, Filter},
};
use solana_kit_core::{Address, accounts::MaybeAccount};

fn account_config() -> AccountConfig {
    AccountConfig {
        commitment: Some(methods::Commitment::Finalized),
        encoding: Some(AccountEncoding::Base64),
    }
}

fn accounts() -> Check {
    let address = Address::from_bytes([0; 32]);
    let account = delivered(
        Account(address, Some(account_config())),
        r#"{"jsonrpc":"2.0","id":1,"method":"accountSubscribe","params":["11111111111111111111111111111111",{"commitment":"finalized","encoding":"base64"}]}"#,
        "accountNotification",
        &contextual(ACCOUNT),
    )?;
    let MaybeAccount::Present(account) = account.value else {
        return Err("account absent");
    };
    crate::require(
        account.lamports == u64::MAX && account.rent_epoch == Some(u64::MAX),
        "account integers lost",
    )?;
    crate::require(
        matches!(account.data, AccountData::Bytes { bytes, .. } if bytes == [1]),
        "account bytes differ",
    )?;
    let raw = format!(r#"{{"pubkey":"{ADDRESS}","account":{ACCOUNT}}}"#);
    let program = delivered(
        Program(
            address,
            Some(ProgramConfig {
                account: account_config(),
                filters: Some(vec![Filter::DataSize(1)]),
            }),
        ),
        r#"{"jsonrpc":"2.0","id":1,"method":"programSubscribe","params":["11111111111111111111111111111111",{"commitment":"finalized","encoding":"base64","filters":[{"dataSize":1}]}]}"#,
        "programNotification",
        &contextual(&raw),
    )?;

    crate::require(
        program.context.slot == u64::MAX
            && program.value.address == address
            && program.value.owner == address,
        "program account differs",
    )
}

fn logs() -> Check {
    let result = delivered(
        Logs(
            LogsFilter::Mentions(Address::from_bytes([0; 32])),
            Some(methods::accounts::CommitmentConfig {
                commitment: Some(methods::Commitment::Processed),
            }),
        ),
        r#"{"jsonrpc":"2.0","id":1,"method":"logsSubscribe","params":[{"mentions":["11111111111111111111111111111111"]},{"commitment":"processed"}]}"#,
        "logsNotification",
        &contextual(&format!(
            r#"{{"signature":"{SIGNATURE}","err":"AccountInUse","logs":["first","last"]}}"#
        )),
    )?;
    crate::require(
        result.value.signature.to_string() == SIGNATURE
            && result.value.logs == ["first", "last"]
            && result.value.err.is_some(),
        "logs result differs",
    )
}

fn chain() -> Check {
    let root = delivered(
        Root,
        r#"{"jsonrpc":"2.0","id":1,"method":"rootSubscribe","params":[]}"#,
        "rootNotification",
        "18446744073709551615",
    )?;
    crate::require(root == u64::MAX, "root integer lost")?;
    let slot = delivered(
        Slot,
        r#"{"jsonrpc":"2.0","id":1,"method":"slotSubscribe","params":[]}"#,
        "slotNotification",
        r#"{"slot":18446744073709551615,"parent":9007199254740993,"root":0}"#,
    )?;
    crate::require(
        slot.slot == u64::MAX && slot.parent == 9007199254740993 && slot.root == 0,
        "slot fields differ",
    )?;
    let vote = delivered(
        Vote,
        r#"{"jsonrpc":"2.0","id":1,"method":"voteSubscribe","params":[]}"#,
        "voteNotification",
        &format!(
            r#"{{"hash":"{ADDRESS}","signature":"{SIGNATURE}","slots":[18446744073709551615],"timestamp":-9223372036854775808,"votePubkey":"{ADDRESS}"}}"#
        ),
    )?;
    crate::require(
        vote.hash.to_string() == ADDRESS
            && vote.signature.to_string() == SIGNATURE
            && vote.vote_pubkey.to_string() == ADDRESS
            && vote.slots == [u64::MAX]
            && vote.timestamp == Some(i64::MIN),
        "vote fields differ",
    )?;
    let vote = delivered(
        Vote,
        r#"{"jsonrpc":"2.0","id":1,"method":"voteSubscribe","params":[]}"#,
        "voteNotification",
        &format!(
            r#"{{"hash":"{ADDRESS}","signature":"{SIGNATURE}","slots":[],"timestamp":null,"votePubkey":"{ADDRESS}"}}"#
        ),
    )?;
    crate::require(
        vote.slots.is_empty() && vote.timestamp.is_none(),
        "nullable vote timestamp differs",
    )
}

pub(super) fn families() -> Check {
    accounts()?;
    logs()?;
    chain()
}
