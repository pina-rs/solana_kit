use super::{
    BrowserWallet, Connection, WalletController, WalletDiscovery, WalletFailure, WalletFuture,
};
use solana_kit_wallet::{
    Wallet, WalletAccount,
    ui::{ChoiceKey, PanelAction, PanelState, WalletChoice, WalletPanel},
};
use std::rc::{Rc, Weak};

/// A real browser controller's owned view projection. UI callbacks carry only
/// opaque keys; a stale key cannot select a newly registered wallet or account.
pub struct BrowserWalletPanel {
    controller: Rc<WalletController>,
    discovery: Rc<WalletDiscovery>,
    wallets: Vec<(ChoiceKey, Weak<BrowserWallet>)>,
    accounts: Vec<(ChoiceKey, WalletAccount)>,
    account_wallet: Option<Weak<BrowserWallet>>,
    chain: String,
    next_key: u64,
    picker_open: bool,
}

impl BrowserWalletPanel {
    /// Uses the actual discovery and authorization owner; installs no singleton.
    pub fn new(discovery: Rc<WalletDiscovery>, chain: String) -> Result<Self, WalletFailure> {
        let controller = Rc::new(WalletController::new(Rc::clone(&discovery), chain.clone())?);
        Ok(Self {
            controller,
            discovery,
            wallets: Vec::new(),
            accounts: Vec::new(),
            account_wallet: None,
            chain,
            next_key: 0,
            picker_open: false,
        })
    }

    /// The same controller supplies state notifications and typed Kit signers.
    pub fn controller(&self) -> &Rc<WalletController> {
        &self.controller
    }

    fn key(&mut self) -> Result<ChoiceKey, WalletFailure> {
        self.next_key = self
            .next_key
            .checked_add(1)
            .ok_or_else(WalletFailure::invalid)?;
        Ok(ChoiceKey(self.next_key))
    }

    /// Reads actual advertised wallet metadata and current authorization.
    pub fn snapshot(&mut self) -> Result<WalletPanel, WalletFailure> {
        let choices = self.wallet_choices()?;
        let state = self.controller.state();

        let visual = match state.connection {
            Connection::Disconnected if state.operating => {
                self.accounts.clear();

                PanelState::Disconnecting
            }
            Connection::Disconnected => {
                self.accounts.clear();

                PanelState::Disconnected
            }
            Connection::Connecting(wallet) => PanelState::Connecting(wallet.name().into()),
            Connection::Connected { wallet, account } => {
                self.refresh_accounts(&wallet)?;

                PanelState::Connected {
                    wallet: wallet.name().into(),
                    account,
                    operating: state.operating,
                }
            }
            Connection::Disposed => {
                self.accounts.clear();

                PanelState::Closed
            }
        };

        Ok(WalletPanel {
            state: visual,
            wallets: choices,
            accounts: self.accounts.clone(),
            picker_open: self.picker_open,
            error: state.error.map(|error| error.code()),
        })
    }

    fn wallet_choices(&mut self) -> Result<Vec<WalletChoice>, WalletFailure> {
        let mut choices = Vec::new();
        let current = self.discovery.wallets();
        self.wallets.retain(|(_, wallet)| {
            wallet
                .upgrade()
                .is_some_and(|old| current.iter().any(|wallet| Rc::ptr_eq(&old, wallet)))
        });

        for wallet in current {
            let key = self.wallets.iter().find_map(|(key, old)| {
                old.upgrade()
                    .filter(|old| Rc::ptr_eq(old, &wallet))
                    .map(|_| *key)
            });

            let key = match key {
                Some(key) => key,
                None => {
                    let key = self.key()?;
                    self.wallets.push((key, Rc::downgrade(&wallet)));

                    key
                }
            };

            let enabled = wallet.is_available()
                && wallet.chains().contains(&self.chain)
                && wallet.feature(solana_kit_wallet::CONNECT).is_ok()
                && wallet.feature(solana_kit_wallet::EVENTS).is_ok();
            choices.push(WalletChoice {
                key,
                name: wallet.name().into(),
                enabled,
            });
        }

        Ok(choices)
    }

    fn refresh_accounts(&mut self, wallet: &Rc<BrowserWallet>) -> Result<(), WalletFailure> {
        let same = self
            .account_wallet
            .as_ref()
            .and_then(Weak::upgrade)
            .is_some_and(|old| Rc::ptr_eq(&old, wallet));

        if !same {
            self.accounts.clear();
        }

        self.account_wallet = Some(Rc::downgrade(wallet));
        let accounts: Vec<_> = wallet
            .accounts()
            .into_iter()
            .filter(|account| account.chains().contains(&self.chain))
            .collect();
        self.accounts.retain(|(_, old)| accounts.contains(old));

        for account in accounts {
            if !self.accounts.iter().any(|(_, old)| old == &account) {
                let key = self.key()?;
                self.accounts.push((key, account));
            }
        }

        Ok(())
    }

    /// Resolves the exact current identity before invoking a real controller action.
    /// The returned future is local and must be owned/polled by the application.
    pub fn dispatch(&mut self, action: PanelAction) -> Result<WalletFuture<()>, WalletFailure> {
        match action {
            PanelAction::Refresh => (),
            PanelAction::Connect(key) => {
                let wallet = self
                    .wallets
                    .iter()
                    .find(|(id, _)| *id == key)
                    .and_then(|(_, wallet)| wallet.upgrade())
                    .ok_or_else(WalletFailure::invalid)?;
                self.picker_open = false;
                return self.controller.connect(wallet, false);
            }
            PanelAction::Disconnect => return self.controller.disconnect(),
            PanelAction::SelectAccount(key) => {
                let account = self
                    .accounts
                    .iter()
                    .find(|(id, _)| *id == key)
                    .map(|(_, account)| account.clone())
                    .ok_or_else(WalletFailure::invalid)?;
                self.controller.select_account(account)?;
            }
            PanelAction::OpenPicker => self.picker_open = true,
            PanelAction::ClosePicker => self.picker_open = false,
            PanelAction::Cancel => self.controller.cancel()?,
            PanelAction::ClearError => self.controller.clear_error()?,
        }

        Ok(Box::pin(std::future::ready(Ok(()))))
    }
}
