//! Optional idiomatic value traits for new public clients leave all existing output unchanged.
use super::{Program, Shape, Variant};
use crate::schema::Error;

/// Structural traits follow every typed field/variant; incompatible equality rejects first.
pub(super) fn emit(program: &Program) -> Result<String, Error> {
    if !super::equality::derives(program)
        .split(',')
        .any(|name| name.trim() == "Eq")
    {
        return Err("structural hashing requires equality-compatible wire values".into());
    }

    let mut output = String::new();

    for item in &program.definitions {
        output.push_str(&value(&item.name, &item.shape));
    }

    for item in &program.instructions {
        output.push_str(&value(&item.name, &Shape::Struct(item.fields.clone())));
    }

    let name = format!("{}InstructionData", program.name);
    let variants = program
        .instructions
        .iter()
        .map(|item| Variant {
            name: item.name.clone(),
            payload: Some(super::Wire::Link(item.name.clone())),
        })
        .collect::<Vec<_>>();
    output.push_str(&value(
        &name,
        &Shape::Tagged {
            prefix: "u8",
            variants,
        },
    ));

    if !program.errors.is_empty() {
        output.push_str(&program_error(&format!("{}Error", program.name)));
    }

    Ok(output)
}

fn value(name: &str, shape: &Shape) -> String {
    let body = match shape {
        Shape::Struct(fields) => fields
            .iter()
            .map(|field| format!("core::hash::Hash::hash(&self.{},state);\n", field.name))
            .collect::<String>(),
        #[cfg(feature = "verification")]
        Shape::Enum(variants) => {
            let arms = variants
                .iter()
                .enumerate()
                .map(|(index, variant)| {
                    format!("Self::{variant}=>core::hash::Hash::hash(&{index}usize,state),\n")
                })
                .collect::<String>();
            format!("match self{{{arms}}}\n")
        }
        Shape::Tagged { variants, .. } => tagged(variants),
    };
    let unused = if matches!(shape,Shape::Struct(fields) if fields.is_empty()) {
        "let _=state;\n"
    } else {
        ""
    };

    format!(
        "impl core::hash::Hash for {name}{{\nfn hash<H:core::hash::Hasher>(&self,state:&mut H){{\n{unused}{body}}}\n}}\nimpl core::fmt::Display for {name}{{\nfn fmt(&self,formatter:&mut core::fmt::Formatter<'_>)->core::fmt::Result{{core::fmt::Debug::fmt(self,formatter)}}\n}}\n"
    )
}

fn tagged(variants: &[Variant]) -> String {
    let mut arms = String::new();

    for (index, variant) in variants.iter().enumerate() {
        let name = &variant.name;
        let tag = format!("core::hash::Hash::hash(&{index}usize,state);");

        if variant.payload.is_some() {
            arms.push_str(&format!(
                "Self::{name}(value)=>{{{tag}core::hash::Hash::hash(value,state);}},\n"
            ));
        } else {
            arms.push_str(&format!("Self::{name}=>{{{tag}}},\n"));
        }
    }

    format!("match self{{{arms}}}\n")
}

fn program_error(name: &str) -> String {
    format!(
        "impl core::hash::Hash for {name}{{\nfn hash<H:core::hash::Hasher>(&self,state:&mut H){{core::hash::Hash::hash(&self.code(),state)}}\n}}\nimpl core::fmt::Display for {name}{{\nfn fmt(&self,formatter:&mut core::fmt::Formatter<'_>)->core::fmt::Result{{match self.message(){{Some(message)=>formatter.write_str(message),None=>write!(formatter,\"unknown program error {{}}\",self.code())}}}}\n}}\nimpl core::error::Error for {name}{{}}\n"
    )
}
