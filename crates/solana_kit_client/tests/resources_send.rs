#![cfg(feature = "resource-estimation")]
//! Native caller executors can move each concrete resource future between threads.
use serde_json::{Value, json};
use solana_kit_client::{
    Client, Limits, Reply, Request, Transport, TransportError,
    resources::{
        EstimateConfig, ResourceMessage, VersionContext, estimate, estimate_and_set,
        estimate_and_set_with_client,
    },
};
use solana_kit_core::{
    Address, Hash,
    transaction::{Lifetime, V1Config},
};
use std::{
    borrow::Cow,
    future::{Ready, ready},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

const COMPUTE_USAGE: u32 = 7777;
const LOADED_USAGE: u32 = 3333;
const SLOT: u64 = 42;
const BLOCK_HEIGHT: u64 = 1000;
const HTTP_OK: u16 = 200;

struct ReadyTransport {
    requests: Arc<AtomicUsize>,
}

impl Transport for ReadyTransport {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let request: Value = serde_json::from_slice(&request.body).expect("valid RPC request");
        assert_eq!(request["method"], "simulateTransaction");
        assert_eq!(request["params"][1]["sigVerify"], false);
        assert_eq!(request["params"][1]["replaceRecentBlockhash"], true);
        self.requests.fetch_add(1, Ordering::SeqCst);
        let response = json!({
            "jsonrpc": "2.0",
            "id": request["id"],
            "result": {
                "context": { "slot": SLOT },
                "value": {
                    "accounts": null, "err": null, "fee": null,
                    "loadedAccountsDataSize": LOADED_USAGE, "loadedAddresses": null,
                    "logs": [], "preBalances": [], "postBalances": [],
                    "preTokenBalances": [], "postTokenBalances": [], "returnData": null,
                    "replacementBlockhash": {
                        "blockhash": Hash::from_bytes([7; 32]).to_string(),
                        "lastValidBlockHeight": BLOCK_HEIGHT
                    },
                    "unitsConsumed": COMPUTE_USAGE
                }
            }
        });

        ready(Ok(Reply {
            status: HTTP_OK,
            body: response.to_string().into_bytes(),
        }))
    }
}

fn client() -> (Client<ReadyTransport>, Arc<AtomicUsize>) {
    let requests = Arc::new(AtomicUsize::new(0));
    let transport = ReadyTransport {
        requests: requests.clone(),
    };

    (Client::new(transport, Limits::default()), requests)
}

fn draft() -> ResourceMessage<'static> {
    ResourceMessage {
        version: VersionContext::V1(V1Config::default()),
        payer: Address::from_bytes([1; 32]),
        lifetime: Lifetime::Blockhash {
            blockhash: Hash::from_bytes([7; 32]),
            last_valid_block_height: BLOCK_HEIGHT,
        },
        instructions: Cow::Owned(vec![]),
    }
}

fn assert_limits(message: &ResourceMessage<'_>) {
    assert_eq!(message.compute_unit_limit(), Some(COMPUTE_USAGE));
    assert_eq!(
        message.loaded_accounts_data_size_limit(),
        Some(LOADED_USAGE)
    );
}

#[tokio::test]
async fn native_resource_futures_run_on_a_caller_executor() {
    let (client, requests) = client();
    let observed =
        tokio::spawn(async move { estimate(&client, &draft(), EstimateConfig::default()).await })
            .await
            .expect("estimation task joined")
            .expect("estimation succeeded");
    assert_eq!(observed.compute_unit_limit, COMPUTE_USAGE);
    assert_eq!(observed.loaded_accounts_data_size_limit, Some(LOADED_USAGE));
    assert_eq!(requests.load(Ordering::SeqCst), 1);

    let (client, requests) = self::client();
    let tuned = tokio::spawn(async move {
        estimate_and_set_with_client(&client, draft(), EstimateConfig::default()).await
    })
    .await
    .expect("client tuning task joined")
    .expect("client tuning succeeded");
    assert_limits(&tuned);
    assert_eq!(requests.load(Ordering::SeqCst), 1);

    let (client, requests) = self::client();
    let tuned = tokio::spawn(async move {
        estimate_and_set(draft(), |message| async move {
            estimate(&client, &message, EstimateConfig::default()).await
        })
        .await
    })
    .await
    .expect("caller tuning task joined")
    .expect("caller tuning succeeded");
    assert_limits(&tuned);
    assert_eq!(requests.load(Ordering::SeqCst), 1);
}
