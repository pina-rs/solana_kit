#![cfg(feature = "crypto")]

use core::{
    future::{Future, Ready, ready},
    task::{Context, Poll, Waker},
};

use solana_kit_core::crypto::{Entropy, ExportableKeypair, Keypair, KeypairImportError};

const TEST_SEED: [u8; 32] = [7; 32];
const LAST_PUBLIC_KEY_BYTE: usize = 63;

struct FixedEntropy(bool);

impl Entropy for FixedEntropy {
    type Error = &'static str;
    type Fill<'a> = Ready<Result<(), Self::Error>>;

    fn fill<'a>(&'a mut self, output: &'a mut [u8]) -> Self::Fill<'a> {
        output.copy_from_slice(&TEST_SEED);

        ready(if self.0 {
            Ok(())
        } else {
            Err("host entropy denied")
        })
    }
}

fn complete<T>(future: impl Future<Output = T>) -> T {
    let mut future = Box::pin(future);
    let mut context = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("the test host must complete immediately"),
    }
}

#[test]
fn backup_roundtrips_into_the_existing_non_exporting_signer() {
    let original = ExportableKeypair::from_seed(&TEST_SEED);
    let backup = original.backup();
    let imported = Keypair::from_bytes(backup.expose_secret()).expect("valid explicit backup");
    let message = b"the exact bytes intentionally authorized";

    assert_eq!(imported.address(), original.signer().address());
    assert_eq!(imported.sign(message), original.signer().sign(message));
    assert_eq!(format!("{backup:?}"), "KeypairBackup(REDACTED)");
    assert_eq!(format!("{original:?}"), "ExportableKeypair(REDACTED)");

    let ordinary = original.into_signer();
    assert_eq!(ordinary.address(), imported.address());
}

#[test]
fn generated_exportable_keys_use_the_supplied_host_and_preserve_its_failure() {
    let key = complete(ExportableKeypair::generate(&mut FixedEntropy(true)))
        .expect("supplied entropy succeeds");
    let expected = Keypair::from_seed(&TEST_SEED);

    assert_eq!(key.signer().address(), expected.address());
    assert_eq!(
        complete(ExportableKeypair::generate(&mut FixedEntropy(false))).unwrap_err(),
        "host entropy denied"
    );
}

#[test]
fn explicit_import_retains_length_and_public_half_validation() {
    let key = ExportableKeypair::from_seed(&TEST_SEED);
    let backup = key.backup();
    let mut invalid = *backup.expose_secret();
    invalid[LAST_PUBLIC_KEY_BYTE] ^= 1;

    assert_eq!(
        ExportableKeypair::from_bytes(&invalid).unwrap_err(),
        KeypairImportError::PublicKeyMismatch
    );
    assert_eq!(
        ExportableKeypair::from_bytes(&invalid[..LAST_PUBLIC_KEY_BYTE]).unwrap_err(),
        KeypairImportError::InvalidLength {
            actual: LAST_PUBLIC_KEY_BYTE
        }
    );
}
