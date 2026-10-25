//! Short public methods delegate account policy and wire data to their owners.
use super::output::Model;

pub(super) fn render(model: &Model) -> String {
    let name = &model.name;
    let lifetime = if model.remaining.is_empty() {
        ""
    } else {
        "<'_>"
    };

    format!(
        "impl {name} {{\n/// Builds for the canonical Token-2022 identity.\npub fn instruction(&self,input:{name}Accounts{lifetime},limits:Limits)->Result<Instruction,Error>{{self.instruction_with_program(PROGRAM,input,limits)}}\n/// Builds with caller-owned addresses and an explicit program identity.\npub fn instruction_with_program(&self,program:Address,input:{name}Accounts{lifetime},limits:Limits)->Result<Instruction,Error>{{\nlet accounts=input.metas(program,limits)?;\nself.validate_accounts(program,&accounts,limits)?;\nOk(Instruction {{program_address:program,accounts,data:{name}Codec.encode(self,limits.max_bytes)?}})\n}}\n/// Parses a complete frame after program and account validation.\npub fn parse_with_program(program:Address,instruction:&Instruction,limits:Limits)->Result<Self,Error>{{\n\nif instruction.program_address!=program {{return Err(Error::InvalidValue);}}\n\nlet value={name}Codec.decode(&instruction.data,limits)?;\nvalue.validate_accounts(program,&instruction.accounts,limits)?;\nOk(value)\n}}\n}}\n"
    )
}

pub(super) fn validation(model: &Model) -> String {
    let name = &model.name;
    let fixed = model
        .roles
        .iter()
        .map(|role| {
            format!(
                "Role {{writable:{},signer:Signer::{},optional:{}}}",
                role.writable, role.signer, role.optional
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let tails = model
        .remaining
        .iter()
        .map(|role| {
            format!(
                "AccountRole::from_privileges({},{})",
                role.signer == "Required",
                role.writable
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let boundary = if model.remaining.len() == 2 {
        "Some(usize::from(self.num_token_accounts))"
    } else {
        "None"
    };

    format!(
        "const FIXED_ROLES:&[Role]=&[{fixed}];\nconst REMAINING_ROLES:&[AccountRole]=&[{tails}];\nimpl {name} {{\npub(crate) fn validate_accounts(&self,program:Address,accounts:&[AccountMeta],limits:Limits)->Result<(),Error>{{\nvalidate_accounts(program,accounts,FIXED_ROLES,REMAINING_ROLES,{boundary},{},limits)\n}}\n}}\n",
        !model.omitted
    )
}
