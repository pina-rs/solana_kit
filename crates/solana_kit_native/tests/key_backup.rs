#![cfg(all(feature = "keys", unix))]

use std::{
    error::Error,
    fs, io,
    os::unix::fs::{PermissionsExt, symlink},
    path::PathBuf,
    sync::{
        Arc, Barrier,
        atomic::{AtomicU64, Ordering},
    },
};

use solana_kit_core::crypto::{Entropy, ExportableKeypair, Keypair};
use solana_kit_native::keys::{KeyFileError, KeyFileStage, NativeEntropy, write_keypair_new};
use tokio_rustls::rustls::crypto::{GetRandomFailed, SecureRandom, ring};

const TEST_SEED: [u8; 32] = [7; 32];
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

fn directory() -> PathBuf {
    let sequence = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
    let directory = std::env::temp_dir().join(format!(
        "solana-kit-public-key-backup-test-{}-{sequence}",
        std::process::id()
    ));
    fs::create_dir(&directory).expect("fresh test directory");

    directory
}

fn keypair() -> ExportableKeypair {
    ExportableKeypair::from_seed(&TEST_SEED)
}

fn assert_unpublished_collision(failure: &KeyFileError) {
    let source = failure
        .source()
        .expect("original collision cause")
        .downcast_ref::<io::Error>()
        .expect("original I/O error");

    assert_eq!(failure.stage(), Some(KeyFileStage::PublishDestination));
    assert_eq!(source.kind(), io::ErrorKind::AlreadyExists);
    assert!(source.raw_os_error().is_some());
    assert!(!failure.published());
    assert!(failure.cleanup_errors().is_empty());
}

#[test]
fn caller_io_conversion_retains_its_cause_without_claiming_a_backup_stage() {
    let original = io::Error::from_raw_os_error(13);
    let original_kind = original.kind();
    let original_code = original.raw_os_error();
    let failure: KeyFileError = original.into();
    let source = failure
        .source()
        .expect("original I/O cause")
        .downcast_ref::<io::Error>()
        .expect("retained I/O error type");

    assert_eq!(failure.stage(), None);
    assert_eq!(source.kind(), original_kind);
    assert_eq!(source.raw_os_error(), original_code);
    assert!(original_code.is_some());
    assert!(!failure.published());
    assert!(failure.cleanup_errors().is_empty());
}

#[test]
fn backup_is_complete_private_json_and_never_overwrites() {
    let directory = directory();
    let destination = directory.join("keypair.json");
    let keypair = keypair();
    write_keypair_new(&destination, &keypair).expect("fresh backup");
    let original = fs::read(&destination).expect("published complete backup");
    let bytes: Vec<u8> = serde_json::from_slice(&original).expect("Solana-keygen JSON array");
    let imported = Keypair::from_bytes(&bytes).expect("matching seed and public key");

    assert_eq!(imported.address(), keypair.signer().address());
    assert_eq!(
        fs::metadata(&destination)
            .expect("permissions")
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    assert_eq!(
        fs::read_dir(&directory).expect("directory entries").count(),
        1
    );

    let failure =
        write_keypair_new(&destination, &keypair).expect_err("existing backup must survive");
    assert_unpublished_collision(&failure);
    assert_eq!(
        fs::read(&destination).expect("unchanged original"),
        original
    );

    fs::remove_file(destination).expect("remove owned fixture");
    fs::remove_dir(directory).expect("no staging artifacts remain");
}

#[test]
fn symlink_destination_preserves_its_target_and_is_not_followed() {
    let directory = directory();
    let target = directory.join("existing.json");
    let destination = directory.join("keypair.json");
    let sentinel = b"existing public test bytes";
    fs::write(&target, sentinel).expect("owned target");
    symlink(&target, &destination).expect("destination link");

    let failure = write_keypair_new(&destination, &keypair()).expect_err("no link overwrite");
    assert_unpublished_collision(&failure);
    assert_eq!(fs::read(&target).expect("target remains intact"), sentinel);
    assert!(
        fs::symlink_metadata(&destination)
            .expect("link survives")
            .is_symlink()
    );

    fs::remove_file(destination).expect("remove owned link");
    fs::remove_file(target).expect("remove owned target");
    fs::remove_dir(directory).expect("no staging artifacts remain");
}

#[test]
fn dangling_destination_link_is_not_replaced_or_followed() {
    let directory = directory();
    let target = directory.join("missing.json");
    let destination = directory.join("keypair.json");
    symlink(&target, &destination).expect("owned dangling link");

    let failure = write_keypair_new(&destination, &keypair()).expect_err("entry already exists");
    assert_unpublished_collision(&failure);
    assert_eq!(fs::read_link(&destination).unwrap(), target);
    assert_eq!(
        fs::symlink_metadata(&target).unwrap_err().kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 1);

    fs::remove_file(destination).expect("remove unchanged link");
    fs::remove_dir(directory).expect("no staging artifacts remain");
}

#[test]
fn destination_directory_remains_unchanged_after_publication_fails() {
    let directory = directory();
    let destination = directory.join("keypair.json");
    let marker = destination.join("existing");
    let sentinel = b"existing directory entry";
    fs::create_dir(&destination).expect("owned destination directory");
    fs::write(&marker, sentinel).expect("owned directory content");

    let failure = write_keypair_new(&destination, &keypair()).expect_err("entry already exists");
    assert_unpublished_collision(&failure);
    assert_eq!(fs::read(&marker).unwrap(), sentinel);
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 1);
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 1);

    fs::remove_file(marker).expect("remove unchanged directory content");
    fs::remove_dir(destination).expect("remove unchanged destination directory");
    fs::remove_dir(directory).expect("no staging artifacts remain");
}

#[test]
fn existing_parent_file_retains_its_cause_and_does_not_publish() {
    let directory = directory();
    let parent = directory.join("parent");
    let sentinel = b"existing parent is a file";
    fs::write(&parent, sentinel).expect("owned parent fixture");

    let failure = write_keypair_new(&parent.join("keypair.json"), &keypair())
        .expect_err("cannot create a staging directory inside a file");
    let source = failure
        .source()
        .unwrap()
        .downcast_ref::<io::Error>()
        .unwrap();

    assert_eq!(failure.stage(), Some(KeyFileStage::CreateParentDirectory));
    assert!(source.raw_os_error().is_some());
    assert!(!failure.published());
    assert!(failure.cleanup_errors().is_empty());
    assert_eq!(fs::read(&parent).unwrap(), sentinel);
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 1);

    fs::remove_file(parent).expect("remove unchanged fixture");
    fs::remove_dir(directory).expect("no staging artifacts remain");
}

#[test]
fn competing_writers_publish_exactly_one_complete_key() {
    let directory = directory();
    let destination = directory.join("keypair.json");
    let start = Arc::new(Barrier::new(2));
    let handles = [7, 8].map(|seed| {
        let destination = destination.clone();
        let start = start.clone();

        std::thread::spawn(move || {
            let keypair = ExportableKeypair::from_seed(&[seed; 32]);
            start.wait();

            write_keypair_new(&destination, &keypair)
        })
    });
    let results = handles.map(|handle| handle.join().expect("writer finishes"));
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);

    for failure in results.iter().filter_map(|result| result.as_ref().err()) {
        assert_unpublished_collision(failure);
    }

    let bytes: Vec<u8> = serde_json::from_slice(&fs::read(&destination).expect("complete winner"))
        .expect("valid complete JSON");
    let imported = Keypair::from_bytes(&bytes).expect("one valid winner");
    assert!(
        [7, 8]
            .iter()
            .any(|seed| { Keypair::from_seed(&[*seed; 32]).address() == imported.address() })
    );
    assert_eq!(
        fs::read_dir(&directory).expect("directory entries").count(),
        1
    );

    fs::remove_file(destination).expect("remove winner");
    fs::remove_dir(directory).expect("no staging artifacts remain");
}

#[derive(Debug)]
struct UnavailableRandom;

impl SecureRandom for UnavailableRandom {
    fn fill(&self, _: &mut [u8]) -> Result<(), GetRandomFailed> {
        Err(GetRandomFailed)
    }
}

#[tokio::test]
async fn provider_failure_remains_a_typed_original_cause() {
    let mut provider = ring::default_provider();
    provider.secure_random = &UnavailableRandom;
    let mut entropy = NativeEntropy::new(Arc::new(provider));
    let error = entropy.fill(&mut [0; 32]).await.expect_err("host failure");
    let _: &GetRandomFailed = error.provider_cause();

    assert_eq!(error.to_string(), "native secure randomness is unavailable");
}
