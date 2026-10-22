//! Account size identifiers from explicitly validated unique discriminators.
use super::Program;

pub(super) fn emit(program: &Program) -> String {
    if program.accounts.is_empty() {
        return String::new();
    }

    let name = format!("{}AccountKind", program.name);
    let mut output = format!(
        "/// Unique account size discriminators; identification does not validate fields.\n#[derive(Clone,Copy,Debug,Eq,PartialEq)]\npub enum {name} {{\n"
    );

    for account in &program.accounts {
        output.push_str(&format!(
            "/// Account of {} bytes.\n{},\n",
            account.size, account.definition.name
        ));
    }

    output.push_str(&format!("}}\n/// Identifies an account by its explicit size discriminator.\npub fn identify_account(data:&[u8])->Result<{name},Error> {{\nmatch data.len() {{\n"));

    for account in &program.accounts {
        output.push_str(&format!(
            "ACCOUNT_SIZE_{} => Ok({name}::{}),\n",
            account.size, account.definition.name
        ));
    }

    output.push_str("_ => Err(Error::InvalidDiscriminator),\n}\n}\n");
    output
}
