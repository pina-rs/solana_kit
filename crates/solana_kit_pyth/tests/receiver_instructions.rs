//! Exact independently executed Dart instruction vectors, including every account role.
#[cfg(feature = "crypto")]
#[path = "receiver_instructions/pda.rs"]
mod pda;

use solana_kit_core::{
    AccountRole, Address, Instruction,
    codecs::{Error, Limits},
};
use solana_kit_pyth::*;

const VAA: &[u8] = include_bytes!("fixtures/vaa.bin");
const ATOMIC: &[u8] = include_bytes!("fixtures/receiver-atomic.bin");
const ATOMIC_OVERRIDE: &[u8] = include_bytes!("fixtures/receiver-atomic-override.bin");
const POSTED: &[u8] = include_bytes!("fixtures/receiver-posted.bin");
const POSTED_OVERRIDE: &[u8] = include_bytes!("fixtures/receiver-posted-override.bin");
const PROOF_HASH_BYTES: usize = 20;
const FIRST_PROOF: u8 = 0xa1;
const SECOND_PROOF: u8 = 0xa2;
const ACCOUNT_COUNT: usize = 7;
const PREFIX_BYTES: usize = 8;
const MESSAGE: &[u8] = &[9, 8, 7];
const LIMITS: Limits = Limits::new(4096, 32);

fn update() -> MerklePriceUpdate<'static> {
    MerklePriceUpdate {
        message: MESSAGE,
        proof: vec![
            [FIRST_PROOF; PROOF_HASH_BYTES],
            [SECOND_PROOF; PROOF_HASH_BYTES],
        ],
    }
}

fn accounts() -> ReceiverAccounts {
    let mut accounts = ReceiverAccounts::new(
        Address::from_bytes([1; 32]),
        Address::from_bytes([3; 32]),
        Address::from_bytes([4; 32]),
        Address::from_bytes([7; 32]),
    );
    accounts.program_address = Address::from_bytes([8; 32]);
    accounts.write_authority = Address::from_bytes([2; 32]);
    accounts
}

#[test]
fn canonical_program_identities_match_the_independent_reference() {
    assert_eq!(
        PYTH_SOLANA_RECEIVER_PROGRAM.to_string(),
        "rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ"
    );
    assert_eq!(
        PYTH_WORMHOLE_PROGRAM.to_string(),
        "HDwcJBJXjL9FpJ7UBsYBtaDjsBUhuLCUYoz3zr8SWWaQ"
    );
    assert_eq!(
        PYTH_PUSH_ORACLE_PROGRAM.to_string(),
        "pythWSnswVUd12oZpeFP8e9CVaEqJg25g1Vtc2biRsT"
    );
    assert_eq!(SYSTEM_PROGRAM, Address::from_bytes([0; 32]));
}

#[test]
fn both_parameter_encoders_match_complete_borsh_reference_bytes() {
    let update = update();
    let posted = PostUpdateParams::new(&update);
    let atomic = PostUpdateAtomicParams::new(VAA, &update);
    assert_eq!(posted.encode(LIMITS).unwrap(), &POSTED[PREFIX_BYTES..]);
    assert_eq!(atomic.encode(LIMITS).unwrap(), &ATOMIC[PREFIX_BYTES..]);
    assert_eq!(
        (posted.treasury_id, atomic.treasury_id),
        (DEFAULT_TREASURY_ID, DEFAULT_TREASURY_ID)
    );
}

#[test]
fn explicit_program_accounts_authority_and_treasury_match_the_reference() {
    let update = update();
    let params = PostUpdateParams {
        merkle_price_update: &update,
        treasury_id: u8::MAX,
    };
    let posted = ReceiverInstruction::PostUpdate {
        accounts: accounts(),
        encoded_vaa: Address::from_bytes([6; 32]),
        params,
    }
    .instruction(LIMITS)
    .unwrap();
    let params = PostUpdateAtomicParams {
        vaa: VAA,
        merkle_price_update: &update,
        treasury_id: u8::MAX,
    };
    let atomic = ReceiverInstruction::PostUpdateAtomic {
        accounts: accounts(),
        guardian_set: Address::from_bytes([5; 32]),
        params,
    }
    .instruction(LIMITS)
    .unwrap();
    assert_eq!(posted.data, POSTED_OVERRIDE);
    assert_eq!(atomic.data, ATOMIC_OVERRIDE);
    check_override(&posted, Address::from_bytes([6; 32]));
    check_override(&atomic, Address::from_bytes([5; 32]));
}

/// Exact roles retain duplicate positions rather than merging or reordering accounts.
fn check_override(instruction: &Instruction, verification: Address) {
    assert_eq!(instruction.program_address, Address::from_bytes([8; 32]));
    let actual: Vec<_> = instruction
        .accounts
        .iter()
        .map(|meta| (meta.address, meta.role))
        .collect();
    assert_eq!(
        actual,
        [
            (Address::from_bytes([1; 32]), AccountRole::WritableSigner),
            (verification, AccountRole::Readonly),
            (Address::from_bytes([3; 32]), AccountRole::Readonly),
            (Address::from_bytes([4; 32]), AccountRole::Writable),
            (Address::from_bytes([7; 32]), AccountRole::WritableSigner),
            (SYSTEM_PROGRAM, AccountRole::Readonly),
            (Address::from_bytes([2; 32]), AccountRole::ReadonlySigner),
        ]
    );
}

#[test]
fn parameter_and_instruction_limits_apply_before_owned_output() {
    let update = update();
    let params = PostUpdateAtomicParams::new(VAA, &update);
    let instruction = ReceiverInstruction::PostUpdateAtomic {
        accounts: accounts(),
        guardian_set: Address::from_bytes([5; 32]),
        params,
    };
    let data_size = ATOMIC.len() - PREFIX_BYTES;
    let proof_count = update.proof.len();

    for bytes in 0..data_size {
        assert_eq!(
            params.encode(Limits::new(bytes, proof_count)),
            Err(Error::LimitExceeded)
        );
    }

    assert_eq!(
        params.encode(Limits::new(data_size, proof_count)).unwrap(),
        &ATOMIC[PREFIX_BYTES..]
    );
    assert_eq!(
        params.encode(Limits::new(data_size, proof_count - 1)),
        Err(Error::LimitExceeded)
    );

    for items in 0..proof_count + ACCOUNT_COUNT {
        assert_eq!(
            instruction.instruction(Limits::new(ATOMIC.len(), items)),
            Err(Error::LimitExceeded)
        );
    }

    let required = Limits::new(ATOMIC.len(), proof_count + ACCOUNT_COUNT);
    assert_eq!(instruction.instruction(required).unwrap().data, ATOMIC);
    assert_eq!(
        instruction.instruction(Limits::new(ATOMIC.len() - 1, required.max_items)),
        Err(Error::LimitExceeded)
    );
}

#[test]
fn empty_and_opaque_inputs_have_wire_semantics_without_authentication() {
    let empty = MerklePriceUpdate {
        message: &[],
        proof: vec![],
    };
    let posted = PostUpdateParams::new(&empty)
        .encode(Limits::new(9, 0))
        .unwrap();
    let atomic = PostUpdateAtomicParams::new(&[], &empty)
        .encode(Limits::new(13, 0))
        .unwrap();
    assert_eq!(posted, [0; 9]);
    assert_eq!(atomic, [0; 13]);

    let context = ReceiverAccounts::new(
        Address::from_bytes([1; 32]),
        Address::from_bytes([3; 32]),
        Address::from_bytes([4; 32]),
        Address::from_bytes([7; 32]),
    );
    let instruction = ReceiverInstruction::PostUpdateAtomic {
        accounts: context,
        guardian_set: Address::from_bytes([5; 32]),
        params: PostUpdateAtomicParams::new(&[], &empty),
    }
    .instruction(Limits::new(21, ACCOUNT_COUNT))
    .unwrap();
    assert_eq!(
        instruction.accounts[0].address,
        instruction.accounts[6].address
    );
    assert_eq!(instruction.accounts[0].role, AccountRole::WritableSigner);
    assert_eq!(instruction.accounts[6].role, AccountRole::ReadonlySigner);
}

#[test]
fn treasury_selection_uses_one_caller_byte_and_preserves_provider_failures() {
    for expected in 0..=u8::MAX {
        let mut calls = 0;
        let actual = random_treasury_id::<()>(|byte| {
            calls += 1;
            byte[0] = expected;
            Ok(())
        })
        .unwrap();
        assert_eq!((actual, calls), (expected, 1));
    }

    let error = random_treasury_id(|byte| {
        byte[0] = 42;
        Err("caller entropy unavailable")
    });
    assert_eq!(error, Err("caller entropy unavailable"));
}

#[test]
fn borrowed_message_proof_and_vaa_inputs_remain_unchanged() {
    let update = update();
    let before = update.clone();
    let vaa = VAA.to_vec();
    PostUpdateParams::new(&update).encode(LIMITS).unwrap();
    PostUpdateAtomicParams::new(&vaa, &update)
        .encode(LIMITS)
        .unwrap();
    assert_eq!(update, before);
    assert_eq!(vaa, VAA);
}
