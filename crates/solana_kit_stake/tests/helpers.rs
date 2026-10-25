use solana_kit_core::{
    Address,
    accounts::{Account, AccountError},
    codecs::{Encoder, Limits},
    plans::{InstructionLeaf, Plan},
};
use solana_kit_stake::*;
const LIMITS: Limits = Limits::new(4096, 4096);
const ALLOCATION: usize = 200;
const RESERVED_BYTE: u8 = 91;

#[cfg_attr(test, test)]
pub fn creation_defaults_and_atomic_plan_match_source() {
    let payer = Address::from_bytes([1; 32]);
    let stake = Address::from_bytes([2; 32]);
    let authorized = Authorized {
        staker: payer,
        withdrawer: stake,
    };
    let plan = create_stake_account_plan(
        payer,
        stake,
        u64::MAX,
        authorized.clone(),
        // The source supplies a zero lockup when no override is present.
        None,
        LIMITS,
    )
    .unwrap();

    assert!(matches!(&plan,Plan::Sequential { divisible:false,plans } if plans.len()==2));
    let leaves = plan.flatten();
    let InstructionLeaf::Instruction(create) = leaves[0] else {
        panic!("concrete create")
    };

    let (data, accounts) = solana_kit_system::CreateAccount::parse(create, LIMITS).unwrap();
    assert_eq!(data.lamports, u64::MAX);
    assert_eq!(data.space, 200);
    assert_eq!(data.program_address, PROGRAM);
    assert_eq!(accounts.payer, payer);
    assert_eq!(accounts.new_account, stake);

    let InstructionLeaf::Instruction(initialize) = leaves[1] else {
        panic!("concrete initialize")
    };
    let initialized = Initialize::parse_instruction(initialize, PROGRAM, LIMITS).unwrap();
    assert_eq!(initialized.arg0, authorized);
    assert_eq!(initialized.arg1.unix_timestamp.value, 0);
    assert_eq!(initialized.arg1.epoch.value, 0);
    assert_eq!(
        initialized.arg1.custodian,
        solana_kit_system::SYSTEM_PROGRAM
    );
    assert_eq!(
        initialize.accounts[1].address,
        solana_kit_core::sysvars::RENT_ADDRESS
    );
}

#[cfg_attr(test, test)]
pub fn delegation_defaults_and_account_metadata_remain_exact() {
    assert_eq!(
        STAKE_CONFIG.to_string(),
        "StakeConfig11111111111111111111111111111111"
    );
    let address = Address::from_bytes([7; 32]);
    let Plan::Single(InstructionLeaf::Instruction(instruction)) =
        delegate_stake_plan(address, address, address, LIMITS).unwrap()
    else {
        panic!("single delegation")
    };
    assert_eq!(instruction.data, [2, 0, 0, 0]);
    assert_eq!(
        instruction.accounts[2].address,
        solana_kit_core::sysvars::CLOCK_ADDRESS
    );

    assert_eq!(
        instruction.accounts[3].address,
        solana_kit_core::sysvars::STAKE_HISTORY_ADDRESS
    );
    assert_eq!(instruction.accounts[4].address, STAKE_CONFIG);
    assert!(instruction.accounts[5].role.is_signer());
}

#[cfg_attr(test, test)]
pub fn fetched_stake_preserves_metadata_padding_and_original_errors() {
    let address = Address::from_bytes([7; 32]);
    let value = StakeStateAccount {
        state: StakeStateV2::Uninitialized,
    };
    let mut data = StakeStateAccountCodec.encode(&value, ALLOCATION).unwrap();
    data.resize(ALLOCATION, RESERVED_BYTE);
    let account = Account {
        address,
        owner: PROGRAM,
        lamports: u64::MAX,
        executable: false,
        space: u64::try_from(ALLOCATION).unwrap(),
        rent_epoch: None,
        data,
    };
    let decoded = decode_account(account.clone(), LIMITS).unwrap();
    assert_metadata(&decoded, &account);
    assert_eq!(decoded.data, value);

    let mut foreign = account.clone();
    foreign.owner = address;
    assert!(matches!(
        decode_account(foreign, LIMITS),
        Err(AccountError::OwnerMismatch { .. })
    ));
    let mut wrong_size = account.clone();
    wrong_size.space -= 1;
    assert_eq!(
        decode_account(wrong_size, LIMITS),
        Err(AccountError::DataSizeMismatch(address))
    );

    let mut malformed = account;
    malformed.data[0] = 255;
    assert!(matches!(
        decode_account(malformed, LIMITS),
        Err(AccountError::Decode { .. })
    ));
}

fn assert_metadata<T>(actual: &Account<T>, expected: &Account<Vec<u8>>) {
    assert_eq!(actual.address, expected.address);
    assert_eq!(actual.owner, expected.owner);
    assert_eq!(actual.lamports, expected.lamports);

    assert_eq!(actual.executable, expected.executable);
    assert_eq!(actual.space, expected.space);
    assert_eq!(actual.rent_epoch, expected.rent_epoch);
}
