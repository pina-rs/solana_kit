//! Discovery fixtures use literal Stake layout offsets and real shared RPC framing.

use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_client::{
    Client, Reply, Request, Transport, TransportError,
    methods::{Commitment, ReadConfig},
};
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_helius::staking::*;
use std::{
    cell::RefCell,
    future::{Ready, ready},
    rc::Rc,
};

type Requests = Rc<RefCell<Vec<serde_json::Value>>>;

const CODEC_BUDGET: usize = 4096;
const STAKE_ALLOCATION: usize = 200;
const MINIMUM_SLOT: u64 = 90;
const EVALUATED_SLOT: u64 = 100;
const WITHDRAWER_OFFSET: usize = 44;

struct Rpc {
    result: RefCell<Option<String>>,
    requests: Requests,
}

impl Transport for Rpc {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let value: serde_json::Value =
            serde_json::from_slice(&request.body).expect("real RPC frame");
        let id = value["id"].as_u64().expect("correlation ID");
        self.requests.borrow_mut().push(value);
        let result = self
            .result
            .borrow_mut()
            .take()
            .expect("one discovery exchange");
        ready(Ok(Reply {
            status: 200,
            body: format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#).into_bytes(),
        }))
    }
}

fn address(seed: u8) -> Address {
    Address::from_bytes([seed; 32])
}

fn query() -> StakeAccountQuery {
    StakeAccountQuery {
        read: ReadConfig {
            commitment: Some(Commitment::Confirmed),
            min_context_slot: Some(MINIMUM_SLOT),
        },
        decode_limits: Limits::new(CODEC_BUDGET, CODEC_BUDGET),
        max_accounts: 10,
    }
}

fn record(seed: u8, withdrawer: Address, voter: Option<Address>) -> serde_json::Value {
    let mut bytes = vec![0; STAKE_ALLOCATION];

    bytes[..4].copy_from_slice(&u32::from(if voter.is_some() { 2_u8 } else { 1_u8 }).to_le_bytes());
    bytes[4..12].copy_from_slice(&123_u64.to_le_bytes());
    bytes[12..44].copy_from_slice(address(9).as_bytes());
    bytes[44..76].copy_from_slice(withdrawer.as_bytes());

    if let Some(voter) = voter {
        bytes[124..156].copy_from_slice(voter.as_bytes());
        bytes[156..164].copy_from_slice(&1000_u64.to_le_bytes());
        bytes[164..172].copy_from_slice(&1_u64.to_le_bytes());
        bytes[172..180].copy_from_slice(&u64::MAX.to_le_bytes());
    }

    serde_json::json!({"pubkey":address(seed).to_string(),"account":{
        "owner":solana_kit_stake::PROGRAM.to_string(),"lamports":u64::MAX,"executable":false,
        "rentEpoch":u64::MAX,"space":STAKE_ALLOCATION,"data":[STANDARD.encode(bytes),"base64"]
    }})
}

fn client(records: Vec<serde_json::Value>, slot: u64) -> (Client<Rpc>, Requests) {
    let requests = Rc::new(RefCell::new(Vec::new()));
    let result = serde_json::json!({"context":{"slot":slot},"value":records}).to_string();
    (
        Client::new(
            Rpc {
                result: RefCell::new(Some(result)),
                requests: Rc::clone(&requests),
            },
            solana_kit_client::Limits::default(),
        ),
        requests,
    )
}

pub(super) fn discovery() {
    let wallet = address(1);
    let (rpc, requests) = client(
        vec![
            record(4, wallet, Some(HELIUS_VALIDATOR)),
            record(5, wallet, Some(address(6))),
            record(7, wallet, None),
        ],
        EVALUATED_SLOT,
    );
    let result = futures::executor::block_on(get_helius_stake_accounts(&rpc, wallet, query()))
        .expect("one contextual typed scan");
    assert_eq!(result.context.slot, EVALUATED_SLOT);
    assert_eq!(result.value.len(), 1);

    let account = &result.value[0];
    assert_eq!(account.address, address(4));
    assert_eq!(account.owner, solana_kit_stake::PROGRAM);
    assert_eq!(account.lamports, u64::MAX);
    assert_eq!(account.rent_epoch, Some(u64::MAX));
    assert_eq!(account.space, STAKE_ALLOCATION as u64);

    let solana_kit_stake::StakeStateV2::Stake(state) = &account.data.state else {
        panic!("delegated state")
    };
    assert_eq!(state.item0.authorized.staker, address(9));
    assert_eq!(state.item0.authorized.withdrawer, wallet);
    assert_eq!(state.item1.delegation.voter_pubkey, HELIUS_VALIDATOR);
    assert_eq!(state.item1.delegation.deactivation_epoch.value, u64::MAX);

    assert_eq!(requests.borrow().len(), 1);
    let frame = &requests.borrow()[0];
    assert_eq!(frame["method"], "getProgramAccounts");
    assert_eq!(
        frame["params"],
        serde_json::json!([solana_kit_stake::PROGRAM.to_string(),{
            "encoding":"base64","commitment":"confirmed",
            "minContextSlot":MINIMUM_SLOT,"withContext":true,
            "filters":[{"memcmp":{"offset":WITHDRAWER_OFFSET,"encoding":"base58","bytes":wallet.to_string()}}]
        }])
    );
    malformed_accounts();
    scan_limits();
    noncontextual_response();
}

fn noncontextual_response() {
    let requests = Rc::new(RefCell::new(Vec::new()));
    let rpc = Client::new(
        Rpc {
            result: RefCell::new(Some("[]".to_owned())),
            requests: Rc::clone(&requests),
        },
        solana_kit_client::Limits::default(),
    );
    assert!(matches!(
        futures::executor::block_on(get_helius_stake_accounts(&rpc, address(1), query())),
        Err(StakeAccountError::Rpc(solana_kit_client::Error::Schema(_)))
    ));
    assert_eq!(requests.borrow().len(), 1);
}

fn failure(record: serde_json::Value, options: StakeAccountQuery) -> StakeAccountError {
    let (rpc, reads) = client(vec![record], 100);
    let result = futures::executor::block_on(get_helius_stake_accounts(&rpc, address(1), options))
        .expect_err("invalid account must not become an empty successful scan");
    assert_eq!(reads.borrow().len(), 1);
    result
}

fn malformed_accounts() {
    let good = record(4, address(1), Some(HELIUS_VALIDATOR));
    let mut foreign = good.clone();
    foreign["account"]["owner"] = serde_json::json!(address(0).to_string());
    assert!(matches!(
        failure(foreign, query()),
        StakeAccountError::Rpc(_)
    ));
    let mut size = good.clone();
    size["account"]["space"] = serde_json::json!(201);
    assert!(matches!(failure(size, query()), StakeAccountError::Rpc(_)));

    let mut executable = good.clone();
    executable["account"]["executable"] = serde_json::json!(true);
    assert!(matches!(
        failure(executable, query()),
        StakeAccountError::Executable(_)
    ));
    assert!(matches!(
        failure(record(4, address(2), Some(HELIUS_VALIDATOR)), query()),
        StakeAccountError::AuthorityMismatch(_)
    ));

    let mut invalid = good.clone();
    let mut bytes = STANDARD
        .decode(
            good["account"]["data"][0]
                .as_str()
                .expect("encoded fixture"),
        )
        .expect("fixture bytes");
    bytes[..4].copy_from_slice(&99_u32.to_le_bytes());
    invalid["account"]["data"][0] = serde_json::json!(STANDARD.encode(bytes));
    assert!(matches!(
        failure(invalid, query()),
        StakeAccountError::Decode(_)
    ));
    let mut budget = query();
    budget.decode_limits = Limits::new(1, 1);
    assert!(matches!(
        failure(good, budget),
        StakeAccountError::Decode(_)
    ));
}

fn scan_limits() {
    let good = record(4, address(1), Some(HELIUS_VALIDATOR));
    let (rpc, reads) = client(vec![good.clone(), good.clone()], 100);
    assert!(matches!(
        futures::executor::block_on(get_helius_stake_accounts(&rpc, address(1), query())),
        Err(StakeAccountError::Rpc(_))
    ));
    assert_eq!(reads.borrow().len(), 1);

    let mut maximum = query();
    maximum.max_accounts = 0;
    assert!(matches!(
        failure(good.clone(), maximum),
        StakeAccountError::AccountLimit
    ));
    let mut minimum = query();
    minimum.read.min_context_slot = Some(EVALUATED_SLOT + 1);
    assert!(matches!(
        failure(good, minimum),
        StakeAccountError::StaleContext
    ));
    let (rpc, _) = client(vec![], 100);
    assert!(
        futures::executor::block_on(get_helius_stake_accounts(&rpc, address(1), query()))
            .expect("valid empty scan")
            .value
            .is_empty()
    );
}
