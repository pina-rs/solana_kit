use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).ok_or("expected source path")?;
    let source = fs::read_to_string(&path)?;
    let language = if path.ends_with(".dart") {
        tree_sitter_dart::LANGUAGE.into()
    } else {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    };

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language)?;

    let tree = parser.parse(&source, None).ok_or("parse failed")?;
    println!("{}", tree.root_node().to_sexp());
    Ok(())
}
