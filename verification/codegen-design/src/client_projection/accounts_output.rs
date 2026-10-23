//! Typed address inputs and checked construction of their exact positional metas.
use super::{Result, output::Model, roles::Role};

/// Emit typed inputs and their exact positional projection from the same role model.
pub(super) fn render(model: &Model) -> Result<String> {
    let name = &model.name;
    let lifetime = if model.remaining.is_empty() {
        ""
    } else {
        "<'a>"
    };

    let role_import = "AccountRole,";
    let mut out = format!(
        "//! Source-pinned account inputs and instruction wrappers.\nuse crate::{{{name},{name}Codec,PROGRAM,invocation::*}};\nuse alloc::vec::Vec;\nuse solana_kit_core::{{Address,AccountMeta,{role_import}Instruction,codecs::{{Decoder,Encoder,Error,Limits}}}};\n/// Caller-owned addresses in exact protocol order.\n#[derive(Clone,Copy,Debug)]\npub struct {name}Accounts{lifetime} {{\n"
    );

    for role in &model.roles {
        let ty = field_type(role);

        out.push_str(&format!(
            "/// Source role `{}`.\npub {}:{ty},\n",
            role.name, role.name
        ));
    }

    for role in &model.remaining {
        out.push_str(&format!(
            "/// Ordered `{}` list; duplicate addresses are preserved.\npub {}:&'a [Address],\n",
            role.name, role.name
        ));
    }

    out.push_str(&format!("}}\nimpl{lifetime} {name}Accounts{lifetime} {{\nfn allocate(self,limits:Limits)->Result<Vec<AccountMeta>,Error>{{\n"));
    out.push_str(&capacity(model));
    out.push_str("\nOk(accounts)\n}\nfn metas(self,program:Address,limits:Limits)->Result<Vec<AccountMeta>,Error>{\n");
    let mutable = if model.roles.is_empty() && model.remaining.is_empty() {
        ""
    } else {
        "mut "
    };
    out.push_str(&format!(
        "let {mutable}accounts=self.allocate(limits)?;\n\n"
    ));

    for role in &model.roles {
        out.push_str(&append(role, model.omitted)?);
    }

    for role in &model.remaining {
        let field = &role.name;
        let signer = role.signer == "Required";
        let writable = role.writable;
        let role = static_role(signer, writable);
        out.push_str(&format!(
            "\nfor address in self.{field} {{accounts.push(meta(*address,{role}));}}\n\n"
        ));
    }

    out.push_str("\nlet _=program;\nOk(accounts)\n}\n}\n");
    Ok(out)
}

fn field_type(role: &Role) -> String {
    let base = if role.signer == "Either" {
        "Authority"
    } else {
        "Address"
    };

    if role.optional || role.default.is_some() {
        return format!("Option<{base}>");
    }

    base.to_owned()
}

/// Preflight actual optional and remaining counts before any account allocation.
fn capacity(model: &Model) -> String {
    if model.roles.is_empty() && model.remaining.is_empty() {
        return "let _=limits;\nlet accounts=Vec::new();\n".to_owned();
    }

    let required = model
        .roles
        .iter()
        .filter(|role| !role.optional || !model.omitted)
        .count();
    let mut count = required.to_string();

    for role in model
        .roles
        .iter()
        .filter(|role| role.optional && model.omitted)
    {
        count.push_str(&format!("+usize::from(self.{}.is_some())", role.name));
    }

    let mut out = format!("let count:usize={count};\n");

    for role in &model.remaining {
        out.push_str(&format!(
            "let count=count.checked_add(self.{}.len()).ok_or(Error::Overflow)?;\n",
            role.name
        ));
    }

    out.push_str("\nlet mut accounts=Vec::new();\n\nif count>limits.max_items {return Err(Error::LimitExceeded);}\n\naccounts.try_reserve_exact(count).map_err(|_|Error::AllocationFailed)?;\n");
    out
}

fn append(role: &Role, omitted: bool) -> Result<String> {
    let field = &role.name;
    let writable = role.writable;
    let signer = role.signer == "Required";
    let static_role = static_role(signer, writable);

    if role.optional {
        return Ok(format!(
            "append_optional(&mut accounts,self.{field},program,{static_role},{omitted});\n"
        ));
    }

    let value = if let Some(default) = &role.default {
        let address: solana_kit_core::Address = default.parse()?;
        format!(
            "self.{field}.unwrap_or(Address::from_bytes({:?}))",
            address.to_bytes()
        )
    } else {
        format!("self.{field}")
    };

    if role.signer == "Either" {
        return Ok(format!(
            "let authority={value};\nlet role=AccountRole::from_privileges(authority.is_signer,{writable});\naccounts.push(meta(authority.address,role));\n"
        ));
    }

    Ok(format!("accounts.push(meta({value},{static_role}));\n"))
}

fn static_role(signer: bool, writable: bool) -> &'static str {
    match (signer, writable) {
        (false, false) => "AccountRole::Readonly",
        (false, true) => "AccountRole::Writable",
        (true, false) => "AccountRole::ReadonlySigner",
        (true, true) => "AccountRole::WritableSigner",
    }
}
