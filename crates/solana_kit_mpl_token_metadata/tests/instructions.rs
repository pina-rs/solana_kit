//! Independently maintained literals exercise every concrete codec and public builder.
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_mpl_token_metadata::MplTokenMetadataInstructionData as InstructionData;
use solana_kit_mpl_token_metadata::*;

const BYTE_BUDGET: usize = 8192;
const ITEM_BUDGET: usize = 4096;
const LIMITS: Limits = Limits::new(BYTE_BUDGET, ITEM_BUDGET);
const NUMBER: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

/// Independent wire tags; these do not use the generated enum's discriminants.
#[derive(Clone, Copy)]
#[repr(u8)]
enum LiteralTag {
    CreateMetadataAccount = 0,
    UpdateMetadataAccount = 1,
    DeprecatedCreateMasterEdition = 2,
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken = 3,
    UpdatePrimarySaleHappenedViaToken = 4,
    DeprecatedSetReservationList = 5,
    DeprecatedCreateReservationList = 6,
    SignMetadata = 7,
    DeprecatedMintPrintingTokensViaToken = 8,
    DeprecatedMintPrintingTokens = 9,
    CreateMasterEdition = 10,
    MintNewEditionFromMasterEditionViaToken = 11,
    ConvertMasterEditionV1ToV2 = 12,
    MintNewEditionFromMasterEditionViaVaultProxy = 13,
    PuffMetadata = 14,
    UpdateMetadataAccountV2 = 15,
    CreateMetadataAccountV2 = 16,
    CreateMasterEditionV3 = 17,
    VerifyCollection = 18,
    Utilize = 19,
    ApproveUseAuthority = 20,
    RevokeUseAuthority = 21,
    UnverifyCollection = 22,
    ApproveCollectionAuthority = 23,
    RevokeCollectionAuthority = 24,
    SetAndVerifyCollection = 25,
    FreezeDelegatedAccount = 26,
    ThawDelegatedAccount = 27,
    RemoveCreatorVerification = 28,
    BurnNft = 29,
    VerifySizedCollectionItem = 30,
    UnverifySizedCollectionItem = 31,
    SetAndVerifySizedCollectionItem = 32,
    CreateMetadataAccountV3 = 33,
    SetCollectionSize = 34,
    SetTokenStandard = 35,
    BubblegumSetCollectionSize = 36,
    BurnEditionNft = 37,
    CreateEscrowAccount = 38,
    CloseEscrowAccount = 39,
    TransferOutOfEscrow = 40,
    Burn = 41,
    Create = 42,
    Mint = 43,
    Delegate = 44,
    Revoke = 45,
    Lock = 46,
    Unlock = 47,
    Migrate = 48,
    Transfer = 49,
    Update = 50,
    Use = 51,
    Verify = 52,
    Unverify = 53,
    Collect = 54,
    Print = 55,
    Resize = 56,
    CloseAccounts = 57,
}

fn literal(tag: LiteralTag) -> Vec<u8> {
    use LiteralTag::*;

    let payload = match tag {
        MintNewEditionFromMasterEditionViaToken
        | MintNewEditionFromMasterEditionViaVaultProxy
        | Utilize
        | ApproveUseAuthority
        | SetCollectionSize
        | BubblegumSetCollectionSize
        | TransferOutOfEscrow => NUMBER.to_vec(),
        UpdateMetadataAccountV2 => vec![0, 0, 0, 0],
        CreateMasterEditionV3 => [&[1][..], &NUMBER].concat(),
        CreateMetadataAccountV3 => [data_v2(), vec![1, 0]].concat(),

        Burn | Print => [&[if matches!(tag, Burn) { 0 } else { 1 }][..], &NUMBER].concat(),
        Create => create_args(),
        Mint | Transfer => [&[0][..], &NUMBER, &[0]].concat(),
        Delegate | Lock | Unlock | Use => vec![0, 0],
        Revoke => vec![14],
        Update => vec![4, 0, 0],
        Verify | Unverify => vec![1],
        _ => Vec::new(),
    };

    [vec![tag as u8], payload].concat()
}

fn data_v2() -> Vec<u8> {
    vec![
        1, 0, 0, 0, b'N', 1, 0, 0, 0, b'S', 1, 0, 0, 0, b'U', 1, 2, 0, 0, 0,
    ]
}

fn create_args() -> Vec<u8> {
    let data = vec![
        1, 0, 0, 0, b'N', 1, 0, 0, 0, b'S', 1, 0, 0, 0, b'U', 1, 2, 0,
    ];
    [
        vec![0],
        data,
        vec![1, 1, 4, 0, 0, 0, 0, 1, 9, 1, 1],
        NUMBER.to_vec(),
    ]
    .concat()
}

fn roles(tag: u8) -> &'static str {
    let line = include_str!("fixtures/roles.txt")
        .lines()
        .nth(usize::from(tag))
        .unwrap();
    let (source_tag, roles) = line.split_once(' ').unwrap();
    assert_eq!(source_tag.parse::<u8>().unwrap(), tag);

    roles
}

fn account_role(flag: u8) -> AccountRole {
    match flag.to_ascii_lowercase() {
        b'r' => AccountRole::Readonly,
        b'w' => AccountRole::Writable,
        b's' => AccountRole::ReadonlySigner,
        b'x' => AccountRole::WritableSigner,
        _ => panic!("invalid literal role"),
    }
}

fn accounts(tag: u8) -> Vec<AccountMeta> {
    roles(tag)
        .bytes()
        .enumerate()
        .map(|(index, flag)| AccountMeta {
            address: Address::from_bytes([u8::try_from(index + 1).unwrap(); 32]),
            role: account_role(flag),
        })
        .collect()
}

fn verify(
    tag: LiteralTag,
    build: impl Fn(&InstructionData, &[AccountMeta], Option<Address>) -> Instruction,
) {
    let wire_tag = tag as u8;
    let bytes = literal(tag);
    let value = MplTokenMetadataInstructionDataCodec
        .decode(&bytes, LIMITS)
        .unwrap();
    let accounts = accounts(wire_tag);
    let instruction = build(&value, &accounts, None);
    assert_eq!(instruction.program_address, PROGRAM);
    assert_eq!(instruction.accounts, accounts);
    assert_eq!(instruction.data, bytes);

    assert_eq!(parse_instruction(&instruction, LIMITS).unwrap(), value);

    assert_eq!(identify_instruction(&bytes).unwrap(), wire_tag);
    assert_eq!(
        MplTokenMetadataInstructionDataCodec
            .encode(&value, BYTE_BUDGET)
            .unwrap(),
        bytes
    );
    let program = Address::from_bytes([97; 32]);
    let explicit = build(&value, &accounts, Some(program));
    assert_eq!(explicit.program_address, program);
    assert_eq!(explicit.data, bytes);
    assert_eq!(
        parse_instruction_with_program(program, &explicit, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction(&explicit, LIMITS).is_err());

    malformed(&instruction);
    privileges(&instruction);
    optional(&instruction, wire_tag);
}

fn malformed(instruction: &Instruction) {
    for end in 0..instruction.data.len() {
        let mut short = instruction.clone();
        short.data.truncate(end);
        assert!(parse_instruction(&short, LIMITS).is_err());
    }

    let mut changed = instruction.clone();
    changed.data.push(0);
    assert!(parse_instruction(&changed, LIMITS).is_err());
    changed = instruction.clone();
    changed.accounts.push(AccountMeta {
        address: Address::from_bytes([71; 32]),
        role: AccountRole::WritableSigner,
    });
    assert!(parse_instruction(&changed, LIMITS).is_err());
    changed = instruction.clone();
    changed.accounts.clear();

    assert!(parse_instruction(&changed, LIMITS).is_err());
    assert!(
        parse_instruction(
            instruction,
            Limits::new(instruction.data.len() - 1, ITEM_BUDGET)
        )
        .is_err()
    );
    assert!(
        parse_instruction(
            instruction,
            Limits::new(BYTE_BUDGET, instruction.accounts.len() - 1)
        )
        .is_err()
    );
}

/// All alternate privileges must reject, including escalations on optional accounts.
fn privileges(instruction: &Instruction) {
    for index in 0..instruction.accounts.len() {
        for role in [
            AccountRole::Readonly,
            AccountRole::Writable,
            AccountRole::ReadonlySigner,
            AccountRole::WritableSigner,
        ] {
            if role == instruction.accounts[index].role {
                continue;
            }

            let mut changed = instruction.clone();
            changed.accounts[index].role = role;
            assert!(parse_instruction(&changed, LIMITS).is_err());
        }
    }
}

fn optional(instruction: &Instruction, tag: u8) {
    let mut changed = instruction.clone();
    let flags = roles(tag);

    if tag < LiteralTag::Burn as u8 {
        let required = flags.bytes().take_while(u8::is_ascii_lowercase).count();

        for count in required..=flags.len() {
            changed.accounts.truncate(count);
            assert!(parse_instruction(&changed, LIMITS).is_ok());
            changed.accounts = instruction.accounts.clone();
        }

        return;
    }

    for (index, flag) in flags
        .bytes()
        .enumerate()
        .filter(|(_, flag)| flag.is_ascii_uppercase())
    {
        changed.accounts[index] = AccountMeta {
            address: PROGRAM,
            role: AccountRole::Readonly,
        };
        assert!(
            parse_instruction(&changed, LIMITS).is_ok(),
            "tag{tag} role{index} {flag}"
        );
        changed.accounts[index].role = AccountRole::Writable;
        assert!(parse_instruction(&changed, LIMITS).is_err());
        changed.accounts[index].role = AccountRole::Readonly;
    }
}

macro_rules! builder {
    ($variant:ident) => {
        |value, accounts, program| {
            let InstructionData::$variant(data) = value else {
                panic!("wrong literal dispatch");
            };

            match program {
                None => data.instruction(accounts, LIMITS).unwrap(),
                Some(program) => data
                    .instruction_with_program(program, accounts, LIMITS)
                    .unwrap(),
            }
        }
    };
}

macro_rules! case {
    ($function:ident, $variant:ident) => {
        #[cfg_attr(test, test)]
        pub fn $function() {
            const EXPECTED: LiteralTag = LiteralTag::$variant;
            verify(EXPECTED, builder!($variant));
        }
    };
}

case!(create_metadata_account, CreateMetadataAccount);
case!(update_metadata_account, UpdateMetadataAccount);
case!(
    deprecated_create_master_edition,
    DeprecatedCreateMasterEdition
);
case!(
    deprecated_mint_edition,
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken
);
case!(update_primary_sale, UpdatePrimarySaleHappenedViaToken);
case!(
    deprecated_set_reservation_list,
    DeprecatedSetReservationList
);
case!(
    deprecated_create_reservation_list,
    DeprecatedCreateReservationList
);
case!(sign_metadata, SignMetadata);

case!(
    deprecated_mint_printing_via_token,
    DeprecatedMintPrintingTokensViaToken
);
case!(deprecated_mint_printing, DeprecatedMintPrintingTokens);
case!(create_master_edition, CreateMasterEdition);
case!(
    mint_edition_via_token,
    MintNewEditionFromMasterEditionViaToken
);
case!(convert_master_edition, ConvertMasterEditionV1ToV2);
case!(
    mint_edition_via_vault,
    MintNewEditionFromMasterEditionViaVaultProxy
);
case!(puff_metadata, PuffMetadata);
case!(update_metadata_v2, UpdateMetadataAccountV2);

case!(create_metadata_v2, CreateMetadataAccountV2);
case!(create_master_edition_v3, CreateMasterEditionV3);
case!(verify_collection, VerifyCollection);
case!(utilize, Utilize);
case!(approve_use_authority, ApproveUseAuthority);
case!(revoke_use_authority, RevokeUseAuthority);
case!(unverify_collection, UnverifyCollection);
case!(approve_collection_authority, ApproveCollectionAuthority);

case!(revoke_collection_authority, RevokeCollectionAuthority);
case!(set_and_verify_collection, SetAndVerifyCollection);
case!(freeze_delegated_account, FreezeDelegatedAccount);
case!(thaw_delegated_account, ThawDelegatedAccount);
case!(remove_creator_verification, RemoveCreatorVerification);
case!(burn_nft, BurnNft);
case!(verify_sized_collection, VerifySizedCollectionItem);
case!(unverify_sized_collection, UnverifySizedCollectionItem);

case!(
    set_and_verify_sized_collection,
    SetAndVerifySizedCollectionItem
);
case!(create_metadata_v3, CreateMetadataAccountV3);
case!(set_collection_size, SetCollectionSize);
case!(set_token_standard, SetTokenStandard);
case!(bubblegum_collection_size, BubblegumSetCollectionSize);
case!(burn_edition_nft, BurnEditionNft);
case!(create_escrow_account, CreateEscrowAccount);
case!(close_escrow_account, CloseEscrowAccount);

case!(transfer_out_of_escrow, TransferOutOfEscrow);
case!(burn, Burn);
case!(create, Create);
case!(mint, Mint);
case!(delegate, Delegate);
case!(revoke, Revoke);
case!(lock, Lock);
case!(unlock, Unlock);

case!(migrate, Migrate);
case!(transfer, Transfer);
case!(update, Update);
case!(use_asset, Use);
case!(verify_asset, Verify);
case!(unverify_asset, Unverify);
case!(collect, Collect);
case!(print, Print);
case!(resize, Resize);
case!(close_accounts, CloseAccounts);
