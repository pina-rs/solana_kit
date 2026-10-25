use super::*;
use std::{
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

struct Signals(AtomicUsize);
const APPROVAL_GENERATION: u64 = 7;

impl Wake for Signals {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn local_action_mailbox_wakes_and_refuses_excess_or_closed_admission() {
    let (commands, mut events) = local_wallet_channel();
    let signals = Arc::new(Signals(AtomicUsize::new(0)));
    let waker = Waker::from(Arc::clone(&signals));
    let mut context = Context::from_waker(&waker);
    let mut next = Box::pin(events.next_action());
    assert!(next.as_mut().poll(&mut context).is_pending());
    commands
        .send(LocalWalletAction::Approve(APPROVAL_GENERATION))
        .unwrap();
    assert_eq!(signals.0.load(Ordering::SeqCst), 1);
    assert_eq!(
        next.as_mut().poll(&mut context),
        Poll::Ready(Ok(LocalWalletAction::Approve(APPROVAL_GENERATION)))
    );
    drop(next);

    const CAPACITY: u64 = 16;

    for generation in 0..CAPACITY {
        commands
            .send(LocalWalletAction::Reject(generation))
            .unwrap();
    }

    assert_eq!(
        commands.send(LocalWalletAction::Cancel),
        Err(LocalQueueError::Unavailable)
    );
    drop(events);
    assert_eq!(
        commands.send(LocalWalletAction::Disconnect),
        Err(LocalQueueError::Unavailable)
    );
}

#[test]
fn approval_and_busy_controls_keep_identity_labels_and_private_material_out_of_view() {
    let (commands, _events) = local_wallet_channel();
    let review = LocalSigningReview {
        generation: 11,
        purpose: "Approve the exact proposed message",
        message: b"visible message bytes",
    };
    let element = local_signing_approval(&review, &commands);
    assert_eq!(element.props.id.as_deref(), Some("wallet_local.approval"));
    let ids = element
        .children
        .iter()
        .filter_map(|child| child.props.id.as_deref())
        .collect::<Vec<_>>();

    assert!(ids.contains(&"wallet_local.Approve(11)"));
    assert!(ids.contains(&"wallet_local.Reject(11)"));
    assert!(!ids.contains(&"wallet_local.Save"));
    assert!(!ids.contains(&"wallet_local.Backup"));

    let busy = local_wallet_panel(Some("public account"), true, false, &commands);
    assert!(
        busy.children
            .iter()
            .any(|child| child.props.id.as_deref() == Some("wallet_local.Cancel"))
    );

    let external = mobile_wallet_panel("public account", false, &commands);
    assert!(
        !external
            .children
            .iter()
            .any(|child| child.props.id.as_deref() == Some("wallet_local.Save"))
    );
}
