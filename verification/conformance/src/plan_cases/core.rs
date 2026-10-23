extern crate std;
use solana_kit_core::{
    Address, Hash, Instruction,
    plans::{
        Error, InstructionBatch, InstructionLeaf, InstructionPlan, Linear, MaxInstructions,
        MessageBuilder, MessageDraft, MessagePacker, PackerFactory, Plan, PlanInput, Planner,
        append_instruction_plan,
    },
    transaction::{self, Lifetime, Transaction, Version},
};
use std::format;
use std::prelude::v1::*;
use std::vec;

const LARGE_INSTRUCTION: usize = 800;
const OVERSIZED_INSTRUCTION: usize = 5000;
const DATA_MODULUS: usize = 251;
const REALLOC_TOTAL: usize = 20_481;
const BASE64_BITS: u32 = 6;
const BYTE_BITS: u32 = 8;

fn instruction(size: usize) -> Instruction {
    Instruction {
        program_address: Address::from_bytes([0; 32]),
        accounts: vec![],
        data: vec![0; size],
    }
}
fn draft() -> MessageDraft {
    MessageDraft {
        version: Version::V0,
        payer: "E9Nykp3rSdza2moQutaJ3K3RSC8E5iFERX2SqLTsQfjJ"
            .parse()
            .unwrap(),
        lifetime: Lifetime::Blockhash {
            blockhash: Hash::from_bytes([0; 32]),
            last_valid_block_height: 1,
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
fn leaf(size: usize) -> InstructionPlan {
    instruction(size).into()
}
fn shape<T>(plan: &Plan<T>, leaf: &impl Fn(&T) -> String) -> String {
    match plan {
        Plan::Single(value) => leaf(value),
        Plan::Sequential { plans, divisible } => format!(
            "{}({})",
            if *divisible { "S" } else { "A" },
            plans
                .iter()
                .map(|plan| shape(plan, leaf))
                .collect::<Vec<_>>()
                .join(",")
        ),
        Plan::Parallel(plans) => format!(
            "P({})",
            plans
                .iter()
                .map(|plan| shape(plan, leaf))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}
pub(super) fn actual_dart_shapes_and_wire_match() -> Result<(), &'static str> {
    let cases = [
        (
            "sequential-count",
            Plan::sequential((0..5).map(|_| leaf(0)).collect()),
        ),
        (
            "parallel-count",
            Plan::Parallel((0..5).map(|_| leaf(0)).collect()),
        ),
        (
            "sequential-bytes",
            Plan::sequential(vec![
                leaf(LARGE_INSTRUCTION),
                leaf(100),
                leaf(LARGE_INSTRUCTION),
            ]),
        ),
        (
            "atomic-bytes",
            Plan::indivisible(vec![leaf(LARGE_INSTRUCTION), leaf(LARGE_INSTRUCTION)]),
        ),
        (
            "packed-bytes",
            Plan::sequential(vec![
                leaf(LARGE_INSTRUCTION),
                Plan::Single(InstructionLeaf::Packer(InstructionBatch(vec![
                    instruction(100),
                    instruction(LARGE_INSTRUCTION),
                ]))),
            ]),
        ),
        (
            "mixed",
            Plan::sequential(vec![
                Plan::Parallel(vec![leaf(LARGE_INSTRUCTION), leaf(LARGE_INSTRUCTION)]),
                leaf(100),
            ]),
        ),
    ];
    let mut oracle = include_str!("../../../instruction-plans/oracle.tsv").lines();

    for (name, input) in cases {
        let plan = Planner::new(Builder, MaxInstructions::new(2).unwrap())
            .plan(&input)
            .unwrap();
        super::assert_eq!(
            oracle.next().unwrap(),
            format!(
                "{name}\t{}",
                shape(&plan, &|message| message.instructions().len().to_string())
            )
        );

        for message in plan.flatten() {
            let encoded = oracle.next().unwrap().strip_prefix("wire\t").unwrap();
            super::assert_eq!(decode_base64(encoded), message.bytes());
            super::assert!(
                Transaction::new(message.clone())
                    .encode(false)
                    .unwrap()
                    .len()
                    <= transaction::PACKET_SIZE
            );
            super::assert_eq!(message.lifetime(), draft().lifetime);
        }
    }

    Ok(())
}
fn decode_base64(input: &str) -> Vec<u8> {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = Vec::new();
    let mut buffer = 0_u32;
    let mut bits = 0;

    for byte in input.bytes().filter(|byte| *byte != b'=') {
        buffer = (buffer << BASE64_BITS)
            | u32::try_from(alphabet.iter().position(|value| *value == byte).unwrap()).unwrap();
        bits += BASE64_BITS;

        if bits >= BYTE_BITS {
            bits -= BYTE_BITS;
            output.push((buffer >> bits) as u8);
        }
    }

    output
}
pub(super) fn tree_inputs_search_transform_and_flatten_preserve_boundaries()
-> Result<(), &'static str> {
    let plan = PlanInput::List(vec![PlanInput::List(vec![PlanInput::Value(1)])]).parse();
    super::assert_eq!(plan, Plan::Single(1));
    let plan = Plan::indivisible(vec![Plan::Single(1), Plan::Parallel(vec![Plan::Single(2)])]);
    super::assert_eq!(plan.flatten(), vec![&1, &2]);
    super::assert!(
        plan.find(&|node| matches!(node, Plan::Parallel(_)))
            .is_some()
    );
    super::assert!(!plan.every(&|node| matches!(node, Plan::Single(_))));
    let mut order = Vec::new();
    let transformed = plan.transform(&mut |node| {
        order.push(node.flatten().len());
        node
    });

    super::assert_eq!(order, [1, 1, 1, 2]);
    super::assert!(matches!(
        transformed,
        Plan::Sequential {
            divisible: false,
            ..
        }
    ));
    super::assert_eq!(
        PlanInput::<u8>::List(vec![]).parse(),
        Plan::sequential(vec![])
    );
    Ok(())
}
pub(super) fn empty_oversized_and_unresolved_fail_without_partial_output()
-> Result<(), &'static str> {
    let mut planner = Planner::new(Builder, MaxInstructions::default());
    super::assert_eq!(
        planner.plan(&Plan::<InstructionLeaf>::Parallel(vec![])),
        Err(Error::EmptyPlan)
    );
    super::assert_eq!(
        planner.plan(&Plan::sequential(vec![
            leaf(1),
            leaf(OVERSIZED_INSTRUCTION)
        ])),
        Err(Error::Compile(transaction::Error::PacketTooLarge))
    );
    let mut invalid = draft();
    invalid.lifetime = Lifetime::Unresolved(Hash::from_bytes([0; 32]));
    super::assert_eq!(
        invalid.compile(MaxInstructions::default()),
        Err(Error::Compile(transaction::Error::UnresolvedLifetime))
    );
    super::assert!(MaxInstructions::new(0).is_err());
    super::assert!(MaxInstructions::new(65).is_err());
    Ok(())
}

pub(super) fn greedy_packer_does_not_consume_rejected_work() -> Result<(), &'static str> {
    let mut packer = InstructionBatch(vec![
        instruction(LARGE_INSTRUCTION),
        instruction(LARGE_INSTRUCTION),
    ])
    .create();
    let mut full = draft();
    full.instructions.push(instruction(1000));
    super::assert!(packer.pack(&full, MaxInstructions::default()).is_err());
    super::assert!(!packer.done());
    super::assert_eq!(
        packer
            .pack(&draft(), MaxInstructions::default())
            .unwrap()
            .instructions
            .len(),
        1
    );
    super::assert!(!packer.done());
    super::assert_eq!(
        packer
            .pack(&draft(), MaxInstructions::default())
            .unwrap()
            .instructions
            .len(),
        1
    );

    super::assert!(packer.done());
    super::assert_eq!(
        packer.pack(&draft(), MaxInstructions::default()),
        Err(Error::PackerComplete)
    );
    Ok(())
}
pub(super) fn exact_packet_boundary_and_linear_chunks_use_compiler() -> Result<(), &'static str> {
    let maximum = MaxInstructions::default();
    let mut exact = draft();
    let size = (0..transaction::PACKET_SIZE)
        .rev()
        .find(|size| {
            exact.instructions = vec![instruction(*size)];
            exact.compile(maximum).is_ok()
        })
        .unwrap();
    super::assert_eq!(
        Transaction::new(exact.compile(maximum).unwrap())
            .encode(false)
            .unwrap()
            .len(),
        transaction::PACKET_SIZE
    );
    exact.instructions = vec![instruction(size + 1)];
    super::assert!(exact.compile(maximum).is_err());
    let factory = Linear {
        total_length: 3000,
        instruction: |offset, length| {
            let mut value = instruction(length);
            value
                .data
                .fill(u8::try_from(offset % DATA_MODULUS).unwrap());
            value
        },
    };
    let mut packer = factory.create();

    let mut total = 0;

    while !packer.done() {
        let message = packer.pack(&draft(), maximum).unwrap();
        total += message.instructions[0].data.len();
        message.compile(maximum).unwrap();
    }

    super::assert_eq!(total, 3000);
    let chunks = InstructionBatch::realloc(REALLOC_TOTAL, instruction);
    super::assert_eq!(
        chunks
            .0
            .iter()
            .map(|instruction| instruction.data.len())
            .collect::<Vec<_>>(),
        [10_240, 10_240, 1]
    );
    Ok(())
}
struct FailingBuilder {
    updates: usize,
}
impl MessageBuilder for FailingBuilder {
    fn create(&mut self) -> Result<MessageDraft, Error> {
        Ok(draft())
    }
    fn update(&mut self, mut draft: MessageDraft) -> Result<MessageDraft, Error> {
        self.updates += 1;

        if self.updates == 2 {
            draft.instructions.push(instruction(OVERSIZED_INSTRUCTION));
        }

        Ok(draft)
    }
}
pub(super) fn stateful_update_overflow_aborts_and_append_is_transactional()
-> Result<(), &'static str> {
    let input = Plan::sequential(vec![
        leaf(LARGE_INSTRUCTION),
        Plan::Single(InstructionLeaf::Packer(InstructionBatch(vec![
            instruction(100),
            instruction(LARGE_INSTRUCTION),
        ]))),
    ]);
    super::assert!(
        Planner::new(FailingBuilder { updates: 0 }, MaxInstructions::default())
            .plan(&input)
            .is_err()
    );
    let original = draft();
    super::assert!(
        append_instruction_plan(
            &Plan::sequential(vec![leaf(1), leaf(OVERSIZED_INSTRUCTION)]),
            &original,
            MaxInstructions::default()
        )
        .is_err()
    );
    super::assert!(original.instructions.is_empty());
    Ok(())
}

#[derive(Clone)]
struct BadPacker(u8);
impl PackerFactory for BadPacker {
    type Packer = Self;
    fn create(&self) -> Self {
        self.clone()
    }
}
impl MessagePacker for BadPacker {
    fn done(&self) -> bool {
        false
    }
    fn pack(&mut self, draft: &MessageDraft, _: MaxInstructions) -> Result<MessageDraft, Error> {
        let mut output = draft.clone();

        match self.0 {
            0 => {}
            1 => output.payer = Address::from_bytes([9; 32]),
            _ => output.instructions.push(instruction(OVERSIZED_INSTRUCTION)),
        }

        Ok(output)
    }
}
pub(super) fn invalid_custom_packer_cannot_spin_lose_work_or_change_metadata()
-> Result<(), &'static str> {
    for (behavior, expected) in [
        (0, Error::NoProgress),
        (1, Error::InvalidPackerOutput),
        (
            2,
            Error::PackedMessage(Box::new(Error::Compile(transaction::Error::PacketTooLarge))),
        ),
    ] {
        let plan = Plan::Single(InstructionLeaf::Packer(BadPacker(behavior)));
        super::assert_eq!(
            Planner::new(Builder, MaxInstructions::default()).plan(&plan),
            Err(expected)
        );
    }

    Ok(())
}
pub(super) fn v1_capacity_and_instruction_limits_are_exact_and_version_specific()
-> Result<(), &'static str> {
    let mut version_one = draft();
    version_one.version = Version::V1;
    let maximum = MaxInstructions::new(64).unwrap();
    let size = (0..transaction::V1_PACKET_SIZE)
        .rev()
        .find(|size| {
            version_one.instructions = vec![instruction(*size)];
            version_one.compile(maximum).is_ok()
        })
        .unwrap();
    let compiled = version_one.compile(maximum).unwrap();
    super::assert_eq!(
        Transaction::new(compiled).encode(false).unwrap().len(),
        transaction::V1_PACKET_SIZE
    );
    version_one.instructions = vec![instruction(size + 1)];
    super::assert!(version_one.compile(maximum).is_err());

    version_one.instructions = vec![instruction(0); maximum.get()];
    super::assert!(version_one.compile(maximum).is_ok());
    version_one.instructions.push(instruction(0));
    super::assert!(matches!(
        version_one.compile(maximum),
        Err(Error::InstructionLimit {
            actual: 65,
            maximum: 64
        })
    ));
    Ok(())
}

pub(super) fn invocation_budget_does_not_leak_into_later_plans() -> Result<(), &'static str> {
    let mut planner = Planner::new(Builder, MaxInstructions::new(2).unwrap());
    let input = Plan::sequential(vec![leaf(0), leaf(0), leaf(0)]);
    super::assert_eq!(
        planner
            .plan_with_maximum(&input, MaxInstructions::new(3).unwrap())
            .unwrap()
            .flatten()
            .len(),
        1
    );
    super::assert_eq!(planner.plan(&input).unwrap().flatten().len(), 2);
    Ok(())
}

struct BrokenBuilder;
impl MessageBuilder for BrokenBuilder {
    fn create(&mut self) -> Result<MessageDraft, Error> {
        Err(Error::Builder("resource unavailable".into()))
    }
}
pub(super) fn builder_failure_preserves_original_cause_without_returning_a_partial_plan()
-> Result<(), &'static str> {
    let error = Planner::new(BrokenBuilder, MaxInstructions::default())
        .plan(&leaf(0))
        .unwrap_err();
    super::assert_eq!(error, Error::Builder("resource unavailable".into()));
    super::assert!(!error.requires_new_candidate());
    Ok(())
}
#[derive(Clone)]
struct RejectingFactory(bool);
struct RejectingPacker {
    next: usize,
    always_reject: bool,
}
impl PackerFactory for RejectingFactory {
    type Packer = RejectingPacker;
    fn create(&self) -> Self::Packer {
        RejectingPacker {
            next: 0,
            always_reject: self.0,
        }
    }
}
impl MessagePacker for RejectingPacker {
    fn done(&self) -> bool {
        self.next == 2
    }
    fn pack(
        &mut self,
        draft: &MessageDraft,
        maximum: MaxInstructions,
    ) -> Result<MessageDraft, Error> {
        if self.always_reject || !draft.instructions.is_empty() {
            return Err(Error::Rejected("requires empty message".into()));
        }

        let mut output = draft.clone();
        output.instructions.push(instruction(self.next));
        output.compile(maximum)?;
        self.next += 1;

        Ok(output)
    }
}
pub(super) fn custom_rejections_try_fresh_candidates_and_factories_restart_each_invocation()
-> Result<(), &'static str> {
    let input: InstructionPlan<RejectingFactory> = Plan::sequential(vec![
        instruction(0).into(),
        Plan::Single(InstructionLeaf::Packer(RejectingFactory(false))),
    ]);
    let mut planner = Planner::new(Builder, MaxInstructions::default());
    super::assert_eq!(planner.plan(&input).unwrap().flatten().len(), 3);
    super::assert_eq!(planner.plan(&input).unwrap().flatten().len(), 3);
    let rejecting = Plan::Single(InstructionLeaf::Packer(RejectingFactory(true)));
    super::assert_eq!(
        planner.plan(&rejecting),
        Err(Error::Rejected("requires empty message".into()))
    );
    Ok(())
}
