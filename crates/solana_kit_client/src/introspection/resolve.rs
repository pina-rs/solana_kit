//! Resolve references once against the validated account table.

use crate::Presence;
use crate::methods::transaction_results::{self as rpc, TransactionView};
use solana_kit_core::{AccountMeta, Address, Instruction};

use super::{
    DecodedRpcTransaction, Error, InstructionTrace, ResolvedInstruction, TracedInstruction,
};

pub(super) fn outer<'a>(
    decoded: &DecodedRpcTransaction<'a>,
) -> Result<Vec<TracedInstruction<'a>>, Error> {
    let instructions = outer_instructions(decoded)?;

    Ok(instructions
        .into_iter()
        .enumerate()
        .map(|(index, instruction)| TracedInstruction {
            instruction,
            trace: InstructionTrace::Outer {
                index,
                stack_height: outer_depth(decoded, index),
            },
        })
        .collect())
}

/// Resolves each wire view against the same validated account table.
fn outer_instructions<'a>(
    decoded: &DecodedRpcTransaction<'a>,
) -> Result<Vec<ResolvedInstruction<'a>>, Error> {
    let instructions = match &decoded.source.transaction {
        TransactionView::Binary(_) => decoded
            .transaction
            .as_ref()
            .ok_or(Error::Header)?
            .message()
            .instructions()
            .iter()
            .map(|instruction| {
                compiled(
                    instruction.program_index(),
                    instruction.accounts(),
                    instruction.data(),
                    &decoded.accounts,
                )
            })
            .collect::<Result<Vec<_>, _>>()?,
        TransactionView::Json(value) => value
            .message
            .instructions
            .iter()
            .map(|instruction| {
                compiled(
                    instruction.program_id_index,
                    &instruction.accounts,
                    &instruction.data,
                    &decoded.accounts,
                )
            })
            .collect::<Result<Vec<_>, _>>()?,
        TransactionView::JsonParsed(value) => value
            .message
            .instructions
            .iter()
            .map(|instruction| resolved(instruction, &decoded.accounts))
            .collect::<Result<Vec<_>, _>>()?,
        TransactionView::Accounts(_) => return Err(Error::AccountsOnly),
    };

    Ok(instructions)
}

fn outer_depth(decoded: &DecodedRpcTransaction<'_>, index: usize) -> Presence<Option<u32>> {
    let height = match &decoded.source.transaction {
        TransactionView::Json(value) => &value.message.instructions[index].stack_height,
        TransactionView::JsonParsed(value) => depth(&value.message.instructions[index]),
        _ => return Presence::Missing,
    };

    copy_depth(height)
}

fn copy_depth(height: &Presence<Option<u32>>) -> Presence<Option<u32>> {
    match height {
        Presence::Missing => Presence::Missing,
        Presence::Present(value) => Presence::Present(*value),
    }
}

pub(super) fn compiled<'a>(
    program: u8,
    indices: &[u8],
    data: &[u8],
    accounts: &[AccountMeta],
) -> Result<ResolvedInstruction<'a>, Error> {
    let program_address = accounts
        .get(usize::from(program))
        .ok_or(Error::Index)?
        .address;
    let accounts = indices
        .iter()
        .map(|index| {
            accounts
                .get(usize::from(*index))
                .copied()
                .ok_or(Error::Index)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ResolvedInstruction::Raw(Instruction {
        program_address,
        accounts,
        data: data.to_vec(),
    }))
}

fn account(address: Address, accounts: &[AccountMeta]) -> Result<AccountMeta, Error> {
    let mut matches = accounts.iter().filter(|account| account.address == address);
    let first = matches.next().copied().ok_or(Error::Address)?;

    if matches.any(|account| account.role != first.role) {
        return Err(Error::Address);
    }

    Ok(first)
}

fn resolved<'a>(
    instruction: &'a rpc::ParsedOrPartialInstruction,
    accounts: &[AccountMeta],
) -> Result<ResolvedInstruction<'a>, Error> {
    match instruction {
        rpc::ParsedOrPartialInstruction::Parsed(instruction) => {
            account(instruction.program_id, accounts)?;

            if !instruction.parsed.get().trim_start().starts_with('{') {
                return Err(Error::ParsedData);
            }

            Ok(ResolvedInstruction::Parsed(instruction))
        }
        rpc::ParsedOrPartialInstruction::Partial(instruction) => {
            account(instruction.program_id, accounts)?;
            let accounts = instruction
                .accounts
                .iter()
                .map(|address| account(*address, accounts))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(ResolvedInstruction::Raw(Instruction {
                program_address: instruction.program_id,
                accounts,
                data: instruction.data.clone(),
            }))
        }
    }
}

pub(super) fn inner<'a>(
    decoded: &DecodedRpcTransaction<'a>,
) -> Result<Vec<TracedInstruction<'a>>, Error> {
    inner_groups(
        decoded.metadata(),
        &decoded.accounts,
        Some(outer(decoded)?.len()),
    )
}

/// Rejects duplicate or out-of-range groups before preserving their trace positions.
pub(super) fn inner_groups<'a>(
    meta: Option<&'a rpc::TransactionMeta>,
    accounts: &[AccountMeta],
    outer_count: Option<usize>,
) -> Result<Vec<TracedInstruction<'a>>, Error> {
    let Some(meta) = meta else {
        return Ok(Vec::new());
    };
    let Presence::Present(Some(groups)) = &meta.inner_instructions else {
        return Ok(Vec::new());
    };
    let mut seen = std::collections::BTreeSet::new();
    let mut result = Vec::new();

    for group in groups {
        let outer_index = usize::from(group.index);

        if outer_count.is_some_and(|count| outer_index >= count) || !seen.insert(outer_index) {
            return Err(Error::InnerGroup);
        }

        for (inner_index, instruction) in group.instructions.iter().enumerate() {
            let (instruction, stack_height) = inner_instruction(instruction, accounts)?;
            result.push(TracedInstruction {
                instruction,
                trace: InstructionTrace::Inner {
                    outer_index,
                    inner_index,
                    stack_height,
                },
            });
        }
    }

    Ok(result)
}

fn depth(instruction: &rpc::ParsedOrPartialInstruction) -> &Presence<Option<u32>> {
    match instruction {
        rpc::ParsedOrPartialInstruction::Parsed(value) => &value.stack_height,
        rpc::ParsedOrPartialInstruction::Partial(value) => &value.stack_height,
    }
}

fn inner_instruction<'a>(
    instruction: &'a rpc::InnerInstruction,
    accounts: &[AccountMeta],
) -> Result<(ResolvedInstruction<'a>, Presence<Option<u32>>), Error> {
    let (instruction, height) = match instruction {
        rpc::InnerInstruction::Compiled(value) => (
            compiled(
                value.program_id_index,
                &value.accounts,
                &value.data,
                accounts,
            )?,
            &value.stack_height,
        ),
        rpc::InnerInstruction::Resolved(value) => (resolved(value, accounts)?, depth(value)),
    };

    let height = copy_depth(height);

    Ok((instruction, height))
}

/// Interleaves each outer instruction with its recorded inner group in wire order.
pub(super) fn walk<'a>(
    decoded: &DecodedRpcTransaction<'a>,
) -> Result<Vec<TracedInstruction<'a>>, Error> {
    let mut groups = std::collections::BTreeMap::<usize, Vec<_>>::new();

    for instruction in inner(decoded)? {
        if let InstructionTrace::Inner { outer_index, .. } = instruction.trace {
            groups.entry(outer_index).or_default().push(instruction);
        }
    }

    let mut result = Vec::new();

    for (index, instruction) in outer(decoded)?.into_iter().enumerate() {
        result.push(instruction);

        if let Some(inner) = groups.remove(&index) {
            result.extend(inner);
        }
    }

    Ok(result)
}
