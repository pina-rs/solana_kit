//! Independent MIT Dart literals and fail-closed source mutations; execution is still pending.
use super::*;
use serde_json::json;

const HEADERS: &[(&str, &str)] = &[
    ("programConfigInit", "b8bcc6c3cd7c75d8"),
    ("programConfigSetAuthority", "eef224b5208fd84b"),
    ("programConfigSetMultisigCreationFee", "65a0f93f9ad7990d"),
    ("programConfigSetTreasury", "6f2ef37590bca26b"),
    ("multisigCreate", "7a4d509f54585ac5"),
    ("multisigCreateV2", "32ddc75d28f58be9"),
    ("multisigAddMember", "01dbd76cb8e5d608"),
    ("multisigRemoveMember", "d975b1d2b691da48"),
    ("multisigSetTimeLock", "949a794dd4fe9b48"),
    ("multisigChangeThreshold", "8d2a0f7ea95c3eb5"),
    ("multisigSetConfigAuthority", "8f5dc78f5ca9c1e8"),
    ("multisigSetRentCollector", "30cc4139d2469c4a"),
    ("multisigAddSpendingLimit", "0bf29f2a56c55973"),
    ("multisigRemoveSpendingLimit", "e4c6886f7b04b271"),
    ("configTransactionCreate", "9bec57e4894b5127"),
    ("configTransactionExecute", "7292f4bdfc8c2428"),
    ("vaultTransactionCreate", "30fa4ea8d0e2dad3"),
    ("transactionBufferCreate", "f5c9716c253f1d59"),
    ("transactionBufferClose", "11b6d0e48818b266"),
    ("transactionBufferExtend", "e69d433805eef592"),
    ("vaultTransactionCreateFromBuffer", "de36954457f630e7"),
    ("vaultTransactionExecute", "c208a15799a419ab"),
    ("batchCreate", "c28e8d1137b914f8"),
    ("batchAddTransaction", "5964e0124546364c"),
    ("batchExecuteTransaction", "ac2cb398157feab4"),
    ("proposalCreate", "dc3c49e01e6c4f9f"),
    ("proposalActivate", "0b225cf89a1b336a"),
    ("proposalApprove", "9025a488bcd82af8"),
    ("proposalReject", "f33e869ce66af687"),
    ("proposalCancel", "1b2a7fed26a354cb"),
    ("proposalCancelV2", "cd29c23ddc8b10f7"),
    ("spendingLimitUse", "1039827fc1149b86"),
    ("configTransactionAccountsClose", "50cb54359770bbba"),
    ("vaultTransactionAccountsClose", "c447bbb00223aaa5"),
    ("vaultBatchTransactionAccountClose", "8612136a814461f7"),
    ("batchAccountsClose", "dac407af82660bff"),
];

#[test]
fn complete_instruction_headers_match_every_independent_dart_literal() {
    let model = source().unwrap();
    let instructions = model["program"]["instructions"].as_array().unwrap();
    assert_eq!(instructions.len(), 36);
    assert_eq!(HEADERS.len(), 36);

    for (node, (name, header)) in instructions.iter().zip(HEADERS) {
        assert_eq!(node["name"], *name);
        assert_eq!(node["arguments"][0]["defaultValue"]["data"], *header);
    }

    let rendered = render(&model).unwrap();
    assert_eq!(rendered, render(&model).unwrap());
    assert!(rendered.contains("pub new_config_authority: Address"));
}

#[test]
fn complete_accounts_errors_named_variants_and_nested_roles_are_retained() {
    const CREATE_FROM_BUFFER: usize = 20;
    const CREATOR_ROLE: usize = 6;
    const CANCEL_V2: usize = 30;
    let model = source().unwrap();
    let program = &model["program"];
    assert_eq!(program["accounts"].as_array().unwrap().len(), 9);
    assert_eq!(program["errors"].as_array().unwrap().len(), 45);
    assert_eq!(program["definedTypes"].as_array().unwrap().len(), 11);
    assert_eq!(
        program["instructions"][CREATE_FROM_BUFFER]["accounts"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    assert_eq!(
        program["instructions"][CREATE_FROM_BUFFER]["accounts"][2]["name"],
        "vaultTransactionCreateCreator"
    );
    assert_eq!(
        program["instructions"][CREATE_FROM_BUFFER]["accounts"][CREATOR_ROLE]["name"],
        "creator"
    );

    assert_eq!(
        program["instructions"][CANCEL_V2]["accounts"][0]["name"],
        "multisig"
    );

    for (code, error) in (6000u32..).zip(program["errors"].as_array().unwrap()) {
        assert_eq!(error["code"], code);
    }

    let rendered = render(&model).unwrap();

    for name in [
        "Multisig",
        "Proposal",
        "VaultTransaction",
        "VaultBatchTransaction",
    ] {
        assert!(rendered.contains(&format!("pub struct {name}Codec")));
    }

    assert!(rendered.contains("pub enum ConfigAction"));
    assert!(rendered.contains("pub enum ProposalStatus"));
    assert!(rendered.contains("pub struct ConfigActionAddSpendingLimitPayload"));
}

#[test]
fn invalid_schemas_preserve_both_existing_output_and_pending_files() {
    const PRIVILEGE_MUTATION_CASE: usize = 101;
    for (index, (pointer, mutation)) in [
        (
            "/program/instructions/0/accounts/0/isSigner",
            json!("false"),
        ),
        (
            "/program/instructions/0/arguments/1/type",
            json!({"kind":"UnknownNode"}),
        ),
        ("/program/accounts/0/discriminators/0/offset", json!(1)),
        (
            "/program/accounts/0/discriminators/0/constant/value/data",
            json!("zz"),
        ),
        (
            "/program/accounts/0/data/fields/0/type",
            json!({"kind":"definedTypeLinkNode","name":"Missing"}),
        ),
        (
            "/program/instructions/21/remainingAccounts/0/privilegePolicy",
            json!("unchecked"),
        ),
        (
            "/program/definedTypes/10/type/variants/0/struct/kind",
            json!("UnknownNode"),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let mut model = source().unwrap();
        *model.pointer_mut(pointer).unwrap() = mutation;
        reject_unchanged(&model, index);
    }

    let mut model = source().unwrap();
    model["program"]["accounts"][1]["discriminators"] =
        model["program"]["accounts"][0]["discriminators"].clone();
    reject_unchanged(&model, 100);

    let mut model = source().unwrap();
    model["program"]["instructions"][21]["remainingAccounts"][0]["isSigner"] = json!(false);
    reject_unchanged(&model, PRIVILEGE_MUTATION_CASE);
}

fn reject_unchanged(model: &Value, index: usize) {
    let directory =
        std::env::temp_dir().join(format!("kit-squads-reject-{}-{index}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let output = directory.join("generated.rs");
    let pending = output.with_extension("rs.pending");
    std::fs::write(&output, b"original output").unwrap();
    std::fs::write(&pending, b"original pending").unwrap();
    let result = render(model).and_then(|source| crate::generate::replace_source(&source, &output));

    assert!(result.is_err());
    assert_eq!(std::fs::read(output).unwrap(), b"original output");
    assert_eq!(std::fs::read(pending).unwrap(), b"original pending");
}
