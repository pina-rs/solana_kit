//! Literal headers are checked through actual generated consumers.
use super::*;
use serde_json::json;

fn policy() -> Policy<'static> {
    Policy {
        name: "computeBudget",
        address: "ComputeBudget111111111111111111111111111111"
            .parse()
            .unwrap(),
    }
}

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../fixtures/compute-budget.json")).unwrap()
}

fn header(item: &mut Value, bytes: &[u8]) {
    let data = bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    item["arguments"][0]["type"] =
        json!({"kind":"fixedSizeTypeNode","size":bytes.len(),"type":{"kind":"bytesTypeNode"}});
    item["arguments"][0]["defaultValue"] =
        json!({"kind":"bytesValueNode","encoding":"base16","data":data});
}

fn execute(root: &Value, name: &str, body: &str) {
    let directory = std::env::temp_dir().join(format!("kit-header-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    replace(root, &policy(), &directory.join("generated.rs")).unwrap();
    boundary::execute_source(&directory, body);
}

#[test]
fn literal_headers_encode_identify_and_parse_without_ordinal_tags() {
    let mut root = fixture();

    for (index, item) in root["program"]["instructions"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .enumerate()
    {
        let mut bytes = [200, 7, 6, 5, 4, 3, 2, 0];
        bytes[7] = u8::try_from(index).unwrap();
        header(item, &bytes);
    }

    execute(
        &root,
        "inline",
        include_str!("../../fixtures/headers/inline.rs"),
    );
}

#[test]
fn literal_header_collisions_prefixes_and_bounds_reject_before_replacement() {
    for bytes in [vec![], vec![1; 33]] {
        let mut root = fixture();
        header(&mut root["program"]["instructions"][0], &bytes);
        assert!(render(&root, &policy()).is_err());
    }

    for bytes in [vec![1], vec![1, 2]] {
        let mut root = fixture();
        header(&mut root["program"]["instructions"][0], &[1]);
        header(&mut root["program"]["instructions"][1], &bytes);
        assert!(render(&root, &policy()).is_err());
    }
}

#[test]
fn ambiguous_literal_header_preserves_both_existing_output_files() {
    let mut root = fixture();
    header(&mut root["program"]["instructions"][0], &[1]);
    header(&mut root["program"]["instructions"][1], &[1, 2]);
    let directory = std::env::temp_dir().join(format!("kit-header-atomic-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let output = directory.join("generated.rs");
    let pending = output.with_extension("rs.pending");
    std::fs::write(&output, b"original output").unwrap();

    std::fs::write(&pending, b"original pending").unwrap();
    assert!(replace(&root, &policy(), &output).is_err());
    assert_eq!(std::fs::read(output).unwrap(), b"original output");
    assert_eq!(std::fs::read(pending).unwrap(), b"original pending");
}

#[test]
fn grouped_dispatch_consumes_full_headers_on_both_sides_of_group_boundary() {
    let mut root = fixture();
    let template = root["program"]["instructions"][2].clone();
    root["program"]["instructions"] = Value::Array(
        (0u16..256)
            .map(|index| {
                let mut item = template.clone();
                item["name"] = json!(format!("variant{index}"));
                header(&mut item, &[199, u8::try_from(index).unwrap()]);
                item
            })
            .collect(),
    );
    execute(
        &root,
        "grouped",
        include_str!("../../fixtures/headers/grouped.rs"),
    );
}

#[test]
fn explicit_instruction_budget_accepts_256_and_preserves_outputs_at_257() {
    let mut root = fixture();
    let template = root["program"]["instructions"][2].clone();
    root["program"]["instructions"] = Value::Array(
        (0u16..256)
            .map(|index| {
                let mut item = template.clone();
                item["name"] = json!(format!("variant{index}"));
                header(&mut item, &index.to_le_bytes());
                item
            })
            .collect(),
    );
    assert!(render(&root, &policy()).is_ok());
    assert!(data::render_program_data(&root, &policy()).is_ok());
    let mut excess = template;
    excess["name"] = json!("variant256");
    header(&mut excess, &[0, 1]);

    root["program"]["instructions"]
        .as_array_mut()
        .unwrap()
        .push(excess);
    let directory =
        std::env::temp_dir().join(format!("kit-instruction-budget-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let output = directory.join("generated.rs");
    let pending = output.with_extension("rs.pending");
    std::fs::write(&output, b"original output").unwrap();
    std::fs::write(&pending, b"original pending").unwrap();

    assert!(replace(&root, &policy(), &output).is_err());
    assert!(data::replace(&root, &policy(), &output).is_err());
    assert_eq!(std::fs::read(output).unwrap(), b"original output");
    assert_eq!(std::fs::read(pending).unwrap(), b"original pending");
}

#[test]
fn complete_compression_model_emits_real_primary_wire_and_strict_invocations() {
    let directory =
        std::env::temp_dir().join(format!("kit-header-compression-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    crate::program_preparation::generate_compression(&directory.join("generated.rs")).unwrap();
    boundary::execute_source(
        &directory,
        include_str!("../../fixtures/headers/compression.rs"),
    );
}
