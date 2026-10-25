//! The renderer's package entrypoint controls its public TypeScript namespace.

use crate::{
    model::{Export, Parsed, Result},
    parse::{descendants, text},
};
use tree_sitter::{Node, Parser};

/// Parses only exported declarations; function bodies contain private locals.
pub(crate) fn parse(source: &str) -> Result<Parsed> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())?;
    let tree = parser
        .parse(source, None)
        .ok_or("TypeScript parser returned no tree")?;
    let mut output = Parsed::default();

    for error in descendants(tree.root_node(), "ERROR") {
        output
            .warnings
            .push(format!("syntax-ERROR:{}", error.start_position().row + 1));
    }

    for node in tree
        .root_node()
        .named_children(&mut tree.root_node().walk())
    {
        if node.kind() != "export_statement" {
            continue;
        }

        exported(node, source, &mut output);
    }

    Ok(output)
}

/// Named reexports filter the defining module before public closure is computed.
fn exported(node: Node<'_>, source: &str, output: &mut Parsed) {
    if let Some(target) = node.child_by_field_name("source") {
        let names = descendants(node, "export_specifier")
            .into_iter()
            .filter_map(|item| item.child_by_field_name("name"))
            .map(|name| text(name, source).to_owned())
            .collect();
        output.exports.push(Export {
            target: text(target, source)
                .trim_matches(['\'', '"'])
                .replace(".js", ".ts"),
            show: names,
            hide: Default::default(),
        });
    } else if let Some(declaration) = node.child_by_field_name("declaration") {
        collect(declaration, source, None, output);
    } else if text(node, source).contains("default renderVisitor") {
        // Codama's default is the same renderer operation as the named export.
    } else {
        output.warnings.push(format!(
            "unhandled-export:{}",
            node.start_position().row + 1
        ));
    }
}

/// Lexical wrappers recurse without becoming declarations of their own.
fn collect(node: Node<'_>, source: &str, parent: Option<&str>, output: &mut Parsed) {
    if node.kind() == "lexical_declaration" {
        for child in node.named_children(&mut node.walk()) {
            collect(child, source, parent, output);
        }

        return;
    }

    let Some(name) = node.child_by_field_name("name") else {
        output.warnings.push(format!(
            "unhandled-declaration:{}:{}",
            node.kind(),
            node.start_position().row + 1
        ));

        return;
    };
    // Private identifiers have a distinct syntax kind; quoted # names remain public.
    if name.kind() == "private_property_identifier" {
        return;
    }

    let name = text(name, source);

    if text(node, source).starts_with("private ") || text(node, source).starts_with("protected ") {
        return;
    }

    let symbol = parent.map_or_else(|| name.to_owned(), |owner| format!("{owner}.{name}"));
    output.declarations.push(crate::model::Declaration {
        symbol,
        root: parent.unwrap_or(name).into(),
        kind: node.kind().into(),
        line: node.start_position().row + 1,
    });

    collect_members(node, source, name, output);
}

/// Only class and interface bodies expose members; function locals stay private.
fn collect_members(node: Node<'_>, source: &str, name: &str, output: &mut Parsed) {
    if let Some(body) = node
        .child_by_field_name("body")
        .filter(|body| matches!(body.kind(), "class_body" | "interface_body"))
    {
        for member in body.named_children(&mut body.walk()) {
            if member.kind() != "comment" {
                collect(member, source, Some(name), output);
            }
        }
    }
}
