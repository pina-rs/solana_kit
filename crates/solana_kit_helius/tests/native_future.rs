#![cfg(not(target_arch = "wasm32"))]

use solana_kit_client::{Reply, TransportError};
use solana_kit_helius::{service::*, webhooks::GetAllWebhooks};
use std::{
    future::{Ready, ready},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

const CODEC_BUDGET: usize = 4096;

struct Native {
    exchanges: Arc<AtomicUsize>,
}

impl ServiceTransport<Service> for Native {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, _request: ServiceRequest) -> Self::Exchange<'_> {
        self.exchanges.fetch_add(1, Ordering::SeqCst);
        ready(Ok(Reply {
            status: 200,
            body: b"[]".to_vec(),
        }))
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_rest_call_retains_send_on_native_executor() {
    let exchanges = Arc::new(AtomicUsize::new(0));
    let client = ServiceClient::new(
        Native {
            exchanges: Arc::clone(&exchanges),
        },
        Budgets::default(),
    );
    let result = tokio::spawn(async move { client.call(GetAllWebhooks {}).await })
        .await
        .expect("native task joined")
        .expect("typed call succeeded");
    assert!(result.is_empty());
    assert_eq!(exchanges.load(Ordering::SeqCst), 1);
}

impl solana_kit_client::Transport for Native {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: solana_kit_client::Request) -> Self::Exchange<'_> {
        self.exchanges.fetch_add(1, Ordering::SeqCst);
        let value: serde_json::Value = serde_json::from_slice(&request.body).expect("RPC frame");
        let id = value["id"].as_u64().expect("correlation ID");

        let result = match value["method"].as_str() {
            Some("getLatestBlockhash") => {
                r#"{"context":{"slot":1},"value":{"blockhash":"11111111111111111111111111111111","lastValidBlockHeight":99}}"#
            }
            Some("getProgramAccounts") => r#"{"context":{"slot":100},"value":[]}"#,
            other => panic!("unexpected native RPC operation {other:?}"),
        };

        ready(Ok(Reply {
            status: 200,
            body: format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#).into_bytes(),
        }))
    }
}

struct NativeSigner;

fn owner() -> solana_kit_core::Address {
    #[cfg(feature = "crypto")]
    {
        solana_kit_core::crypto::Keypair::from_seed(&[1; 32]).address()
    }
    #[cfg(not(feature = "crypto"))]
    {
        solana_kit_core::Address::from_bytes([1; 32])
    }
}

impl solana_kit_client::plans::Signer for NativeSigner {
    type Error = core::convert::Infallible;
    type Sign<'a> = Ready<Result<solana_kit_core::transaction::Transaction, Self::Error>>;

    fn sign(&self, message: solana_kit_core::transaction::Message) -> Self::Sign<'_> {
        let transaction = solana_kit_core::transaction::Transaction::new(message);
        #[cfg(feature = "crypto")]
        let transaction = transaction
            .sign(&solana_kit_core::crypto::Keypair::from_seed(&[1; 32]))
            .expect("fixture owner signature");
        #[cfg(not(feature = "crypto"))]
        let transaction = transaction
            .with_signature(
                owner(),
                transaction.message().bytes(),
                solana_kit_core::Signature::from_bytes([7; 64]),
            )
            .expect("structural signature");
        ready(Ok(transaction))
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_staking_call_retains_send_on_native_executor() {
    let exchanges = Arc::new(AtomicUsize::new(0));
    let client = solana_kit_client::Client::new(
        Native {
            exchanges: Arc::clone(&exchanges),
        },
        solana_kit_client::Limits::default(),
    );
    let transaction = tokio::spawn(async move {
        solana_kit_helius::staking::create_unstake_transaction(
            &client,
            &NativeSigner,
            solana_kit_helius::staking::UnstakeRequest {
                owner: owner(),
                stake_account: solana_kit_core::Address::from_bytes([2; 32]),
            },
            solana_kit_helius::staking::PreparationOptions {
                commitment: None,
                instruction_limits: solana_kit_core::codecs::Limits::new(
                    CODEC_BUDGET,
                    CODEC_BUDGET,
                ),
            },
        )
        .await
    })
    .await
    .expect("native task joined")
    .expect("prepared transaction");
    assert_eq!(transaction.message().instructions().len(), 1);
    assert_eq!(exchanges.load(Ordering::SeqCst), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_stake_discovery_retains_send_on_native_executor() {
    let exchanges = Arc::new(AtomicUsize::new(0));
    let client = solana_kit_client::Client::new(
        Native {
            exchanges: Arc::clone(&exchanges),
        },
        solana_kit_client::Limits::default(),
    );
    let accounts = tokio::spawn(async move {
        solana_kit_helius::staking::get_helius_stake_accounts(
            &client,
            owner(),
            solana_kit_helius::staking::StakeAccountQuery {
                read: solana_kit_client::methods::ReadConfig {
                    commitment: None,
                    min_context_slot: Some(90),
                },
                decode_limits: solana_kit_core::codecs::Limits::new(CODEC_BUDGET, CODEC_BUDGET),
                max_accounts: 10,
            },
        )
        .await
    })
    .await
    .expect("native task joined")
    .expect("typed discovery");
    assert_eq!(accounts.context.slot, 100);
    assert!(accounts.value.is_empty());
    assert_eq!(exchanges.load(Ordering::SeqCst), 1);
}
