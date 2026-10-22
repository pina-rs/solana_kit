//! Accepted portable Memo and Associated Token Account cases.

use solana_kit_associated_token_account::{
    AssociatedTokenInstruction, AssociatedTokenSeeds, CreateAccounts, SYSTEM_PROGRAM, TOKEN_PROGRAM,
};
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_memo::{add_memo, parse_instruction};

const MEMO_LIMITS: Limits = Limits::new(32, 32);
const CREATE_LIMITS: Limits = Limits::new(1, 6);

/// Memo and ATA cases follow the preserved first 81 case groups.
pub(crate) fn cases() -> [crate::CaseResult; 2] {
    [
        crate::case("memo-utf8-signers", memo()),
        crate::case("associated-token-metadata-pda", associated_token()),
    ]
}

/// Executes raw UTF-8 and signer parsing without any host capability.
pub fn memo() -> Result<(), &'static str> {
    let signer = Address::from_bytes([7; 32]);
    let limits = MEMO_LIMITS;
    let instruction = add_memo("A\0語", &[signer], limits).map_err(|_| "memo build")?;

    if instruction.data != [65, 0, 232, 170, 158] {
        return Err("memo literal bytes");
    }

    let parsed = parse_instruction(&instruction, limits).map_err(|_| "memo parse")?;

    if parsed.memo != "A\0語" || parsed.signers != [signer] {
        return Err("memo payload or signer");
    }

    Ok(())
}

/// Executes ATA construction, strict parsing and an independent PDA literal.
pub fn associated_token() -> Result<(), &'static str> {
    let seeds = AssociatedTokenSeeds {
        owner: "2ojv9BAiHUrvsm9gxDe7fJSzbNZSJcxZvf8dqmWGHG8S"
            .parse()
            .map_err(|_| "owner")?,
        mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
            .parse()
            .map_err(|_| "mint")?,
        token_program: TOKEN_PROGRAM,
    };
    let derived = seeds.find().map_err(|_| "PDA derivation")?;
    let expected: Address = "9KaA7vEBUdRCcBWxfuMjxYwKfvu8Us3Cg5gkhVFt2LNk"
        .parse()
        .map_err(|_| "PDA literal")?;

    if derived.address() != expected || derived.bump() != 254 {
        return Err("PDA literal mismatch");
    }

    let value = AssociatedTokenInstruction::CreateIdempotent(CreateAccounts {
        payer: seeds.owner,
        associated_token: expected,
        owner: seeds.owner,
        mint: seeds.mint,
        system_program: SYSTEM_PROGRAM,
        token_program: seeds.token_program,
    });
    let instruction = value.instruction(CREATE_LIMITS).map_err(|_| "ATA build")?;
    let parsed =
        solana_kit_associated_token_account::parse_instruction(&instruction, CREATE_LIMITS)
            .map_err(|_| "ATA parse")?;

    if instruction.data != [1] || parsed != value {
        return Err("ATA bytes or parsed accounts");
    }

    Ok(())
}
