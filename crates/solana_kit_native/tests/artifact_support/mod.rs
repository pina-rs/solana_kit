//! Explicit owned test roots are independent of ambient project discovery.

use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};
use tokio::fs;

static NEXT: AtomicUsize = AtomicUsize::new(0);
pub const PROGRAM: [u8; 5] = [1, 2, 3, 4, 255];

pub struct Directory(pub PathBuf);

impl Directory {
    pub async fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "solana-kit-artifacts-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).await.unwrap();

        Self(path)
    }

    pub async fn root(&self, name: &str) -> PathBuf {
        let root = self.0.join(name);
        fs::create_dir_all(root.join("deploy")).await.unwrap();
        fs::create_dir(root.join("idl")).await.unwrap();

        root
    }

    pub async fn pair(&self, root: &Path, key: &[u8]) {
        fs::write(root.join("deploy/demo.so"), PROGRAM)
            .await
            .unwrap();
        fs::write(root.join("deploy/demo-keypair.json"), key)
            .await
            .unwrap();
    }

    pub async fn finish(self) {
        fs::remove_dir_all(self.0).await.unwrap();
    }
}
