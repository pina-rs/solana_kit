//! Independent source-byte cases; these do not call a generated encoder to build expectations.
use solana_kit_core::{AccountMeta, AccountRole, Address};
mod wire;
pub use wire::hex;
use wire::{actions, spending};

pub struct Case {
    pub header: &'static str,
    pub payload: Payload,
    pub roles: &'static str,
    pub optional: &'static [usize],
    pub remaining: bool,
}

#[derive(Clone, Copy)]
pub enum Payload {
    Empty,
    Init,
    Address,
    Fee,
    Create,
    Member,
    Remove,
    TimeLock,
    Threshold,
    Memo,
    Rent,
    Spending,
    Actions,
    Vault,
    Buffer,
    Bytes,
    Batch,
    AddTransaction,
    Proposal,
    Use,
}

macro_rules! case {
    ($header:literal, $payload:ident, $roles:literal, $optional:expr, $remaining:literal) => {
        Case {
            header: $header,
            payload: Payload::$payload,
            roles: $roles,
            optional: $optional,
            remaining: $remaining,
        }
    };
}

pub const CASES: &[Case] = &[
    case!("b8bcc6c3cd7c75d8", Init, "w,ws,r", &[], false),
    case!("eef224b5208fd84b", Address, "w,s", &[], false),
    case!("65a0f93f9ad7990d", Fee, "w,s", &[], false),
    case!("6f2ef37590bca26b", Address, "w,s", &[], false),
    case!("7a4d509f54585ac5", Empty, "r", &[], false),
    case!("32ddc75d28f58be9", Create, "r,w,w,s,ws,r", &[], false),
    case!("01dbd76cb8e5d608", Member, "w,s,ws,r", &[2, 3], false),
    case!("d975b1d2b691da48", Remove, "w,s,ws,r", &[2, 3], false),
    case!("949a794dd4fe9b48", TimeLock, "w,s,ws,r", &[2, 3], false),
    case!("8d2a0f7ea95c3eb5", Threshold, "w,s,ws,r", &[2, 3], false),
    case!("8f5dc78f5ca9c1e8", Remove, "w,s,ws,r", &[2, 3], false),
    case!("30cc4139d2469c4a", Rent, "w,s,ws,r", &[2, 3], false),
    case!("0bf29f2a56c55973", Spending, "r,s,w,ws,r", &[], false),
    case!("e4c6886f7b04b271", Memo, "r,s,w,w", &[], false),
    case!("9bec57e4894b5127", Actions, "w,w,s,ws,r", &[], false),
    case!("7292f4bdfc8c2428", Empty, "w,s,w,r,ws,r", &[4, 5], true),
    case!("30fa4ea8d0e2dad3", Vault, "w,w,s,ws,r", &[], false),
    case!("f5c9716c253f1d59", Buffer, "r,w,s,ws,r", &[], false),
    case!("11b6d0e48818b266", Empty, "r,w,s", &[], false),
    case!("e69d433805eef592", Bytes, "r,w,s", &[], false),
    case!("de36954457f630e7", Vault, "w,w,s,ws,r,w,ws", &[], false),
    case!("c208a15799a419ab", Empty, "r,w,r,s", &[], true),
    case!("c28e8d1137b914f8", Batch, "w,w,s,ws,r", &[], false),
    case!(
        "5964e0124546364c",
        AddTransaction,
        "r,r,w,w,s,ws,r",
        &[],
        false
    ),
    case!("ac2cb398157feab4", Empty, "r,s,w,w,r", &[], true),
    case!("dc3c49e01e6c4f9f", Proposal, "r,w,s,ws,r", &[], false),
    case!("0b225cf89a1b336a", Empty, "r,ws,w", &[], false),
    case!("9025a488bcd82af8", Memo, "r,ws,w", &[], false),
    case!("f33e869ce66af687", Memo, "r,ws,w", &[], false),
    case!("1b2a7fed26a354cb", Memo, "r,ws,w", &[], false),
    case!("cd29c23ddc8b10f7", Memo, "r,ws,w,r", &[], false),
    case!(
        "1039827fc1149b86",
        Use,
        "r,s,w,w,w,r,r,w,w,r",
        &[5, 6, 7, 8, 9],
        false
    ),
    case!("50cb54359770bbba", Empty, "r,w,w,w,r", &[], false),
    case!("c447bbb00223aaa5", Empty, "r,w,w,w,r", &[], false),
    case!("8612136a814461f7", Empty, "r,r,w,w,w,r", &[], false),
    case!("dac407af82660bff", Empty, "r,w,w,w,r", &[], false),
];

pub fn accounts(source: &str) -> Vec<AccountMeta> {
    source
        .split(',')
        .zip(1u8..)
        .map(|(role, byte)| AccountMeta {
            address: Address::from_bytes([byte; 32]),
            role: match role {
                "r" => AccountRole::Readonly,
                "w" => AccountRole::Writable,
                "s" => AccountRole::ReadonlySigner,
                "ws" => AccountRole::WritableSigner,
                _ => panic!("unknown fixture role"),
            },
        })
        .collect()
}

pub fn payload(kind: Payload) -> Vec<u8> {
    let memo = || [vec![1, 2, 0, 0, 0], "é".as_bytes().to_vec()].concat();
    let address = || vec![7; 32];
    let amount = || 0x0102_0304_0506_0708u64.to_le_bytes().to_vec();
    let blob = || vec![2, 0, 0, 0, 0xa1, 0xb2];

    match kind {
        Payload::Empty => Vec::new(),
        Payload::Init => [address(), amount(), vec![8; 32]].concat(),
        Payload::Address => address(),
        Payload::Fee => amount(),
        Payload::Create => [
            vec![1],
            address(),
            vec![1, 0, 1, 0, 0, 0],
            vec![8; 32],
            vec![7, 17, 0, 0, 0, 0],
            memo(),
        ]
        .concat(),
        Payload::Member => [vec![8; 32], vec![7], memo()].concat(),
        Payload::Remove => [address(), memo()].concat(),
        Payload::TimeLock => [17u32.to_le_bytes().to_vec(), memo()].concat(),
        Payload::Threshold => [1u16.to_le_bytes().to_vec(), memo()].concat(),
        Payload::Memo => memo(),
        Payload::Rent => [vec![1], address(), memo()].concat(),
        Payload::Spending => [spending(), memo()].concat(),
        Payload::Actions => [actions(), memo()].concat(),
        Payload::Vault => [vec![2, 1], blob(), memo()].concat(),
        Payload::Buffer => [vec![3, 2], vec![7; 32], vec![2, 0], blob()].concat(),
        Payload::Bytes => blob(),
        Payload::Batch => [vec![2], memo()].concat(),
        Payload::AddTransaction => [vec![1], blob()].concat(),
        Payload::Proposal => [amount(), vec![1]].concat(),
        Payload::Use => [amount(), vec![9], memo()].concat(),
    }
}
