#![cfg(all(feature = "surfpool", feature = "keys"))]

mod artifact_support;
#[path = "artifact_support/wire.rs"]
mod wire;
use artifact_support::{Directory, PROGRAM};
use solana_kit_core::{
    Address,
    crypto::{ExportableKeypair, KeypairImportError},
};
use solana_kit_native::surfpool::{
    DeployError, DeployFile, DeployProgramError, DeployProgramOptions, ProgramArtifacts,
    deploy_program, discover_program_artifacts,
};
use std::{error::Error, num::NonZeroUsize, sync::atomic::Ordering};
use tokio::fs;
use wire::Wire;

fn fixture_key() -> (Vec<u8>, Address) {
    let key = ExportableKeypair::from_seed(&[7; 32]);
    let backup = key.backup();

    (
        serde_json::to_vec(&backup.expose_secret()[..]).unwrap(),
        key.signer().address(),
    )
}

fn options() -> DeployProgramOptions {
    DeployProgramOptions {
        chunk_bytes: NonZeroUsize::new(2).unwrap(),
        ..Default::default()
    }
}

async fn artifacts(directory: &Directory) -> (ProgramArtifacts, Address) {
    let root = directory.root("target").await;
    let (json, address) = fixture_key();
    directory.pair(&root, &json).await;

    (
        discover_program_artifacts("demo", &[root]).await.unwrap(),
        address,
    )
}

#[tokio::test]
async fn verified_key_address_drives_ordered_writes_and_the_future_is_send() {
    let directory = Directory::new().await;
    let (artifacts, address) = artifacts(&directory).await;
    let (client, observed) = Wire::default().client();
    fn assert_send<T: Send>(_: T) {}
    assert_send(deploy_program(&client, &artifacts, options()));

    assert_eq!(
        deploy_program(&client, &artifacts, options())
            .await
            .unwrap(),
        address
    );
    let frames = observed.frames.lock().unwrap();
    assert_eq!(frames.len(), 3);

    for (frame, (hex, offset)) in frames.iter().zip([("0102", 0), ("0304", 2), ("ff", 4)]) {
        assert_eq!(frame["method"], "surfnet_writeProgram");
        assert_eq!(
            frame["params"],
            serde_json::json!([address.to_string(), hex, offset])
        );
    }

    drop(frames);
    directory.finish().await;
}

#[tokio::test]
async fn selected_malformed_key_is_fatal_before_program_read_with_no_later_root_fallback() {
    let directory = Directory::new().await;
    let first = directory.root("first").await;
    let second = directory.root("second").await;
    directory.pair(&first, b"private-invalid-json").await;
    directory.pair(&second, &fixture_key().0).await;

    let selected = discover_program_artifacts("demo", &[first, second])
        .await
        .unwrap();
    assert_eq!(selected.candidate_index(), 0);
    fs::remove_file(selected.program_path()).await.unwrap();
    let (client, observed) = Wire::default().client();
    let failure = deploy_program(&client, &selected, options())
        .await
        .unwrap_err();

    assert!(matches!(&failure, DeployProgramError::KeyJson(cause) if cause.is_syntax()));
    assert!(!format!("{failure:?} {failure}").contains("private-invalid-json"));
    assert!(failure.source().is_none());
    assert!(observed.frames.lock().unwrap().is_empty());
    directory.finish().await;
}

#[tokio::test]
async fn key_json_requires_exactly_64_integer_bytes_and_a_matching_public_half() {
    let directory = Directory::new().await;
    let (artifacts, _) = artifacts(&directory).await;
    let (client, observed) = Wire::default().client();

    for json in [
        format!("[{}]", vec!["0"; 63].join(",")),
        format!("[{}]", vec!["0"; 65].join(",")),
        "[-1]".into(),
        "[256]".into(),
        "[1.0]".into(),
        "[\"1\"]".into(),
        "{}".into(),
        "\"private-key-json-sentinel\"".into(),
    ] {
        fs::write(artifacts.keypair_path(), json).await.unwrap();
        assert!(matches!(
            deploy_program(&client, &artifacts, options()).await,
            Err(DeployProgramError::KeyJson(_))
        ));
    }

    let mut bytes: Vec<u8> = serde_json::from_slice(&fixture_key().0).unwrap();
    bytes[63] ^= 1;
    fs::write(
        artifacts.keypair_path(),
        serde_json::to_vec(&bytes).unwrap(),
    )
    .await
    .unwrap();
    assert!(matches!(
        deploy_program(&client, &artifacts, options()).await,
        Err(DeployProgramError::KeyImport(
            KeypairImportError::PublicKeyMismatch
        ))
    ));
    assert!(observed.frames.lock().unwrap().is_empty());
    directory.finish().await;
}

#[tokio::test]
async fn key_file_limit_and_regular_type_are_checked_before_any_rpc() {
    let directory = Directory::new().await;
    let (artifacts, _) = artifacts(&directory).await;
    let (client, observed) = Wire::default().client();
    let mut options = options();
    options.max_keypair_bytes = 1;
    assert!(matches!(
        deploy_program(&client, &artifacts, options).await,
        Err(DeployProgramError::KeyFile(DeployError::TooLarge {
            input: DeployFile::Keypair,
            limit: 1
        }))
    ));

    fs::remove_file(artifacts.keypair_path()).await.unwrap();
    fs::create_dir(artifacts.keypair_path()).await.unwrap();
    assert!(matches!(
        deploy_program(&client, &artifacts, options).await,
        Err(DeployProgramError::KeyFile(DeployError::NotRegular {
            input: DeployFile::Keypair
        }))
    ));
    assert!(observed.frames.lock().unwrap().is_empty());
    directory.finish().await;
}

#[tokio::test]
async fn late_idl_creation_is_ignored_and_selected_idl_removal_fails_after_writes() {
    let directory = Directory::new().await;
    let (artifacts, address) = artifacts(&directory).await;
    let root = artifacts.program_path().parent().unwrap().parent().unwrap();
    let idl = root.join("idl/demo.json");
    fs::write(&idl, b"{}").await.unwrap();
    let (client, observed) = Wire::default().client();
    assert_eq!(
        deploy_program(&client, &artifacts, options())
            .await
            .unwrap(),
        address
    );
    assert_eq!(observed.frames.lock().unwrap().len(), 3);

    let selected = discover_program_artifacts("demo", &[root.to_path_buf()])
        .await
        .unwrap();
    fs::remove_file(&idl).await.unwrap();
    let (client, observed) = Wire::default().client();
    let failure = deploy_program(&client, &selected, options())
        .await
        .unwrap_err();
    assert!(matches!(
        &failure,
        DeployProgramError::Deploy(DeployError::Io { input: DeployFile::Idl, cause })
            if cause.kind() == std::io::ErrorKind::NotFound
    ));
    assert_eq!(observed.frames.lock().unwrap().len(), 3);
    directory.finish().await;
}

#[tokio::test]
async fn explicit_program_and_idl_caps_and_key_limit_override_reach_existing_deploy() {
    let directory = Directory::new().await;
    let (artifacts, address) = artifacts(&directory).await;
    let root = artifacts.program_path().parent().unwrap().parent().unwrap();
    fs::write(root.join("idl/demo.json"), b"{}").await.unwrap();
    let selected = discover_program_artifacts("demo", &[root.to_path_buf()])
        .await
        .unwrap();
    let mut options = options();
    options.max_keypair_bytes = fixture_key().0.len();
    options.max_program_bytes = 4;

    let (client, observed) = Wire::default().client();
    assert!(matches!(
        deploy_program(&client, &selected, options).await,
        Err(DeployProgramError::Deploy(DeployError::TooLarge {
            input: DeployFile::Program,
            limit: 4
        }))
    ));
    assert!(observed.frames.lock().unwrap().is_empty());

    options.max_program_bytes = PROGRAM.len();
    options.max_idl_bytes = 1;
    assert!(matches!(
        deploy_program(&client, &selected, options).await,
        Err(DeployProgramError::Deploy(DeployError::TooLarge {
            input: DeployFile::Idl,
            limit: 1
        }))
    ));
    assert_eq!(observed.frames.lock().unwrap().len(), 3);

    let (client, observed) = Wire::default().client();
    options.max_idl_bytes = 2;
    assert_eq!(
        deploy_program(&client, &selected, options).await.unwrap(),
        address
    );
    assert_eq!(observed.frames.lock().unwrap().len(), 4);
    directory.finish().await;
}

#[tokio::test]
async fn cancellation_drops_active_exchange_and_starts_no_later_chunk_or_idl() {
    let directory = Directory::new().await;
    let (artifacts, _) = artifacts(&directory).await;
    let (client, observed) = Wire {
        pending_at: Some(2),
        ..Default::default()
    }
    .client();
    let mut future = Box::pin(deploy_program(&client, &artifacts, options()));
    tokio::select! {
        result = &mut future => panic!("deployment should be paused: {result:?}"),
        _ = observed.pending.notified() => {}
    }
    drop(future);
    tokio::task::yield_now().await;

    assert_eq!(observed.frames.lock().unwrap().len(), 2);
    assert_eq!(observed.drops.load(Ordering::Relaxed), 1);
    directory.finish().await;
}

#[tokio::test]
async fn write_failure_keeps_original_rpc_cause_without_retry_or_secret_diagnostics() {
    let directory = Directory::new().await;
    let (artifacts, _) = artifacts(&directory).await;
    let (client, observed) = Wire {
        fail_at: Some(2),
        ..Default::default()
    }
    .client();
    let failure = deploy_program(&client, &artifacts, options())
        .await
        .unwrap_err();

    assert_eq!(observed.frames.lock().unwrap().len(), 2);
    assert!(!format!("{failure:?} {failure}").contains("private-provider-sentinel"));
    assert!(failure.source().is_none());

    match failure {
        DeployProgramError::Deploy(DeployError::Deploy(
            solana_kit_surfpool::deploy::DeployError::Rpc(cause),
        )) => {
            assert_eq!(cause.rpc_code(), Some(-32000));
            assert_eq!(cause.message(), Some("private-provider-sentinel"));
        }
        other => panic!("expected retained RPC failure: {other}"),
    }

    directory.finish().await;
}

#[tokio::test]
async fn selected_program_replacement_and_key_removal_keep_typed_file_failures() {
    let directory = Directory::new().await;
    let (artifacts, _) = artifacts(&directory).await;
    fs::remove_file(artifacts.program_path()).await.unwrap();
    fs::create_dir(artifacts.program_path()).await.unwrap();
    let (client, observed) = Wire::default().client();

    assert!(matches!(
        deploy_program(&client, &artifacts, options()).await,
        Err(DeployProgramError::Deploy(DeployError::NotRegular {
            input: DeployFile::Program,
        }))
    ));

    fs::remove_file(artifacts.keypair_path()).await.unwrap();
    let failure = deploy_program(&client, &artifacts, options())
        .await
        .unwrap_err();
    assert!(matches!(
        &failure,
        DeployProgramError::KeyFile(DeployError::Io { input: DeployFile::Keypair, cause })
            if cause.kind() == std::io::ErrorKind::NotFound
    ));
    assert!(failure.source().is_none());
    assert!(observed.frames.lock().unwrap().is_empty());
    directory.finish().await;
}

fn key_json_with_first_lexeme(seed: u8, first: &str) -> (String, Address) {
    let key = ExportableKeypair::from_seed(&[seed; 32]);
    let backup = key.backup();
    assert_eq!(backup.expose_secret()[0], seed);
    let tail = backup.expose_secret()[1..]
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",");

    (format!("[{first},{tail}]"), key.signer().address())
}

#[tokio::test]
async fn full_length_key_arrays_preserve_negative_zero_and_reject_nonbyte_numbers() {
    let directory = Directory::new().await;
    let (artifacts, _) = artifacts(&directory).await;
    let (json, expected) = key_json_with_first_lexeme(0, "-0");
    fs::write(artifacts.keypair_path(), &json).await.unwrap();
    let (client, observed) = Wire::default().client();

    assert_eq!(
        deploy_program(&client, &artifacts, options())
            .await
            .unwrap(),
        expected
    );
    assert_eq!(observed.frames.lock().unwrap().len(), 3);

    let (client, observed) = Wire::default().client();

    for first in ["1.0", "1e0", "256"] {
        let (json, _) = key_json_with_first_lexeme(1, first);
        let elements: Vec<&serde_json::value::RawValue> = serde_json::from_str(&json).unwrap();
        assert_eq!(elements.len(), 64);
        assert_eq!(elements[0].get(), first);
        fs::write(artifacts.keypair_path(), &json).await.unwrap();
        let failure = deploy_program(&client, &artifacts, options())
            .await
            .unwrap_err();

        match failure {
            DeployProgramError::KeyJson(cause) => {
                assert!(cause.is_data());
                assert!(cause.to_string().contains("integer in 0..255"));
                assert!(!cause.to_string().contains("length"));
            }
            other => panic!("expected numeric JSON validation failure: {other}"),
        }

        assert!(observed.frames.lock().unwrap().is_empty());
    }

    directory.finish().await;
}
