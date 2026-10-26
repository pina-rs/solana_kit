//! Every public error variant and exact MIT source message has a literal independent expectation.
use solana_kit_squads::SquadsMultisigError as E;

const ERRORS: &[E] = &[
    E::DuplicateMember,
    E::EmptyMembers,
    E::TooManyMembers,
    E::InvalidThreshold,
    E::Unauthorized,
    E::NotAMember,
    E::InvalidTransactionMessage,
    E::StaleProposal,
    E::InvalidProposalStatus,
    E::InvalidTransactionIndex,
    E::AlreadyApproved,
    E::AlreadyRejected,
    E::AlreadyCancelled,
    E::InvalidNumberOfAccounts,
    E::InvalidAccount,
    E::RemoveLastMember,
    E::NoVoters,
    E::NoProposers,
    E::NoExecutors,
    E::InvalidStaleTransactionIndex,
    E::NotSupportedForControlled,
    E::TimeLockNotReleased,
    E::NoActions,
    E::MissingAccount,
    E::InvalidMint,
    E::InvalidDestination,
    E::SpendingLimitExceeded,
    E::DecimalsMismatch,
    E::UnknownPermission,
    E::ProtectedAccount,
    E::TimeLockExceedsMaxAllowed,
    E::IllegalAccountOwner,
    E::RentReclamationDisabled,
    E::InvalidRentCollector,
    E::ProposalForAnotherMultisig,
    E::TransactionForAnotherMultisig,
    E::TransactionNotMatchingProposal,
    E::TransactionNotLastInBatch,
    E::BatchNotEmpty,
    E::SpendingLimitInvalidAmount,
    E::InvalidInstructionArgs,
    E::FinalBufferHashMismatch,
    E::FinalBufferSizeExceeded,
    E::FinalBufferSizeMismatch,
    E::MultisigCreateDeprecated,
];

pub fn verify() {
    let fixtures: Vec<_> = include_str!("../fixtures/errors.tsv").lines().collect();
    assert_eq!(ERRORS.len(), 45);
    assert_eq!(fixtures.len(), 45);

    for ((code, expected), fixture) in (6000u32..).zip(ERRORS).zip(fixtures) {
        let (literal_code, message) = fixture.split_once('\t').unwrap();
        assert_eq!(literal_code.parse::<u32>().unwrap(), code);
        assert_eq!(E::from_code(code), *expected);
        assert_eq!(expected.code(), code);
        assert_eq!(expected.message(), Some(message));
    }

    for code in [0, 5999, 6045, u32::MAX] {
        assert_eq!(E::from_code(code), E::Unknown(code));
        assert_eq!(E::from_code(code).code(), code);
        assert_eq!(E::from_code(code).message(), None);
    }
}
