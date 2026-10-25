use solana_kit_core::{
    Address,
    accounts::{Account, AccountError},
    codecs::{Decoder, Encoder, Error, Limits},
    plans::{InstructionLeaf, Plan},
};
use solana_kit_loader::*;
const LIMITS: Limits = Limits::new(4096, 4096);
const WRITE_HEADER_BYTES: usize = 16;

fn deployment() -> v3::DeployWithMaxDataLenAccounts {
    let address = Address::from_bytes([7; 32]);
    v3::DeployWithMaxDataLenAccounts {
        payer_account: address,
        program_data_account: address,
        program_account: address,
        buffer_account: address,
        rent_sysvar: solana_kit_core::sysvars::RENT_ADDRESS,
        clock_sysvar: solana_kit_core::sysvars::CLOCK_ADDRESS,
        system_program: Address::from_bytes([0; 32]),
        authority: address,
    }
}

#[cfg_attr(test, test)]
pub fn deploy_chunks_and_capacity_defaults_match_source_order() {
    let plan = deployment()
        .plan(&[10, 11, 12, 13, 14], None, 2, LIMITS)
        .unwrap();

    assert!(matches!(&plan,Plan::Sequential {divisible:true,plans} if plans.len()==4));
    let leaves = plan.flatten();

    for (index, expected) in [[10, 11].as_slice(), [12, 13].as_slice(), [14].as_slice()]
        .into_iter()
        .enumerate()
    {
        let InstructionLeaf::Instruction(instruction) = leaves[index] else {
            panic!("write")
        };
        let write = v3::Write::parse_instruction(instruction, v3::PROGRAM, LIMITS).unwrap();

        assert_eq!(write.offset, u32::try_from(index * 2).unwrap());
        assert_eq!(write.bytes, expected);
    }

    let InstructionLeaf::Instruction(last) = leaves[3] else {
        panic!("deploy")
    };
    assert_eq!(
        v3::DeployWithMaxDataLen::parse_instruction(last, v3::PROGRAM, LIMITS)
            .unwrap()
            .max_data_len,
        5
    );
    let empty = deployment()
        .plan(&[], Some(u64::MAX), DEFAULT_WRITE_CHUNK_SIZE, LIMITS)
        .unwrap();
    assert_eq!(empty.flatten().len(), 1);
    assert_write_limits();
}

fn assert_write_limits() {
    assert_eq!(
        deployment().plan(&[], None, 0, LIMITS),
        Err(Error::InvalidValue)
    );
    assert_eq!(
        deployment().plan(&[1, 2], None, 1, Limits::new(4096, 2)),
        Err(Error::LimitExceeded)
    );

    let one_chunk = &[1, 2, 3];
    let header_only = Limits::new(WRITE_HEADER_BYTES, LIMITS.max_items);
    assert_eq!(
        deployment().plan(one_chunk, None, one_chunk.len(), header_only),
        Err(Error::LimitExceeded)
    );
}

#[cfg_attr(test, test)]
pub fn upgrade_follows_all_writes_and_preserves_spill_identity() {
    let address = Address::from_bytes([8; 32]);
    let spill = Address::from_bytes([9; 32]);
    let accounts = v3::UpgradeAccounts {
        program_data_account: address,
        program_account: address,
        buffer_account: address,
        spill_account: spill,
        rent_sysvar: solana_kit_core::sysvars::RENT_ADDRESS,
        clock_sysvar: solana_kit_core::sysvars::CLOCK_ADDRESS,
        authority: address,
    };
    let plan = accounts.plan(&[1, 2, 3], 2, LIMITS).unwrap();
    let leaves = plan.flatten();
    assert_eq!(leaves.len(), 3);
    let InstructionLeaf::Instruction(last) = leaves[2] else {
        panic!("upgrade")
    };

    assert_eq!(last.data, [3, 0, 0, 0]);
    assert_eq!(last.accounts[3].address, spill);
}

#[cfg_attr(test, test)]
pub fn fetched_loader_retains_metadata_and_validates_owner_size_and_header() {
    let address = Address::from_bytes([7; 32]);
    let value = UpgradeableState::ProgramData {
        slot: u64::MAX,
        authority: Some(address),
    };
    let mut data = UpgradeableStateCodec
        .encode(&value, LIMITS.max_bytes)
        .unwrap();
    data.extend_from_slice(&[11, 12, 13]);
    let account = Account {
        address,
        owner: v3::PROGRAM,
        lamports: u64::MAX,
        executable: true,
        space: 48,
        rent_epoch: Some(9),
        data,
    };
    let decoded = decode_upgradeable_account(account.clone(), LIMITS).unwrap();
    assert_metadata(&decoded, &account);
    assert_eq!(decoded.data, value);

    let mut foreign = account.clone();
    foreign.owner = address;
    assert!(matches!(
        decode_upgradeable_account(foreign, LIMITS),
        Err(AccountError::OwnerMismatch { .. })
    ));
    let mut wrong_size = account.clone();
    wrong_size.space -= 1;
    assert_eq!(
        decode_upgradeable_account(wrong_size, LIMITS),
        Err(AccountError::DataSizeMismatch(address))
    );

    let mut malformed = account;
    malformed.data[0] = 255;
    assert!(matches!(
        decode_upgradeable_account(malformed, LIMITS),
        Err(AccountError::Decode { .. })
    ));
    assert!(
        UpgradeableStateCodec
            .decode(&[255, 0, 0, 0], LIMITS)
            .is_err()
    );
}

fn assert_metadata<T>(actual: &Account<T>, expected: &Account<Vec<u8>>) {
    assert_eq!(actual.address, expected.address);
    assert_eq!(actual.owner, expected.owner);
    assert_eq!(actual.lamports, expected.lamports);

    assert_eq!(actual.executable, expected.executable);
    assert_eq!(actual.space, expected.space);
    assert_eq!(actual.rent_epoch, expected.rent_epoch);
}
