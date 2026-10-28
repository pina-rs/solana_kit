//! Real read-only file failures preserve their primitive and original I/O cause.

use super::{KeyFileStage, write_staged};
use solana_kit_core::crypto::ExportableKeypair;
use std::{error::Error, fs::File, io::Write};

#[test]
fn readonly_file_failure_identifies_writing_before_sync_or_publication() {
    let path = std::env::current_exe().expect("running test executable");
    let mut file = File::open(path).expect("open a real file without write access");
    let before = file.metadata().unwrap().len();
    let expected = file
        .write_all(b"public control")
        .expect_err("read-only file");
    let keypair = ExportableKeypair::from_seed(&[7; 32]);

    let failure = write_staged(&mut file, &keypair).expect_err("write must fail before sync");
    let source = failure
        .source()
        .unwrap()
        .downcast_ref::<std::io::Error>()
        .unwrap();

    assert_eq!(failure.stage(), Some(KeyFileStage::WriteStagingFile));
    assert_eq!(source.kind(), expected.kind());
    assert_eq!(source.raw_os_error(), expected.raw_os_error());
    assert!(source.raw_os_error().is_some());
    assert!(!failure.published());
    assert!(failure.cleanup_errors().is_empty());
    assert_eq!(file.metadata().unwrap().len(), before);
}
