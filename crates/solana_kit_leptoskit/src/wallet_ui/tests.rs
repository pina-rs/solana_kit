use super::*;
use solana_kit_wallet::ui::{PanelAction, PanelState, WalletPanel};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

struct Signals(AtomicUsize);
impl Wake for Signals {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn actual_action_queue_prioritizes_user_cancel_and_wakes_idle_owner() {
    let (commands, mut events) = panel_channel(1).unwrap();
    let signals = Arc::new(Signals(AtomicUsize::new(0)));
    let waker = Waker::from(Arc::clone(&signals));
    let mut context = Context::from_waker(&waker);
    assert!(events.poll_next(&mut context).is_pending());
    commands.refresh().unwrap();
    commands.refresh().unwrap();
    commands.send(PanelAction::Cancel).unwrap();
    assert_eq!(
        commands.send(PanelAction::Disconnect),
        Err(PanelQueueError::Capacity)
    );
    assert_eq!(signals.0.load(Ordering::SeqCst), 1);
    assert_eq!(
        events.poll_next(&mut context),
        Poll::Ready(Ok(Some(PanelAction::Cancel)))
    );
    assert_eq!(
        events.poll_next(&mut context),
        Poll::Ready(Ok(Some(PanelAction::Refresh)))
    );
    assert!(events.poll_next(&mut context).is_pending());
    drop(events);
    assert_eq!(
        commands.send(PanelAction::OpenPicker),
        Err(PanelQueueError::Closed)
    );
}

#[test]
fn disconnected_and_busy_views_have_accessible_status_and_cancel_controls() {
    let (commands, _events) = panel_channel(4).unwrap();
    let mut model = WalletPanel {
        state: PanelState::Disconnected,
        wallets: Vec::new(),
        accounts: Vec::new(),
        picker_open: true,
        error: None,
    };
    let view = wallet_panel(&model, &commands);
    assert_eq!(view.props.id.as_deref(), Some("wallet_ui.panel"));
    assert!(
        view.children
            .iter()
            .any(|child| child.props.id.as_deref() == Some("wallet_ui.picker"))
    );
    model.state = PanelState::Connecting("Actual discovered wallet".into());
    let view = wallet_panel(&model, &commands);
    assert!(
        view.children
            .iter()
            .any(|child| child.props.id.as_deref() == Some("wallet_ui.cancel"))
    );
    assert!(
        !view
            .children
            .iter()
            .any(|child| child.props.id.as_deref() == Some("wallet_ui.picker"))
    );

    model.state = PanelState::Disconnecting;
    let view = wallet_panel(&model, &commands);
    assert_eq!(
        view.children[0].props.text.as_deref(),
        Some("Disconnecting wallet")
    );
    assert!(
        view.children
            .iter()
            .any(|child| child.props.id.as_deref() == Some("wallet_ui.cancel"))
    );
}
