//! Shared receiver accounts preserve seven positional privileges without deduplication.
use crate::{PYTH_SOLANA_RECEIVER_PROGRAM, SYSTEM_PROGRAM};
use solana_kit_core::{AccountMeta, AccountRole, Address};

/// Caller-selected receiver context shared by both supported posting instructions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReceiverAccounts {
    /// Program to invoke; custom deployments remain caller-owned.
    pub program_address: Address,
    /// Fee payer, a writable signer.
    pub payer: Address,
    /// Receiver configuration, readonly.
    pub config: Address,
    /// Selected treasury, writable.
    pub treasury: Address,
    /// Price-update account, a writable signer.
    pub price_update_account: Address,
    /// Future write authority, a readonly signer; may duplicate the payer.
    pub write_authority: Address,
}

impl ReceiverAccounts {
    /// Uses the default receiver program and payer as write authority.
    /// Explicit account identities are not authenticated or derived by this constructor.
    pub const fn new(
        payer: Address,
        config: Address,
        treasury: Address,
        price_update_account: Address,
    ) -> Self {
        Self {
            program_address: PYTH_SOLANA_RECEIVER_PROGRAM,
            payer,
            config,
            treasury,
            price_update_account,
            write_authority: payer,
        }
    }

    /// Verification is guardian-set or posted-VAA identity according to the instruction.
    pub(super) fn metadata(self, verification: Address) -> [AccountMeta; 7] {
        [
            AccountMeta {
                address: self.payer,
                role: AccountRole::WritableSigner,
            },
            AccountMeta {
                address: verification,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: self.config,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: self.treasury,
                role: AccountRole::Writable,
            },
            AccountMeta {
                address: self.price_update_account,
                role: AccountRole::WritableSigner,
            },
            AccountMeta {
                address: SYSTEM_PROGRAM,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: self.write_authority,
                role: AccountRole::ReadonlySigner,
            },
        ]
    }
}
