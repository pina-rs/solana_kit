//! Kit conversions reject incomplete data and preserve positional privileges.

use crate::require;
use alloc::{string::ToString, vec};
use solana_kit_client::rest::Budgets;
use solana_kit_core::{AccountRole, Address, Hash, transaction::Lifetime};
use solana_kit_jupiter::{
    AccountMetaPayload, BlockhashWithMetadata, InstructionPayload, Integer, LookupTablePayload,
    decode_build,
};

pub(super) fn instructions() -> Result<(), &'static str> {
    let body = br#"{"swapInstruction":{"programId":"11111111111111111111111111111111","accounts":[{"pubkey":"11111111111111111111111111111111","isSigner":true,"isWritable":false},{"pubkey":"11111111111111111111111111111111","isSigner":false,"isWritable":true}],"data":"AQID"}}"#;
    let build =
        decode_build(body, Budgets::default()).map_err(|_| "instruction fixture rejected")?;
    let payload = build
        .swap_instruction
        .ok_or("swap instruction disappeared")?;
    let instruction = payload
        .to_instruction()
        .map_err(|_| "complete instruction conversion failed")?;
    require(
        instruction.program_address == Address::from_bytes([0; 32])
            && instruction.data == [1, 2, 3],
        "instruction program or bytes changed",
    )?;
    require(
        instruction.accounts.len() == 2
            && instruction.accounts[0].address == instruction.accounts[1].address
            && instruction.accounts[0].role == AccountRole::ReadonlySigner
            && instruction.accounts[1].role == AccountRole::Writable,
        "duplicate account positions or privileges changed",
    )?;
    incomplete_instructions(payload)?;

    privilege_combinations()
}

fn incomplete_instructions(payload: InstructionPayload) -> Result<(), &'static str> {
    let variants = [
        InstructionPayload::default(),
        InstructionPayload {
            program_id: Some("invalid".into()),
            ..payload.clone()
        },
        InstructionPayload {
            accounts: None,
            ..payload.clone()
        },
        InstructionPayload {
            data: None,
            ..payload.clone()
        },
        InstructionPayload {
            data: Some("%%%".into()),
            ..payload.clone()
        },
        InstructionPayload {
            accounts: Some(vec![AccountMetaPayload::default()]),
            ..payload
        },
    ];

    for variant in variants {
        require(
            variant.to_instruction().is_err(),
            "incomplete or malformed instruction converted",
        )?;
    }

    Ok(())
}

fn privilege_combinations() -> Result<(), &'static str> {
    let address = Address::from_bytes([0; 32]);

    for (signer, writable, role) in [
        (false, false, AccountRole::Readonly),
        (false, true, AccountRole::Writable),
        (true, false, AccountRole::ReadonlySigner),
        (true, true, AccountRole::WritableSigner),
    ] {
        let mut account = AccountMetaPayload {
            pubkey: Some(address.to_string()),
            is_signer: Some(signer),
            is_writable: Some(writable),
        };
        require(
            account.to_account_meta().map(|meta| meta.role).ok() == Some(role),
            "account privilege combination changed",
        )?;
        account.is_signer = None;
        require(
            account.to_account_meta().is_err(),
            "missing signer flag defaulted",
        )?;
        account.is_signer = Some(signer);
        account.is_writable = None;
        require(
            account.to_account_meta().is_err(),
            "missing writable flag defaulted",
        )?;
    }

    Ok(())
}

pub(super) fn metadata() -> Result<(), &'static str> {
    let first = Address::from_bytes([1; 32]);
    let second = Address::from_bytes([2; 32]);
    let mut payload = LookupTablePayload {
        address: first.to_string(),
        addresses: vec![second.to_string(), first.to_string(), second.to_string()],
    };
    let table = payload
        .to_lookup_table()
        .map_err(|_| "lookup conversion failed")?;
    require(
        table.address == first && table.addresses == [second, first, second],
        "lookup conversion changed indices",
    )?;
    payload.addresses = vec![first.to_string(); 257];
    require(
        payload.to_lookup_table().is_err(),
        "oversized lookup table accepted",
    )?;
    payload.addresses = vec!["invalid".into()];

    require(
        payload.to_lookup_table().is_err(),
        "invalid loaded address accepted",
    )?;
    payload.addresses.clear();
    payload.address = "invalid".into();
    require(
        payload.to_lookup_table().is_err(),
        "invalid table address accepted",
    )?;
    lifetimes()
}

fn lifetimes() -> Result<(), &'static str> {
    let mut metadata = BlockhashWithMetadata {
        blockhash: vec![3; 32],
        last_valid_block_height: Integer::from(u64::MAX),
    };
    require(
        metadata.to_lifetime().ok()
            == Some(Lifetime::Blockhash {
                blockhash: Hash::from_bytes([3; 32]),
                last_valid_block_height: u64::MAX,
            }),
        "blockhash lifetime changed",
    )?;

    for height in ["18446744073709551616", "-1"] {
        metadata.last_valid_block_height = height
            .parse()
            .map_err(|_| "expiry integer fixture rejected")?;
        require(
            metadata.to_lifetime().is_err(),
            "out-of-range lifetime height accepted",
        )?;
    }

    metadata.last_valid_block_height = Integer::from(1);
    metadata.blockhash.pop();
    require(metadata.to_lifetime().is_err(), "short blockhash accepted")?;
    metadata.blockhash = vec![3; 33];
    require(metadata.to_lifetime().is_err(), "long blockhash accepted")
}
