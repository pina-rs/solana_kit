//! Explicit native wallet selection, private persistence and signing review.
use leptoskit_native::{Element, EventOutcome, Live, NativeEventSlot, NativeKind};
use std::{
    future::Future,
    sync::{Arc, Mutex, mpsc},
    task::{Poll, Waker},
};

/// User intents emitted by actual native buttons.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalWalletAction {
    /// Generate a fresh local key with host entropy.
    Create,
    /// Load an intentionally saved private wallet.
    Load,
    /// Restore an explicitly created private copy inside the app sandbox.
    RestoreBackup,
    /// Persist the current key without overwriting a prior wallet.
    Save,
    /// Make an explicit second private copy inside the application sandbox.
    Backup,
    /// Review the application's exact proposed message before signing.
    Review,
    /// Approve only the identified current review.
    Approve(u64),
    /// Reject only the identified current review.
    Reject(u64),
    /// Cancel pending work and discard local signing authority.
    Cancel,
    /// Await close and discard local signing authority.
    Disconnect,
    /// Explicitly select Android MWA, when the platform reports availability.
    Android,
}

/// Redacted queue failure; callers must surface rejected user actions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalQueueError {
    /// The bounded queue is full or the receiver is gone.
    Unavailable,
    /// The shared wake state was poisoned.
    Poisoned,
}

/// Cloneable button endpoint, containing no wallet or private key.
#[derive(Clone)]
pub struct LocalWalletCommands {
    sender: mpsc::SyncSender<LocalWalletAction>,
    waker: Arc<Mutex<Option<Waker>>>,
}

/// Application-owned action receiver, awaited on its existing local executor.
pub struct LocalWalletEvents {
    receiver: mpsc::Receiver<LocalWalletAction>,
    waker: Arc<Mutex<Option<Waker>>>,
}

/// Creates one finite action channel without a task or runtime.
pub fn local_wallet_channel() -> (LocalWalletCommands, LocalWalletEvents) {
    let (sender, receiver) = mpsc::sync_channel(16);
    let waker = Arc::new(Mutex::new(None));
    (
        LocalWalletCommands {
            sender,
            waker: Arc::clone(&waker),
        },
        LocalWalletEvents { receiver, waker },
    )
}

impl LocalWalletCommands {
    /// Admits a user action and wakes the application outside the shared lock.
    pub fn send(&self, action: LocalWalletAction) -> Result<(), LocalQueueError> {
        let mut state = self.waker.lock().map_err(|_| LocalQueueError::Poisoned)?;
        self.sender
            .try_send(action)
            .map_err(|_| LocalQueueError::Unavailable)?;
        let waker = state.take();
        drop(state);

        if let Some(waker) = waker {
            waker.wake();
        }
        Ok(())
    }
}

impl LocalWalletEvents {
    /// Waits for the next action; dropping this future leaves queued actions intact.
    pub fn next(
        &mut self,
    ) -> impl Future<Output = Result<LocalWalletAction, LocalQueueError>> + '_ {
        std::future::poll_fn(|context| {
            *self.waker.lock().map_err(|_| LocalQueueError::Poisoned)? =
                Some(context.waker().clone());

            match self.receiver.try_recv() {
                Ok(action) => Poll::Ready(Ok(action)),
                Err(mpsc::TryRecvError::Empty) => Poll::Pending,
                Err(mpsc::TryRecvError::Disconnected) => {
                    Poll::Ready(Err(LocalQueueError::Unavailable))
                }
            }
        })
    }
}

/// A review binds one non-reused application generation to the exact message.
pub struct LocalSigningReview<'a> {
    /// Non-reused review identity, validated again by the action owner.
    pub generation: u64,
    /// Application explanation of the requested effect.
    pub purpose: &'a str,
    /// The exact bytes that approval will sign.
    pub message: &'a [u8],
}

/// Renders real action buttons. The application executes their intents and
/// returns a fresh snapshot; a view never contains signing or export authority.
pub fn local_wallet_panel(
    address: Option<&str>,
    busy: bool,
    android: bool,
    commands: &LocalWalletCommands,
) -> Element {
    let mut view = base("Local Solana wallet", busy);

    if busy {
        return view.child(button(
            "Cancel request",
            LocalWalletAction::Cancel,
            commands,
        ));
    }

    if let Some(address) = address {
        view = view.child(Element::text(address).id("wallet_local.account"));

        for (label, action) in [
            ("Review signature", LocalWalletAction::Review),
            ("Save wallet privately", LocalWalletAction::Save),
            ("Create private copy", LocalWalletAction::Backup),
            ("Disconnect wallet", LocalWalletAction::Disconnect),
        ] {
            view = view.child(button(label, action, commands));
        }

        return view.child(Element::text(
            "Private copies stay in this app. Uninstalling the app can remove them.",
        ));
    }

    view = view
        .child(button(
            "Create local wallet",
            LocalWalletAction::Create,
            commands,
        ))
        .child(button(
            "Load saved wallet",
            LocalWalletAction::Load,
            commands,
        ))
        .child(button(
            "Restore private copy",
            LocalWalletAction::RestoreBackup,
            commands,
        ));

    if android {
        view = view.child(button(
            "Connect Android wallet",
            LocalWalletAction::Android,
            commands,
        ));
    }
    view
}

/// External wallets expose signing and disconnect; they own their key storage.
pub fn mobile_wallet_panel(address: &str, busy: bool, commands: &LocalWalletCommands) -> Element {
    let view = base("Android Solana wallet", busy)
        .child(Element::text(address).id("wallet_mobile.account"));

    if busy {
        return view.child(button(
            "Cancel request",
            LocalWalletAction::Cancel,
            commands,
        ));
    }

    view.child(button(
        "Request wallet signature",
        LocalWalletAction::Review,
        commands,
    ))
    .child(button(
        "Disconnect wallet",
        LocalWalletAction::Disconnect,
        commands,
    ))
}

/// Signing remains pending until the matching explicit Approve action arrives.
/// Only byte count and a digest are displayed; secret keys never reach the UI.
pub fn local_signing_approval(
    review: &LocalSigningReview<'_>,
    commands: &LocalWalletCommands,
) -> Element {
    let digest = solana_kit_core::crypto::hash(&[review.message]);
    base("Approve local wallet signature", false)
        .id("wallet_local.approval")
        .role(leptoskit_native::Role::Dialog)
        .aria_modal(true)
        .child(Element::text(review.purpose))
        .child(Element::text(format!(
            "{} bytes · SHA-256 {digest}",
            review.message.len()
        )))
        .child(button(
            "Approve signature",
            LocalWalletAction::Approve(review.generation),
            commands,
        ))
        .child(button(
            "Reject signature",
            LocalWalletAction::Reject(review.generation),
            commands,
        ))
}

fn base(label: &str, busy: bool) -> Element {
    Element::new(NativeKind::View)
        .class("w-full max-w-[560px] flex-col p-4 gap-y-3")
        .aria_label(label)
        .aria_busy(busy)
        .child(Element::text(label).aria_live(Live::Polite))
}

fn button(label: &str, action: LocalWalletAction, commands: &LocalWalletCommands) -> Element {
    let commands = commands.clone();
    let mut element = Element::new(NativeKind::Button);
    element.props.text = Some(label.into());
    element
        .class("w-full min-h-[44px] px-3 py-2")
        .id(format!("wallet_local.{action:?}"))
        .aria_label(label)
        .on(NativeEventSlot::Press, move |_| {
            match commands.send(action) {
                Ok(()) => EventOutcome::Handled,
                Err(_) => EventOutcome::Unhandled,
            }
        })
}
