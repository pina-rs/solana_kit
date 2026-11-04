//! Verification-only receipts carry action kinds and explicit public journey data.
use leptoskit_bridge::{
    Error, MethodId,
    guest::{Call, Client},
};
use solana_kit_leptoskit::{WalletSessionError, wallet_ui::LocalWalletAction};
use std::{cell::RefCell, future::Future, pin::Pin, task::Context};

const LIMIT: usize = 128;
const RECEIPT_BYTES: usize = 512;

thread_local! {
    static CALLS: RefCell<Vec<Call>> = const { RefCell::new(Vec::new()) };
}

pub(super) fn press(node: i64, handled: i32) {
    let mut bytes = [0; 12];
    bytes[..8].copy_from_slice(&node.to_le_bytes());
    bytes[8..].copy_from_slice(&handled.to_le_bytes());
    send(0, &bytes);
}

pub(super) fn action(action: LocalWalletAction) {
    let label = match action {
        LocalWalletAction::Create => "create",
        LocalWalletAction::Load => "load",
        LocalWalletAction::RestoreBackup => "restore",
        LocalWalletAction::Save => "save",
        LocalWalletAction::Backup => "backup",
        LocalWalletAction::Review => "review",
        LocalWalletAction::Approve(_) => "approve",
        LocalWalletAction::Reject(_) => "reject",
        LocalWalletAction::Cancel => "cancel",
        LocalWalletAction::Disconnect => "disconnect",
        LocalWalletAction::Android => "android",
    };

    send(1, label.as_bytes());
}

pub(super) fn result(result: &Result<(), super::AppError>) {
    let category = match result {
        Ok(()) => 0,
        Err(error) => match error.downcast_ref::<WalletSessionError>() {
            Some(WalletSessionError::Bridge(Error::Native)) => 1,
            Some(WalletSessionError::Bridge(Error::Cancelled)) => 2,
            _ => 3,
        },
    };

    send(2, &[category]);
}

/// Retains only public journey evidence: labels, addresses, signatures and
/// lamport amounts. Private wallet material never reaches this observer.
pub(super) fn receipt(line: &str) {
    if line.len() > RECEIPT_BYTES || line.contains(['\n', '\r']) {
        return;
    }

    send(3, line.as_bytes());
}

fn send(method: u32, payload: &[u8]) {
    CALLS.with_borrow_mut(|calls| {
        if calls.len() == LIMIT {
            return;
        }

        if let Ok(client) = Client::new("solana-journey-observer")
            && let Ok(call) = client.call(MethodId(method), payload)
        {
            calls.push(call);
        }
    });
}

pub(super) fn poll() {
    let mut context = Context::from_waker(futures::task::noop_waker_ref());
    CALLS.with_borrow_mut(|calls| {
        calls.retain_mut(|call| Pin::new(call).poll(&mut context).is_pending());
    });
}
