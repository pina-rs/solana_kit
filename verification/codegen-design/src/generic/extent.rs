//! Checked widths and frame ownership for the finite normalized model.
use super::{Field, Program, Shape, Wire};
use crate::schema::Error;
const ADDRESS_BYTES: usize = 32;
const U32_BYTES: usize = 4;
const U64_BYTES: usize = 8;

#[derive(Clone, Copy)]
pub(super) struct Extent {
    pub(super) minimum: usize,
    pub(super) fixed: Option<usize>,
    pub(super) tail: bool,
}
impl Extent {
    fn fixed(size: usize) -> Self {
        Self {
            minimum: size,
            fixed: Some(size),
            tail: false,
        }
    }
    fn variable(minimum: usize, tail: bool) -> Self {
        Self {
            minimum,
            fixed: None,
            tail,
        }
    }
    fn followed_by(self, next: Self) -> Result<Self, Error> {
        if self.tail {
            return Err("unframed remainder before another field".into());
        }

        Ok(Self {
            minimum: self
                .minimum
                .checked_add(next.minimum)
                .ok_or("layout overflow")?,
            fixed: self
                .fixed
                .zip(next.fixed)
                .map(|(a, b)| a.checked_add(b).ok_or("layout overflow"))
                .transpose()?,
            tail: next.tail,
        })
    }
}

pub(super) fn fields(
    program: &Program,
    fields: &[Field],
    active: &mut Vec<String>,
) -> Result<Extent, Error> {
    fields
        .iter()
        .try_fold(Extent::fixed(0), |prior, field| {
            prior.followed_by(wire(program, &field.wire, active)?)
        })
        .and_then(portable)
}

/// Every recursive branch proves its byte extent and ownership of the frame end.
pub(super) fn wire(
    program: &Program,
    value: &Wire,
    active: &mut Vec<String>,
) -> Result<Extent, Error> {
    portable(wire_extent(program, value, active)?)
}

/// Static widths must be representable on every supported 32-bit Wasm target.
fn portable(output: Extent) -> Result<Extent, Error> {
    u32::try_from(output.minimum).map_err(|_| "wire extent exceeds portable u32 bounds")?;

    if let Some(fixed) = output.fixed {
        u32::try_from(fixed).map_err(|_| "fixed extent exceeds portable u32 bounds")?;
    }

    Ok(output)
}

fn wire_extent(program: &Program, value: &Wire, active: &mut Vec<String>) -> Result<Extent, Error> {
    match value {
        Wire::Number(name) => Ok(Extent::fixed(width(name)?)),
        Wire::Bool => Ok(Extent::fixed(1)),
        Wire::Address => Ok(Extent::fixed(ADDRESS_BYTES)),
        Wire::Text | Wire::Bytes => Ok(Extent::variable(0, true)),
        Wire::Link(name) => linked(program, name, active),
        Wire::Option {
            item,
            prefix,
            fixed,
        } => option(program, item, prefix, *fixed, active),
        Wire::Array { item, count } => array(program, item, *count, active),
        Wire::Prefix { item, prefix } => {
            wire(program, item, active)?;
            Ok(Extent::variable(width(prefix)?, false))
        }
    }
}
fn width(name: &str) -> Result<usize, Error> {
    match name {
        "u8" => Ok(1),
        "u32" => Ok(U32_BYTES),
        "u64" => Ok(U64_BYTES),
        _ => Err("unsupported integer width".into()),
    }
}
fn linked(program: &Program, name: &str, active: &mut Vec<String>) -> Result<Extent, Error> {
    if active.iter().any(|item| item == name) {
        return Err("cyclic linked layout".into());
    }

    active.push(name.to_owned());
    let definition = program
        .definitions
        .iter()
        .find(|item| item.name == name)
        .ok_or("unresolved linked layout")?;

    let result = match &definition.shape {
        Shape::Enum(_) => Extent::fixed(1),
        Shape::Struct(items) => fields(program, items, active)?,
    };

    active.pop();
    Ok(result)
}
fn option(
    program: &Program,
    item: &Wire,
    prefix: &str,
    fixed: bool,
    active: &mut Vec<String>,
) -> Result<Extent, Error> {
    owned(item)?;
    let inner = wire(program, item, active)?;
    let width = width(prefix)?;

    if !fixed {
        return Ok(Extent::variable(width, inner.tail));
    }

    let payload = inner
        .fixed
        .ok_or("fixed option payload must have fixed width")?;
    Ok(Extent::fixed(
        width.checked_add(payload).ok_or("layout overflow")?,
    ))
}
fn array(
    program: &Program,
    item: &Wire,
    count: Option<usize>,
    active: &mut Vec<String>,
) -> Result<Extent, Error> {
    owned(item)?;
    let inner = wire(program, item, active)?;

    if inner.tail || inner.minimum == 0 {
        return Err("array elements must make framed progress".into());
    }

    let Some(count) = count else {
        return Ok(Extent::variable(0, true));
    };
    Ok(Extent {
        minimum: inner.minimum.checked_mul(count).ok_or("layout overflow")?,
        fixed: inner
            .fixed
            .map(|width| width.checked_mul(count).ok_or("layout overflow"))
            .transpose()?,
        tail: false,
    })
}
fn owned(wire: &Wire) -> Result<(), Error> {
    match wire {
        Wire::Text | Wire::Bytes => Err("borrowed payload needs an owned codec binding".into()),
        Wire::Prefix { item, .. } => owned(item),
        _ => Ok(()),
    }
}
