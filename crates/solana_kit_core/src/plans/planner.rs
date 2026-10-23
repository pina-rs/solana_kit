//! Candidate arena and deterministic traversal; indices replace shared mutable trees.
use super::{
    AsyncMessageBuilder, Error, InstructionLeaf, InstructionPlan, MaxInstructions, MessageBuilder,
    MessageDraft, MessagePacker, PackerFactory, Plan, TransactionPlan,
};
use alloc::{vec, vec::Vec};
use core::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

/// One invocation's candidate owner. The builder and packers never outlive planning.
pub struct Planner<B> {
    builder: B,
    maximum: MaxInstructions,
}
impl<B> Planner<B> {
    /// Creates a planner with an explicit construction policy and instruction budget.
    pub const fn new(builder: B, maximum: MaxInstructions) -> Self {
        Self { builder, maximum }
    }
}
impl<B: MessageBuilder> Planner<B> {
    /// Plans synchronously using the same traversal as asynchronous policies.
    ///
    /// # Errors
    /// Returns construction, compilation, empty-plan and packing failures transactionally.
    pub fn plan<F: PackerFactory>(
        &mut self,
        plan: &InstructionPlan<F>,
    ) -> Result<TransactionPlan, Error> {
        self.plan_with_maximum(plan, self.maximum)
    }

    /// Overrides the instruction budget for this invocation.
    ///
    /// # Errors
    /// Returns the same failures as [`Self::plan`].
    pub fn plan_with_maximum<F: PackerFactory>(
        &mut self,
        plan: &InstructionPlan<F>,
        maximum: MaxInstructions,
    ) -> Result<TransactionPlan, Error> {
        let future = pin!(self.plan_async_with_maximum(plan, maximum));

        // The blanket adapter contains only Ready futures. This is one poll of
        // that closed synchronous adapter, not an executor for arbitrary futures.
        match future.poll(&mut Context::from_waker(Waker::noop())) {
            Poll::Ready(result) => result,
            Poll::Pending => unreachable!("synchronous builder adapter is always ready"),
        }
    }
}
impl<B: AsyncMessageBuilder> Planner<B> {
    /// Awaits caller-owned construction and update futures at candidate decisions.
    ///
    /// # Errors
    /// No partial tree escapes any builder, compilation or packing failure.
    pub async fn plan_async<F: PackerFactory>(
        &mut self,
        plan: &InstructionPlan<F>,
    ) -> Result<TransactionPlan, Error> {
        self.plan_async_with_maximum(plan, self.maximum).await
    }

    /// Awaits planning with an invocation-only instruction budget.
    ///
    /// # Errors
    /// Returns the same transactional failures as [`Self::plan_async`].
    pub async fn plan_async_with_maximum<F: PackerFactory>(
        &mut self,
        plan: &InstructionPlan<F>,
        maximum: MaxInstructions,
    ) -> Result<TransactionPlan, Error> {
        let mut traversal = Traversal {
            builder: &mut self.builder,
            maximum,
            messages: Vec::new(),
        };
        let tree = traversal.visit(plan).await?.ok_or(Error::EmptyPlan)?;
        let compiled = traversal
            .messages
            .iter()
            .map(|draft| draft.compile(maximum))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tree.map(&mut |index| compiled[index].clone()))
    }
}

#[derive(Clone, Copy)]
enum Parent {
    Root,
    Sequential(bool),
    Parallel,
}
struct Traversal<'a, B> {
    builder: &'a mut B,
    maximum: MaxInstructions,
    messages: Vec<MessageDraft>,
}
struct Group<'a, F> {
    remaining: alloc::vec::IntoIter<&'a InstructionPlan<F>>,
    kind: Parent,
    candidates: Vec<usize>,
    output: Vec<Plan<usize>>,
}
enum Task<'a, F> {
    Enter(&'a InstructionPlan<F>, Parent, Vec<usize>),
    Children(Group<'a, F>),
    Resume(Group<'a, F>),
}
impl<B: AsyncMessageBuilder> Traversal<'_, B> {
    async fn visit<F: PackerFactory>(
        &mut self,
        plan: &InstructionPlan<F>,
    ) -> Result<Option<Plan<usize>>, Error> {
        let mut tasks = vec![Task::Enter(plan, Parent::Root, Vec::new())];
        let mut result = None;

        while let Some(task) = tasks.pop() {
            match task {
                Task::Enter(plan, parent, candidates) => {
                    result = self.enter(plan, parent, candidates, &mut tasks).await?;
                }
                Task::Children(group) => result = group.schedule(&mut tasks),
                Task::Resume(mut group) => {
                    group.accept(result.take());
                    tasks.push(Task::Children(group));
                }
            }
        }

        Ok(result)
    }

    async fn enter<'a, F: PackerFactory>(
        &mut self,
        plan: &'a InstructionPlan<F>,
        parent: Parent,
        candidates: Vec<usize>,
        tasks: &mut Vec<Task<'a, F>>,
    ) -> Result<Option<Plan<usize>>, Error> {
        match plan {
            Plan::Single(InstructionLeaf::Instruction(instruction)) => {
                self.leaf(
                    &candidates,
                    |draft| {
                        let mut output = draft.clone();
                        output.instructions.push(instruction.clone());
                        Ok(output)
                    },
                    false,
                )
                .await
            }
            Plan::Single(InstructionLeaf::Packer(factory)) => {
                self.packer(factory, parent, &candidates).await
            }
            Plan::Sequential { plans, divisible } => {
                self.sequential(plans, *divisible, parent, candidates, tasks)
                    .await?;
                Ok(None)
            }
            Plan::Parallel(plans) => {
                let ordered =
                    plans
                        .iter()
                        .filter(|plan| !matches!(plan, Plan::Single(InstructionLeaf::Packer(_))))
                        .chain(plans.iter().filter(|plan| {
                            matches!(plan, Plan::Single(InstructionLeaf::Packer(_)))
                        }))
                        .collect::<Vec<_>>();
                tasks.push(Task::Children(Group {
                    remaining: ordered.into_iter(),
                    kind: Parent::Parallel,
                    candidates,
                    output: Vec::new(),
                }));
                Ok(None)
            }
        }
    }

    async fn sequential<'a, F: PackerFactory>(
        &mut self,
        plans: &'a [InstructionPlan<F>],
        divisible: bool,
        parent: Parent,
        candidates: Vec<usize>,
        tasks: &mut Vec<Task<'a, F>>,
    ) -> Result<(), Error> {
        let whole =
            !matches!(parent, Parent::Root) && (matches!(parent, Parent::Parallel) || !divisible);
        let maximum = self.maximum;

        if whole
            && self
                .select(
                    &candidates,
                    |draft| append_all(plans, draft, maximum),
                    false,
                )
                .await?
                .is_some()
        {
            return Ok(());
        }

        let candidates = if whole {
            Vec::new()
        } else {
            candidates.into_iter().take(1).collect()
        };
        tasks.push(Task::Children(Group {
            remaining: plans.iter().collect::<Vec<_>>().into_iter(),
            kind: Parent::Sequential(divisible),
            candidates,
            output: Vec::new(),
        }));
        Ok(())
    }

    async fn packer<F: PackerFactory>(
        &mut self,
        factory: &F,
        parent: Parent,
        candidates: &[usize],
    ) -> Result<Option<Plan<usize>>, Error> {
        let mut packer = factory.create();
        let mut output = Vec::new();
        let maximum = self.maximum;

        while !packer.done() {
            let pack = |draft: &MessageDraft| pack_checked(&mut packer, draft, maximum);

            if let Some(child) = self.leaf(candidates, pack, true).await? {
                output.push(child);
            }
        }

        let divisible = match parent {
            Parent::Parallel => None,
            Parent::Sequential(value) => Some(value),
            Parent::Root => Some(true),
        };

        Ok(collapse(output, divisible))
    }

    async fn leaf(
        &mut self,
        candidates: &[usize],
        mut append: impl FnMut(&MessageDraft) -> Result<MessageDraft, Error>,
        stateful: bool,
    ) -> Result<Option<Plan<usize>>, Error> {
        if self
            .select(candidates, &mut append, stateful)
            .await?
            .is_some()
        {
            return Ok(None);
        }

        let fresh = self.builder.create_async().await?;
        let proposed = append(&fresh)?;
        let updated = self.update_checked(proposed).await?;
        updated.compile(self.maximum)?;
        let index = self.messages.len();
        self.messages.push(updated);

        Ok(Some(Plan::Single(index)))
    }

    async fn select(
        &mut self,
        candidates: &[usize],
        mut append: impl FnMut(&MessageDraft) -> Result<MessageDraft, Error>,
        stateful: bool,
    ) -> Result<Option<usize>, Error> {
        for &index in candidates {
            if let Some(message) = self.propose(index, &mut append, stateful).await? {
                self.messages[index] = message;

                return Ok(Some(index));
            }
        }

        Ok(None)
    }

    async fn update_checked(&mut self, proposed: MessageDraft) -> Result<MessageDraft, Error> {
        let updated = self.builder.update_async(proposed.clone()).await?;

        if !updated.instructions.starts_with(&proposed.instructions) {
            return Err(Error::InvalidPackerOutput);
        }

        updated.compile(self.maximum)?;
        Ok(updated)
    }

    // A consumed packer cannot be retried after an update failure: that would
    // silently discard its instructions. Pure proposals may try another draft.
    async fn propose(
        &mut self,
        index: usize,
        append: &mut impl FnMut(&MessageDraft) -> Result<MessageDraft, Error>,
        stateful: bool,
    ) -> Result<Option<MessageDraft>, Error> {
        let proposed = match append(&self.messages[index]) {
            Ok(proposed) => proposed,

            Err(error) if error.requires_new_candidate() => return Ok(None),
            Err(error) => return Err(error),
        };

        let updated = self.update_checked(proposed).await;

        match updated {
            Ok(message) => Ok(Some(message)),

            Err(error) if !stateful && error.requires_new_candidate() => Ok(None),
            Err(error) => Err(error),
        }
    }
}
impl<'a, F> Group<'a, F> {
    fn schedule(mut self, tasks: &mut Vec<Task<'a, F>>) -> Option<Plan<usize>> {
        let Some(child) = self.remaining.next() else {
            let divisible = match self.kind {
                Parent::Sequential(value) => Some(value),
                _ => None,
            };

            return collapse(self.output, divisible);
        };
        let parent = self.kind;
        let candidates = self.candidates.clone();
        tasks.push(Task::Resume(self));
        tasks.push(Task::Enter(child, parent, candidates));
        None
    }
    fn accept(&mut self, child: Option<Plan<usize>>) {
        let Some(child) = child else {
            return;
        };

        match self.kind {
            Parent::Sequential(divisible) => {
                self.candidates = sequential_candidate(&child).into_iter().collect();
                push_sequential(&mut self.output, child, divisible);
            }
            _ => {
                self.candidates.extend(child.flatten().into_iter().copied());
                push_parallel(&mut self.output, child);
            }
        }
    }
}
fn collapse(mut plans: Vec<Plan<usize>>, divisible: Option<bool>) -> Option<Plan<usize>> {
    match plans.len() {
        0 => None,
        1 => plans.pop(),
        _ => Some(match divisible {
            Some(divisible) => Plan::Sequential { plans, divisible },
            None => Plan::Parallel(plans),
        }),
    }
}
fn sequential_candidate(plan: &Plan<usize>) -> Option<usize> {
    match plan {
        Plan::Single(index) => Some(*index),
        Plan::Sequential { plans, .. } => plans.last().and_then(sequential_candidate),
        Plan::Parallel(_) => None,
    }
}

/// Appends every leaf in depth-first order to a single draft.
///
/// # Errors
/// Rejects capacity overflow and invalid packer output without exposing partial output.
pub fn append_instruction_plan<F: PackerFactory>(
    plan: &InstructionPlan<F>,
    draft: &MessageDraft,
    maximum: MaxInstructions,
) -> Result<MessageDraft, Error> {
    append_all(core::slice::from_ref(plan), draft, maximum)
}
fn append_all<F: PackerFactory>(
    plans: &[InstructionPlan<F>],
    draft: &MessageDraft,
    maximum: MaxInstructions,
) -> Result<MessageDraft, Error> {
    let mut output = draft.clone();

    for leaf in plans.iter().flat_map(Plan::flatten) {
        output = append_leaf(leaf, output, maximum)?;
    }

    Ok(output)
}
fn pack_checked(
    packer: &mut impl MessagePacker,
    draft: &MessageDraft,
    maximum: MaxInstructions,
) -> Result<MessageDraft, Error> {
    let output = packer.pack(draft, maximum)?;
    let mut unchanged_metadata = output.clone();
    unchanged_metadata.instructions = draft.instructions.clone();

    if unchanged_metadata != *draft || !output.instructions.starts_with(&draft.instructions) {
        return Err(Error::InvalidPackerOutput);
    }

    if output.instructions == draft.instructions {
        return Err(Error::NoProgress);
    }

    output
        .compile(maximum)
        .map_err(|error| Error::PackedMessage(alloc::boxed::Box::new(error)))?;
    Ok(output)
}

fn push_sequential(output: &mut Vec<Plan<usize>>, child: Plan<usize>, divisible: bool) {
    match child {
        Plan::Sequential {
            plans,
            divisible: child_divisible,
        } if child_divisible || !divisible => output.extend(plans),
        child => output.push(child),
    }
}
fn push_parallel(output: &mut Vec<Plan<usize>>, child: Plan<usize>) {
    match child {
        Plan::Parallel(plans) => output.extend(plans),
        child => output.push(child),
    }
}
fn append_leaf<Factory: PackerFactory>(
    leaf: &InstructionLeaf<Factory>,
    mut draft: MessageDraft,
    maximum: MaxInstructions,
) -> Result<MessageDraft, Error> {
    match leaf {
        InstructionLeaf::Instruction(instruction) => {
            draft.instructions.push(instruction.clone());
            draft.compile(maximum)?;
        }
        InstructionLeaf::Packer(factory) => {
            let mut packer = factory.create();

            while !packer.done() {
                draft = pack_checked(&mut packer, &draft, maximum)?;
            }
        }
    }

    Ok(draft)
}
