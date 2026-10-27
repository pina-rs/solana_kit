//! Independently derived PDA addresses and complete default receiver metadata.
use super::*;
use alloc::string::ToString;

pub(super) fn run() -> Result<(), &'static str> {
    require(
        config_address(PYTH_SOLANA_RECEIVER_PROGRAM)
            .map_err(|_| "config PDA")?
            .address()
            .to_string()
            == "DaWUKXCyXsnzcvLUyeJRWou8KTn7XtadgTsdhJ6RHS7b",
        "receiver config seed differs",
    )?;

    for (id, expected) in [
        (0, "8hQfT7SVhkCrzUSgBq6u2wYEt1sH3xmofZ5ss3YaydZW"),
        (u8::MAX, "BBDPTkYigxggxLnQZWZWdTmt7T9KHmLdWUthrw4ZCJaT"),
    ] {
        require(
            treasury_address(id, PYTH_SOLANA_RECEIVER_PROGRAM)
                .map_err(|_| "treasury PDA")?
                .address()
                .to_string()
                == expected,
            "receiver treasury seed differs",
        )?;
    }

    for (index, expected) in [
        (0, "C7RmcKdjeFscYSyekkmCjvHcnBQd7qJDkeB9RmRtuB3L"),
        (u32::MAX, "Gjah6v9yf6eLjMbUHkg5ndERrvEf7boCYho9HdEcaLec"),
    ] {
        require(
            guardian_set_address(index, PYTH_WORMHOLE_PROGRAM)
                .map_err(|_| "guardian PDA")?
                .address()
                .to_string()
                == expected,
            "guardian big-endian seed differs",
        )?;
    }

    let atomic = solana_kit_core::crypto::hash(&[b"global:post_update_atomic"]).to_bytes();
    let posted = solana_kit_core::crypto::hash(&[b"global:post_update"]).to_bytes();
    require(
        POST_UPDATE_ATOMIC_DISCRIMINATOR == atomic[..PREFIX_BYTES]
            && POST_UPDATE_DISCRIMINATOR == posted[..PREFIX_BYTES],
        "receiver Anchor domain differs",
    )?;
    defaults()
}

/// The highest-bump derivation produces the entire independently observed default instruction.
fn defaults() -> Result<(), &'static str> {
    let update = update();
    let payer = Address::from_bytes([1; 32]);
    let context = ReceiverAccounts::derive(
        payer,
        Address::from_bytes([7; 32]),
        DEFAULT_TREASURY_ID,
        PYTH_SOLANA_RECEIVER_PROGRAM,
    )
    .map_err(|_| "receiver accounts")?;
    let index = get_guardian_set_index(VAA).map_err(|_| "VAA index")?;
    let guardian =
        guardian_set_address(index, PYTH_WORMHOLE_PROGRAM).map_err(|_| "guardian account")?;
    let atomic = ReceiverInstruction::PostUpdateAtomic {
        accounts: context,
        guardian_set: guardian.address(),
        params: PostUpdateAtomicParams::new(VAA, &update),
    }
    .instruction(LIMITS)
    .map_err(|_| "default atomic")?;
    let posted = ReceiverInstruction::PostUpdate {
        accounts: context,
        encoded_vaa: Address::from_bytes([6; 32]),
        params: PostUpdateParams::new(&update),
    }
    .instruction(LIMITS)
    .map_err(|_| "default posted")?;

    check_defaults(&atomic, &posted, payer)
}

/// Default identities are compared with the independently observed receiver accounts.
fn check_defaults(
    atomic: &solana_kit_core::Instruction,
    posted: &solana_kit_core::Instruction,
    payer: Address,
) -> Result<(), &'static str> {
    require(
        atomic.data == ATOMIC && posted.data == POSTED,
        "default receiver data differs",
    )?;
    require(
        atomic.accounts.len() == ACCOUNT_COUNT && posted.accounts.len() == ACCOUNT_COUNT,
        "default account count differs",
    )?;
    require(
        atomic.accounts[0].address == payer && atomic.accounts[ACCOUNT_COUNT - 1].address == payer,
        "receiver default authority differs",
    )?;
    require(
        atomic.accounts[1].address.to_string() == "5gxPdahvSzcKySxXxPuRXZZ9s6h8hZ88XDVKavWpaQGn",
        "default guardian account differs",
    )?;
    require(
        posted.accounts[1].address == Address::from_bytes([6; 32])
            && posted.accounts[2..] == atomic.accounts[2..],
        "posted receiver accounts differ",
    )
}
