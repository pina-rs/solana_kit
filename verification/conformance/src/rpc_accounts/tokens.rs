//! Exact quantities remain independent from node display values.
use super::*;
use solana_kit_client::methods::tokens::*;

pub(super) const AMOUNT: &str = r#"{"amount":"18446744073709551615","decimals":255,"uiAmount":null,"uiAmountString":"scaled independently"}"#;

pub(super) fn quantities() -> Result<(), &'static str> {
    exact_balance()?;
    independent_display()?;
    selectors()
}

/// The exact integer amount and nullable display fields are independent contracts.
fn exact_balance() -> Result<(), &'static str> {
    let params = format!(r#"["{KEY}"]"#);
    let got = call(
        TokenAccountBalance(address(), None),
        "getTokenAccountBalance",
        &params,
        &context(AMOUNT),
    )
    .map_err(|_| "token balance failed")?;
    require(
        got.context.slot == u64::MAX
            && got.value.amount.0 == u64::MAX
            && got.value.decimals == 255
            && got.value.ui_amount.is_none()
            && got.value.ui_amount_string == "scaled independently",
        "exact token balance changed",
    )
}

/// Tiny display numbers cannot change the raw integer balance or largest-account list.
fn independent_display() -> Result<(), &'static str> {
    let display = AMOUNT.replace("null", "1e-280");
    let got = call(
        TokenSupply(address(), Some(CommitmentConfig::default())),
        "getTokenSupply",
        &format!(r#"["{KEY}",{{}}]"#),
        &context(&display),
    )
    .map_err(|_| "token supply failed")?;
    require(
        got.value.amount.0 == u64::MAX && got.value.ui_amount == Some(1e-280),
        "token display determined raw amount",
    )?;
    let largest = format!(
        r#"[{{"address":"{OTHER}",{}}}]"#,
        &display[1..display.len() - 1]
    );
    let got = call(
        TokenLargestAccounts(
            address(),
            Some(CommitmentConfig {
                commitment: Some(Commitment::Confirmed),
            }),
        ),
        "getTokenLargestAccounts",
        &format!(r#"["{KEY}",{{"commitment":"confirmed"}}]"#),
        &context(&largest),
    )
    .map_err(|_| "largest token accounts failed")?;

    require(
        got.value.len() == 1
            && got.value[0].address == Address::from_bytes([1; 32])
            && got.value[0].balance.amount.0 == u64::MAX,
        "largest token values changed",
    )
}

fn selectors() -> Result<(), &'static str> {
    let got = call(
        TokenAccountsByOwner(address(), TokenAccountFilter::ProgramId(address()), None),
        "getTokenAccountsByOwner",
        &format!(r#"["{KEY}",{{"programId":"{KEY}"}}]"#),
        &context(&keyed(r#""Ldp""#)),
    )
    .map_err(|_| "token owner selector failed")?;
    require(
        got.value.len() == 1
            && got.value[0].owner == address()
            && got.value[0].lamports == u64::MAX,
        "token owner values changed",
    )?;
    let got = call(
        TokenAccountsByDelegate(
            address(),
            TokenAccountFilter::Mint(Address::from_bytes([1; 32])),
            Some(config(AccountEncoding::Base64)),
        ),
        "getTokenAccountsByDelegate",
        &format!(r#"["{KEY}",{{"mint":"{OTHER}"}},{{"encoding":"base64"}}]"#),
        &context(&keyed(r#"["AQID","base64"]"#)),
    )
    .map_err(|_| "token delegate selector failed")?;
    require(
        got.context.slot == u64::MAX
            && got.value.len() == 1
            && got.value[0].address == Address::from_bytes([1; 32]),
        "token delegate values changed",
    )
}
