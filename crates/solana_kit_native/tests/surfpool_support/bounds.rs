//! Bounds are exercised before program RPC and after writes for deferred IDLs.

use super::*;

#[tokio::test]
async fn program_limits_apply_to_both_sources_and_allow_explicit_overrides() {
    let directory = Directory::new().await;
    let path = directory.write("program.so", &PROGRAM).await;

    for source in [ProgramSource::Bytes(&PROGRAM), ProgramSource::Path(&path)] {
        let (client, observed) = Mock::default().client();
        let mut options = options(source);
        options.max_program_bytes = PROGRAM.len() - 1;

        assert!(matches!(
            deploy(&client, options).await,
            Err(DeployError::TooLarge {
                input: DeployFile::Program,
                limit: 4
            })
        ));
        assert!(observed.requests.borrow().is_empty());
        options.max_program_bytes = PROGRAM.len();
        assert_eq!(deploy(&client, options).await.unwrap(), ADDRESS);
        assert_program_writes(&observed);
    }

    directory.finish().await;
}

#[tokio::test]
async fn oversized_sparse_program_and_nonregular_paths_fail_before_rpc() {
    let directory = Directory::new().await;
    let path = directory.0.join("oversized.so");
    let file = fs::File::create(&path).await.unwrap();
    file.set_len(MAX_PROGRAM_BYTES as u64 + 1).await.unwrap();
    drop(file);
    let (client, observed) = Mock::default().client();
    let options = DeployOptions::new(ADDRESS, ProgramSource::Path(&path));

    assert!(matches!(
        deploy(&client, options).await,
        Err(DeployError::TooLarge {
            input: DeployFile::Program,
            limit: MAX_PROGRAM_BYTES
        })
    ));
    let options = DeployOptions::new(ADDRESS, ProgramSource::Path(&directory.0));
    assert!(matches!(
        deploy(&client, options).await,
        Err(DeployError::NotRegular {
            input: DeployFile::Program
        })
    ));
    assert!(observed.requests.borrow().is_empty());
    directory.finish().await;
}

#[tokio::test]
async fn idl_limit_and_file_type_fail_only_after_program_writes() {
    let directory = Directory::new().await;
    let path = directory.write("idl.json", br#"{"unknown":true}"#).await;
    let (client, observed) = Mock::default().client();
    let mut options = options(ProgramSource::Bytes(&PROGRAM));
    options.idl_path = Some(&path);
    options.max_idl_bytes = 2;

    assert!(matches!(
        deploy(&client, options).await,
        Err(DeployError::TooLarge {
            input: DeployFile::Idl,
            limit: 2
        })
    ));
    assert_program_writes(&observed);
    assert_eq!(observed.requests.borrow().len(), 3);

    let (client, observed) = Mock::default().client();
    options.max_idl_bytes = fs::metadata(&path).await.unwrap().len().try_into().unwrap();
    assert_eq!(deploy(&client, options).await.unwrap(), ADDRESS);
    assert_program_writes(&observed);
    assert_eq!(observed.requests.borrow().len(), 4);

    let (client, observed) = Mock::default().client();
    options.idl_path = Some(&directory.0);
    assert!(matches!(
        deploy(&client, options).await,
        Err(DeployError::NotRegular {
            input: DeployFile::Idl
        })
    ));
    assert_program_writes(&observed);
    assert_eq!(observed.requests.borrow().len(), 3);
    directory.finish().await;
}

#[tokio::test]
async fn empty_program_needs_no_writes_and_zero_limit_is_finite() {
    let (client, observed) = Mock::default().client();
    let mut options = DeployOptions::new(ADDRESS, ProgramSource::Bytes(&[]));
    options.max_program_bytes = 0;
    options.max_idl_bytes = 0;

    assert_eq!(deploy(&client, options).await.unwrap(), ADDRESS);
    assert!(observed.requests.borrow().is_empty());
    options.program = ProgramSource::Bytes(&[1]);
    assert!(matches!(
        deploy(&client, options).await,
        Err(DeployError::TooLarge {
            input: DeployFile::Program,
            limit: 0
        })
    ));
    assert!(observed.requests.borrow().is_empty());
}

#[tokio::test]
async fn missing_program_retains_typed_io_without_starting_rpc() {
    let directory = Directory::new().await;
    let path = directory.0.join(PRIVATE);
    let (client, observed) = Mock::default().client();
    let options = options(ProgramSource::Path(&path));
    let failure = deploy(&client, options).await.unwrap_err();

    assert!(matches!(
        &failure,
        DeployError::Io { input: DeployFile::Program, cause }
            if cause.kind() == io::ErrorKind::NotFound
    ));
    assert!(failure.source().is_none());
    assert!(!format!("{failure:?} {failure} {options:?}").contains(PRIVATE));
    assert!(observed.requests.borrow().is_empty());
    directory.finish().await;
}
