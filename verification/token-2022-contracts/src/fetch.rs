use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_client::{Client, Reply, Request, Transport, TransportError, methods::ReadConfig};
use solana_kit_core::{Address, accounts::MaybeAccount, codecs::Limits};
use solana_kit_token_2022::{PROGRAM, fetch::*};
use std::{
    future::{Future, Ready, ready},
    task::{Context, Poll, Waker},
};
const RPC_BYTES: usize = 100_000;
const MINT_BYTES: usize = 82;
const FETCH_ITEMS: usize = 64;

struct Memory(String);
impl Transport for Memory {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let request = String::from_utf8(request.body).unwrap();
        assert!(request.contains("base64"));
        assert!(!request.contains("dataSlice"));
        ready(Ok(Reply {
            status: 200,
            body: self.0.clone().into_bytes(),
        }))
    }
}
fn run<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);

    match future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("memory transport is ready"),
    }
}
fn client(result: &str) -> Client<Memory> {
    Client::new(
        Memory(format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{{\"context\":{{\"slot\":18446744073709551615}},\"value\":{result}}}}}"
        )),
        solana_kit_client::Limits {
            bytes: RPC_BYTES,
            depth: 64,
        },
    )
}
fn account(owner: Address, space: usize) -> String {
    let mut bytes = vec![0; MINT_BYTES];
    bytes[45] = 1;
    format!(
        "{{\"data\":[\"{}\",\"base64\"],\"executable\":false,\"lamports\":18446744073709551615,\"owner\":\"{owner}\",\"rentEpoch\":18446744073709551615,\"space\":{space}}}",
        STANDARD.encode(bytes)
    )
}

pub(crate) fn explicit_fetch_preserves_context_metadata_and_missing_identity() {
    let address = Address::from_bytes([7; 32]);
    let rpc = client(&account(PROGRAM, MINT_BYTES));
    let result = run(fetch_account(
        &rpc,
        address,
        None,
        ReadConfig::default(),
        Limits::new(1000, FETCH_ITEMS),
    ))
    .unwrap();
    assert_eq!(result.context.slot, u64::MAX);
    assert_eq!(result.value.address, address);
    assert_eq!(result.value.lamports, u64::MAX);
    assert_eq!(result.value.rent_epoch, Some(u64::MAX));
    let rpc = client("null");

    let result = run(fetch_maybe_account(
        &rpc,
        address,
        None,
        ReadConfig::default(),
        Limits::new(1000, FETCH_ITEMS),
    ))
    .unwrap();

    assert!(matches!(result.value,MaybeAccount::Missing(found) if found==address));
    let rpc = client("null");
    let missing = run(fetch_account(
        &rpc,
        address,
        None,
        ReadConfig::default(),
        Limits::new(1000, FETCH_ITEMS),
    ));
    assert!(matches!(missing, Err(FetchError::Missing(found)) if found==address));
}

pub(crate) fn explicit_batch_owner_and_allocation_failures_are_preserved() {
    let address = Address::from_bytes([7; 32]);
    let rpc = client(&format!(
        "[{},null,{}]",
        account(PROGRAM, MINT_BYTES),
        account(PROGRAM, MINT_BYTES)
    ));
    let result = run(fetch_maybe_accounts(
        &rpc,
        vec![address, address, address],
        None,
        ReadConfig::default(),
        Limits::new(1000, FETCH_ITEMS),
    ))
    .unwrap();
    assert_eq!(result.value.len(), 3);

    assert!(matches!(result.value[1],MaybeAccount::Missing(found) if found==address));
    let rpc = client(&account(address, MINT_BYTES));
    assert!(
        run(fetch_account(
            &rpc,
            address,
            None,
            ReadConfig::default(),
            Limits::new(1000, FETCH_ITEMS)
        ))
        .is_err()
    );
    let rpc = client(&account(PROGRAM, MINT_BYTES + 1));

    assert!(
        run(fetch_account(
            &rpc,
            address,
            None,
            ReadConfig::default(),
            Limits::new(1000, FETCH_ITEMS)
        ))
        .is_err()
    );
    let rpc = client("[]");
    assert!(
        run(fetch_maybe_accounts(
            &rpc,
            vec![address],
            None,
            ReadConfig::default(),
            Limits::new(1000, FETCH_ITEMS)
        ))
        .is_err()
    );
}
