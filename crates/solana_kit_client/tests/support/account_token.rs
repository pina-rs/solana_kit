//! Public-client fixture transport shared by scoped integration tests.
use solana_kit_client::{
    Client, Error, Limits, Reply, Request, Transport, TransportError,
    methods::{
        Method,
        accounts::{AccountConfig, AccountEncoding},
    },
};
use solana_kit_core::Address;
use std::future::{Ready, ready};

pub const ADDRESS: &str = "11111111111111111111111111111111";
pub const ACCOUNT: &str = r#"{"lamports":18446744073709551615,"owner":"11111111111111111111111111111111","executable":false,"rentEpoch":18446744073709551615,"space":3,"data":["AQID","base64"]}"#;
pub const AMOUNT: &str = r#"{"amount":"18446744073709551615","decimals":255,"uiAmount":null,"uiAmountString":"0.0000000000000000000000000000000000000001"}"#;
struct Script {
    name: &'static str,
    params: String,
    result: String,
}
impl Transport for Script {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let expected = format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"{}","params":{}}}"#,
            self.name, self.params
        );
        assert_eq!(String::from_utf8(request.body).unwrap(), expected);
        ready(Ok(Reply {
            status: 200,
            body: format!(r#"{{"jsonrpc":"2.0","id":1,"result":{}}}"#, self.result).into_bytes(),
        }))
    }
}
pub fn call<M: Method>(method: M, params: &str, result: &str) -> Result<M::Response, Error> {
    let script = Script {
        name: M::NAME,
        params: params.to_owned(),
        result: result.to_owned(),
    };
    futures::executor::block_on(Client::new(script, Limits::default()).call(method))
}
pub fn a() -> Address {
    Address::from_bytes([0; 32])
}
pub fn config() -> AccountConfig {
    AccountConfig {
        encoding: Some(AccountEncoding::Base64),
        ..Default::default()
    }
}
pub fn contextual(value: &str) -> String {
    format!(r#"{{"context":{{"slot":18446744073709551615}},"value":{value}}}"#)
}
pub fn account_params() -> String {
    format!(r#"["{ADDRESS}",{{"encoding":"base64"}}]"#)
}

pub fn present(
    value: solana_kit_core::accounts::MaybeAccount<
        solana_kit_client::methods::accounts::AccountData,
    >,
) -> solana_kit_core::accounts::Account<solana_kit_client::methods::accounts::AccountData> {
    let solana_kit_core::accounts::MaybeAccount::Present(value) = value else {
        panic!()
    };
    value
}
