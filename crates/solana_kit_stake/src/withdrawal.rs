//! Client withdrawal limits adapted from Stake program b37e4f76 (Apache-2.0).

use crate::{Delegation, StakeStateAccount, StakeStateAccountCodec, StakeStateV2};
use solana_kit_core::{
    Address,
    accounts::Account,
    codecs::{Encoder, Error},
    sysvars::{Clock, StakeHistoryEntry},
};

/// Proposed transaction signer identities; these values do not prove signatures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WithdrawalSigners {
    /// Withdrawal authority, or the account itself for an uninitialized account.
    pub withdrawer: Address,
    /// Optional lockup custodian that the proposed transaction will also require to sign.
    pub custodian: Option<Address>,
}

/// Caller-supplied observations and the rules of the deployed Stake program.
#[derive(Clone, Copy, Debug)]
pub struct WithdrawalContext<'a> {
    /// Clock from the same bank as the account and history.
    pub clock: &'a Clock,
    /// Complete newest-first history; the activation API defines missing-history fallback.
    pub history: &'a [StakeHistoryEntry],
    /// Required reserve under the deployed program, excluding delegated stake.
    /// Current programs calculate this from live Rent and the actual data allocation.
    pub rent_exempt_reserve: u64,
    /// First reduced-rate epoch for the deployed program; current integer rules use `Some(0)`.
    pub new_rate_activation_epoch: Option<u64>,
}

/// Disjoint valid amounts: a partial range and, when inactive, complete closure.
///
/// A full inactive withdrawal may close an account even when an intermediate amount
/// would leave less than its reserve. These observations do not authenticate RPC
/// data, supply signatures, validate a destination or guarantee later execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WithdrawalLimits {
    /// Largest partial request retaining the required reserve; `None` means no such request.
    pub max_partial_lamports: Option<u64>,
    /// Entire observed balance when closing the account is allowed by its stake state.
    pub close_lamports: Option<u64>,
}

impl WithdrawalLimits {
    /// Checks both valid sets without treating the gap before closure as withdrawable.
    #[must_use]
    pub fn permits(&self, lamports: u64) -> bool {
        self.max_partial_lamports
            .is_some_and(|maximum| lamports <= maximum)
            || self.close_lamports == Some(lamports)
    }
}

/// A rejected account, signer proposal, lockup, history or reserve calculation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WithdrawalError {
    /// Wrong owner, executable account, inconsistent allocation or unsupported state.
    InvalidAccount,
    /// The proposed withdrawal signer does not control this account.
    AuthorityMismatch,
    /// Epoch or timestamp lockup remains active without its proposed custodian signer.
    LockupInForce,
    /// Adding exposed stake to the required reserve overflowed a lamport amount.
    ReserveOverflow,
    /// The activation calculation rejected the supplied history or delegation.
    History(Error),
}

impl core::fmt::Display for WithdrawalError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "stake withdrawal assessment failed: {self:?}")
    }
}

impl core::error::Error for WithdrawalError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::History(error) => Some(error),
            _ => None,
        }
    }
}

/// Assesses the proposed signers and amount ranges against an observed account.
///
/// Until deactivation, all delegated stake remains reserved, including warming stake.
/// During cooldown, only effective stake remains reserved. Initialized and inactive
/// accounts may close completely; partial withdrawals must retain the supplied reserve.
/// Both lockup boundaries must have passed unless the matching custodian will sign.
///
/// The caller selects the deployed program's rent and rate rules and supplies coherent
/// state. Malformed metadata, insufficient authority, lockup, reserve overflow and invalid
/// activation inputs are errors; none is converted to a successful zero amount.
pub fn assess_withdrawal(
    account: &Account<StakeStateAccount>,
    signers: WithdrawalSigners,
    context: WithdrawalContext<'_>,
) -> Result<WithdrawalLimits, WithdrawalError> {
    validate_account(account)?;

    let (meta, delegation) = match &account.data.state {
        StakeStateV2::Stake(value) => (&value.item0, Some(&value.item1.delegation)),
        StakeStateV2::Initialized(meta) => (meta, None),
        StakeStateV2::Uninitialized if account.address == signers.withdrawer => {
            return Ok(limits(account.lamports, 0, false));
        }
        StakeStateV2::Uninitialized => return Err(WithdrawalError::AuthorityMismatch),
        StakeStateV2::RewardsPool => return Err(WithdrawalError::InvalidAccount),
    };

    if meta.authorized.withdrawer != signers.withdrawer {
        return Err(WithdrawalError::AuthorityMismatch);
    }

    let lockup = &meta.lockup;
    let locked = lockup.epoch.value > context.clock.epoch
        || lockup.unix_timestamp.value > context.clock.unix_timestamp;

    if locked && signers.custodian != Some(lockup.custodian) {
        return Err(WithdrawalError::LockupInForce);
    }

    let exposed = match delegation {
        Some(delegation) => exposed_stake(delegation, context)?,
        None => 0,
    };
    let reserve = exposed
        .checked_add(context.rent_exempt_reserve)
        .ok_or(WithdrawalError::ReserveOverflow)?;
    Ok(limits(account.lamports, reserve, exposed != 0))
}

/// Warming stake remains reserved; only cooldown may reduce the exposed amount.
fn exposed_stake(
    delegation: &Delegation,
    context: WithdrawalContext<'_>,
) -> Result<u64, WithdrawalError> {
    let status = delegation
        .activation_status_v2(
            context.clock.epoch,
            context.history,
            context.new_rate_activation_epoch,
        )
        .map_err(WithdrawalError::History)?;

    if context.clock.epoch < delegation.deactivation_epoch.value {
        return Ok(delegation.stake);
    }

    Ok(status.effective)
}

/// Checks public account values without assuming they originated in a trusted decoder.
fn validate_account(account: &Account<StakeStateAccount>) -> Result<(), WithdrawalError> {
    if account.owner != crate::PROGRAM || account.executable {
        return Err(WithdrawalError::InvalidAccount);
    }

    if account.space == 0 && matches!(account.data.state, StakeStateV2::Uninitialized) {
        return Ok(());
    }

    let required = StakeStateAccountCodec
        .encoded_len(&account.data)
        .map_err(|_| WithdrawalError::InvalidAccount)?;

    if u64::try_from(required).map_or(true, |required| required > account.space) {
        return Err(WithdrawalError::InvalidAccount);
    }

    Ok(())
}

/// Retains the full-close exception independently of the reserve-preserving range.
fn limits(balance: u64, reserve: u64, staked: bool) -> WithdrawalLimits {
    WithdrawalLimits {
        max_partial_lamports: balance.checked_sub(reserve.max(1)),
        close_lamports: (!staked).then_some(balance),
    }
}
