//! Typed provider replay uses the real public JSON-RPC owner without external networking.
#[cfg(feature = "das")]
mod enabled {
    use super::super::fixtures::LIMITS;
    use core::{
        future::{Future, Ready, ready},
        marker::PhantomData,
        pin::{Pin, pin},
        task::{Context, Poll, Waker},
    };
    use serde_json::{Value, json};
    use solana_kit_client::{Client, Reply, Request, Transport, TransportError};
    use solana_kit_core::Address;
    use solana_kit_mpl_bubblegum::das::{
        DasApi, DasAsset, DasAssetProof, DasError, HeliusDasClient, bind_asset_proof,
        get_asset_with_proof,
    };
    use std::{
        cell::{Cell, RefCell},
        collections::VecDeque,
        rc::Rc,
    };

    struct Replay {
        responses: RefCell<VecDeque<Value>>,
        requests: Rc<RefCell<Vec<Value>>>,
    }

    impl Transport for Replay {
        type Exchange<'a> = Ready<Result<Reply, TransportError>>;

        fn exchange(&self, request: Request) -> Self::Exchange<'_> {
            let request: Value = serde_json::from_slice(&request.body).unwrap();
            let result = self.responses.borrow_mut().pop_front().unwrap();
            let envelope = json!({"jsonrpc":"2.0","id":request["id"],"result":result});
            self.requests.borrow_mut().push(request);
            ready(Ok(Reply {
                status: 200,
                body: serde_json::to_vec(&envelope).unwrap(),
            }))
        }
    }

    fn completed<F: Future>(future: F) -> F::Output {
        let mut future = pin!(future);
        let mut context = Context::from_waker(Waker::noop());

        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("owned in-memory reply unexpectedly pending"),
        }
    }

    fn fixture() -> Value {
        const DISPLAY_BASIS_POINTS: u16 = 250;
        let zero = "11111111111111111111111111111111";
        json!({"id":zero,"ownership":{"owner":zero,"frozen":false},
            "compression":{"compressed":true,"data_hash":zero,"creator_hash":zero,
                "asset_hash":zero,"tree":zero,"seq":18446744073709551615u64,"leaf_id":0},
            "content":{"json_uri":"uri","metadata":{"name":"name","symbol":"sym"}},
            "creators":[{"address":zero,"share":100,"verified":true}],
            "royalty":{"basis_points":DISPLAY_BASIS_POINTS,"sfbp_inherited":true}})
    }

    fn replay_client(responses: Vec<Value>) -> (HeliusDasClient<Replay>, Rc<RefCell<Vec<Value>>>) {
        let requests = Rc::new(RefCell::new(Vec::new()));
        let replay = Replay {
            responses: RefCell::new(responses.into()),
            requests: requests.clone(),
        };
        (
            HeliusDasClient::new(
                Client::new(replay, solana_kit_client::Limits::default()),
                LIMITS,
            ),
            requests,
        )
    }

    pub fn provider_normalization() {
        let (client, requests) = replay_client(vec![fixture()]);
        let id = Address::from_bytes([0; 32]);
        let asset = completed(client.get_asset(id)).unwrap();
        assert_eq!(asset.id, id);
        assert_eq!(asset.sequence, u64::MAX);
        assert_eq!(asset.metadata.name, "name");
        assert_eq!(asset.metadata.uri, "uri");
        assert_eq!(asset.metadata.seller_fee_basis_points, 250);

        assert_eq!(asset.raw_royalties.seller_fee_basis_points, Some(u16::MAX));
        assert_eq!(asset.raw_royalties.creators, Some(vec![]));
        assert_eq!(requests.borrow()[0]["method"], "getAsset");
        assert_eq!(requests.borrow()[0]["params"], json!([id.to_string()]));
    }

    pub fn malformed_provider() {
        for field in ["id", "ownership", "compression"] {
            let mut value = fixture();
            value.as_object_mut().unwrap().remove(field);
            let (client, _) = replay_client(vec![value]);
            assert!(completed(client.get_asset(Address::from_bytes([0; 32]))).is_err());
        }

        let mut value = fixture();
        value["ownership"]["owner"] = json!("invalid0");
        let (client, _) = replay_client(vec![value]);
        assert!(completed(client.get_asset(Address::from_bytes([0; 32]))).is_err());
        let (client, _) = replay_client(vec![json!({"root":"11111111111111111111111111111111"})]);
        assert!(completed(client.get_asset_proof(Address::from_bytes([0; 32]))).is_err());
    }

    pub fn exact_proof_binding() {
        let zero = "11111111111111111111111111111111";
        let proof = json!({"root":zero,"proof":[],"node_index":1,"leaf":zero,"tree_id":zero});
        let (client, _) = replay_client(vec![fixture(), proof]);
        let id = Address::from_bytes([0; 32]);
        let asset = completed(client.get_asset(id)).unwrap();
        let proof = completed(client.get_asset_proof(id)).unwrap();
        let bound = bind_asset_proof(id, asset.clone(), proof.clone(), LIMITS).unwrap();
        assert_eq!(bound.proof.index, 0);

        assert_eq!(bound.current_metadata.seller_fee_basis_points, u16::MAX);
        assert!(bound.current_metadata.creators.is_empty());
        assert_eq!(bound.asset.metadata.creators.len(), 1);
        let mut bad = proof.clone();
        bad.node_index = 0;
        assert!(bind_asset_proof(id, asset.clone(), bad, LIMITS).is_err());
        let mut bad = proof;
        bad.tree = Address::from_bytes([1; 32]);

        assert!(bind_asset_proof(id, asset, bad, LIMITS).is_err());
    }

    struct Controlled<T> {
        failed: bool,
        polls: Rc<Cell<usize>>,
        drops: Rc<Cell<usize>>,
        output: PhantomData<fn() -> T>,
    }

    impl<T> Future for Controlled<T> {
        type Output = Result<T, &'static str>;

        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
            self.polls.set(self.polls.get() + 1);

            if self.failed {
                return Poll::Ready(Err("proof provider failed"));
            }

            Poll::Pending
        }
    }

    impl<T> Drop for Controlled<T> {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
        }
    }

    struct ControlledApi {
        failed: bool,
        polls: Rc<Cell<usize>>,
        drops: Rc<Cell<usize>>,
    }

    impl ControlledApi {
        fn request<T>(&self, failed: bool) -> Controlled<T> {
            Controlled {
                failed,
                polls: self.polls.clone(),
                drops: self.drops.clone(),
                output: PhantomData,
            }
        }
    }

    impl DasApi for ControlledApi {
        type Error = &'static str;

        fn get_asset(
            &self,
            _: Address,
        ) -> impl Future<Output = Result<DasAsset, Self::Error>> + '_ {
            self.request(false)
        }

        fn get_asset_proof(
            &self,
            _: Address,
        ) -> impl Future<Output = Result<DasAssetProof, Self::Error>> + '_ {
            self.request(self.failed)
        }
    }

    pub fn request_cancellation() {
        for failed in [false, true] {
            cancellation_case(failed);
        }
    }

    fn cancellation_case(failed: bool) {
        let polls = Rc::new(Cell::new(0));
        let drops = Rc::new(Cell::new(0));
        let api = ControlledApi {
            failed,
            polls: polls.clone(),
            drops: drops.clone(),
        };
        let mut future = Box::pin(get_asset_with_proof(
            &api,
            Address::from_bytes([0; 32]),
            LIMITS,
        ));
        let result = future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()));

        if failed {
            assert!(matches!(
                result,
                Poll::Ready(Err(DasError::Provider("proof provider failed")))
            ));
        } else {
            assert!(result.is_pending());
            assert_eq!(drops.get(), 0);
        }

        assert_eq!(polls.get(), 2);
        drop(future);
        assert_eq!(drops.get(), 2);
    }

    #[test]
    fn complete_typed_provider_replay() {
        provider_normalization();
    }
    #[test]
    fn required_fields_and_hash_shapes_fail() {
        malformed_provider();
    }
    #[test]
    fn raw_royalties_and_index_bindings() {
        exact_proof_binding();
    }
    #[test]
    fn provider_failure_and_outer_drop_cancel_pending_work() {
        request_cancellation();
    }
}

#[cfg(feature = "das")]
pub use enabled::{
    exact_proof_binding, malformed_provider, provider_normalization, request_cancellation,
};
