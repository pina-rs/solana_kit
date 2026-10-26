//! A test owns only its fresh private application directory.
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub(crate) struct Sandbox(pub(crate) PathBuf);

impl Sandbox {
    pub(crate) fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("solana-generated-guest-{nonce}-{sequence}"));
        fs::create_dir(&path).unwrap();
        Self(path.canonicalize().unwrap())
    }

    pub(crate) fn config(&self, http: &str, socket: &str) {
        let json = serde_json::json!({"http":http,"socket":socket});
        fs::write(
            self.0.join("journey.json"),
            serde_json::to_vec(&json).unwrap(),
        )
        .unwrap();
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
