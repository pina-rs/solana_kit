//! Malformed responses and bounded requests fail through the public Client.
use super::*;
use solana_kit_client::{Error, methods::tokens::*};

pub(super) fn boundaries() -> Result<(), &'static str> {
    accounts_and_cardinality()?;
    token_fields()?;
    request_boundaries()
}

fn accounts_and_cardinality() -> Result<(), &'static str> {
    for data in [
        r#"["AB==","base64"]"#,
        r#"["AQI=","base64"]"#,
        r#"["AQID","base58"]"#,
        r#"["AQID","unknown"]"#,
        "null",
    ] {
        require(
            call(
                AccountInfoWithConfig(address(), Some(config(AccountEncoding::Base64))),
                "getAccountInfo",
                &format!(r#"["{KEY}",{{"encoding":"base64"}}]"#),
                &context(&account(data)),
            )
            .is_err(),
            "malformed account accepted",
        )?;
    }
    for body in [
        account(r#"["AQID","base64"]"#).replace(KEY, "bad"),
        account(r#"["AQID","base64"]"#).replace(r#""space":3"#, r#""space":null"#),
        account(r#"["AQID","base64"]"#).replace(r#""rentEpoch""#, r#""missing""#),
    ] {
        require(
            call(
                AccountInfoWithConfig(address(), Some(config(AccountEncoding::Base64))),
                "getAccountInfo",
                &format!(r#"["{KEY}",{{"encoding":"base64"}}]"#),
                &context(&body),
            )
            .is_err(),
            "invalid account metadata accepted",
        )?;
    }

    for value in ["[]", "[null,null]"] {
        require(
            call(
                MultipleAccounts(vec![address()], None),
                "getMultipleAccounts",
                &format!(r#"[["{KEY}"]]"#),
                &context(value),
            )
            .is_err(),
            "wrong account cardinality accepted",
        )?;
    }

    let duplicate = format!(
        r#"[{{"pubkey":"{OTHER}","account":{0}}},{{"pubkey":"{OTHER}","account":{0}}}]"#,
        account(r#""Ldp""#)
    );
    require(
        call(
            ProgramAccounts(address(), None),
            "getProgramAccounts",
            &format!(r#"["{KEY}"]"#),
            &duplicate,
        )
        .is_err(),
        "duplicate keyed account accepted",
    )?;
    require(
        call(
            ProgramAccounts(address(), None),
            "getProgramAccounts",
            &format!(r#"["{KEY}"]"#),
            &context("[]"),
        )
        .is_err(),
        "unexpected program context accepted",
    )
}

fn token_fields() -> Result<(), &'static str> {
    for amount in ["", "-1", "+1", "1.0", "1e2", "18446744073709551616"] {
        let body = tokens::AMOUNT.replace(MAX, amount);
        require(
            call(
                TokenSupply(address(), None),
                "getTokenSupply",
                &format!(r#"["{KEY}"]"#),
                &context(&body),
            )
            .is_err(),
            "invalid token amount accepted",
        )?;
    }
    for body in [
        tokens::AMOUNT.replace("255", "256"),
        tokens::AMOUNT.replace("null", "1e400"),
        tokens::AMOUNT.replace(r#""uiAmount":null,"#, ""),
        tokens::AMOUNT.replace(r#""amount":"18446744073709551615""#, r#""amount":1"#),
    ] {
        require(
            call(
                TokenAccountBalance(address(), None),
                "getTokenAccountBalance",
                &format!(r#"["{KEY}"]"#),
                &context(&body),
            )
            .is_err(),
            "invalid token field accepted",
        )?;
    }
    let wrong_owner = keyed(r#""Ldp""#).replace(
        &format!(r#""owner":"{KEY}""#),
        &format!(r#""owner":"{OTHER}""#),
    );
    require(
        call(
            TokenAccountsByOwner(address(), TokenAccountFilter::ProgramId(address()), None),
            "getTokenAccountsByOwner",
            &format!(r#"["{KEY}",{{"programId":"{KEY}"}}]"#),
            &context(&wrong_owner),
        )
        .is_err(),
        "wrong token program accepted",
    )
}

fn request_boundaries() -> Result<(), &'static str> {
    require(
        Memcmp::new(0, vec![0; 128], FilterEncoding::Base58).is_ok()
            && Memcmp::new(0, vec![0; 129], FilterEncoding::Base64).is_err()
            && Memcmp::new(u64::MAX, vec![0], FilterEncoding::Bytes).is_err(),
        "memcmp bounds changed",
    )?;
    let keys = vec![format!(r#""{KEY}""#); 100].join(",");
    let nulls = vec!["null"; 100].join(",");
    let got = call(
        MultipleAccounts(vec![address(); 100], Some(AccountConfig::default())),
        "getMultipleAccounts",
        &format!("[[{keys}],{{}}]"),
        &context(&format!("[{nulls}]")),
    )
    .map_err(|_| "100 account boundary failed")?;
    require(got.value.len() == 100, "100 account count changed")?;
    require(
        matches!(
            call(
                MultipleAccounts(vec![address(); 101], None),
                "getMultipleAccounts",
                "unused",
                "[]"
            ),
            Err(Error::Json(_))
        ),
        "101 accounts accepted",
    )?;
    for config in [
        AccountConfig {
            encoding: Some(AccountEncoding::JsonParsed),
            data_slice: Some(DataSlice {
                offset: 0,
                length: 1,
            }),
            ..Default::default()
        },
        AccountConfig {
            data_slice: Some(DataSlice {
                offset: u64::MAX,
                length: 1,
            }),
            ..Default::default()
        },
    ] {
        require(
            matches!(
                call(
                    AccountInfoWithConfig(address(), Some(config)),
                    "getAccountInfo",
                    "unused",
                    "null"
                ),
                Err(Error::Json(_))
            ),
            "invalid slice request accepted",
        )?;
    }

    for count in [4, 5] {
        let got = call(
            ProgramAccounts(
                address(),
                Some(ProgramAccountsConfig {
                    filters: Some(vec![Filter::TokenAccountState; count]),
                    ..Default::default()
                }),
            ),
            "getProgramAccounts",
            &format!(
                r#"["{KEY}",{{"filters":[{}]}}]"#,
                vec![r#""tokenAccountState""#; count].join(",")
            ),
            "[]",
        );
        require(got.is_ok() == (count == 4), "program filter count changed")?;
    }

    Ok(())
}
