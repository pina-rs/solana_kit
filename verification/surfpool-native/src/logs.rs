//! Two owned drainers share a hard file cap and are joined before directory removal.

use crate::{Error, Result};
use std::{
    fs::File,
    io::{Read, Write},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

const LOG_BYTES: usize = 512 * 1024;
const BUFFER_BYTES: usize = 8192;
const DRAIN: Duration = Duration::from_secs(2);
const POLL: Duration = Duration::from_millis(10);

/// Holds every drainer handle; no asynchronous cleanup task is detached.
pub(crate) struct Logs {
    output: Arc<Mutex<(File, usize)>>,
    failed: Arc<AtomicBool>,
    readers: Vec<JoinHandle<()>>,
}

impl Logs {
    /// Owns an exclusively created private runtime log.
    pub(crate) fn new(file: File) -> Self {
        Self {
            output: Arc::new(Mutex::new((file, 0))),
            failed: Arc::new(AtomicBool::new(false)),
            readers: Vec::new(),
        }
    }

    /// Drains excess bytes while recording a fatal cap violation.
    pub(crate) fn read(&mut self, input: impl Read + Send + 'static) {
        let output = Arc::clone(&self.output);
        let failed = Arc::clone(&self.failed);
        self.readers
            .push(thread::spawn(move || drain(input, &output, &failed)));
    }

    /// Makes filesystem errors and output overflow observable while the child lives.
    pub(crate) fn check(&self) -> Result<()> {
        if self.failed.load(Ordering::Relaxed) {
            return Err(Error::LogLimit);
        }

        Ok(())
    }

    /// Joins only completed readers; a retained pipe yields an explicit bounded error.
    pub(crate) async fn finish(&mut self) -> Result<()> {
        let deadline = Instant::now() + DRAIN;

        while self.readers.iter().any(|reader| !reader.is_finished()) {
            if Instant::now() >= deadline {
                return Err(Error::CleanupUnconfirmed);
            }

            tokio::time::sleep(POLL).await;
        }

        for reader in self.readers.drain(..) {
            reader.join().map_err(|_| Error::CleanupUnconfirmed)?;
        }

        self.check()
    }
}

/// Writes at most the shared cap and uses constant memory for both pipes.
fn drain(mut input: impl Read, output: &Mutex<(File, usize)>, failed: &AtomicBool) {
    let mut buffer = [0; BUFFER_BYTES];

    loop {
        let count = match input.read(&mut buffer) {
            Ok(0) => return,
            Ok(count) => count,
            Err(_) => {
                failed.store(true, Ordering::Relaxed);
                return;
            }
        };

        let Ok(mut output) = output.lock() else {
            failed.store(true, Ordering::Relaxed);

            return;
        };
        let kept = LOG_BYTES.saturating_sub(output.1).min(count);

        if output.0.write_all(&buffer[..kept]).is_err() || kept < count {
            failed.store(true, Ordering::Relaxed);
        }

        output.1 += kept;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, fs::OpenOptions, io::Cursor, time::SystemTime};

    #[tokio::test(flavor = "current_thread")]
    async fn finish_reports_overflow_even_without_an_earlier_poll() {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "solana-kit-log-overflow-{}-{nonce}",
            std::process::id()
        ));
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        let mut logs = Logs::new(file);
        logs.read(Cursor::new(vec![0; LOG_BYTES + 1]));

        let result = logs.finish().await;
        let length = fs::metadata(&path).unwrap().len();
        drop(logs);
        fs::remove_file(path).unwrap();

        assert!(matches!(result, Err(Error::LogLimit)), "{result:?}");
        assert_eq!(length, LOG_BYTES as u64);
    }
}
