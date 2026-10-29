#![cfg(all(feature = "surfpool", any(unix, windows)))]

mod process_fixture;
mod process_support;

use process_support::{Fixture, SHORT_WAIT, WAIT, calls, close_policy, wait_calls, wait_listening};
use solana_kit_native::surfpool::process::{
    CloseError, ClosePolicy, InvalidProcessOptions, OwnedSurfpool, ShutdownError, ShutdownRequest,
    SpawnError, StartError, StartupError,
};
use std::{
    error::Error, future::Future, net::TcpListener, process::Command, task::Poll, time::Duration,
};
use tokio::fs;

#[test]
fn fixture_entry() {
    if std::env::var_os("SOLANA_KIT_PROCESS_CHILD").is_none() {
        return;
    }

    std::process::exit(process_fixture::run());
}

#[tokio::test]
async fn explicit_child_reaches_sealed_rpc_readiness_and_closes_idempotently() {
    let fixture = Fixture::new("progress").await;
    let expected = fixture.options.ports;
    let (mut owner, directory) = fixture.spawn();
    fn assert_send<T: Send>(_: T) {}
    assert_send(owner.wait_rpc_ready(WAIT));
    owner.wait_rpc_ready(WAIT).await.unwrap();
    assert_send(owner.close(close_policy()));

    assert_eq!(owner.ports(), expected);
    assert_eq!(owner.runtime_directory(), directory);
    assert!(owner.rpc_endpoint().starts_with("http://127.0.0.1:"));
    assert!(owner.exit_status().is_none());

    let first = owner.close(close_policy()).await.unwrap();
    assert!(first.status.success());
    assert!(!first.forced);
    assert!(matches!(first.request, ShutdownRequest::Acknowledged));

    let before = calls(&directory).await;
    assert!(owner.close(close_policy()).await.unwrap().status.success());
    assert_eq!(calls(&directory).await, before);
    assert!(directory.exists());
    fs::remove_dir_all(directory).await.unwrap();
}

#[tokio::test]
async fn nonzero_and_zero_early_exit_are_cached_without_later_signals() {
    for (mode, success) in [("early-success", true), ("early-failure", false)] {
        let (mut owner, directory) = Fixture::new(mode).await.spawn();
        let failure = owner.wait_rpc_ready(WAIT).await.unwrap_err();

        assert!(matches!(failure, StartError::Exited(status) if status.success() == success));
        assert_eq!(owner.exit_status().unwrap().success(), success);
        assert_eq!(
            owner.close(close_policy()).await.unwrap().status.success(),
            success
        );
        assert_eq!(
            owner.close(close_policy()).await.unwrap().status.success(),
            success
        );
        fs::remove_dir_all(directory).await.unwrap();
    }
}

#[tokio::test]
async fn explicit_environment_and_null_output_flood_do_not_prevent_rpc_readiness() {
    for mode in ["environment", "flood"] {
        let (mut owner, directory) = Fixture::new(mode).await.spawn();
        owner.wait_rpc_ready(WAIT).await.unwrap();
        owner.close(close_policy()).await.unwrap();
        fs::remove_dir_all(directory).await.unwrap();
    }
}

#[tokio::test]
async fn stock_environment_overrides_cannot_escape_explicit_configuration_or_home() {
    for lowercase in [false, true] {
        let fixture = Fixture::new("environment")
            .await
            .with_stock_overrides(lowercase);
        let (mut owner, directory) = fixture.spawn();
        owner.wait_rpc_ready(WAIT).await.unwrap();
        owner.close(close_policy()).await.unwrap();
        fs::remove_dir_all(directory).await.unwrap();
    }
}

#[tokio::test]
async fn malformed_failed_unsealed_or_incomplete_startup_never_becomes_ready() {
    for mode in [
        "malformed",
        "failed",
        "unsealed",
        "incomplete",
        "unknown",
        "task-failed",
        "task-error",
    ] {
        let (mut owner, directory) = Fixture::new(mode).await.spawn();
        let failure = owner.wait_rpc_ready(WAIT).await.unwrap_err();
        assert!(matches!(
            failure,
            StartError::Startup(
                StartupError::Malformed | StartupError::Failed | StartupError::Inconsistent
            )
        ));
        assert!(!format!("{failure:?} {failure}").contains("private-startup-sentinel"));
        assert!(failure.source().is_none());
        assert!(owner.close(close_policy()).await.unwrap().forced);
        assert!(
            !calls(&directory)
                .await
                .lines()
                .any(|method| method == "exit")
        );
        fs::remove_dir_all(directory).await.unwrap();
    }
}

#[tokio::test]
async fn wrong_genesis_or_changed_shutdown_identity_never_receives_exit() {
    for mode in ["wrong-genesis", "switch-genesis"] {
        let (mut owner, directory) = Fixture::new(mode).await.spawn();

        if mode == "wrong-genesis" {
            assert!(matches!(
                owner.wait_rpc_ready(WAIT).await,
                Err(StartError::Identity)
            ));
        } else {
            owner.wait_rpc_ready(WAIT).await.unwrap();
        }

        let receipt = owner.close(close_policy()).await.unwrap();
        assert!(receipt.forced);

        if mode == "switch-genesis" {
            assert!(matches!(
                receipt.request,
                ShutdownRequest::Failed(ShutdownError::Identity)
            ));
        }

        assert!(
            !calls(&directory)
                .await
                .lines()
                .any(|method| method == "exit")
        );
        fs::remove_dir_all(directory).await.unwrap();
    }
}

#[tokio::test]
async fn a_stalled_request_is_bounded_by_the_whole_readiness_deadline() {
    let (mut owner, directory) = Fixture::new("stall").await.spawn();
    wait_listening(&directory).await;
    let result = tokio::time::timeout(WAIT, owner.wait_rpc_ready(SHORT_WAIT))
        .await
        .unwrap();
    assert!(matches!(result, Err(StartError::Deadline)));
    assert!(
        calls(&directory)
            .await
            .lines()
            .any(|method| method == "getGenesisHash")
    );
    assert!(owner.exit_status().is_none());
    assert!(owner.close(close_policy()).await.unwrap().forced);
    fs::remove_dir_all(directory).await.unwrap();
}

#[tokio::test]
async fn cancelling_readiness_retains_the_owner_for_explicit_close() {
    let (mut owner, directory) = Fixture::new("stall").await.spawn();

    {
        let ready = owner.wait_rpc_ready(WAIT);
        tokio::pin!(ready);
        tokio::select! {
            result = &mut ready => panic!("readiness should remain stalled: {result:?}"),
            _ = wait_calls(&directory, "getGenesisHash", 1) => {}
        }
    }

    assert!(owner.exit_status().is_none());
    assert!(owner.close(close_policy()).await.unwrap().forced);
    fs::remove_dir_all(directory).await.unwrap();
}

#[tokio::test]
async fn cancelling_close_during_identity_or_exit_retains_an_uncertain_request() {
    for (mode, method, count) in [
        ("stall-close", "getGenesisHash", 2),
        ("stall-exit", "exit", 1),
    ] {
        let (mut owner, directory) = Fixture::new(mode).await.spawn();
        owner.wait_rpc_ready(WAIT).await.unwrap();

        {
            let mut policy = close_policy();
            policy.request_timeout = WAIT;
            policy.graceful_timeout = WAIT;
            let close = owner.close(policy);
            tokio::pin!(close);
            tokio::select! {
                result = &mut close => panic!("close should still be pending: {result:?}"),
                _ = wait_calls(&directory, method, count) => {}
            }
        }

        assert!(owner.exit_status().is_none());
        assert!(matches!(
            owner.wait_rpc_ready(WAIT).await,
            Err(StartError::Closing)
        ));
        let receipt = owner.close(close_policy()).await.unwrap();
        assert!(receipt.forced);
        assert!(matches!(receipt.request, ShutdownRequest::Unconfirmed));
        assert_eq!(
            calls(&directory)
                .await
                .lines()
                .filter(|line| *line == method)
                .count(),
            count
        );
        fs::remove_dir_all(directory).await.unwrap();
    }
}

#[tokio::test]
async fn acknowledged_but_ignored_exit_is_force_terminated_then_reaped() {
    let (mut owner, directory) = Fixture::new("ignore-exit").await.spawn();
    owner.wait_rpc_ready(WAIT).await.unwrap();
    let receipt = owner.close(close_policy()).await.unwrap();
    assert!(receipt.forced);
    assert!(matches!(receipt.request, ShutdownRequest::Acknowledged));
    assert!(owner.exit_status().is_some());
    assert!(owner.close(close_policy()).await.unwrap().forced);
    fs::remove_dir_all(directory).await.unwrap();
}

#[tokio::test]
async fn cancelling_the_graceful_wait_retains_an_unverified_child_for_explicit_close() {
    let (mut owner, directory) = Fixture::new("ready").await.spawn();
    wait_listening(&directory).await;

    {
        let policy = ClosePolicy {
            graceful_timeout: WAIT,
            ..close_policy()
        };
        let close = owner.close(policy);
        tokio::pin!(close);
        std::future::poll_fn(|context| {
            assert!(close.as_mut().poll(context).is_pending());

            Poll::Ready(())
        })
        .await;
    }

    assert!(owner.exit_status().is_none());
    let receipt = owner.close(close_policy()).await.unwrap();
    assert!(receipt.forced);
    assert!(matches!(receipt.request, ShutdownRequest::NotRequested));
    assert!(calls(&directory).await.is_empty());
    fs::remove_dir_all(directory).await.unwrap();
}

#[tokio::test]
async fn fixed_rpc_collision_cannot_be_accepted_and_studio_collision_is_not_a_claim() {
    for websocket in [false, true] {
        let fixture = Fixture::new("ready").await;
        let ports = fixture.options.ports;
        let port = if websocket {
            ports.websocket
        } else {
            ports.rpc
        };

        let blocker = TcpListener::bind(("127.0.0.1", port.get())).unwrap();
        let (mut owner, directory) = fixture.spawn();
        let result = owner.wait_rpc_ready(SHORT_WAIT).await;
        assert!(matches!(
            result,
            Err(StartError::Exited(_) | StartError::Deadline)
        ));
        owner.close(close_policy()).await.unwrap();
        drop(blocker);
        fs::remove_dir_all(directory).await.unwrap();
    }

    let fixture = Fixture::new("ready").await;
    let blocker = TcpListener::bind(("127.0.0.1", fixture.options.ports.studio.get())).unwrap();
    let (mut owner, directory) = fixture.spawn();
    owner.wait_rpc_ready(WAIT).await.unwrap();
    owner.close(close_policy()).await.unwrap();
    drop(blocker);
    fs::remove_dir_all(directory).await.unwrap();
}

#[tokio::test]
async fn invalid_preparation_never_creates_a_child_and_preserves_original_io() {
    let fixture = Fixture::new("ready").await;
    let directory = fixture.directory.clone();
    let failure = OwnedSurfpool::spawn(Command::new("relative-private-sentinel"), fixture.options)
        .unwrap_err();
    assert!(matches!(
        failure,
        SpawnError::Invalid(InvalidProcessOptions::Executable)
    ));
    assert!(!format!("{failure:?} {failure}").contains("private-sentinel"));
    assert!(!directory.join("fixture-calls").exists());
    fs::remove_dir_all(directory).await.unwrap();

    let fixture = Fixture::new("ready").await;
    let missing = fixture.directory.join("missing-private-sentinel");
    let directory = fixture.directory.clone();
    let failure = OwnedSurfpool::spawn(Command::new(missing), fixture.options).unwrap_err();
    assert!(matches!(
        &failure,
        SpawnError::Io { cause, .. } if cause.kind() == std::io::ErrorKind::NotFound
    ));
    assert!(failure.source().is_none());
    fs::remove_dir_all(directory).await.unwrap();
}

#[tokio::test]
async fn invalid_close_budget_retains_the_child_and_runtime_for_a_valid_attempt() {
    let (mut owner, directory) = Fixture::new("ready").await.spawn();
    owner.wait_rpc_ready(WAIT).await.unwrap();
    let mut invalid = close_policy();
    invalid.reap_timeout = Duration::ZERO;
    assert!(matches!(
        owner.close(invalid).await,
        Err(CloseError::InvalidPolicy)
    ));
    assert!(directory.exists());
    assert!(owner.exit_status().is_none());
    owner.close(close_policy()).await.unwrap();

    fs::remove_dir_all(directory).await.unwrap();
}

#[tokio::test]
async fn independent_owners_keep_distinct_ports_directories_and_child_lifetimes() {
    let (mut first, first_directory) = Fixture::new("ready").await.spawn();
    let (mut second, second_directory) = Fixture::new("ready").await.spawn();
    assert_ne!(first.ports(), second.ports());
    assert_ne!(first_directory, second_directory);
    let (first_ready, second_ready) =
        tokio::join!(first.wait_rpc_ready(WAIT), second.wait_rpc_ready(WAIT));
    first_ready.unwrap();
    second_ready.unwrap();

    first.close(close_policy()).await.unwrap();
    assert!(second.exit_status().is_none());
    second.wait_rpc_ready(WAIT).await.unwrap();
    second.close(close_policy()).await.unwrap();
    fs::remove_dir_all(first_directory).await.unwrap();
    fs::remove_dir_all(second_directory).await.unwrap();
}

#[tokio::test]
async fn typed_rpc_failures_are_terminal_without_provider_text_retry_or_diagnostics() {
    let (mut owner, directory) = Fixture::new("rpc-error").await.spawn();
    let failure = owner.wait_rpc_ready(WAIT).await.unwrap_err();
    assert!(!format!("{failure:?} {failure}").contains("private-rpc-sentinel"));
    assert!(failure.source().is_none());

    match failure {
        StartError::Rpc(solana_kit_client::Error::Remote(cause)) => {
            assert_eq!(cause.message, "private-rpc-sentinel");
        }
        other => panic!("expected retained remote RPC cause: {other}"),
    }

    assert_eq!(calls(&directory).await.lines().count(), 1);
    assert!(owner.close(close_policy()).await.unwrap().forced);
    fs::remove_dir_all(directory).await.unwrap();
}

#[tokio::test]
async fn shutdown_rpc_failure_survives_confirmed_reaping_without_implicit_diagnostics() {
    let (mut owner, directory) = Fixture::new("exit-error").await.spawn();
    owner.wait_rpc_ready(WAIT).await.unwrap();
    let receipt = owner.close(close_policy()).await.unwrap();
    assert!(receipt.forced);
    assert!(!format!("{receipt:?}").contains("private-rpc-sentinel"));

    match &receipt.request {
        ShutdownRequest::Failed(ShutdownError::Rpc(solana_kit_client::Error::Remote(cause))) => {
            assert_eq!(cause.message, "private-rpc-sentinel");
        }
        other => panic!("expected retained shutdown RPC cause: {other:?}"),
    }

    let before = calls(&directory).await;
    assert!(owner.close(close_policy()).await.unwrap().forced);
    assert_eq!(before, calls(&directory).await);
    fs::remove_dir_all(directory).await.unwrap();
}

#[tokio::test]
async fn unsupported_network_fixed_port_and_actual_spawn_failures_precede_readiness() {
    for setting in ["offline", "host", "ports"] {
        let mut fixture = Fixture::new("ready").await;

        match setting {
            "offline" => fixture.options.config.offline = false,
            "host" => fixture.options.config.host = "external.invalid".into(),
            _ => fixture.options.ports.websocket = fixture.options.ports.rpc,
        }

        let directory = fixture.directory.clone();
        let failure = OwnedSurfpool::spawn(Fixture::command(), fixture.options).unwrap_err();
        assert!(matches!(
            failure,
            SpawnError::Invalid(_) | SpawnError::Config(_)
        ));
        assert!(!directory.join("fixture-calls").exists());
        fs::remove_dir_all(directory).await.unwrap();
    }

    let fixture = Fixture::new("ready").await;
    let directory = fixture.directory.clone();
    let executable = directory.join("not-an-executable");
    fs::write(&executable, b"plain fixture data").await.unwrap();
    let failure = OwnedSurfpool::spawn(Command::new(executable), fixture.options).unwrap_err();
    assert!(matches!(
        &failure,
        SpawnError::Io { stage: solana_kit_native::surfpool::process::SpawnStage::Spawn, cause }
            if cause.raw_os_error().is_some()
    ));
    assert!(failure.source().is_none());
    fs::remove_dir_all(directory).await.unwrap();
}
