//! Every added schema retains the common client's correlation and error boundary.
use super::*;
use solana_kit_client::{
    Client, Error, Limits, Reply, Request, Transport, TransportError, methods::Method,
};
use std::future::{Ready, ready};
struct Envelope(&'static str);
impl Transport for Envelope {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, _: Request) -> Self::Exchange<'_> {
        ready(Ok(Reply {
            status: 200,
            body: self.0.as_bytes().to_vec(),
        }))
    }
}
fn guard<Operation: Method>(make: impl Fn() -> Operation) {
    let wrong = Client::new(
        Envelope(r#"{"jsonrpc":"2.0","id":2,"result":null}"#),
        Limits::default(),
    );
    assert!(matches!(
        futures::executor::block_on(wrong.call(make())),
        Err(Error::Correlation)
    ));
    let remote = Client::new(
        Envelope(
            r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32004,"message":"fixture","data":1e400}}"#,
        ),
        Limits::default(),
    );
    let Err(Error::Remote(error)) = futures::executor::block_on(remote.call(make())) else {
        panic!()
    };
    assert_eq!(error.code, -32004);
}
#[test]
fn all_catalog_methods_preserve_correlation_and_remote_failures() {
    guard(|| Balance(a(), None));
    guard(|| LatestBlockhash(None));
    guard(|| AccountInfoWithConfig(a(), None));
    guard(|| MultipleAccounts(vec![a()], None));
    guard(|| ProgramAccounts(a(), None));
    guard(|| MinimumBalanceForRentExemption(0, None));
    guard(|| BlockhashValid(Hash::from_bytes([0; 32]), None));
}

#[test]
fn token_methods_preserve_correlation_and_remote_failures() {
    guard(|| TokenAccountBalance(a(), None));

    guard(|| TokenSupply(a(), None));
    guard(|| TokenLargestAccounts(a(), None));
    guard(|| TokenAccountsByOwner(a(), TokenAccountFilter::Mint(a()), None));
    guard(|| TokenAccountsByDelegate(a(), TokenAccountFilter::Mint(a()), None));
}
