use super::*;
use serde_json::json;
use std::fs;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../fixtures/compute-budget.json")).unwrap()
}
fn policy() -> Policy<'static> {
    Policy {
        name: "computeBudget",
        address: "ComputeBudget111111111111111111111111111111"
            .parse()
            .unwrap(),
    }
}

#[test]
fn actual_generation_is_repeatable_and_system_is_unchanged() {
    let root = fixture();
    let destination = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/solana_kit_compute_budget/src/generated.rs");
    let expected = fs::read(&destination).unwrap();
    let temporary =
        std::env::temp_dir().join(format!("kit-generic-repeat-{}.rs", std::process::id()));

    for _ in 0..2 {
        replace(&root, &policy(), &temporary).unwrap();
        assert_eq!(fs::read(&temporary).unwrap(), expected);
    }

    fs::remove_file(temporary).unwrap();
}

fn preserved(root: &Value, state: u8, index: usize) {
    let output = std::env::temp_dir().join(format!(
        "kit-generic-reject-{}-{index}-{state}.rs",
        std::process::id()
    ));
    let pending = output.with_extension("rs.pending");

    if state & 1 != 0 {
        fs::write(&output, b"output sentinel").unwrap();
    }

    if state & 2 != 0 {
        fs::write(&pending, b"pending sentinel").unwrap();
    }

    assert!(
        replace(root, &policy(), &output).is_err(),
        "mutation {index}"
    );
    assert_eq!(output.exists(), state & 1 != 0);

    assert_eq!(pending.exists(), state & 2 != 0);

    if output.exists() {
        assert_eq!(fs::read(&output).unwrap(), b"output sentinel");
        fs::remove_file(output).unwrap();
    }

    if pending.exists() {
        assert_eq!(fs::read(&pending).unwrap(), b"pending sentinel");
        fs::remove_file(pending).unwrap();
    }
}

#[test]
fn unsupported_schema_and_injections_preserve_all_destination_states() {
    let pointers = [
        "/program/instructions/0/arguments/0/type/kind",
        "/program/instructions/0/arguments/0/defaultValue/kind",
        "/program/instructions/0/arguments/1/type/format",
        "/program/instructions/0/kind",
        "/program/publicKey",
        "/program/name",
    ];
    let mut mutations = Vec::new();

    for pointer in pointers {
        for value in [Value::Null, json!(false), json!("../bad"), json!("foreign")] {
            let mut root = fixture();
            *root.pointer_mut(pointer).unwrap() = value;
            mutations.push(root);
        }
    }

    for property in ["min", "max", "size", "nullable", "offset"] {
        let mut root = fixture();

        root["program"]["instructions"][0]["arguments"][1]["type"][property] = json!(1);
        mutations.push(root);
    }
    for bad_name in [
        "../Escape",
        "x/y",
        "x\\y",
        "9bad",
        "é",
        "Error",
        "SetComputeUnitLimitCodec",
        "fn",
        "A\";panic!()",
    ] {
        let mut root = fixture();
        root["program"]["instructions"][0]["name"] = json!(bad_name);
        mutations.push(root);
    }
    let mut duplicate = fixture();
    duplicate["program"]["instructions"][1] = duplicate["program"]["instructions"][0].clone();
    mutations.push(duplicate);
    let mut foreign = fixture();
    foreign["program"]["publicKey"] = json!(Address::from_bytes([9; 32]).to_string());

    mutations.push(foreign);
    let mut accounts = fixture();
    accounts["program"]["instructions"][0]["accounts"] =
        json!([{"kind":"instructionAccountNode","name":"payer"}]);
    mutations.push(accounts);

    check_all_states(&mutations);
}

fn check_all_states(mutations: &[Value]) {
    for (index, root) in mutations.iter().enumerate() {
        for state in 0..4 {
            preserved(root, state, index);
        }
    }
}

fn linked() -> Value {
    let mut root = fixture();
    root["program"]["definedTypes"] = json!([
        serde_json::from_str::<Value>(include_str!("../../fixtures/token-account-state.json")).unwrap(),
        {"kind":"definedTypeNode","name":"settings","type":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","name":"mode","type":{"kind":"definedTypeLinkNode","name":"accountState"}},{"kind":"structFieldTypeNode","name":"count","type":{"kind":"numberTypeNode","format":"u32","endian":"le"}}]}}
    ]);
    root["program"]["instructions"][0]["arguments"][1]["type"] =
        json!({"kind":"definedTypeLinkNode","name":"settings"});
    root
}

#[test]
fn linked_struct_enum_layout_executes_in_real_generated_consumer() {
    let root = linked();
    let directory = std::env::temp_dir().join(format!("kit-generic-linked-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    replace(&root, &policy(), &directory.join("generated.rs")).unwrap();
    let source = "extern crate alloc; include!(\"generated.rs\"); fn main() { let value = RequestUnits { units: Settings { mode: AccountState::Initialized, count: u32::MAX }, additional_fee: 7 }; let bytes = RequestUnitsCodec.encode(&value, 10).unwrap(); assert_eq!(bytes, [0,1,255,255,255,255,7,0,0,0]); assert_eq!(RequestUnitsCodec.decode(&bytes, Limits::new(10,0)).unwrap(), value); assert!(RequestUnitsCodec.decode(&[0,3,0,0,0,0,0,0,0,0], Limits::new(10,0)).is_err()); }";
    super::boundary::execute_source(&directory, source);
}

#[test]
fn links_reject_unresolved_duplicates_cycles_and_semantic_constraints() {
    let mut roots = Vec::new();
    let mut unresolved = linked();
    unresolved["program"]["definedTypes"][1]["type"]["fields"][0]["type"]["name"] =
        json!("missing");
    roots.push(unresolved);
    let mut cycle = linked();
    cycle["program"]["definedTypes"][1]["type"]["fields"][0]["type"]["name"] = json!("settings");
    roots.push(cycle);

    let mut duplicate = linked();
    duplicate["program"]["definedTypes"][1]["name"] = json!("accountState");
    roots.push(duplicate);
    let mut constrained = linked();
    constrained["program"]["definedTypes"][0]["type"]["variants"][0]["value"] = json!(5);
    roots.push(constrained);

    for (index, root) in roots.iter().enumerate() {
        for state in 0..4 {
            preserved(root, state, 100 + index);
        }
    }
}

#[test]
fn explicit_trusted_policy_honors_a_different_valid_program_identity_and_name() {
    let mut root = fixture();
    let address = Address::from_bytes([47; 32]);
    root["program"]["name"] = json!("differentProgram");
    root["program"]["publicKey"] = json!(address.to_string());
    assert!(render(&root, &policy()).is_err());
    let output = render(
        &root,
        &Policy {
            name: "differentProgram",
            address,
        },
    )
    .unwrap();
    assert!(output.contains("DifferentProgramInstructionData"));
    assert!(output.contains(&format!("[{}]", ["47"; 32].join(","))));
}

#[test]
fn valid_input_cannot_replace_an_existing_pending_file() {
    let output =
        std::env::temp_dir().join(format!("kit-generic-pending-{}.rs", std::process::id()));
    let pending = output.with_extension("rs.pending");
    fs::write(&output, b"old output").unwrap();
    fs::write(&pending, b"existing pending").unwrap();
    assert!(replace(&fixture(), &policy(), &output).is_err());
    assert_eq!(fs::read(&output).unwrap(), b"old output");
    assert_eq!(fs::read(&pending).unwrap(), b"existing pending");
    fs::remove_file(output).unwrap();
    fs::remove_file(pending).unwrap();
}
