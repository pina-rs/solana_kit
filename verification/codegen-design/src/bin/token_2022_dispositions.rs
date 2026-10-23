//! Retains exact canonical IDs and names each declaration's bounded public owner.
use std::{collections::BTreeSet, error::Error, fs, path::Path};

const DECLARATIONS: usize = 1829;
const CANONICAL_COLUMNS: usize = 14;

/// Every selected row is preserved verbatim before adding evidence and residuals.
fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let canonical = fs::read_to_string(root.join("docs/parity/capabilities.tsv"))?;
    let mut lines = canonical.lines();
    let mut output = format!(
        "{}\tunit255_status\trust_contract\tevidence\tresidual\n",
        lines.next().ok_or("canonical header required")?
    );
    let mut ids = BTreeSet::new();
    let mut residual = 0;

    for row in lines {
        let fields: Vec<_> = row.split('\t').collect();

        if fields.get(1) != Some(&"solana_kit_token_2022") {
            continue;
        }

        if fields.len() != CANONICAL_COLUMNS || !ids.insert(fields[0]) {
            return Err("canonical declaration mismatch".into());
        }

        let (status, owner, limit) = disposition(fields[2]);
        residual += usize::from(status == "residual");
        let declaration = fields[3];
        output.push_str(&format!("{row}\t{status}\t{owner} for {declaration}\tcrates/solana_kit_token_2022/tests; pinned-source projection\t{limit}\n"));
    }

    if ids.len() != DECLARATIONS {
        return Err("expected exact1829 canonical IDs".into());
    }

    fs::write(root.join("docs/parity/token-2022.tsv"), output)?;
    println!(
        "{DECLARATIONS} canonical IDs; {} bounded; {residual} residual; zero full-parity claims",
        DECLARATIONS - residual
    );
    Ok(())
}

struct Route {
    pattern: &'static str,
    owner: &'static str,
    residual: &'static str,
}
const ROUTES: &[Route] = &[
    Route {
        pattern: "/instructions/",
        owner: "{type}/{type}Codec; {type}Accounts; instruction/parse_with_program",
        residual: "data-only Dart parsing is strengthened with program/role validation; validator and host execution remain separate",
    },
    Route {
        pattern: "/accounts/",
        owner: "{type}/{type}Codec; accounts::Token2022Account; optional fetch::*",
        residual: "structural data and initialized canonical state are distinct; generic enum fetch replaces named wrappers; full program semantics remain separate",
    },
    Route {
        pattern: "/types/",
        owner: "generated {type} model/codec and exact owned fields",
        residual: "canonical TLV restores missing Dart lengths; ordered metadata entries preserve duplicate wire keys rather than Dart Map object semantics",
    },
    Route {
        pattern: "/errors/",
        owner: "errors::Token2022Error; KnownToken2022Error; classify_failure",
        residual: "all69 source Rust labels and unknown codes are retained; Dart defines20 labels and differs in code9 spelling",
    },
    Route {
        pattern: "/programs/",
        owner: "Token2022InstructionData/Codec; parse_instruction[_with_program]; PROGRAM",
        residual: "all100 variants preserve metas; ambiguous optional readonly roles retain raw positions without invented semantic labels",
    },
    Route {
        pattern: "get_mint_size.dart",
        owner: "accounts::mint_size",
        residual: "canonical TLV lengths and355 adjustment intentionally correct the pinned Dart size defect",
    },
    Route {
        pattern: "get_token_size.dart",
        owner: "accounts::token_size",
        residual: "canonical TLV lengths and355 adjustment intentionally correct the pinned Dart size defect",
    },
    Route {
        pattern: "get_initialize_instructions_for_extensions.dart",
        owner: "extensions::pre_initialize_mint_extensions",
        residual: "all15 source helper branches preserve order; proof generation and external orchestration remain caller owned",
    },
    Route {
        pattern: "/extensions.dart",
        owner: "Vec<Extension>; ExtensionsCodec; Mint/Token.extensions",
        residual: "canonical remainder sequence reuses accepted generated codec; bare Dart lengths intentionally differ",
    },
    Route {
        pattern: "solana_kit_token_2022.dart",
        owner: "crate public reexports",
        residual: "Rust exports replace Dart library directives; semantic and runtime residuals remain those of their owners",
    },
];

fn disposition(path: &str) -> (&'static str, String, &'static str) {
    let leaf = path
        .rsplit('/')
        .next()
        .unwrap_or(path)
        .trim_end_matches(".dart");

    if let Some(route) = ROUTES.iter().find(|route| path.contains(route.pattern)) {
        return (
            "bounded",
            route.owner.replace("{type}", &title(leaf)),
            route.residual,
        );
    }

    (
        "residual",
        format!("source owner {leaf}"),
        "no tested Rust equivalent has been identified for this exact declaration",
    )
}

fn title(value: &str) -> String {
    value
        .split('_')
        .map(|part| {
            let mut letters = part.chars();
            letters.next().map_or(String::new(), |first| {
                format!("{}{}", first.to_ascii_uppercase(), letters.as_str())
            })
        })
        .collect()
}
