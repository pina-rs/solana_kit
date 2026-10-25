//! Pending admission keeps guest startup separate from the Store-drop interval.

use super::{MemoryOutputPipe, OUTPUT_BYTES, STORE_DROP_MILLIS, unload};
use std::{future::pending, time::Duration};
use wasmtime_wasi::p2::OutputStream;

#[tokio::test]
async fn startup_can_outlast_drop_interval_before_guest_becomes_pending() {
    let output = MemoryOutputPipe::new(OUTPUT_BYTES);
    let mut guest_output = output.clone();
    let run = async move {
        tokio::time::sleep(Duration::from_millis(STORE_DROP_MILLIS * 2)).await;
        guest_output
            .write(b"PENDING unload\n".to_vec().into())
            .unwrap();

        pending().await
    };

    unload(run, &output).await.unwrap();
}

#[tokio::test]
async fn printed_marker_does_not_admit_a_completed_guest() {
    let output = MemoryOutputPipe::new(OUTPUT_BYTES);
    let mut guest_output = output.clone();
    let run = async move {
        guest_output
            .write(b"PENDING unload\n".to_vec().into())
            .unwrap();

        Ok(Ok(()))
    };
    let error = unload(run, &output).await.unwrap_err();

    assert!(
        error
            .to_string()
            .contains("guest completed before pending I/O")
    );
}

#[tokio::test]
async fn guest_failure_after_pending_remains_a_failure() {
    let output = MemoryOutputPipe::new(OUTPUT_BYTES);
    let mut guest_output = output.clone();
    let run = async move {
        guest_output
            .write(b"PENDING unload\n".to_vec().into())
            .unwrap();
        tokio::time::sleep(Duration::from_millis(1)).await;

        Err(wasmtime::Error::msg("guest failure"))
    };
    let error = unload(run, &output).await.unwrap_err();

    assert!(
        error
            .to_string()
            .contains("pending guest completed before Store drop")
    );
    assert!(error.to_string().contains("guest failure"));
}
