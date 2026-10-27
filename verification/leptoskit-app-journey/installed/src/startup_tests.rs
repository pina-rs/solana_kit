//! Execute the actual runtime writer and controller reader against owned files.
use super::{retain, runtime_startup::Startup};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct Sandbox {
    root: PathBuf,
    data: PathBuf,
    output: PathBuf,
}

impl Sandbox {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("solana-startup-{nonce}-{sequence}"));
        let data = root.join("data");
        let output = root.join("output");
        fs::create_dir(&root).unwrap();
        fs::create_dir(&data).unwrap();
        fs::create_dir(&output).unwrap();
        Self { root, data, output }
    }

    fn receipt(&self) -> PathBuf {
        self.output.join("journey-startup.txt")
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

#[test]
fn observed_startup_survives_application_data_cleanup() {
    let sandbox = Sandbox::new();
    let mut startup = Startup::create(&sandbox.data).unwrap();
    startup.record("configuration-ready").unwrap();
    startup.record("service-activation-entered").unwrap();
    startup.observe(false, false).unwrap();
    startup.observe(true, true).unwrap();
    drop(startup);
    retain(&sandbox.data, &sandbox.output).unwrap();
    fs::remove_dir_all(&sandbox.data).unwrap();
    assert_eq!(
        fs::read_to_string(sandbox.receipt()).unwrap(),
        "factory-entered\nconfiguration-ready\nservice-activation-entered\nservice-polled\nroot-mounted\nrender-pending\n"
    );
}

#[test]
fn repeated_service_polls_do_not_grow_the_receipt() {
    let sandbox = Sandbox::new();
    let mut startup = Startup::create(&sandbox.data).unwrap();

    for _ in 0..10_000 {
        startup.observe(true, true).unwrap();
    }

    retain(&sandbox.data, &sandbox.output).unwrap();
    assert_eq!(
        fs::read_to_string(sandbox.receipt()).unwrap(),
        "factory-entered\nservice-polled\nroot-mounted\nrender-pending\n"
    );
}

#[test]
fn a_new_factory_call_discards_old_startup_observations() {
    let sandbox = Sandbox::new();
    let mut startup = Startup::create(&sandbox.data).unwrap();
    startup.observe(true, true).unwrap();
    drop(startup);
    drop(Startup::create(&sandbox.data).unwrap());
    retain(&sandbox.data, &sandbox.output).unwrap();
    assert_eq!(fs::read(sandbox.receipt()).unwrap(), b"factory-entered\n");
}

#[test]
fn a_missing_receipt_is_reported_without_inventing_a_startup_phase() {
    let sandbox = Sandbox::new();
    retain(&sandbox.data, &sandbox.output).unwrap();
    assert_eq!(
        fs::read(sandbox.receipt()).unwrap(),
        b"receipt-missing-in-controller-data\n"
    );
}

#[test]
fn unknown_empty_malformed_and_oversized_data_cannot_be_retained() {
    let sandbox = Sandbox::new();

    for bytes in [
        b"unexpected\n".to_vec(),
        Vec::new(),
        vec![255],
        b"factory-entered\n".repeat(100),
    ] {
        fs::write(sandbox.data.join("journey-startup.txt"), bytes).unwrap();
        assert!(retain(&sandbox.data, &sandbox.output).is_err());
        assert!(!sandbox.receipt().exists());
    }
}

#[test]
fn an_unreadable_receipt_is_an_error_and_preserves_application_data() {
    let sandbox = Sandbox::new();
    let source = sandbox.data.join("journey-startup.txt");
    fs::create_dir(&source).unwrap();
    assert!(retain(&sandbox.data, &sandbox.output).is_err());
    assert!(source.is_dir());
    assert!(!sandbox.receipt().exists());
}
