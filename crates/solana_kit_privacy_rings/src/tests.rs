//! Instruction goldens, account order, and recorded indexer examples.

use std::cell::RefCell;
use std::future::Ready;

use solana_kit_client::methods::Method;
use solana_kit_client::{Client, Limits, Reply, Request, Transport, TransportError};
use solana_kit_core::address_constants::{SYSTEM_PROGRAM, TOKEN_PROGRAM};
use solana_kit_core::{AccountRole, Address};

use crate::encode::{
    AnonymousRingProof, Bsb22, EddsaProof, Message, TransferLeg, write_confidential, write_transact,
};
use crate::{
    AssetDeposit, CPI_AUTHORITY, CacheAccess, CacheWrite, CacheWriter, Circuit,
    ConfidentialPlaintext, ConfidentialTransfer, DEPOSIT_TAG, Deposit, DepositAsset, Error,
    GetEncryptedUtxosByTags, GetMerkleProofs, GetNonInclusionProofs, GetNullifierQueueElements,
    GetShieldedTransactionsByNullifiers, GetShieldedTransactionsBySignature,
    GetShieldedTransactionsByTags, GetUserRecords, HistoryFilters, Input, MAX_CACHE_WRITES,
    OwnerTag, Proof, SHIELDED_POOL_PROGRAM, SOL_INTERFACE, STATE_TREE, Settlement, ShieldedOutput,
    SlotBounds, SplDeposit, SplDepositAccounts, SplWithdrawalAccounts, TRANSACT_TAG, Transact,
    TransactBody, TreeContext, USER_REGISTRY_PROGRAM, Withdraw,
};

const DEPOSIT_GOLDEN: &str = "01000100070707070707070707070707070707070707070707070707070707070707070709090909090909090909090909090909090909090909090909090909090909092a000000000000000102006869";
const TRANSACT_GOLDEN: &str = "ffffffffffffffff040404040404040404040404040404040404040404040404040404040404040404030303030303030303030303030303030101070000000000000000000101010101010101010101010101010101010101010101010101010101010101010102000008080808080808080808080808080808080808080808080808080808080808080000010203000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b000104000500";
const PLAINTEXT_GOLDEN: &str = "0100000000000000630000000000000002020202020202020202020202020202020202020202020202020202020202020000";
const FIXTURE_5: u8 = 5;
const FIXTURE_6: u8 = 6;
const FIXTURE_7: u8 = 7;
const FIXTURE_8: u8 = 8;
const FIXTURE_9: u8 = 9;
const FIXTURE_12: u8 = 12;
const FIXTURE_13: u8 = 13;
const FIXTURE_20: u8 = 20;
const SECOND_NOTE_OFFSET: usize = 6 + 1 + 32 + 32 + 8 + 1;
const SPL_GROUP_INDEX: usize = 5;
const SPL_TAIL_LEN: usize = 5;
const READ_SLOT: u64 = 5;
const MAINNET_SLOT: u64 = 467_025_494;

fn hex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&text[index..index + 2], 16).expect("hex"))
        .collect()
}

fn address(byte: u8) -> Address {
    Address::from_bytes([byte; 32])
}

fn published(text: &str) -> Address {
    text.parse().expect("published address")
}

fn sample_body() -> TransactBody {
    TransactBody {
        expiry_unix_ts: u64::MAX,
        tx_viewing_pk: [4; 33],
        salt: [3; 16],
        data_hash: None,
        ring_data_hash: None,
        outputs: vec![ShieldedOutput {
            utxo_hash: [1; 32],
            owner_tag: OwnerTag::Account(2),
            data: None,
        }],
        messages: Vec::new(),
        private_tx_hash: [8; 32],
        circuit: Circuit::ConfidentialSigner(1, 2, 3),
        proof: Proof::zeroed(),
        inputs: vec![Input {
            nullifier_hash: [11; 32],
            tree_index: 0,
        }],
        tree_contexts: vec![TreeContext {
            utxo_tree_root_index: 4,
            nullifier_tree_root_index: 5,
        }],
    }
}

#[test]
fn published_addresses_match_the_base58_constants() {
    assert_eq!(
        SHIELDED_POOL_PROGRAM,
        published("ring1GaLCuJnkUMVkX2JeZLTTX17wKsV4fZmPVzXANW")
    );
    assert_eq!(
        USER_REGISTRY_PROGRAM,
        published("regy3WAeRXMYuMYxr5iK3Bf9Arr3PcxGxyzAddbiqDQ")
    );
    assert_eq!(
        SOL_INTERFACE,
        published("CiNmas621VuyofiDSrEPfTXHKrdZuB6jMKECyNiMpLXE")
    );
    assert_eq!(
        CPI_AUTHORITY,
        published("CaM8DbtJ2mSecgaHYo9xeT7VSKq7AnaqEUm5p7e5LW4a")
    );
    assert_eq!(
        STATE_TREE,
        published("777pvSGe5LaBd9nCczvE2FXEauiwtfGJ5XRpjKyniGor")
    );
}

#[test]
fn deposit_blinding_matches_the_pinned_vector() {
    assert_eq!(
        crate::deposit_blinding(&[1; 32], 7),
        [
            0x00, 0x4d, 0x1e, 0x40, 0xd2, 0x14, 0x24, 0x11, 0xb3, 0x9e, 0x40, 0xc7, 0x63, 0xdb,
            0x9d, 0xef, 0xb6, 0xca, 0x70, 0xab, 0xea, 0xad, 0x20, 0x67, 0x44, 0x43, 0xd3, 0x3f,
            0x88, 0xbe, 0x67, 0x3a,
        ]
    );
    assert_ne!(
        crate::deposit_blinding(&[1; 32], 7),
        crate::deposit_blinding(&[1; 32], 8)
    );
}

#[test]
fn deposit_payload_matches_wincode() {
    let instruction = Deposit {
        tree: address(1),
        depositor: address(2),
        deposits: vec![AssetDeposit {
            asset: DepositAsset::Sol,
            view_tag: [7; 32],
            owner: [9; 32],
            amount: 42,
            memo: Some(b"hi".to_vec()),
        }],
    }
    .instruction()
    .expect("deposit");
    assert_eq!(instruction.program_address, SHIELDED_POOL_PROGRAM);
    assert_eq!(instruction.data[0], DEPOSIT_TAG);
    assert_eq!(&instruction.data[1..], hex(DEPOSIT_GOLDEN).as_slice());
    assert_eq!(
        instruction
            .accounts
            .iter()
            .map(|meta| (meta.address, meta.role))
            .collect::<Vec<_>>(),
        vec![
            (address(1), AccountRole::Writable),
            (address(2), AccountRole::WritableSigner),
            (SHIELDED_POOL_PROGRAM, AccountRole::Readonly),
            (SYSTEM_PROGRAM, AccountRole::Readonly),
            (SOL_INTERFACE, AccountRole::Writable),
        ]
    );
}

#[test]
fn deposit_puts_sol_ahead_of_an_earlier_spl_entry() {
    let spl = SplDeposit {
        mint: address(FIXTURE_5),
        user_token: address(FIXTURE_6),
        token_program: TOKEN_PROGRAM,
        interface: address(FIXTURE_8),
        interface_bump: 9,
    };
    let instruction = Deposit {
        tree: address(1),
        depositor: address(2),
        deposits: vec![
            AssetDeposit {
                asset: DepositAsset::Spl(spl),
                view_tag: [3; 32],
                owner: [4; 32],
                amount: 5,
                memo: None,
            },
            AssetDeposit {
                asset: DepositAsset::Sol,
                view_tag: [3; 32],
                owner: [4; 32],
                amount: 6,
                memo: None,
            },
        ],
    }
    .instruction()
    .expect("mixed deposit");
    let data = &instruction.data;
    assert_eq!(&data[1..5], &[2, 0, 1, 9]);
    assert_eq!(data[5], 2);
    assert_eq!(data[6], 1);
    let second_asset = SECOND_NOTE_OFFSET;
    assert_eq!(data[second_asset], 0);

    assert_eq!(
        instruction.accounts[SPL_GROUP_INDEX].address, TOKEN_PROGRAM,
        "SPL group follows the SOL group"
    );
    assert_eq!(instruction.accounts[6].address, address(FIXTURE_5));
    assert_eq!(instruction.accounts[8].address, address(FIXTURE_8));
}

#[test]
fn deposit_rejects_conflicts_and_empty_batches() {
    let first = SplDeposit {
        mint: address(FIXTURE_5),
        user_token: address(FIXTURE_6),
        token_program: TOKEN_PROGRAM,
        interface: address(FIXTURE_8),
        interface_bump: 9,
    };
    let mut second = first;
    second.user_token = address(FIXTURE_7);
    let conflict = Deposit {
        tree: address(1),
        depositor: address(2),
        deposits: vec![
            AssetDeposit {
                asset: DepositAsset::Spl(first),
                view_tag: [1; 32],
                owner: [1; 32],
                amount: 1,
                memo: None,
            },
            AssetDeposit {
                asset: DepositAsset::Spl(second),
                view_tag: [1; 32],
                owner: [1; 32],
                amount: 1,
                memo: None,
            },
        ],
    };
    assert!(matches!(
        conflict.instruction(),
        Err(Error::SettlementMismatch)
    ));
    assert!(matches!(
        Deposit {
            tree: address(1),
            depositor: address(2),
            deposits: vec![AssetDeposit {
                asset: DepositAsset::Sol,
                view_tag: [1; 32],
                owner: [1; 32],
                amount: 0,
                memo: None,
            }],
        }
        .instruction(),
        Err(Error::ZeroAmount)
    ));
}

#[test]
fn transact_payload_matches_wincode_and_withdraw_accounts() {
    let body = sample_body();
    let encoded = write_transact(&body, &[TransferLeg::SolWithdrawal(7)]).expect("bytes");
    assert_eq!(encoded, hex(TRANSACT_GOLDEN));

    let recipient = address(FIXTURE_12);
    let instruction = Withdraw {
        payer: address(2),
        input_trees: vec![STATE_TREE],
        output_tree: STATE_TREE,
        owner: address(3),
        nullifier_accounts: vec![address(FIXTURE_13)],
        settlement: Settlement::SolWithdrawal {
            recipient,
            amount: 7,
        },
        body,
    }
    .instruction()
    .expect("withdraw");
    assert_eq!(instruction.data[0], TRANSACT_TAG);
    assert_eq!(&instruction.data[1..], hex(TRANSACT_GOLDEN).as_slice());
    assert_eq!(
        instruction
            .accounts
            .iter()
            .map(|meta| (meta.address, meta.role))
            .collect::<Vec<_>>(),
        vec![
            (address(2), AccountRole::WritableSigner),
            (STATE_TREE, AccountRole::Writable),
            (SHIELDED_POOL_PROGRAM, AccountRole::Readonly),
            (SYSTEM_PROGRAM, AccountRole::Readonly),
            (STATE_TREE, AccountRole::Writable),
            (address(FIXTURE_13), AccountRole::Writable),
            (address(3), AccountRole::ReadonlySigner),
            (SOL_INTERFACE, AccountRole::Writable),
            (recipient, AccountRole::Writable),
        ]
    );
}

#[test]
fn confidential_transfer_has_no_public_leg() {
    let instruction = ConfidentialTransfer {
        payer: address(2),
        input_trees: vec![address(4)],
        output_tree: address(FIXTURE_5),
        owner: address(3),
        additional_owners: vec![address(FIXTURE_6)],
        nullifier_accounts: vec![address(FIXTURE_7)],
        body: sample_body(),
    }
    .instruction()
    .expect("transfer");
    assert_eq!(instruction.accounts.len(), 8);
    assert!(
        instruction
            .accounts
            .iter()
            .all(|meta| meta.address != SOL_INTERFACE)
    );
    assert_eq!(instruction.accounts[6].address, address(3));
    assert_eq!(instruction.accounts[7].address, address(FIXTURE_6));
    assert_eq!(instruction.accounts[6].role, AccountRole::ReadonlySigner);
}

#[test]
fn spl_withdraw_appends_the_cpi_authority() {
    let mut body = sample_body();
    body.inputs.clear();
    body.tree_contexts.clear();
    body.inputs.push(Input {
        nullifier_hash: [1; 32],
        tree_index: 0,
    });
    body.tree_contexts.push(TreeContext {
        utxo_tree_root_index: 0,
        nullifier_tree_root_index: 0,
    });
    let accounts = SplWithdrawalAccounts {
        mint: address(FIXTURE_8),
        interface: address(FIXTURE_9),
        user_token: address(10),
        token_program: TOKEN_PROGRAM,
    };
    let instruction = Withdraw {
        payer: address(2),
        input_trees: vec![address(4)],
        output_tree: address(FIXTURE_5),
        owner: address(3),
        nullifier_accounts: vec![address(FIXTURE_7)],
        settlement: Settlement::SplWithdrawal {
            accounts,
            amount: 9,
            interface_bump: 4,
        },
        body,
    }
    .instruction()
    .expect("spl withdraw");
    let tail: Vec<_> = instruction
        .accounts
        .iter()
        .rev()
        .take(SPL_TAIL_LEN)
        .map(|meta| meta.address)
        .collect();

    assert_eq!(
        tail,
        vec![
            TOKEN_PROGRAM,
            address(10),
            address(FIXTURE_9),
            address(FIXTURE_8),
            CPI_AUTHORITY,
        ]
    );
}

#[test]
fn transact_rejects_a_third_tree_and_a_zero_leg() {
    let mut body = sample_body();
    body.tree_contexts.push(body.tree_contexts[0]);
    body.tree_contexts.push(body.tree_contexts[0]);
    let too_many = Withdraw {
        payer: address(1),
        input_trees: vec![address(1), address(2), address(3)],
        output_tree: address(4),
        owner: address(FIXTURE_5),
        nullifier_accounts: vec![address(FIXTURE_6)],
        settlement: Settlement::SolWithdrawal {
            recipient: address(FIXTURE_7),
            amount: 1,
        },
        body: body.clone(),
    };
    assert!(matches!(too_many.instruction(), Err(Error::Limit(_))));

    let zero = Withdraw {
        payer: address(1),
        input_trees: vec![address(1)],
        output_tree: address(4),
        owner: address(FIXTURE_5),
        nullifier_accounts: vec![address(FIXTURE_6)],
        settlement: Settlement::SolWithdrawal {
            recipient: address(FIXTURE_7),
            amount: 0,
        },
        body: sample_body(),
    };
    assert!(matches!(zero.instruction(), Err(Error::ZeroAmount)));
    let deposit_leg = Withdraw {
        settlement: Settlement::SolDeposit {
            recipient: address(FIXTURE_7),
            amount: 1,
        },
        ..zero
    };
    assert!(matches!(
        deposit_leg.instruction(),
        Err(Error::SettlementMismatch)
    ));
}

#[test]
fn two_withdrawal_legs_share_one_transact() {
    let mut transaction = transact_for(sample_body());
    transaction.settlements.push(Settlement::SolWithdrawal {
        recipient: address(FIXTURE_8),
        amount: 8,
    });
    let instruction = transaction.instruction().expect("two legs");
    let recipients = instruction
        .accounts
        .iter()
        .filter(|meta| meta.address == address(FIXTURE_12) || meta.address == address(FIXTURE_8))
        .count();
    assert_eq!(recipients, 2);
}

fn transact_for(body: TransactBody) -> Transact {
    Transact {
        payer: address(2),
        input_trees: vec![STATE_TREE],
        output_tree: STATE_TREE,
        owner_signers: vec![address(3)],
        nullifier_accounts: vec![address(FIXTURE_13)],
        settlements: vec![Settlement::SolWithdrawal {
            recipient: address(FIXTURE_12),
            amount: 7,
        }],
        read_cache: None,
        write_cache: None,
        body,
    }
}

#[test]
fn transact_rejects_ring_selectors_and_appends_cache_accounts() {
    let mut ring = sample_body();
    ring.circuit = Circuit::ConfidentialRingEddsa(1, 2, 3);
    assert!(matches!(
        transact_for(ring).instruction(),
        Err(Error::Limit("transact circuit"))
    ));

    let mut cached = sample_body();
    cached.circuit = Circuit::ConfidentialSignerCached(
        1,
        2,
        3,
        CacheAccess {
            read_bitmap: 1 << READ_SLOT,
            write_slots: [CacheWrite::NONE; MAX_CACHE_WRITES],
        },
    );
    assert!(matches!(
        transact_for(cached.clone()).instruction(),
        Err(Error::SettlementMismatch)
    ));
    let mut with_read = transact_for(cached);
    with_read.read_cache = Some(address(FIXTURE_20));
    let instruction = with_read.instruction().expect("read cache");
    let tail = instruction.accounts.last().expect("cache account");
    assert_eq!(tail.address, address(FIXTURE_20));

    assert_eq!(tail.role, AccountRole::Readonly);

    let mut writing = sample_body();
    let mut slots = [CacheWrite::NONE; MAX_CACHE_WRITES];
    slots[0] = CacheWrite { output: 0, slot: 1 };
    writing.circuit = Circuit::ConfidentialSignerCached(
        1,
        2,
        3,
        CacheAccess {
            read_bitmap: 0,
            write_slots: slots,
        },
    );
    let mut with_write = transact_for(writing);
    with_write.write_cache = Some(CacheWriter {
        cache: address(21),
        writer: address(22),
    });
    let instruction = with_write.instruction().expect("write cache");
    let accounts = &instruction.accounts;

    let write = &accounts[accounts.len() - 2];
    let writer = accounts.last().expect("writer");
    assert_eq!(write.address, address(21));
    assert_eq!(write.role, AccountRole::Writable);
    assert_eq!(writer.address, address(22));
    assert_eq!(writer.role, AccountRole::ReadonlySigner);
}

#[test]
fn confidential_plaintext_matches_wincode() {
    let note = ConfidentialPlaintext {
        asset_id: 1,
        amount: 99,
        blinding: [2; 32],
        ring_program_id: None,
        records: Vec::new(),
    };
    assert_eq!(write_confidential(&note), hex(PLAINTEXT_GOLDEN));
    let parsed = crate::read_confidential(&hex(PLAINTEXT_GOLDEN)).expect("read");
    assert_eq!(parsed.asset_id, 1);
    assert_eq!(parsed.amount, 99);
    assert_eq!(parsed.blinding, [2; 32]);
    assert_eq!(parsed.ring_program_id, None);
}

#[test]
fn indexer_urls_carry_the_method_and_no_key() {
    let url = crate::Cluster::Mainnet
        .indexer_url(GetEncryptedUtxosByTags::NAME)
        .expect("url");
    assert_eq!(
        url,
        "https://mainnet.helius-rpc.com/v1/spr/getEncryptedUtxosByTags"
    );
    assert!(crate::Cluster::Devnet.indexer_url("bad-name").is_err());
    assert_eq!(
        crate::local_indexer_url("getUserRecords").expect("local"),
        "http://127.0.0.1:8784/getUserRecords"
    );
}

#[test]
fn tag_query_matches_the_documented_mainnet_request() {
    let request = GetEncryptedUtxosByTags {
        tags: vec!["5Vt1Mnq2b9ynV1EMXS564i5ZDuNudxm8ZG7yCXfH55M".to_owned()],
        filters: Some(HistoryFilters {
            slot: Some(SlotBounds {
                gte: Some(MAINNET_SLOT),
                lte: Some(MAINNET_SLOT),
                ..SlotBounds::default()
            }),
        }),
        limit: Some(1),
        min_context_slot: None,
        pagination_token: None,
        ring_program_id: None,
    };
    request.validate().expect("bounds");
    assert_eq!(
        serde_json::to_value(&request).expect("params"),
        serde_json::json!({
            "tags": ["5Vt1Mnq2b9ynV1EMXS564i5ZDuNudxm8ZG7yCXfH55M"],
            "filters": { "slot": { "gte": MAINNET_SLOT, "lte": MAINNET_SLOT } },
            "limit": 1
        })
    );
    let empty = GetEncryptedUtxosByTags {
        tags: Vec::new(),
        filters: None,
        limit: Some(0),
        min_context_slot: None,
        pagination_token: None,
        ring_program_id: None,
    };
    assert!(empty.validate().is_err());
}

#[test]
fn recorded_mainnet_examples_decode() {
    let utxos = GetEncryptedUtxosByTags {
        tags: vec!["5Vt1Mnq2b9ynV1EMXS564i5ZDuNudxm8ZG7yCXfH55M".to_owned()],
        filters: None,
        limit: None,
        min_context_slot: None,
        pagination_token: None,
        ring_program_id: None,
    };
    let page = utxos
        .decode(include_str!("../tests/fixtures/mainnet_utxos.json"))
        .expect("utxo page");
    assert_eq!(page.context.slot, 467_025_624);
    assert_eq!(page.context.block_time, 1_794_164_634);
    assert_eq!(page.output_tree_id, Some(0));
    assert_eq!(page.pagination_token, None);
    assert_eq!(page.data[0].outputs.len(), 2);
    assert_eq!(
        page.data[0].outputs[0]
            .output_slot
            .output_context
            .leaf_index,
        3_555
    );

    assert_eq!(
        page.data[0].outputs[1]
            .output_slot
            .output_context
            .leaf_index,
        3_556
    );
    assert_eq!(
        page.data[0].outputs[0].output_slot.output_context.tree,
        "777pvSGe5LaBd9nCczvE2FXEauiwtfGJ5XRpjKyniGor"
    );
    let payload = crate::decode_base64(&page.data[0].outputs[0].output_slot.payload).expect("b64");
    assert_eq!(&payload[..7], &[1, 0x54, 0, 0, 0, 3, 4]);

    let signature = GetShieldedTransactionsBySignature {
        tx_signature: page.data[0].signature.clone(),
    };
    let events = signature
        .decode(include_str!("../tests/fixtures/mainnet_by_signature.json"))
        .expect("signature page");
    assert_eq!(events.transactions[0].event_index, 0);
    assert_eq!(events.transactions[0].transaction.nullifiers.len(), 1);
    assert!(!events.transactions[0].transaction.proofless);

    let records = GetUserRecords {
        owners: vec!["5Vt1Mnq2b9ynV1EMXS564i5ZDuNudxm8ZG7yCXfH55M".to_owned()],
    };
    let registry = records
        .decode(include_str!("../tests/fixtures/mainnet_user_record.json"))
        .expect("registry");
    let record = registry.records[0].as_ref().expect("registered");
    assert_eq!(record.merge_encryption, 0);
    assert_eq!(
        record.viewing_pubkey,
        "BIs8UXWNMRXj0LYmli1ENBZyF5MaxY28XHJEj0+5z8Y2"
    );
    assert!(
        GetUserRecords {
            owners: vec!["5Vt1Mnq2b9ynV1EMXS564i5ZDuNudxm8ZG7yCXfH55M".to_owned()],
        }
        .decode(r#"{"context":{"blockTime":1,"slot":2},"records":[null]}"#)
        .expect("missing record")
        .records[0]
            .is_none()
    );
}

#[test]
fn history_and_proof_shapes_round_trip() {
    let history = GetShieldedTransactionsByTags {
        tags: vec!["5Vt1Mnq2b9ynV1EMXS564i5ZDuNudxm8ZG7yCXfH55M".to_owned()],
        filters: Some(HistoryFilters {
            slot: Some(SlotBounds {
                gte: Some(MAINNET_SLOT),
                lte: Some(MAINNET_SLOT),
                ..SlotBounds::default()
            }),
        }),
        limit: Some(1),
        min_context_slot: None,
        pagination_token: None,
        ring_program_id: None,
    };
    let page = history
        .decode(
            r#"{
                "context":{"blockTime":1794164634,"slot":467025625},
                "outputTreeId":0,
                "data":[{
                    "slot":467025494,
                    "signature":"5QVkeFTWxaAiwo1MBrDVvsLuLHoAfsvzXWz9pYDTUyFUkWipfVQ73XaUT74qfLau3HLyxsaoMRKWZGSmF6pdsfJf",
                    "events":[{
                        "slot":467025494,
                        "txSignature":"5QVkeFTWxaAiwo1MBrDVvsLuLHoAfsvzXWz9pYDTUyFUkWipfVQ73XaUT74qfLau3HLyxsaoMRKWZGSmF6pdsfJf",
                        "eventIndex":0,
                        "txViewingPk":"BOP9u/X3TGpU3rM+hv3T4WngCphDcowWQDKIC/ZqK3gH",
                        "salt":"w1HG4IspgkY2JxMc4I94JA==",
                        "outputSlots":[],
                        "messages":[],
                        "nullifiers":["282dZeNR7hN6Lcj7oPFZok2qKuRZ5SHzzMzQr8AKwwXS"],
                        "proofless":false
                    }]
                }],
                "paginationToken":null
            }"#,
        )
        .expect("history");
    assert_eq!(page.context.slot, 467_025_625);
    assert_eq!(
        page.data[0].events[0].nullifiers[0],
        "282dZeNR7hN6Lcj7oPFZok2qKuRZ5SHzzMzQr8AKwwXS"
    );

    let proofs = GetMerkleProofs {
        tree_account: "777pvSGe5LaBd9nCczvE2FXEauiwtfGJ5XRpjKyniGor".to_owned(),
        leaves: vec!["SkgpyqjEjyfg2J3H9xjAcnPmpvqjvTL5PkckJBFKhaY".to_owned()],
    };
    assert_eq!(
        serde_json::to_value(&proofs).expect("proof request"),
        serde_json::json!({
            "treeAccount": "777pvSGe5LaBd9nCczvE2FXEauiwtfGJ5XRpjKyniGor",
            "leaves": ["SkgpyqjEjyfg2J3H9xjAcnPmpvqjvTL5PkckJBFKhaY"]
        })
    );
    let proof = proofs
        .decode(
            r#"{
                "context":{"blockTime":1,"slot":2},
                "proofs":[{
                    "leaf":"SkgpyqjEjyfg2J3H9xjAcnPmpvqjvTL5PkckJBFKhaY",
                    "merkleContext":{"treeType":1,"tree":"777pvSGe5LaBd9nCczvE2FXEauiwtfGJ5XRpjKyniGor"},
                    "path":["SkgpyqjEjyfg2J3H9xjAcnPmpvqjvTL5PkckJBFKhaY"],
                    "leafIndex":9,
                    "root":"SkgpyqjEjyfg2J3H9xjAcnPmpvqjvTL5PkckJBFKhaY",
                    "rootSeq":4,
                    "rootIndex":5
                }]
            }"#,
        )
        .expect("schema-shaped proof");
    assert_eq!(proof.proofs[0].root_index, 5);

    let absent = GetNonInclusionProofs {
        tree_account: proofs.tree_account.clone(),
        leaves: vec!["cGfHiC6Kgg3FpFZvgwGcswsCRtp4aBP2fzuXRQPizuN".to_owned()],
    };
    let non_inclusion = absent
        .decode(
            r#"{
                "context":{"blockTime":1,"slot":2},
                "proofs":[{
                    "leaf":"cGfHiC6Kgg3FpFZvgwGcswsCRtp4aBP2fzuXRQPizuN",
                    "merkleContext":{"treeType":2,"tree":"777pvSGe5LaBd9nCczvE2FXEauiwtfGJ5XRpjKyniGor"},
                    "path":[],
                    "lowElement":"cGfHiC6Kgg3FpFZvgwGcswsCRtp4aBP2fzuXRQPizuN",
                    "lowElementIndex":0,
                    "highElement":"cGfHiC6Kgg3FpFZvgwGcswsCRtp4aBP2fzuXRQPizuN",
                    "highElementIndex":1,
                    "root":"cGfHiC6Kgg3FpFZvgwGcswsCRtp4aBP2fzuXRQPizuN",
                    "rootSeq":3,
                    "rootIndex":4
                }]
            }"#,
        )
        .expect("schema-shaped non-inclusion");
    assert_eq!(non_inclusion.proofs[0].high_element_index, 1);
    assert!(proofs.decode("[1,2]").is_err());

    let spent = GetShieldedTransactionsByNullifiers {
        nullifiers: vec!["282dZeNR7hN6Lcj7oPFZok2qKuRZ5SHzzMzQr8AKwwXS".to_owned()],
        filters: None,
        limit: Some(1),
        min_context_slot: None,
        pagination_token: None,
    };
    spent.validate().expect("nullifier bounds");
    assert!(
        GetMerkleProofs {
            tree_account: proofs.tree_account,
            leaves: Vec::new(),
        }
        .validate()
        .is_err()
    );
    assert!(
        GetUserRecords {
            owners: vec!["owner".to_owned()],
        }
        .validate()
        .is_ok()
    );
    assert!(
        GetMerkleProofs {
            tree_account: "tree".to_owned(),
            leaves: vec!["leaf".to_owned()],
        }
        .validate()
        .is_ok()
    );
}

struct Recorded {
    result: &'static str,
    seen: std::rc::Rc<RefCell<Option<serde_json::Value>>>,
}

impl Transport for Recorded {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let value: serde_json::Value = serde_json::from_slice(&request.body).expect("frame");
        let id = value["id"].as_u64().expect("numeric id");
        *self.seen.borrow_mut() = Some(value);
        std::future::ready(Ok(Reply {
            status: 200,
            body: format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{}}}"#, self.result).into_bytes(),
        }))
    }
}

#[test]
fn client_call_posts_the_method_to_one_exchange() {
    let seen = std::rc::Rc::new(RefCell::new(None));
    let client = Client::new(
        Recorded {
            result: include_str!("../tests/fixtures/mainnet_user_record.json"),
            seen: std::rc::Rc::clone(&seen),
        },
        Limits::default(),
    );
    let request = GetUserRecords {
        owners: vec!["5Vt1Mnq2b9ynV1EMXS564i5ZDuNudxm8ZG7yCXfH55M".to_owned()],
    };
    let page = futures::executor::block_on(client.call(request)).expect("call");
    assert_eq!(page.records.len(), 1);
    let frame = seen.borrow().clone().expect("request");
    assert_eq!(frame["method"], "getUserRecords");
    assert!(frame["params"].is_object());

    assert!(frame["id"].is_number());
}

fn one_deposit(asset: DepositAsset) -> AssetDeposit {
    AssetDeposit {
        asset,
        view_tag: [3; 32],
        owner: [4; 32],
        amount: 1,
        memo: None,
    }
}

fn spl_at(byte: u8) -> DepositAsset {
    DepositAsset::Spl(SplDeposit {
        mint: address(byte),
        user_token: address(FIXTURE_6),
        token_program: TOKEN_PROGRAM,
        interface: address(FIXTURE_8),
        interface_bump: FIXTURE_9,
    })
}

#[test]
fn deposit_bounds_and_mint_conflicts_fail() {
    let empty = Deposit {
        tree: address(1),
        depositor: address(2),
        deposits: Vec::new(),
    };
    assert!(empty.instruction().is_err());
    let mut many = Vec::new();

    for _ in 0..=u8::MAX {
        many.push(one_deposit(DepositAsset::Sol));
    }

    assert!(
        Deposit {
            tree: address(1),
            depositor: address(2),
            deposits: many,
        }
        .instruction()
        .is_err()
    );
    let mut assets = vec![one_deposit(DepositAsset::Sol)];

    for byte in [FIXTURE_5, FIXTURE_6, FIXTURE_7, FIXTURE_8, FIXTURE_9] {
        assets.push(one_deposit(spl_at(byte)));
    }

    assert!(
        Deposit {
            tree: address(1),
            depositor: address(2),
            deposits: assets,
        }
        .instruction()
        .is_err()
    );
    let first = spl_at(FIXTURE_5);
    let second = DepositAsset::Spl(SplDeposit {
        mint: address(FIXTURE_5),
        user_token: address(FIXTURE_6),
        token_program: TOKEN_PROGRAM,
        interface: address(FIXTURE_8),
        interface_bump: FIXTURE_7,
    });
    assert!(
        Deposit {
            tree: address(1),
            depositor: address(2),
            deposits: vec![one_deposit(first), one_deposit(second)],
        }
        .instruction()
        .is_err()
    );
}

#[test]
fn identical_spl_deposits_share_one_group() {
    let repeated = spl_at(FIXTURE_5);
    assert!(
        Deposit {
            tree: address(1),
            depositor: address(2),
            deposits: vec![one_deposit(repeated), one_deposit(repeated)],
        }
        .instruction()
        .is_ok()
    );
}

fn bsb22() -> Bsb22 {
    Bsb22 {
        commitment: [1; 32],
        commitment_pok: [2; 32],
    }
}

fn cache_access() -> CacheAccess {
    CacheAccess {
        read_bitmap: 1,
        write_slots: [CacheWrite::NONE; MAX_CACHE_WRITES],
    }
}

#[test]
fn every_circuit_selector_and_public_leg_encodes() {
    let anonymous = AnonymousRingProof {
        bsb22: bsb22(),
        default_owner_tag: Some([3; 32]),
    };
    let eddsa = EddsaProof {
        bsb22: bsb22(),
        owner_pk: [4; 32],
    };
    let cache = cache_access();
    let circuits = [
        Circuit::ConfidentialRingEddsa(1, 2, 3),
        Circuit::RingAuthority(1, 2, 3),
        Circuit::AnonymousRingEddsa(1, 2, 3, anonymous),
        Circuit::ConfidentialRingEddsaCached(1, 2, 3, cache),
        Circuit::AnonymousRingEddsaCached(1, 2, 3, anonymous, cache),
        Circuit::ConfidentialEddsa(1, 2, 3, eddsa),
        Circuit::ConfidentialEddsaCached(1, 2, 3, eddsa, cache),
    ];

    for circuit in circuits {
        let mut body = sample_body();
        body.circuit = circuit;
        body.data_hash = Some([5; 32]);
        body.outputs[0].owner_tag = OwnerTag::Inline([6; 32]);
        body.outputs[0].data = Some(vec![7]);
        body.messages.push(Message {
            view_tag: [8; 32],
            data: vec![FIXTURE_9],
        });
        let _ = circuit.cache();
        assert!(
            write_transact(
                &body,
                &[
                    TransferLeg::SolDeposit(1),
                    TransferLeg::SplDeposit {
                        amount: 2,
                        interface_bump: 3,
                    }
                ]
            )
            .is_ok()
        );
    }
}

#[test]
fn anonymous_ring_allows_a_missing_owner_tag() {
    let mut absent = sample_body();
    absent.circuit = Circuit::AnonymousRingEddsa(
        1,
        2,
        3,
        AnonymousRingProof {
            bsb22: bsb22(),
            default_owner_tag: None,
        },
    );
    assert!(write_transact(&absent, &[]).is_ok());
}

#[test]
fn short_confidential_notes_fail_to_parse() {
    assert!(crate::read_confidential(&[]).is_err());
    let mut short_ring = vec![0; 49];
    short_ring[48] = 1;
    assert!(crate::read_confidential(&short_ring).is_err());
}

#[test]
fn confidential_ring_note_round_trips() {
    let note = ConfidentialPlaintext {
        asset_id: 1,
        amount: 2,
        blinding: [3; 32],
        ring_program_id: Some([4; 32]),
        records: vec![1, 0, 0, 0, 9],
    };
    let encoded = write_confidential(&note);
    assert_eq!(crate::read_confidential(&encoded).expect("note"), note);
}

#[test]
fn transact_bounds_reject_unbalanced_accounts() {
    let mut transaction = transact_for(sample_body());
    transaction.settlements = vec![Settlement::SolDeposit {
        recipient: address(FIXTURE_8),
        amount: 1,
    }];
    assert!(transaction.instruction().is_ok());
    transaction.settlements = vec![Settlement::SplDeposit {
        accounts: SplDepositAccounts {
            mint: address(FIXTURE_5),
            interface: address(FIXTURE_8),
            token_authority: address(FIXTURE_6),
            user_token: address(FIXTURE_7),
            token_program: TOKEN_PROGRAM,
        },
        amount: 1,
        interface_bump: FIXTURE_9,
    }];
    assert!(transaction.instruction().is_ok());
    let mut cached = sample_body();
    cached.circuit = Circuit::ConfidentialSignerCached(
        1,
        2,
        3,
        CacheAccess {
            read_bitmap: 0,
            write_slots: [CacheWrite::NONE; MAX_CACHE_WRITES],
        },
    );
    assert!(transact_for(cached).instruction().is_err());
    let mut wide = sample_body();
    wide.outputs = vec![wide.outputs[0].clone(); 33];
    assert!(transact_for(wide).instruction().is_err());
    let mut trees = transact_for(sample_body());
    trees.input_trees.clear();
    assert!(trees.instruction().is_err());
    trees = transact_for(sample_body());
    trees.body.tree_contexts.clear();
    assert!(trees.instruction().is_err());
    trees = transact_for(sample_body());
    trees.body.inputs[0].tree_index = 1;
    assert!(trees.instruction().is_err());
    trees = transact_for(sample_body());
    trees.settlements = vec![
        Settlement::SolWithdrawal {
            recipient: address(FIXTURE_8),
            amount: 1,
        };
        33
    ];
    assert!(trees.instruction().is_err());
}

#[test]
fn transact_rejects_a_short_nullifier_list() {
    let mut trees = transact_for(sample_body());
    trees.nullifier_accounts.pop();
    assert!(trees.instruction().is_err());
}

#[test]
fn rpc_validators_reject_empty_pages() {
    let tags = GetEncryptedUtxosByTags {
        tags: Vec::new(),
        filters: None,
        limit: Some(0),
        min_context_slot: None,
        pagination_token: None,
        ring_program_id: None,
    };
    assert!(tags.validate().is_err());
    let nullifiers = GetShieldedTransactionsByNullifiers {
        nullifiers: Vec::new(),
        filters: None,
        limit: None,
        min_context_slot: None,
        pagination_token: None,
    };
    assert!(nullifiers.validate().is_err());
    let leaves = GetNonInclusionProofs {
        tree_account: "tree".to_owned(),
        leaves: Vec::new(),
    };
    assert!(leaves.validate().is_err());
    assert!(
        GetMerkleProofs {
            tree_account: "tree".to_owned(),
            leaves: Vec::new(),
        }
        .validate()
        .is_err()
    );
    let history = GetShieldedTransactionsByTags {
        tags: Vec::new(),
        filters: None,
        limit: Some(0),
        min_context_slot: None,
        pagination_token: None,
        ring_program_id: None,
    };
    assert!(history.validate().is_err());
    assert!(GetUserRecords { owners: Vec::new() }.validate().is_err());
}

#[test]
fn rpc_validators_accept_one_entry() {
    let mut tags = GetEncryptedUtxosByTags {
        tags: vec!["tag".to_owned()],
        filters: None,
        limit: None,
        min_context_slot: None,
        pagination_token: None,
        ring_program_id: None,
    };
    assert!(tags.validate().is_ok());
    tags.limit = Some(0);
    assert!(tags.validate().is_err());
    tags.limit = Some(1);
    assert!(tags.validate().is_ok());
    assert!(
        GetShieldedTransactionsByTags {
            tags: vec!["tag".to_owned()],
            filters: None,
            limit: None,
            min_context_slot: None,
            pagination_token: None,
            ring_program_id: None,
        }
        .validate()
        .is_ok()
    );
    assert!(
        GetShieldedTransactionsByNullifiers {
            nullifiers: vec!["nullifier".to_owned()],
            filters: None,
            limit: None,
            min_context_slot: None,
            pagination_token: None,
        }
        .validate()
        .is_ok()
    );
    assert!(
        GetNonInclusionProofs {
            tree_account: "tree".to_owned(),
            leaves: vec!["leaf".to_owned()],
        }
        .validate()
        .is_ok()
    );
    assert!(
        GetMerkleProofs {
            tree_account: "tree".to_owned(),
            leaves: vec!["leaf".to_owned()],
        }
        .validate()
        .is_ok()
    );
    assert!(
        GetUserRecords {
            owners: vec!["owner".to_owned()],
        }
        .validate()
        .is_ok()
    );
}

#[test]
fn nullifier_queue_uses_the_indexer_page_shape() {
    let request = GetNullifierQueueElements {
        tree_account: "tree".to_owned(),
        start_seq: 4,
        limit: 2,
    };
    assert!(request.validate().is_ok());
    let params = serde_json::to_value(&request).expect("params");
    assert_eq!(params["treeAccount"], "tree");
    assert_eq!(params["startSeq"], 4);
    assert_eq!(params["limit"], 2);
    assert_eq!(
        <GetNullifierQueueElements as Method>::NAME,
        "getNullifierQueueElements"
    );
    let value = bs58::encode([7u8; 32]).into_string();
    let page = request
        .decode(queue_page_with(&value).as_str())
        .expect("page");
    assert_eq!(page.elements[0].seq, 4);
    assert_eq!(page.elements[0].value, value);

    for value in ["nf", "0", "11"] {
        let error = <GetNullifierQueueElements as Method>::decode(
            &request,
            queue_page_with(value).as_str(),
        )
        .expect_err("queue value schema error");
        assert!(error.to_string().contains("queue value"), "{error}");
    }

    assert!(rejects_queue_bound("", 0, 1));
    assert!(rejects_queue_bound("tree", 0, 0));
    assert!(rejects_queue_bound("tree", 0, 1_001));
}

fn queue_page_with(value: &str) -> String {
    format!(
        r#"{{"context":{{"blockTime":1,"slot":2}},"elements":[{{"seq":4,"value":"{value}"}}]}}"#
    )
}

fn rejects_queue_bound(tree_account: &str, start_seq: u64, limit: u64) -> bool {
    GetNullifierQueueElements {
        tree_account: tree_account.to_owned(),
        start_seq,
        limit,
    }
    .validate()
    .is_err()
}
