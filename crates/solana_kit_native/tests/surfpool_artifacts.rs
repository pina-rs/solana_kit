#![cfg(feature = "surfpool")]

mod artifact_support;
use artifact_support::{Directory, PROGRAM};
#[cfg(unix)]
use solana_kit_native::surfpool::ArtifactRole;
use solana_kit_native::surfpool::{DiscoveryError, discover_program_artifacts};
#[cfg(unix)]
use std::error::Error;
use std::{
    future::Future,
    path::{Path, PathBuf},
    pin::pin,
    task::{Context, Poll, Waker},
};
use tokio::fs;

#[tokio::test]
async fn caller_order_selects_same_root_pair_and_only_its_idl() {
    let directory = Directory::new().await;
    let partial = directory.root("partial").await;
    let second = directory.root("second").await;
    let first = directory.root("first").await;
    fs::write(partial.join("deploy/demo.so"), PROGRAM)
        .await
        .unwrap();
    fs::write(partial.join("idl/demo.json"), b"orphan")
        .await
        .unwrap();
    directory.pair(&first, b"first key").await;

    directory.pair(&second, b"second key").await;
    fs::write(second.join("idl/demo.json"), b"second IDL")
        .await
        .unwrap();
    let roots = [partial, first.clone(), second.clone()];
    let selected = discover_program_artifacts("demo", &roots).await.unwrap();

    assert_eq!(selected.candidate_index(), 1);
    assert_eq!(selected.program_path(), first.join("deploy/demo.so"));
    assert_eq!(
        selected.keypair_path(),
        first.join("deploy/demo-keypair.json")
    );
    assert_eq!(selected.idl_path(), None);

    let reversed = discover_program_artifacts("demo", &[second.clone(), first])
        .await
        .unwrap();
    assert_eq!(reversed.candidate_index(), 0);
    assert_eq!(
        reversed.idl_path(),
        Some(second.join("idl/demo.json").as_path())
    );
    assert!(!format!("{selected:?} {reversed:?}").contains(directory.0.to_str().unwrap()));
    directory.finish().await;
}

#[tokio::test]
async fn partial_pairs_never_combine_and_nonregular_candidates_count_absent() {
    let directory = Directory::new().await;
    let program_only = directory.root("program-only").await;
    let key_only = directory.root("key-only").await;
    fs::write(program_only.join("deploy/demo.so"), PROGRAM)
        .await
        .unwrap();
    fs::write(key_only.join("deploy/demo-keypair.json"), b"key")
        .await
        .unwrap();
    fs::create_dir(key_only.join("deploy/demo.so"))
        .await
        .unwrap();

    assert!(matches!(
        discover_program_artifacts("demo", &[program_only, key_only]).await,
        Err(DiscoveryError::NotFound)
    ));
    directory.finish().await;
}

#[test]
fn names_and_all_roots_validate_without_a_tokio_runtime_or_any_filesystem_work() {
    let absolute = if cfg!(windows) {
        PathBuf::from(r"C:\never-probed")
    } else {
        PathBuf::from("/never-probed")
    };

    for name in [
        "", ".", "..", "a/b", r"a\b", "C:demo", "a\0b", "a\nb", "a\u{85}b",
    ] {
        let mut future = pin!(discover_program_artifacts(
            name,
            std::slice::from_ref(&absolute)
        ));
        assert!(matches!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(Err(DiscoveryError::InvalidName))
        ));
    }

    for invalid in [PathBuf::new(), PathBuf::from("relative")] {
        let roots = [absolute.clone(), invalid];
        let mut future = pin!(discover_program_artifacts("demo", &roots));
        assert!(matches!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(Err(DiscoveryError::InvalidRoot { index: 1 }))
        ));
    }

    let mut future = pin!(discover_program_artifacts("demo", &[]));
    assert!(matches!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Ready(Err(DiscoveryError::NoRoots))
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn nonmissing_metadata_failure_retains_its_index_role_and_typed_cause() {
    let directory = Directory::new().await;
    let root = directory.0.join("private-path-sentinel");
    fs::write(&root, b"a file cannot contain deploy")
        .await
        .unwrap();
    let failure = discover_program_artifacts("demo", &[root])
        .await
        .unwrap_err();

    assert!(
        matches!(&failure, DiscoveryError::Io { index: 0, role: ArtifactRole::Program, cause }
        if cause.kind() == std::io::ErrorKind::NotADirectory)
    );
    assert!(!format!("{failure:?} {failure}").contains("private-path-sentinel"));
    assert!(failure.source().is_none());
    directory.finish().await;
}

#[tokio::test]
async fn absent_idl_is_a_presence_snapshot_and_filename_dots_are_literal() {
    let directory = Directory::new().await;
    let root = directory.root("target").await;
    directory.pair(&root, b"not read during discovery").await;
    let selected = discover_program_artifacts("demo", std::slice::from_ref(&root))
        .await
        .unwrap();
    fs::write(root.join("idl/demo.json"), b"created later")
        .await
        .unwrap();

    assert!(selected.idl_path().is_none());
    assert_eq!(
        selected.program_path().file_name(),
        Some(Path::new("demo.so").as_os_str())
    );
    assert!(matches!(
        discover_program_artifacts("demo.other", &[root]).await,
        Err(DiscoveryError::NotFound)
    ));
    directory.finish().await;
}

/// Both selected metadata roles must fail before considering a valid later root.
#[cfg(unix)]
#[tokio::test]
async fn symlink_loops_preserve_keypair_and_idl_metadata_roles_without_fallback() {
    for (role, relative) in [
        (ArtifactRole::Keypair, "deploy/demo-keypair.json"),
        (ArtifactRole::Idl, "idl/demo.json"),
    ] {
        let directory = Directory::new().await;
        let first = directory.root("first").await;
        let later = directory.root("later").await;
        directory.pair(&first, b"first key").await;
        directory.pair(&later, b"later key").await;
        let path = first.join(relative);

        if role == ArtifactRole::Keypair {
            fs::remove_file(&path).await.unwrap();
        }

        fs::symlink(Path::new(path.file_name().unwrap()), &path)
            .await
            .unwrap();
        let failure = discover_program_artifacts("demo", &[first, later])
            .await
            .unwrap_err();

        match &failure {
            DiscoveryError::Io {
                index,
                role: actual,
                cause,
            } => {
                assert_eq!(*index, 0);
                assert_eq!(*actual, role);
                assert_ne!(cause.kind(), std::io::ErrorKind::NotFound);
                assert!(cause.raw_os_error().is_some());
            }
            other => panic!("expected original loop metadata failure: {other}"),
        }

        assert!(failure.source().is_none());
        assert!(!format!("{failure:?} {failure}").contains(directory.0.to_str().unwrap()));
        directory.finish().await;
    }
}
