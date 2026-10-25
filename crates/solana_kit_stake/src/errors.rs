// Generated from the complete pinned Stake error table.
const ERROR_NO_CREDITS_TO_REDEEM: u32 = 0;
const ERROR_LOCKUP_IN_FORCE: u32 = 1;
const ERROR_ALREADY_DEACTIVATED: u32 = 2;
const ERROR_TOO_SOON_TO_REDELEGATE: u32 = 3;
const ERROR_INSUFFICIENT_STAKE: u32 = 4;
const ERROR_MERGE_TRANSIENT_STAKE: u32 = 5;
const ERROR_MERGE_MISMATCH: u32 = 6;
const ERROR_CUSTODIAN_MISSING: u32 = 7;

const ERROR_CUSTODIAN_SIGNATURE_MISSING: u32 = 8;
const ERROR_INSUFFICIENT_REFERENCE_VOTES: u32 = 9;
const ERROR_VOTE_ADDRESS_MISMATCH: u32 = 10;
const ERROR_MINIMUM_DELINQUENT_EPOCHS_FOR_DEACTIVATION_NOT_MET: u32 = 11;
const ERROR_INSUFFICIENT_DELEGATION: u32 = 12;
const ERROR_REDELEGATE_TRANSIENT_OR_INACTIVE_STAKE: u32 = 13;
const ERROR_REDELEGATE_TO_SAME_VOTE_ACCOUNT: u32 = 14;
const ERROR_REDELEGATED_STAKE_MUST_FULLY_ACTIVATE_BEFORE_DEACTIVATION_IS_PERMITTED: u32 = 15;
const ERROR_EPOCH_REWARDS_ACTIVE: u32 = 16;

/// Known Stake errors with future codes retained losslessly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StakeError {
    /// Pinned custom error 0.
    NoCreditsToRedeem,
    /// Pinned custom error 1.
    LockupInForce,
    /// Pinned custom error 2.
    AlreadyDeactivated,
    /// Pinned custom error 3.
    TooSoonToRedelegate,
    /// Pinned custom error 4.
    InsufficientStake,
    /// Pinned custom error 5.
    MergeTransientStake,
    /// Pinned custom error 6.
    MergeMismatch,
    /// Pinned custom error 7.
    CustodianMissing,
    /// Pinned custom error 8.
    CustodianSignatureMissing,
    /// Pinned custom error 9.
    InsufficientReferenceVotes,
    /// Pinned custom error 10.
    VoteAddressMismatch,
    /// Pinned custom error 11.
    MinimumDelinquentEpochsForDeactivationNotMet,
    /// Pinned custom error 12.
    InsufficientDelegation,
    /// Pinned custom error 13.
    RedelegateTransientOrInactiveStake,
    /// Pinned custom error 14.
    RedelegateToSameVoteAccount,
    /// Pinned custom error 15.
    RedelegatedStakeMustFullyActivateBeforeDeactivationIsPermitted,
    /// Pinned custom error 16.
    EpochRewardsActive,
    /// An unknown custom code.
    Unknown(u32),
}
impl StakeError {
    /// Decodes the exact u32 program code.
    pub const fn from_code(code: u32) -> Self {
        match code {
            ERROR_NO_CREDITS_TO_REDEEM => Self::NoCreditsToRedeem,
            ERROR_LOCKUP_IN_FORCE => Self::LockupInForce,
            ERROR_ALREADY_DEACTIVATED => Self::AlreadyDeactivated,
            ERROR_TOO_SOON_TO_REDELEGATE => Self::TooSoonToRedelegate,
            ERROR_INSUFFICIENT_STAKE => Self::InsufficientStake,
            ERROR_MERGE_TRANSIENT_STAKE => Self::MergeTransientStake,
            ERROR_MERGE_MISMATCH => Self::MergeMismatch,
            ERROR_CUSTODIAN_MISSING => Self::CustodianMissing,
            ERROR_CUSTODIAN_SIGNATURE_MISSING => Self::CustodianSignatureMissing,
            ERROR_INSUFFICIENT_REFERENCE_VOTES => Self::InsufficientReferenceVotes,
            ERROR_VOTE_ADDRESS_MISMATCH => Self::VoteAddressMismatch,
            ERROR_MINIMUM_DELINQUENT_EPOCHS_FOR_DEACTIVATION_NOT_MET => {
                Self::MinimumDelinquentEpochsForDeactivationNotMet
            }
            ERROR_INSUFFICIENT_DELEGATION => Self::InsufficientDelegation,
            ERROR_REDELEGATE_TRANSIENT_OR_INACTIVE_STAKE => {
                Self::RedelegateTransientOrInactiveStake
            }
            ERROR_REDELEGATE_TO_SAME_VOTE_ACCOUNT => Self::RedelegateToSameVoteAccount,
            ERROR_REDELEGATED_STAKE_MUST_FULLY_ACTIVATE_BEFORE_DEACTIVATION_IS_PERMITTED => {
                Self::RedelegatedStakeMustFullyActivateBeforeDeactivationIsPermitted
            }
            ERROR_EPOCH_REWARDS_ACTIVE => Self::EpochRewardsActive,
            _ => Self::Unknown(code),
        }
    }
    /// Recovers the original custom code.
    pub const fn code(self) -> u32 {
        match self {
            Self::NoCreditsToRedeem => ERROR_NO_CREDITS_TO_REDEEM,
            Self::LockupInForce => ERROR_LOCKUP_IN_FORCE,
            Self::AlreadyDeactivated => ERROR_ALREADY_DEACTIVATED,
            Self::TooSoonToRedelegate => ERROR_TOO_SOON_TO_REDELEGATE,
            Self::InsufficientStake => ERROR_INSUFFICIENT_STAKE,
            Self::MergeTransientStake => ERROR_MERGE_TRANSIENT_STAKE,
            Self::MergeMismatch => ERROR_MERGE_MISMATCH,
            Self::CustodianMissing => ERROR_CUSTODIAN_MISSING,
            Self::CustodianSignatureMissing => ERROR_CUSTODIAN_SIGNATURE_MISSING,
            Self::InsufficientReferenceVotes => ERROR_INSUFFICIENT_REFERENCE_VOTES,
            Self::VoteAddressMismatch => ERROR_VOTE_ADDRESS_MISMATCH,
            Self::MinimumDelinquentEpochsForDeactivationNotMet => {
                ERROR_MINIMUM_DELINQUENT_EPOCHS_FOR_DEACTIVATION_NOT_MET
            }
            Self::InsufficientDelegation => ERROR_INSUFFICIENT_DELEGATION,
            Self::RedelegateTransientOrInactiveStake => {
                ERROR_REDELEGATE_TRANSIENT_OR_INACTIVE_STAKE
            }
            Self::RedelegateToSameVoteAccount => ERROR_REDELEGATE_TO_SAME_VOTE_ACCOUNT,
            Self::RedelegatedStakeMustFullyActivateBeforeDeactivationIsPermitted => {
                ERROR_REDELEGATED_STAKE_MUST_FULLY_ACTIVATE_BEFORE_DEACTIVATION_IS_PERMITTED
            }
            Self::EpochRewardsActive => ERROR_EPOCH_REWARDS_ACTIVE,
            Self::Unknown(code) => code,
        }
    }
    /// Returns the exact pinned source message for known codes.
    pub const fn message(self) -> Option<&'static str> {
        match self {
            Self::NoCreditsToRedeem => Some("Not enough credits to redeem"),
            Self::LockupInForce => Some("Lockup has not yet expired"),
            Self::AlreadyDeactivated => Some("Stake already deactivated"),
            Self::TooSoonToRedelegate => Some("One re-delegation permitted per epoch"),
            Self::InsufficientStake => Some("Split amount is more than is staked"),
            Self::MergeTransientStake => {
                Some("Stake account with transient stake cannot be merged")
            }
            Self::MergeMismatch => {
                Some("Stake account merge failed due to different authority, lockups or state")
            }
            Self::CustodianMissing => Some("Custodian address not present"),
            Self::CustodianSignatureMissing => Some("Custodian signature not present"),
            Self::InsufficientReferenceVotes => {
                Some("Insufficient voting activity in the reference vote account")
            }
            Self::VoteAddressMismatch => {
                Some("Stake account is not delegated to the provided vote account")
            }
            Self::MinimumDelinquentEpochsForDeactivationNotMet => Some(
                "Stake account has not been delinquent for the minimum epochs required for deactivation",
            ),
            Self::InsufficientDelegation => Some("Delegation amount is less than the minimum"),
            Self::RedelegateTransientOrInactiveStake => {
                Some("Stake account with transient or inactive stake cannot be redelegated")
            }
            Self::RedelegateToSameVoteAccount => {
                Some("Stake redelegation to the same vote account is not permitted")
            }
            Self::RedelegatedStakeMustFullyActivateBeforeDeactivationIsPermitted => {
                Some("Redelegated stake must be fully activated before deactivation")
            }
            Self::EpochRewardsActive => {
                Some("Stake action is not permitted while the epoch rewards period is active")
            }
            Self::Unknown(_) => None,
        }
    }
    /// Attributes an indexed failure before interpreting its custom code.
    pub fn from_failure(
        failure: &solana_kit_core::program_error::InstructionFailure,
        programs: &[solana_kit_core::Address],
        program: solana_kit_core::Address,
    ) -> Option<Self> {
        if !failure.is_program_error(programs, program, None) {
            return None;
        }

        match failure.error {
            solana_kit_core::program_error::ProgramError::Custom(code) => {
                Some(Self::from_code(code))
            }
            _ => None,
        }
    }
}
