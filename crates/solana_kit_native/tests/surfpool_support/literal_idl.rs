//! Raw request assertions do not use the Value decoder whose token handling is at issue.

use super::surfpool_support::{ADDRESS, Directory, PROGRAM, options as deployment_options};
use serde_json::value::RawValue;
use solana_kit_client::{Client, Error, Limits, Reply, Request, Transport, TransportError};
use solana_kit_native::surfpool::{DeployError, ProgramSource, deploy};
use solana_kit_surfpool::{Object, Surfnet, deploy::DeployError as ProgramDeployError};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    future::{Ready, ready},
    rc::Rc,
};

type Frames = Rc<RefCell<Vec<Vec<u8>>>>;
const MAX_ACCEPTED_DEPTH: usize = 127;
const HUGE_INTEGER: &str = "340282366920938463463374607431768211457";

struct RawWire(Frames);

impl Transport for RawWire {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let envelope: BTreeMap<String, &RawValue> =
            serde_json::from_slice(&request.body).expect("valid request envelope");
        let body = format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":null}}"#,
            envelope["id"].get()
        );
        self.0.borrow_mut().push(request.body);

        ready(Ok(Reply {
            status: 200,
            body: body.into_bytes(),
        }))
    }
}

fn recording_client() -> (Surfnet<RawWire>, Frames) {
    let frames = Frames::default();
    let client = Client::new(RawWire(Rc::clone(&frames)), Limits::default());

    (Surfnet::new(client), frames)
}

#[tokio::test]
async fn reserved_json_keys_stay_literal_objects_on_the_register_idl_wire() {
    let directory = Directory::new().await;

    for literal in [
        r#"{"$serde_json::private::Number":"123"}"#,
        r#"{"$serde_json::private::RawValue":"{\"replacement\":true}"}"#,
    ] {
        let idl = format!(r#"{{"address":"old","huge":{HUGE_INTEGER},"unknown":[{literal}]}}"#);
        let path = directory.write("reserved-idl.json", idl.as_bytes()).await;
        let mut options = deployment_options(ProgramSource::Bytes(&PROGRAM));
        options.idl_path = Some(&path);
        let (client, frames) = recording_client();

        assert_eq!(deploy(&client, options).await.unwrap(), ADDRESS);

        let frames = frames.borrow();
        assert_eq!(frames.len(), 4);
        let envelope: BTreeMap<String, &RawValue> = serde_json::from_slice(&frames[3]).unwrap();
        assert_eq!(envelope["method"].get(), r#""surfnet_registerIdl""#);
        let params: Vec<&RawValue> = serde_json::from_str(envelope["params"].get()).unwrap();
        let registered: BTreeMap<String, &RawValue> =
            serde_json::from_str(params[0].get()).unwrap();

        assert_eq!(registered["address"].get(), format!("\"{ADDRESS}\""));
        assert_eq!(registered["huge"].get(), HUGE_INTEGER);
        assert_eq!(registered["unknown"].get(), format!("[{literal}]"));
    }

    directory.finish().await;
}

/// The smaller RPC budget rejects after IDL parsing, identifying which guard fired.
#[tokio::test]
async fn literal_container_decoding_keeps_the_original_serde_json_depth_boundary() {
    let directory = Directory::new().await;

    for depth in [MAX_ACCEPTED_DEPTH, MAX_ACCEPTED_DEPTH + 1] {
        let idl = format!(
            r#"{{"unknown":{}0{}}}"#,
            "[".repeat(depth - 1),
            "]".repeat(depth - 1)
        );
        let original = serde_json::from_str::<Object>(&idl);
        assert_eq!(original.is_ok(), depth == MAX_ACCEPTED_DEPTH);

        let path = directory.write("nested-idl.json", idl.as_bytes()).await;
        let mut options = deployment_options(ProgramSource::Bytes(&PROGRAM));
        options.idl_path = Some(&path);
        let (client, frames) = recording_client();
        let failure = deploy(&client, options).await.unwrap_err();

        if depth == MAX_ACCEPTED_DEPTH {
            assert!(matches!(
                failure,
                DeployError::Deploy(ProgramDeployError::Rpc(cause))
                    if matches!(cause.cause, Error::DepthLimit)
            ));
        } else {
            assert!(matches!(failure, DeployError::IdlNesting));
        }

        assert_eq!(frames.borrow().len(), 3);
    }

    directory.finish().await;
}

/// Last-wins insertion must not skip validation of an earlier duplicate value.
#[tokio::test]
async fn duplicate_keys_keep_last_wins_but_reject_invalid_overwritten_values() {
    let directory = Directory::new().await;
    let path = directory
        .write("duplicate-idl.json", br#"{"unknown":1,"unknown":2}"#)
        .await;
    let mut options = deployment_options(ProgramSource::Bytes(&PROGRAM));
    options.idl_path = Some(&path);
    let (client, frames) = recording_client();
    assert_eq!(deploy(&client, options).await.unwrap(), ADDRESS);

    {
        let frames = frames.borrow();
        let envelope: BTreeMap<String, &RawValue> = serde_json::from_slice(&frames[3]).unwrap();
        let params: Vec<&RawValue> = serde_json::from_str(envelope["params"].get()).unwrap();
        let registered: BTreeMap<String, &RawValue> =
            serde_json::from_str(params[0].get()).unwrap();
        assert_eq!(registered["unknown"].get(), "2");
    }

    let deep_duplicate = format!(
        r#"{{"unknown":{}0{},"unknown":0}}"#,
        "[".repeat(MAX_ACCEPTED_DEPTH),
        "]".repeat(MAX_ACCEPTED_DEPTH)
    );

    for (idl, nesting) in [
        (deep_duplicate.as_str(), true),
        (r#"{"unknown":"\uD800","unknown":0}"#, false),
    ] {
        assert!(serde_json::from_str::<Object>(idl).is_err());
        let path = directory
            .write("invalid-duplicate-idl.json", idl.as_bytes())
            .await;
        let mut options = deployment_options(ProgramSource::Bytes(&PROGRAM));
        options.idl_path = Some(&path);
        let (client, frames) = recording_client();
        let failure = deploy(&client, options).await.unwrap_err();

        if nesting {
            assert!(matches!(failure, DeployError::IdlNesting));
        } else {
            assert!(matches!(failure, DeployError::IdlJson(cause) if cause.is_syntax()));
        }

        assert_eq!(frames.borrow().len(), 3);
    }

    directory.finish().await;
}
