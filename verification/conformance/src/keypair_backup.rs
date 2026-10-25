//! Portable explicit-backup callers, independent of any host random source.

use alloc::vec::Vec;
use solana_kit_core::crypto::{ExportableKeypair, Keypair};

const PUBLIC_TEST_SEED: [u8; 32] = [7; 32];

pub(crate) fn cases() -> Vec<crate::CaseResult> {
    let keypair = ExportableKeypair::from_seed(&PUBLIC_TEST_SEED);
    let backup = keypair.backup();
    let imported = Keypair::from_bytes(backup.expose_secret()).expect("validated explicit backup");
    let message = b"portable explicit backup caller";
    let matches = imported.address() == keypair.signer().address()
        && imported.sign(message) == keypair.signer().sign(message);
    let ordinary = keypair.into_signer();
    let consumed = ordinary.address() == imported.address();

    alloc::vec![
        crate::case(
            "explicit-backup-roundtrip",
            if matches {
                Ok(())
            } else {
                Err("backup signer changed")
            }
        ),
        crate::case(
            "explicit-backup-consume-export-capability",
            if consumed {
                Ok(())
            } else {
                Err("consumed signer changed")
            }
        ),
    ]
}
