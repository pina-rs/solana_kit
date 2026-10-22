//! Explicit subscription wire fixtures from the accepted typed contracts.
pub const ADDRESS: &str = "11111111111111111111111111111111";
pub const SIGNATURE: &str = concat!(
    "11111111111111111111111111111111",
    "11111111111111111111111111111111"
);
pub const ACCOUNT: &str = r#"{"executable":false,"lamports":9007199254740993,"owner":"11111111111111111111111111111111","rentEpoch":18446744073709551615,"space":1,"data":["AQ==","base64"]}"#;
pub fn result(method: &str) -> String {
    match method {
        "accountSubscribe" => format!(r#"{{"context":{{"slot":1}},"value":{ACCOUNT}}}"#),
        "programSubscribe" => format!(
            r#"{{"context":{{"slot":1}},"value":{{"pubkey":"{ADDRESS}","account":{ACCOUNT}}}}}"#
        ),
        "logsSubscribe" => format!(
            r#"{{"context":{{"slot":1}},"value":{{"signature":"{SIGNATURE}","err":null,"logs":["hello"]}}}}"#
        ),
        "signatureSubscribe" => r#"{"context":{"slot":1},"value":{"err":"AccountInUse"}}"#.into(),
        "blockSubscribe" => {
            r#"{"context":{"slot":1},"value":{"slot":1,"block":null,"err":"BlockStoreError"}}"#
                .into()
        }
        "slotSubscribe" => r#"{"slot":2,"parent":1,"root":0}"#.into(),
        "rootSubscribe" => "18446744073709551615".into(),
        "voteSubscribe" => format!(
            r#"{{"hash":"{ADDRESS}","signature":"{SIGNATURE}","slots":[1,2],"timestamp":null,"votePubkey":"{ADDRESS}"}}"#
        ),
        "slotsUpdatesSubscribe" => {
            r#"{"type":"dead","slot":2,"timestamp":3,"err":"bank failure"}"#.into()
        }
        _ => panic!("unexpected fixture family"),
    }
}
