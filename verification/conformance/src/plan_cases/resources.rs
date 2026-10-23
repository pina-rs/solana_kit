extern crate std;
use super::polling::block_on;
use solana_kit_core::{
    Address, Hash, Instruction,
    plans::*,
    transaction::{self, Lifetime, Version},
};
use std::prelude::v1::*;
use std::vec;
use std::{cell::Cell, rc::Rc};

struct Pure(Rc<Cell<usize>>);
impl MessageBuilder for Pure {
    fn create(&mut self) -> Result<MessageDraft, Error> {
        self.0.set(self.0.get() + 1);
        Ok(MessageDraft {
            version: Version::V0,
            payer: Address::from_bytes([1; 32]),
            lifetime: Lifetime::Blockhash {
                blockhash: Hash::from_bytes([0; 32]),
                last_valid_block_height: 1,
            },
            instructions: vec![],
            lookup_tables: vec![],
            config: None,
        })
    }
}
fn instruction(length: usize) -> Instruction {
    Instruction {
        program_address: Address::from_bytes([9; 32]),
        accounts: vec![],
        data: vec![1; length],
    }
}

pub(super) fn explicit_512_depth_and_width_inputs_preserve_output_and_packet_budgets()
-> Result<(), &'static str> {
    let creates = Rc::new(Cell::new(0));
    let mut planner = Planner::new(Pure(creates.clone()), MaxInstructions::new(8).unwrap());
    let mut deep: InstructionPlan = instruction(1).into();

    for index in 0..512 {
        deep = if index % 2 == 0 {
            Plan::sequential(vec![deep])
        } else {
            Plan::Parallel(vec![deep])
        };
    }

    let result = block_on(planner.plan_async(&deep)).unwrap();
    super::assert_eq!(result.flatten().len(), 1);
    super::assert_eq!(creates.get(), 1);
    let wide: InstructionPlan = Plan::Parallel((0..512).map(|_| instruction(1).into()).collect());
    let result = block_on(planner.plan_async(&wide)).unwrap();
    super::assert_eq!(result.flatten().len(), 64);
    super::assert_eq!(creates.get(), 65);

    for message in result.flatten() {
        super::assert_eq!(message.instructions().len(), 8);
        super::assert!(
            solana_kit_core::transaction::Transaction::new(message.clone())
                .encode(false)
                .unwrap()
                .len()
                <= 1232
        );
    }

    Ok(())
}

struct Invalid;
impl PackerFactory for Invalid {
    type Packer = InvalidCursor;
    fn create(&self) -> InvalidCursor {
        InvalidCursor(false)
    }
}
struct InvalidCursor(bool);
impl MessagePacker for InvalidCursor {
    fn done(&self) -> bool {
        self.0
    }
    fn pack(&mut self, draft: &MessageDraft, _: MaxInstructions) -> Result<MessageDraft, Error> {
        let mut output = draft.clone();
        output.instructions.push(instruction(1233));
        self.0 = true;
        Ok(output)
    }
}
pub(super) fn successful_consumed_packer_with_invalid_compiled_output_never_retries()
-> Result<(), &'static str> {
    let creates = Rc::new(Cell::new(0));
    let mut planner = Planner::new(Pure(creates.clone()), MaxInstructions::default());
    let input = Plan::sequential(vec![
        instruction(1).into(),
        Plan::Single(InstructionLeaf::Packer(Invalid)),
    ]);
    super::assert_eq!(
        block_on(planner.plan_async(&input)),
        Err(Error::PackedMessage(Box::new(Error::Compile(
            transaction::Error::PacketTooLarge
        ))))
    );
    super::assert_eq!(creates.get(), 1);
    Ok(())
}
