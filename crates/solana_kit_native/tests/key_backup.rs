#![cfg(all(feature = "keys", unix))]

use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
    path::PathBuf,
    sync::{
        Arc, Barrier,
        atomic::{AtomicU64, Ordering},
    },
};

use solana_kit_core::crypto::{Entropy, ExportableKeypair, Keypair};
use solana_kit_native::keys::{NativeEntropy, write_keypair_new};
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
    assert!(!failure.published());
    assert!(failure.cleanup_errors().is_empty());
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
    assert!(!failure.published());
    assert!(failure.cleanup_errors().is_empty());
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
