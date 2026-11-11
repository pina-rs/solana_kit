//! Public deposit (shield) instructions.
//!
//! Account order follows `spr_instruction::instruction::deposit` at zolana v0.8.0:
//! tree, depositor, the shielded-pool program, then one settlement group per asset.
//! SOL is group 0 when any entry deposits SOL. SPL groups follow in first-seen order.

use solana_kit_core::{AccountMeta, AccountRole, Address, Instruction};

use crate::Error;
use crate::address::{DEPOSIT_TAG, MAX_DEPOSIT_ASSETS, SHIELDED_POOL_PROGRAM, SOL_INTERFACE};
use crate::encode::Writer;

/// SPL or Token-2022 funding accounts for one mint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplDeposit {
    /// Mint being shielded.
    pub mint: Address,
    /// Owner's token account. The depositor signs as its authority.
    pub user_token: Address,
    /// Token program or Token-2022 program.
    pub token_program: Address,
    /// Per-mint SPL interface PDA.
    pub interface: Address,
    /// Canonical bump of `interface`.
    pub interface_bump: u8,
}

/// Asset settled by one deposit entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DepositAsset {
    /// Native SOL. The depositor's system account is the source.
    Sol,
    /// An SPL or Token-2022 mint.
    Spl(SplDeposit),
}

/// One shielded output created by a deposit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetDeposit {
    /// Asset this entry funds.
    pub asset: DepositAsset,
    /// Indexing tag. In the permissionless ring this is the recipient wallet address.
    pub view_tag: [u8; 32],
    /// Recipient `owner_hash`.
    pub owner: [u8; 32],
    /// Amount in lamports or base units.
    pub amount: u64,
    /// Optional cleartext memo. It is not committed into the note hash.
    pub memo: Option<Vec<u8>>,
}

/// A batched public deposit. The owner signs `depositor`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deposit {
    /// State tree that receives the new notes.
    pub tree: Address,
    /// Fee payer and source authority.
    pub depositor: Address,
    /// Notes to append. At least one, and at most 255.
    pub deposits: Vec<AssetDeposit>,
}

#[derive(Clone, Copy)]
struct SplGroup {
    accounts: SplDeposit,
}

struct Layout {
    has_sol: bool,
    spl: Vec<SplGroup>,
}

impl Layout {
    fn new(deposits: &[AssetDeposit]) -> Result<Self, Error> {
        if deposits.is_empty() {
            return Err(Error::Limit("deposit batch is empty"));
        }
        if deposits.len() > usize::from(u8::MAX) {
            return Err(Error::Limit("deposit batch exceeds 255 entries"));
        }
        let mut has_sol = false;
        let mut spl = Vec::new();
        for deposit in deposits {
            match deposit.asset {
                DepositAsset::Sol => has_sol = true,
                DepositAsset::Spl(accounts) => push_spl(&mut spl, accounts)?,
            }
        }
        let asset_count = usize::from(has_sol) + spl.len();
        if asset_count > MAX_DEPOSIT_ASSETS {
            return Err(Error::Limit("deposit batch exceeds five assets"));
        }
        Ok(Self { has_sol, spl })
    }

    fn asset_index(&self, asset: DepositAsset) -> Result<u8, Error> {
        let index = match asset {
            DepositAsset::Sol => 0,
            DepositAsset::Spl(accounts) => {
                let position = self
                    .spl
                    .iter()
                    .position(|group| group.accounts.mint == accounts.mint)
                    .ok_or(Error::SettlementMismatch)?;
                usize::from(self.has_sol) + position
            }
        };
        u8::try_from(index).map_err(|_| Error::Limit("deposit asset index"))
    }
}

fn push_spl(groups: &mut Vec<SplGroup>, accounts: SplDeposit) -> Result<(), Error> {
    if let Some(existing) = groups
        .iter()
        .find(|group| group.accounts.mint == accounts.mint)
    {
        if existing.accounts.user_token != accounts.user_token
            || existing.accounts.token_program != accounts.token_program
            || existing.accounts.interface != accounts.interface
            || existing.accounts.interface_bump != accounts.interface_bump
        {
            return Err(Error::SettlementMismatch);
        }
        return Ok(());
    }
    groups.push(SplGroup { accounts });
    Ok(())
}

impl Deposit {
    /// Builds the shielded-pool `deposit` instruction. Nothing is signed or sent.
    ///
    /// # Errors
    /// Rejects an empty batch, more than five assets, a memo longer than `u16`,
    /// or SPL entries for one mint that name different token accounts.
    pub fn instruction(&self) -> Result<Instruction, Error> {
        let layout = Layout::new(&self.deposits)?;
        let mut writer = Writer::new();
        writer.u8(DEPOSIT_TAG);
        writer.count(usize::from(layout.has_sol) + layout.spl.len())?;
        if layout.has_sol {
            writer.u8(0);
        }
        for group in &layout.spl {
            writer.u8(1);
            writer.u8(group.accounts.interface_bump);
        }
        writer.count(self.deposits.len())?;
        for deposit in &self.deposits {
            if deposit.amount == 0 {
                return Err(Error::ZeroAmount);
            }
            writer.u8(layout.asset_index(deposit.asset)?);
            writer.raw(&deposit.view_tag);
            writer.raw(&deposit.owner);
            writer.u64(deposit.amount);
            match &deposit.memo {
                None => writer.u8(0),
                Some(memo) => {
                    writer.u8(1);
                    writer.bytes_u16(memo)?;
                }
            }
        }

        let mut accounts = vec![
            meta(self.tree, AccountRole::Writable),
            meta(self.depositor, AccountRole::WritableSigner),
            meta(SHIELDED_POOL_PROGRAM, AccountRole::Readonly),
        ];
        if layout.has_sol {
            accounts.push(meta(
                solana_kit_core::address_constants::SYSTEM_PROGRAM,
                AccountRole::Readonly,
            ));
            accounts.push(meta(SOL_INTERFACE, AccountRole::Writable));
        }
        for group in &layout.spl {
            let spl = group.accounts;
            accounts.extend([
                meta(spl.token_program, AccountRole::Readonly),
                meta(spl.mint, AccountRole::Readonly),
                meta(spl.user_token, AccountRole::Writable),
                meta(spl.interface, AccountRole::Writable),
            ]);
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
