#![cfg(feature = "surfpool")]

mod surfpool_support;

use solana_kit_native::surfpool::{
    DeployError, DeployFile, DeployOptions, MAX_IDL_BYTES, MAX_PROGRAM_BYTES, ProgramSource, deploy,
};
use solana_kit_surfpool::deploy::{DeployError as ProgramDeployError, PROGRAM_CHUNK_BYTES};
use std::{
    error::Error,
    future::Future,
    io,
    pin::pin,
    task::{Context, Waker},
};
use surfpool_support::{
    ADDRESS, Directory, Mock, PRIVATE, PROGRAM, assert_program_writes, options,
};
use tokio::fs;

#[tokio::test]
async fn bytes_and_explicit_file_produce_identical_ordered_writes_and_address() {
    let directory = Directory::new().await;
    let path = directory.write("program.so", &PROGRAM).await;
    let (bytes_client, bytes_observed) = Mock::default().client();
    let (file_client, file_observed) = Mock::default().client();

    assert_eq!(
        deploy(&bytes_client, options(ProgramSource::Bytes(&PROGRAM)))
            .await
            .unwrap(),
        ADDRESS
    );
    assert_eq!(
        deploy(&file_client, options(ProgramSource::Path(&path)))
            .await
            .unwrap(),
        ADDRESS
    );
    assert_program_writes(&bytes_observed);
    assert_program_writes(&file_observed);
    assert_eq!(
        *bytes_observed.requests.borrow(),
        *file_observed.requests.borrow()
    );
    directory.finish().await;
}

#[tokio::test]
async fn default_chunks_reuse_the_portable_three_kibibyte_wire_budget() {
    let bytes = vec![7; PROGRAM_CHUNK_BYTES + 1];
    let options = DeployOptions::new(ADDRESS, ProgramSource::Bytes(&bytes));
    let (client, observed) = Mock::default().client();

    assert_eq!(options.chunk_bytes.get(), 3 * 1024);
    assert_eq!(options.max_program_bytes, MAX_PROGRAM_BYTES);
    assert_eq!(options.max_idl_bytes, MAX_IDL_BYTES);
    assert_eq!(deploy(&client, options).await.unwrap(), ADDRESS);

    let requests = observed.requests.borrow();
    assert_eq!(requests.len(), 2);
    assert_eq!(
        requests[0]["params"][1].as_str().unwrap().len(),
        2 * PROGRAM_CHUNK_BYTES
    );
    assert_eq!(requests[1]["params"][2], PROGRAM_CHUNK_BYTES);
    assert_eq!(requests[1]["params"][1], "07");
}

#[tokio::test]
async fn idl_is_read_after_writes_and_preserves_unknown_fields_and_exact_integers() {
    let directory = Directory::new().await;
    let idl_path = directory.0.join("late-idl.json");
    let idl = br#"{"address":"previous","unknown":{"huge":340282366920938463463374607431768211457,"array":[true,null,{"extension":"retained"}]}}"#;
    let (client, observed) = Mock {
        create_idl_after: Some((3, idl_path.clone(), idl.to_vec())),
        ..Mock::default()
    }
    .client();
    let mut options = options(ProgramSource::Bytes(&PROGRAM));
    options.idl_path = Some(&idl_path);

    assert_eq!(deploy(&client, options).await.unwrap(), ADDRESS);
    assert_program_writes(&observed);

    {
        let requests = observed.requests.borrow();
        assert_eq!(requests.len(), 4);
        assert_eq!(requests[3]["method"], "surfnet_registerIdl");
        let mut expected: serde_json::Value = serde_json::from_slice(idl).unwrap();
        expected["address"] = ADDRESS.to_string().into();

        assert_eq!(requests[3]["params"], serde_json::json!([expected]));
        assert_eq!(
            requests[3]["params"][0]["unknown"]["huge"].to_string(),
            "340282366920938463463374607431768211457"
        );
    }

    directory.finish().await;
}

#[test]
fn native_deployment_future_retains_send() {
    fn require_send(_: impl Send) {}

    let transport = solana_kit_native::NativeHttp::new(
        "http://127.0.0.1:8899",
        solana_kit_native::HttpConfig::default(),
    )
    .unwrap();
    let client = solana_kit_surfpool::Surfnet::new(solana_kit_client::Client::new(
        transport,
        solana_kit_client::Limits::default(),
    ));
    require_send(deploy(&client, options(ProgramSource::Bytes(&PROGRAM))));
}

#[tokio::test]
async fn malformed_or_non_object_idl_retains_all_earlier_writes() {
    let directory = Directory::new().await;

    for bytes in [
        b"{invalid".as_slice(),
        b"[]",
        b"null",
        b"true",
        b"\"private-input-sentinel\"",
    ] {
        let path = directory.write("invalid-idl.json", bytes).await;
        let (client, observed) = Mock::default().client();
        let mut options = options(ProgramSource::Bytes(&PROGRAM));
        options.idl_path = Some(&path);
        let failure = deploy(&client, options).await.unwrap_err();

        assert!(
            matches!(&failure, DeployError::IdlJson(cause) if cause.is_data() || cause.is_syntax())
        );
        assert_program_writes(&observed);
        assert_eq!(observed.requests.borrow().len(), 3);
        assert!(failure.source().is_none());
        assert!(!format!("{failure:?} {failure}").contains(PRIVATE));
    }

    directory.finish().await;
}

#[tokio::test]
async fn missing_idl_keeps_writes_and_the_original_io_cause_without_path_diagnostics() {
    let directory = Directory::new().await;
    let path = directory.0.join(PRIVATE);
    let (client, observed) = Mock::default().client();
    let mut options = options(ProgramSource::Bytes(&PROGRAM));
    options.idl_path = Some(&path);
    let failure = deploy(&client, options).await.unwrap_err();

    assert!(matches!(
        &failure,
        DeployError::Io { input: DeployFile::Idl, cause }
            if cause.kind() == io::ErrorKind::NotFound
    ));
    assert_program_writes(&observed);
    assert_eq!(observed.requests.borrow().len(), 3);
    assert!(failure.source().is_none());
    assert!(!format!("{failure:?} {failure} {options:?}").contains(PRIVATE));
    directory.finish().await;
}

#[tokio::test]
async fn rpc_failure_stops_before_the_invalid_idl_and_retains_the_original_remote_error() {
    let directory = Directory::new().await;
    let path = directory.write("invalid-idl.json", b"not JSON").await;
    let (client, observed) = Mock {
        fail_at: Some(2),
        ..Mock::default()
    }
    .client();
    let mut options = options(ProgramSource::Bytes(&PROGRAM));
    options.idl_path = Some(&path);
    let failure = deploy(&client, options).await.unwrap_err();

    assert_eq!(observed.requests.borrow().len(), 2);
    assert_eq!(observed.accepted_writes.get(), 1);
    assert!(!format!("{failure:?} {failure}").contains(PRIVATE));
    assert!(failure.source().is_none());

    match failure {
        DeployError::Deploy(ProgramDeployError::Rpc(cause)) => {
            assert_eq!(cause.method, "surfnet_writeProgram");
            assert_eq!(cause.rpc_code(), Some(-32000));
            assert_eq!(cause.message(), Some(PRIVATE));
        }
        other => panic!("expected original write failure: {other}"),
    }

    directory.finish().await;
}

#[tokio::test]
async fn register_failure_preserves_completed_program_writes_without_retry() {
    let directory = Directory::new().await;
    let path = directory.write("idl.json", b"{}").await;
    let (client, observed) = Mock {
        fail_at: Some(4),
        ..Mock::default()
    }
    .client();
    let mut options = options(ProgramSource::Bytes(&PROGRAM));
    options.idl_path = Some(&path);
    let failure = deploy(&client, options).await.unwrap_err();

    assert_program_writes(&observed);
    assert_eq!(observed.requests.borrow().len(), 4);
    assert!(matches!(
        failure,
        DeployError::Deploy(ProgramDeployError::Rpc(cause))
            if cause.method == "surfnet_registerIdl" && cause.message() == Some(PRIVATE)
    ));
    directory.finish().await;
}

#[test]
fn cancellation_drops_the_active_exchange_and_never_starts_later_writes_or_idl() {
    let path = std::path::Path::new("private-input-sentinel-does-not-exist.json");
    let (client, observed) = Mock {
        pending_at: Some(2),
        ..Mock::default()
    }
    .client();
    let mut options = options(ProgramSource::Bytes(&PROGRAM));
    options.idl_path = Some(path);
    drop(deploy(&client, options));
    assert!(observed.requests.borrow().is_empty());

    {
        let mut future = pin!(deploy(&client, options));
        assert!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        assert_eq!(observed.requests.borrow().len(), 2);
        assert_eq!(observed.accepted_writes.get(), 1);
        assert_eq!(observed.pending_drops.get(), 0);
    }

    assert_eq!(observed.pending_drops.get(), 1);
    assert_eq!(observed.requests.borrow().len(), 2);
}

#[path = "surfpool_support/bounds.rs"]
mod bounds;

#[path = "surfpool_support/literal_idl.rs"]
mod literal_idl;
