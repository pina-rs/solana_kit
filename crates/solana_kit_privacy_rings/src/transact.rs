//! Confidential transfers and withdrawals.
//!
//! Both are the shielded-pool `transact` instruction (tag 16). A confidential
//! transfer has no public settlement leg. A withdrawal adds a SOL or SPL
//! withdrawal leg. The owner is an `owner_signers` entry and must sign the
//! transaction; this crate never signs.

use solana_kit_core::{AccountMeta, AccountRole, Address, Instruction};

use crate::Error;
use crate::address::{
    CPI_AUTHORITY, MAX_INPUT_TREES, MAX_INTERFACE_TRANSFERS, MAX_OUTPUTS, SHIELDED_POOL_PROGRAM,
    SOL_INTERFACE, TRANSACT_TAG,
};
use crate::encode::{TransactBody, TransferLeg, Writer, write_transact};

/// SPL accounts for a public deposit leg inside `transact`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplDepositAccounts {
    /// Mint.
    pub mint: Address,
    /// SPL interface PDA.
    pub interface: Address,
    /// Token-account authority. This account must sign.
    pub token_authority: Address,
    /// Source token account.
    pub user_token: Address,
    /// Token program or Token-2022 program.
    pub token_program: Address,
}

/// SPL accounts for a public withdrawal leg.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplWithdrawalAccounts {
    /// Mint.
    pub mint: Address,
    /// SPL interface PDA.
    pub interface: Address,
    /// Destination token account.
    pub user_token: Address,
    /// Token program or Token-2022 program.
    pub token_program: Address,
}

/// One public settlement leg and the accounts it appends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Settlement {
    /// Move public SOL into the pool.
    SolDeposit {
        /// System account that funds the deposit. It must sign.
        recipient: Address,
        /// Lamports.
        amount: u64,
    },
    /// Move private SOL to a public system account.
    SolWithdrawal {
        /// System account that receives the lamports.
        recipient: Address,
        /// Lamports.
        amount: u64,
    },
    /// Move public SPL into the pool.
    SplDeposit {
        /// Funding accounts.
        accounts: SplDepositAccounts,
        /// Amount in base units.
        amount: u64,
        /// SPL interface bump.
        interface_bump: u8,
    },
    /// Move private SPL to a public token account.
    SplWithdrawal {
        /// Destination accounts.
        accounts: SplWithdrawalAccounts,
        /// Amount in base units.
        amount: u64,
        /// SPL interface bump.
        interface_bump: u8,
    },
}

impl Settlement {
    fn leg(self) -> TransferLeg {
        match self {
            Self::SolDeposit { amount, .. } => TransferLeg::SolDeposit(amount),
            Self::SolWithdrawal { amount, .. } => TransferLeg::SolWithdrawal(amount),
            Self::SplDeposit {
                amount,
                interface_bump,
                ..
            } => TransferLeg::SplDeposit {
                amount,
                interface_bump,
            },
            Self::SplWithdrawal {
                amount,
                interface_bump,
                ..
            } => TransferLeg::SplWithdrawal {
                amount,
                interface_bump,
            },
        }
    }

    fn accounts(self, out: &mut Vec<AccountMeta>) {
        match self {
            Self::SolDeposit { recipient, .. } => {
                out.push(meta(SOL_INTERFACE, AccountRole::Writable));
                out.push(meta(recipient, AccountRole::WritableSigner));
            }
            Self::SolWithdrawal { recipient, .. } => {
                out.push(meta(SOL_INTERFACE, AccountRole::Writable));
                out.push(meta(recipient, AccountRole::Writable));
            }
            Self::SplDeposit { accounts, .. } => out.extend([
                meta(accounts.mint, AccountRole::Readonly),
                meta(accounts.interface, AccountRole::Writable),
                meta(accounts.token_authority, AccountRole::ReadonlySigner),
                meta(accounts.user_token, AccountRole::Writable),
                meta(accounts.token_program, AccountRole::Readonly),
            ]),
            Self::SplWithdrawal { accounts, .. } => out.extend([
                meta(CPI_AUTHORITY, AccountRole::Readonly),
                meta(accounts.mint, AccountRole::Readonly),
                meta(accounts.interface, AccountRole::Writable),
                meta(accounts.user_token, AccountRole::Writable),
                meta(accounts.token_program, AccountRole::Readonly),
            ]),
        }
    }
}

/// Writable cache account and the signer that authorizes the write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CacheWriter {
    /// Cache account. The instruction marks it writable.
    pub cache: Address,
    /// Cache writer. This account must sign.
    pub writer: Address,
}

/// A `transact` instruction. Confidential transfer and withdraw are constructors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transact {
    /// Fee payer. This account must sign.
    pub payer: Address,
    /// Input state trees, in `body.tree_contexts` order.
    pub input_trees: Vec<Address>,
    /// Tree that receives new notes.
    pub output_tree: Address,
    /// Spending owners. Each must sign. A confidential spend includes the owner.
    pub owner_signers: Vec<Address>,
    /// One writable nullifier PDA per input, in input order.
    pub nullifier_accounts: Vec<Address>,
    /// Public legs. Empty for a confidential transfer.
    pub settlements: Vec<Settlement>,
    /// Read-only cache. Required when the circuit reads a cache slot.
    pub read_cache: Option<Address>,
    /// Write cache and its signer. Required when the circuit writes a cache slot.
    pub write_cache: Option<CacheWriter>,
    /// Proof, ciphertexts, and private fields.
    pub body: TransactBody,
}

/// Confidential transfer to another private balance.
///
/// The asset and amount stay in `body` and the proof. This constructor adds no
/// public settlement leg, so the transfer does not reveal them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfidentialTransfer {
    /// Fee payer, who must sign.
    pub payer: Address,
    /// Input trees.
    pub input_trees: Vec<Address>,
    /// Output tree.
    pub output_tree: Address,
    /// Owner who can spend the input notes. Must sign.
    pub owner: Address,
    /// Extra owner signers, after `owner`.
    pub additional_owners: Vec<Address>,
    /// Nullifier PDAs parallel to `body.inputs`.
    pub nullifier_accounts: Vec<Address>,
    /// Proof and encrypted outputs.
    pub body: TransactBody,
}

/// Withdraw (unshield) to a public balance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Withdraw {
    /// Fee payer, who must sign.
    pub payer: Address,
    /// Input trees.
    pub input_trees: Vec<Address>,
    /// Output tree for any private change.
    pub output_tree: Address,
    /// Owner who can spend the input notes. Must sign.
    pub owner: Address,
    /// Nullifier PDAs parallel to `body.inputs`.
    pub nullifier_accounts: Vec<Address>,
    /// Public destination. SOL and SPL use different account lists.
    pub settlement: Settlement,
    /// Proof and any remaining private outputs.
    pub body: TransactBody,
}

impl ConfidentialTransfer {
    /// Builds a `transact` with no public settlement leg.
    ///
    /// # Errors
    /// Rejects a body that does not line up with the supplied trees and nullifiers.
    pub fn instruction(&self) -> Result<Instruction, Error> {
        let mut owners = Vec::with_capacity(1 + self.additional_owners.len());
        owners.push(self.owner);
        owners.extend(self.additional_owners.iter().copied());
        Transact {
            payer: self.payer,
            input_trees: self.input_trees.clone(),
            output_tree: self.output_tree,
            owner_signers: owners,
            nullifier_accounts: self.nullifier_accounts.clone(),
            settlements: Vec::new(),
            read_cache: None,
            write_cache: None,
            body: self.body.clone(),
        }
        .instruction()
    }
}

impl Withdraw {
    /// Builds a `transact` whose only public leg is the withdrawal.
    ///
    /// # Errors
    /// Rejects a deposit leg, a zero amount, or a body that does not line up
    /// with the accounts.
    pub fn instruction(&self) -> Result<Instruction, Error> {
        match self.settlement {
            Settlement::SolWithdrawal { .. } | Settlement::SplWithdrawal { .. } => {}
            Settlement::SolDeposit { .. } | Settlement::SplDeposit { .. } => {
                return Err(Error::SettlementMismatch);
            }
        }
        Transact {
            payer: self.payer,
            input_trees: self.input_trees.clone(),
            output_tree: self.output_tree,
            owner_signers: vec![self.owner],
            nullifier_accounts: self.nullifier_accounts.clone(),
            settlements: vec![self.settlement],
            read_cache: None,
            write_cache: None,
            body: self.body.clone(),
        }
        .instruction()
    }
}

impl Transact {
    /// Builds the instruction. The caller signs `payer` and every owner signer.
    ///
    /// # Errors
    /// Rejects a ring selector, a cache selector whose accounts are missing,
    /// mismatched input, tree, output, or settlement counts, a zero public
    /// amount, or a ciphertext longer than `u16`.
    pub fn instruction(&self) -> Result<Instruction, Error> {
        if !self.body.circuit.fits_transact() {
            return Err(Error::Limit("transact circuit"));
        }
        let cache = self.body.circuit.cache();
        let reads = cache.is_some_and(crate::encode::CacheAccess::reads);
        let writes = cache.is_some_and(crate::encode::CacheAccess::writes_cache);
        if cache.is_some() && !reads && !writes {
            return Err(Error::Limit("transact cache"));
        }
        if reads != self.read_cache.is_some() || writes != self.write_cache.is_some() {
            return Err(Error::SettlementMismatch);
        }
        if self.input_trees.is_empty() || self.input_trees.len() > MAX_INPUT_TREES {
            return Err(Error::Limit("transact input trees"));
        }
        if self.input_trees.len() != self.body.tree_contexts.len() {
            return Err(Error::SettlementMismatch);
        }
        if self.nullifier_accounts.len() != self.body.inputs.len() {
            return Err(Error::SettlementMismatch);
        }
        if self.body.outputs.len() > MAX_OUTPUTS {
            return Err(Error::Limit("transact outputs"));
        }
        if self.settlements.len() > MAX_INTERFACE_TRANSFERS {
            return Err(Error::Limit("transact settlement legs"));
        }
        for input in &self.body.inputs {
            if usize::from(input.tree_index) >= self.input_trees.len() {
                return Err(Error::SettlementMismatch);
            }
        }

        let legs: Vec<TransferLeg> = self
            .settlements
            .iter()
            .copied()
            .map(Settlement::leg)
            .collect();
        let mut writer = Writer::new();
        writer.u8(TRANSACT_TAG);
        writer.raw(&write_transact(&self.body, &legs)?);

        let mut accounts = vec![
            meta(self.payer, AccountRole::WritableSigner),
            meta(self.output_tree, AccountRole::Writable),
            meta(SHIELDED_POOL_PROGRAM, AccountRole::Readonly),
            meta(
                solana_kit_core::address_constants::SYSTEM_PROGRAM,
                AccountRole::Readonly,
            ),
        ];
        accounts.extend(
            self.input_trees
                .iter()
                .copied()
                .map(|tree| meta(tree, AccountRole::Writable)),
        );
        accounts.extend(
            self.nullifier_accounts
                .iter()
                .copied()
                .map(|account| meta(account, AccountRole::Writable)),
        );
        accounts.extend(
            self.owner_signers
                .iter()
                .copied()
                .map(|signer| meta(signer, AccountRole::ReadonlySigner)),
        );
        for settlement in &self.settlements {
            settlement.accounts(&mut accounts);
        }
        if let Some(cache) = self.read_cache {
            accounts.push(meta(cache, AccountRole::Readonly));
        }
        if let Some(cache) = self.write_cache {
            accounts.push(meta(cache.cache, AccountRole::Writable));
            accounts.push(meta(cache.writer, AccountRole::ReadonlySigner));
        }

        Ok(Instruction {
            program_address: SHIELDED_POOL_PROGRAM,
            accounts,
            data: writer.finish(),
        })
    }
}

fn meta(address: Address, role: AccountRole) -> AccountMeta {
    AccountMeta { address, role }
}
