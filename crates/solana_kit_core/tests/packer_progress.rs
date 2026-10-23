use solana_kit_core::{
    Address, Hash, Instruction,
    plans::{
        Error, InstructionLeaf, MaxInstructions, MessageBuilder, MessageDraft, MessagePacker,
        PackerFactory, Plan, Planner, append_instruction_plan,
    },
    transaction::{Lifetime, Version},
};
use std::{
    cell::Cell,
    future::Future,
    pin::pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

#[derive(Clone, Copy)]
enum Behavior {
    Unchanged,
    Complete,
    Append,
    AlreadyDone,
    Metadata,
    Prefix,
}

#[derive(Clone)]
struct Factory {
    behavior: Behavior,
    calls: Rc<Cell<usize>>,
}

struct Cursor {
    factory: Factory,
    done: bool,
}

impl PackerFactory for Factory {
    type Packer = Cursor;

    fn create(&self) -> Cursor {
        Cursor {
            factory: self.clone(),
            done: matches!(self.behavior, Behavior::AlreadyDone),
        }
    }
}

impl MessagePacker for Cursor {
    fn done(&self) -> bool {
        self.done
    }

    fn pack(&mut self, draft: &MessageDraft, _: MaxInstructions) -> Result<MessageDraft, Error> {
        let mut output = draft.clone();
        self.factory.calls.set(self.factory.calls.get() + 1);

        match self.factory.behavior {
            Behavior::Unchanged => {}
            Behavior::Complete => self.done = true,
            Behavior::Append => {
                output.instructions.push(instruction(2));
                self.done = true;
            }
            Behavior::AlreadyDone => panic!("completed packer was called"),
            Behavior::Metadata => {
                output.payer = Address::from_bytes([9; 32]);
                self.done = true;
            }
            Behavior::Prefix => {
                output.instructions.clear();
                output.instructions.push(instruction(3));
                self.done = true;
            }
        }

        Ok(output)
    }
}

fn instruction(value: u8) -> Instruction {
    Instruction {
        program_address: Address::from_bytes([0; 32]),
        accounts: vec![],
        data: vec![value],
    }
}

fn draft() -> MessageDraft {
    MessageDraft {
        version: Version::V0,
        payer: Address::from_bytes([1; 32]),
        lifetime: Lifetime::Blockhash {
            blockhash: Hash::from_bytes([2; 32]),
            last_valid_block_height: 3,
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

struct Observed {
    planned: [Result<solana_kit_core::plans::TransactionPlan, Error>; 2],
    appended: Result<MessageDraft, Error>,
    calls: usize,
}

fn observe(behavior: Behavior, initial: &MessageDraft) -> Observed {
    let maximum = MaxInstructions::default();
    let calls = Rc::new(Cell::new(0));
    let packer = Plan::Single(InstructionLeaf::Packer(Factory {
        behavior,
        calls: calls.clone(),
    }));
    let mut leaves: Vec<_> = initial
        .instructions
        .iter()
        .cloned()
        .map(Into::into)
        .collect();
    leaves.push(packer.clone());
    let input = Plan::sequential(leaves);
    let sync = Planner::new(Builder, maximum).plan(&input);
    let mut planner = Planner::new(Builder, maximum);
    let future = pin!(planner.plan_async(&input));

    let Poll::Ready(asynchronous) = future.poll(&mut Context::from_waker(Waker::noop())) else {
        panic!("ready builder unexpectedly pending");
    };

    let appended = append_instruction_plan(&packer, initial, maximum);

    Observed {
        planned: [sync, asynchronous],
        appended,
        calls: calls.get(),
    }
}

#[test]
fn completing_packers_must_append_work_in_every_entrypoint() {
    let maximum = MaxInstructions::default();
    let mut original = draft();
    original.instructions.push(instruction(1));

    for (behavior, expected) in [
        (Behavior::Unchanged, Error::NoProgress),
        (Behavior::Complete, Error::NoProgress),
        (Behavior::Metadata, Error::InvalidPackerOutput),
        (Behavior::Prefix, Error::InvalidPackerOutput),
    ] {
        let actual = observe(behavior, &original);
        assert_eq!(
            actual.planned,
            [Err(expected.clone()), Err(expected.clone())]
        );
        assert_eq!(actual.appended, Err(expected));
        assert_eq!(actual.calls, 3);
    }

    for (behavior, instructions, calls) in [
        (Behavior::Append, vec![instruction(1), instruction(2)], 3),
        (Behavior::AlreadyDone, vec![instruction(1)], 0),
    ] {
        let actual = observe(behavior, &original);
        let mut expected = original.clone();
        expected.instructions = instructions;
        let compiled = expected.compile(maximum).unwrap();

        assert_eq!(
            actual.planned,
            [
                Ok(Plan::Single(compiled.clone())),
                Ok(Plan::Single(compiled))
            ]
        );
        assert_eq!(actual.appended, Ok(expected));
        assert_eq!(actual.calls, calls);
    }

    let empty = observe(Behavior::Complete, &draft());
    assert_eq!(
        empty.planned,
        [Err(Error::NoProgress), Err(Error::NoProgress)]
    );
    assert_eq!(empty.appended, Err(Error::NoProgress));
    assert_eq!(empty.calls, 3);
    assert!(!Error::NoProgress.requires_new_candidate());
    assert_eq!(original.instructions, vec![instruction(1)]);
    assert_eq!(original.lifetime, draft().lifetime);
}
