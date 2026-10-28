//! Funding is sequential and stops at the first failure without replaying writes.

use crate::{Surfnet, builders::*, errors::SurfpoolRpcError, types::*};
use solana_kit_client::Transport;
use solana_kit_core::{Address, address_constants::TOKEN_PROGRAM};

impl<T: Transport> Surfnet<T> {
    /// Assigns one account's lamport balance, creating the account if necessary.
    pub async fn fund_sol(&self, address: Address, lamports: u64) -> Result<(), SurfpoolRpcError> {
        self.execute(SetAccount::new(address).with_lamports(lamports))
            .await?;
        Ok(())
    }

    /// Assigns balances in order; dropping the future cancels the active exchange.
    pub async fn fund_sol_many(
        &self,
        accounts: &[SolAccountFunding],
    ) -> Result<(), SurfpoolRpcError> {
        for account in accounts {
            self.fund_sol(account.address, account.lamports).await?;
        }

        Ok(())
    }

    /// Assigns a token balance under the canonical SPL Token program.
    pub async fn fund_token(
        &self,
        owner: Address,
        mint: Address,
        amount: u64,
    ) -> Result<(), SurfpoolRpcError> {
        self.set_token_balance(owner, mint, amount, TOKEN_PROGRAM)
            .await
    }

    /// Assigns one exact token balance under an explicit token program.
    pub async fn set_token_balance(
        &self,
        owner: Address,
        mint: Address,
        amount: u64,
        token_program: Address,
    ) -> Result<(), SurfpoolRpcError> {
        self.execute(
            SetTokenAccount::new(owner, mint)
                .with_amount(amount)
                .with_token_program(token_program),
        )
        .await?;
        Ok(())
    }

    /// Assigns several token balances in source order under an explicit program.
    pub async fn fund_token_many(
        &self,
        owners: &[Address],
        mint: Address,
        amount: u64,
        token_program: Address,
    ) -> Result<(), SurfpoolRpcError> {
        for owner in owners {
            self.set_token_balance(*owner, mint, amount, token_program)
                .await?;
        }

        Ok(())
    }

    /// Applies advanced token mutations with an explicit token program.
    pub async fn set_token_account(
        &self,
        owner: Address,
        mint: Address,
        update: SetTokenAccountUpdate,
        token_program: Address,
    ) -> Result<(), SurfpoolRpcError> {
        self.execute(SetTokenAccount {
            owner,
            mint,
            update,
            token_program: Some(token_program),
        })
        .await?;
        Ok(())
    }
}

/// Derives an associated token address using Kit's optional deterministic crypto.
#[cfg(feature = "crypto")]
pub fn get_ata(
    owner: Address,
    mint: Address,
    token_program: Address,
) -> Result<Address, solana_kit_core::crypto::PdaError> {
    solana_kit_associated_token_account::AssociatedTokenSeeds {
        owner,
        mint,
        token_program,
    }
    .find()
    .map(|derived| derived.address())
}
