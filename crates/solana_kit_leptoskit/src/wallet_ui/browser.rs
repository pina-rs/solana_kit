use super::{PanelCommands, PanelEvents, PanelQueueError, panel_channel, wallet_panel};
use leptoskit_native::Element;
use solana_kit_browser::wallet::{
    BrowserWalletPanel, ControllerSubscription, WalletDiscovery, WalletFailure, WalletFuture,
};
use solana_kit_wallet::{RegistryError, RegistrySubscription, ui::PanelAction};
use std::{
    cell::Cell,
    error::Error,
    fmt,
    rc::Rc,
    task::{Context, Poll},
};

const ACTION_BUDGET: usize = 32;

/// A view boundary failure retains its original typed source.
#[derive(Debug)]
pub enum BrowserWalletViewError {
    /// Browser discovery or authorization failed.
    Wallet(WalletFailure),
    /// The bounded UI command owner is unavailable.
    Queue(PanelQueueError),
    /// Registry observation could not be installed.
    Registry(RegistryError),
}

impl fmt::Display for BrowserWalletViewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("wallet view boundary failed")
    }
}

impl Error for BrowserWalletViewError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Wallet(error) => Some(error),
            Self::Queue(error) => Some(error),
            Self::Registry(error) => Some(error),
        }
    }
}

/// One rendered update and its original operation failure, if any.
pub struct BrowserWalletUpdate {
    /// The accessible view of the actual current authorization state.
    pub element: Element,
    /// Original browser/controller failure; diagnostics remain redacted.
    pub failure: Option<WalletFailure>,
}

/// A local owner polled through short borrows. It never captures an application,
/// spawns a runtime, or blocks cancellation while a wallet promise is pending.
pub struct BrowserWalletView {
    panel: BrowserWalletPanel,
    commands: PanelCommands,
    events: PanelEvents,
    pending: Option<WalletFuture<()>>,
    queue_failure: Rc<Cell<Option<PanelQueueError>>>,
    controller_subscription: Option<ControllerSubscription>,
    registry_subscription: Option<RegistrySubscription>,
}

impl BrowserWalletView {
    /// Installs notifications over actual Window discovery and one chain.
    pub fn new(
        discovery: Rc<WalletDiscovery>,
        chain: String,
    ) -> Result<Self, BrowserWalletViewError> {
        let panel = BrowserWalletPanel::new(Rc::clone(&discovery), chain)
            .map_err(BrowserWalletViewError::Wallet)?;
        let (commands, events) =
            panel_channel(ACTION_BUDGET).map_err(BrowserWalletViewError::Queue)?;
        let queue_failure = Rc::new(Cell::new(None));
        let on_change = notification(commands.clone(), Rc::clone(&queue_failure));
        let controller_subscription = panel
            .controller()
            .on_change(move |_| on_change())
            .map_err(BrowserWalletViewError::Wallet)?;
        let on_registry = notification(commands.clone(), Rc::clone(&queue_failure));
        let registry_subscription = discovery
            .registry()
            .on_event(move |_| on_registry())
            .map_err(BrowserWalletViewError::Registry)?;
        commands.refresh().map_err(BrowserWalletViewError::Queue)?;
        Ok(Self {
            panel,
            commands,
            events,
            pending: None,
            queue_failure,
            controller_subscription: Some(controller_subscription),
            registry_subscription: Some(registry_subscription),
        })
    }

    /// The real owner supplies typed Kit signers and exact authorization snapshots.
    pub fn panel(&self) -> &BrowserWalletPanel {
        &self.panel
    }

    /// Drives at most 32 actions, then renders independently of pending prompts.
    /// Register the supplied waker with the application's existing local executor.
    pub fn poll_update(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Result<BrowserWalletUpdate, BrowserWalletViewError>> {
        if let Some(error) = self.queue_failure.take() {
            return Poll::Ready(Err(BrowserWalletViewError::Queue(error)));
        }

        let (mut changed, mut failure) = self.drain(context)?;

        if let Some(operation) = &mut self.pending
            && let Poll::Ready(result) = operation.as_mut().poll(context)
        {
            self.pending.take();
            changed = true;

            if let Err(error) = result {
                failure = Some(error);
            }
        }

        if !changed {
            return Poll::Pending;
        }

        let model = self
            .panel
            .snapshot()
            .map_err(BrowserWalletViewError::Wallet)?;
        Poll::Ready(Ok(BrowserWalletUpdate {
            element: wallet_panel(&model, &self.commands),
            failure,
        }))
    }

    fn drain(
        &mut self,
        context: &mut Context<'_>,
    ) -> Result<(bool, Option<WalletFailure>), BrowserWalletViewError> {
        let mut changed = false;
        let mut failure = None;

        for index in 0..ACTION_BUDGET {
            match self.events.poll_next(context) {
                Poll::Ready(Ok(Some(action))) => {
                    changed = true;
                    failure = self.dispatch(action).err().or(failure);
                }
                Poll::Ready(Err(error)) => return Err(BrowserWalletViewError::Queue(error)),
                Poll::Ready(Ok(None)) | Poll::Pending => break,
            }

            if index + 1 == ACTION_BUDGET {
                context.waker().wake_by_ref();
            }
        }

        Ok((changed, failure))
    }

    fn dispatch(&mut self, action: PanelAction) -> Result<(), WalletFailure> {
        let operation = self.panel.dispatch(action)?;

        match action {
            PanelAction::Connect(_) | PanelAction::Disconnect => self.pending = Some(operation),
            PanelAction::Cancel | PanelAction::SelectAccount(_) => self.pending = None,
            _ => (),
        }

        Ok(())
    }
}

impl Drop for BrowserWalletView {
    fn drop(&mut self) {
        self.panel.controller().dispose();
        self.pending.take();
        self.controller_subscription.take();
        self.registry_subscription.take();
    }
}

fn notification(commands: PanelCommands, failed: Rc<Cell<Option<PanelQueueError>>>) -> impl Fn() {
    move || {
        if let Err(error) = commands.refresh() {
            failed.set(Some(error));
        }
    }
}
