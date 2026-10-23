extern crate std;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Hash, Instruction,
    plans::{
        Error, InstructionBatch, InstructionLeaf, InstructionPlan, MaxInstructions, MessageBuilder,
        MessageDraft, MessagePacker, PackerFactory, Plan, Planner,
    },
    transaction::{self, Lifetime, LookupTable, Transaction, Version},
};
use std::prelude::v1::*;
use std::vec;
use std::{cell::Cell, rc::Rc};

const EXACT_DATA: usize = 1060;

fn instruction(tag: u8, length: usize) -> Instruction {
    Instruction {
        program_address: Address::from_bytes([9; 32]),
        accounts: vec![],
        data: vec![tag; length],
    }
}
fn draft() -> MessageDraft {
    MessageDraft {
        version: Version::V0,
        payer: Address::from_bytes([1; 32]),
        lifetime: Lifetime::Blockhash {
            blockhash: Hash::from_bytes([2; 32]),
            last_valid_block_height: 77,
        },
        instructions: vec![],
        lookup_tables: vec![],
        config: None,
    }
}
struct Builder;
impl MessageBuilder for Builder {
    fn create(&mut self) -> Result<MessageDraft, Error> {
        Ok(draft())
    }
}

pub(super) fn literal_boundary_fit_overflow_and_rejected_cursor_rollback()
-> Result<(), &'static str> {
    let mut exact = draft();
    exact.instructions.push(instruction(3, EXACT_DATA));
    let packet = Transaction::new(exact.compile(MaxInstructions::default()).unwrap())
        .encode(false)
        .unwrap();
    super::assert_eq!(packet.len(), 1232);
    exact.instructions[0].data.push(3);
    super::assert_eq!(
        exact.compile(MaxInstructions::default()),
        Err(Error::Compile(transaction::Error::PacketTooLarge))
    );
    let first = instruction(3, EXACT_DATA);
    let second = instruction(4, EXACT_DATA);

    let mut packer = InstructionBatch(vec![first.clone(), second.clone()]).create();
    super::assert!(packer.pack(&exact, MaxInstructions::default()).is_err());
    super::assert!(!packer.done());
    let accepted = packer.pack(&draft(), MaxInstructions::default()).unwrap();
    super::assert_eq!(accepted.instructions, [first]);
    super::assert!(!packer.done());
    let accepted = packer.pack(&draft(), MaxInstructions::default()).unwrap();
    super::assert_eq!(accepted.instructions, [second]);
    super::assert!(packer.done());
    Ok(())
}

struct LaterFailure(Rc<Cell<usize>>);
impl MessageBuilder for LaterFailure {
    fn create(&mut self) -> Result<MessageDraft, Error> {
        self.0.set(self.0.get() + 1);

        if self.0.get() == 2 {
            return Err(Error::Builder("second resource denied".into()));
        }

        Ok(draft())
    }
}
pub(super) fn builder_error_after_accepted_candidate_returns_original_failure()
-> Result<(), &'static str> {
    let count = Rc::new(Cell::new(0));
    let input: InstructionPlan = Plan::sequential(vec![
        instruction(1, EXACT_DATA).into(),
        instruction(2, EXACT_DATA).into(),
    ]);
    let result = Planner::new(LaterFailure(count.clone()), MaxInstructions::default()).plan(&input);
    super::assert_eq!(result, Err(Error::Builder("second resource denied".into())));
    super::assert_eq!(count.get(), 2);
    Ok(())
}

#[derive(Clone)]
struct Stalled(Rc<Cell<usize>>);
impl PackerFactory for Stalled {
    type Packer = Self;
    fn create(&self) -> Self {
        self.clone()
    }
}
impl MessagePacker for Stalled {
    fn done(&self) -> bool {
        false
    }
    fn pack(
        &mut self,
        candidate: &MessageDraft,
        _: MaxInstructions,
    ) -> Result<MessageDraft, Error> {
        self.0.set(self.0.get() + 1);
        std::assert!(self.0.get() <= 2, "unbounded empty-candidate retry");
        Ok(candidate.clone())
    }
}
pub(super) fn unchanged_empty_candidate_fails_in_one_attempt() -> Result<(), &'static str> {
    let count = Rc::new(Cell::new(0));
    let input = Plan::Single(InstructionLeaf::Packer(Stalled(count.clone())));
    let result = Planner::new(Builder, MaxInstructions::default()).plan(&input);
    super::assert_eq!(result, Err(Error::NoProgress));
    super::assert_eq!(count.get(), 1);
    Ok(())
}
pub(super) fn initially_complete_packer_neither_creates_empty_message_nor_loses_prior_work()
-> Result<(), &'static str> {
    let empty: InstructionPlan = Plan::Single(InstructionLeaf::Packer(InstructionBatch(vec![])));
    let mut planner = Planner::new(Builder, MaxInstructions::default());
    super::assert_eq!(planner.plan(&empty), Err(Error::EmptyPlan));
    let input = Plan::sequential(vec![instruction(5, 7).into(), empty]);
    let result = planner.plan(&input).unwrap();
    super::assert_eq!(result.flatten().len(), 1);
    super::assert_eq!(result.flatten()[0].instructions()[0].data(), [5; 7]);
    Ok(())
}

struct SnapshotBuilder(MessageDraft);
impl MessageBuilder for SnapshotBuilder {
    fn create(&mut self) -> Result<MessageDraft, Error> {
        Ok(self.0.clone())
    }
}
pub(super) fn lookup_signer_duplicate_positions_and_wire_order_survive_planning()
-> Result<(), &'static str> {
    let mut source = draft();
    source.lookup_tables.push(LookupTable {
        address: Address::from_bytes([50; 32]),
        addresses: vec![Address::from_bytes([44; 32]), Address::from_bytes([33; 32])],
    });
    let accounts = [
        (44, AccountRole::Readonly),
        (33, AccountRole::Writable),
        (44, AccountRole::Readonly),
        (2, AccountRole::ReadonlySigner),
    ]
    .map(|(byte, role)| AccountMeta {
        address: Address::from_bytes([byte; 32]),
        role,
    });
    let input: InstructionPlan = Instruction {
        program_address: Address::from_bytes([9; 32]),
        accounts: accounts.to_vec(),
        data: vec![77],
    }
    .into();
    let result = Planner::new(SnapshotBuilder(source.clone()), MaxInstructions::default())
        .plan(&input)
        .unwrap();
    let message = result.flatten()[0];
    let mut expected = vec![128, 2, 1, 1, 3];
    expected.extend([1; 32]);

    expected.extend([2; 32]);
    expected.extend([9; 32]);
    expected.extend([2; 32]);
    expected.extend([1, 2, 4, 4, 3, 4, 1, 1, 77, 1]);
    expected.extend([50; 32]);
    expected.extend([1, 1, 1, 0]);
    super::assert_eq!(message.bytes(), expected);
    super::assert_eq!(message.instructions()[0].accounts(), [4, 3, 4, 1]);
    super::assert_eq!(message.lookups()[0].writable_indices(), [1]);
    super::assert_eq!(message.lookups()[0].readonly_indices(), [0]);
    super::assert_eq!(message.lifetime(), source.lifetime);
    Ok(())
}
