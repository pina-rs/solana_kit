use solana_kit_core::transaction::{DurableNonce, Lifetime, Message, Version};
use solana_kit_core::{AccountRole, Address, Hash, Instruction};

const PREFIX: u8 = 5;
const SHARED_PAYER: u8 = 6;
const DUPLICATE_ROLES: u8 = 7;
const PREFIX_BYTE: u8 = 42;
const CHANGED_AUTHORITY: u8 = 5;
const HEX_RADIX: u32 = 16;
const LIFETIME_CLASS_COLUMN: usize = 6;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn metas(instructions: &[Instruction]) -> String {
    instructions
        .iter()
        .map(|instruction| {
            let accounts = instruction
                .accounts
                .iter()
                .map(|meta| {
                    let role = match meta.role {
                        AccountRole::Readonly => 0,
                        AccountRole::Writable => 1,
                        AccountRole::ReadonlySigner => 2,
                        AccountRole::WritableSigner => 3,
                    };

                    format!("{}:{role}", meta.address)
                })
                .collect::<Vec<_>>()
                .join(",");

            format!(
                "{},{accounts},{}",
                instruction.program_address,
                hex(&instruction.data)
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn initial(scenario: u8, nonce: DurableNonce) -> Vec<Instruction> {
    let mut advance = nonce.advance();

    match scenario {
        1 | 2 => vec![advance.instruction()],
        3 => {
            advance.nonce_account = Address::from_bytes([4; 32]);
            vec![advance.instruction()]
        }
        4 => {
            advance.nonce_authority = Address::from_bytes([CHANGED_AUTHORITY; 32]);
            vec![advance.instruction()]
        }
        PREFIX => vec![Instruction {
            program_address: Address::from_bytes([9; 32]),
            accounts: vec![],
            data: vec![PREFIX_BYTE],
        }],
        _ => vec![],
    }
}

fn compile_row(row: &[&str]) -> (Message, DurableNonce) {
    let scenario: u8 = row[1].parse().unwrap();

    let version = match row[0] {
        "legacy" => Version::Legacy,
        "v0" => Version::V0,
        "v1" => Version::V1,
        _ => panic!("unexpected version"),
    };

    let authority = match scenario {
        SHARED_PAYER => Address::from_bytes([1; 32]),
        DUPLICATE_ROLES => Address::from_bytes([2; 32]),
        _ => Address::from_bytes([3; 32]),
    };
    let nonce = DurableNonce::new(
        row[2].parse().unwrap(),
        Address::from_bytes([2; 32]),
        authority,
    );
    let instructions = nonce.prepare_instructions(&initial(scenario, nonce));
    assert_eq!(metas(&instructions), row[4]);
    assert_eq!(nonce.token().to_string(), row[3]);
    assert_eq!(nonce.prepare_instructions(&instructions), instructions);

    let message = Message::compile(
        version,
        Address::from_bytes([1; 32]),
        Lifetime::DurableNonce(nonce),
        &instructions,
        &[],
    )
    .unwrap();

    (message, nonce)
}

fn inspect_row(message: &Message, nonce: DurableNonce, row: &[&str]) {
    let dart_bytes: Vec<_> = row[5]
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(core::str::from_utf8(pair).unwrap(), HEX_RADIX).unwrap())
        .collect();
    assert_eq!(Message::decode(&dart_bytes).unwrap().bytes(), dart_bytes);
    let received = Message::decode(message.bytes()).unwrap();
    assert_eq!(
        received.lifetime(),
        Lifetime::Unresolved(row[3].parse::<Hash>().unwrap())
    );
    assert_eq!(
        received.nonce_advance().unwrap().is_some(),
        row[LIFETIME_CLASS_COLUMN] == "DurableNonceLifetimeConstraint"
    );
    assert_eq!(message.lifetime(), Lifetime::DurableNonce(nonce));
}

#[test]
fn actual_pinned_dart_bytes_and_ordered_metas() {
    let literals = include_str!("../dart-literals.tsv");
    assert_eq!(literals.lines().count(), 30);

    for line in literals.lines() {
        let row: Vec<_> = line.split('\t').collect();
        assert_eq!(row.len(), 7);
        let (message, nonce) = compile_row(&row);

        assert_eq!(
            hex(message.bytes()),
            row[5],
            "{} scenario {}",
            row[0],
            row[1]
        );
        inspect_row(&message, nonce, &row);
    }
}
