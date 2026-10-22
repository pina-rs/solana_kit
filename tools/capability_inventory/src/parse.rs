//! Syntax-aware Dart extraction; unfamiliar declarations become explicit gaps.

use crate::model::{Declaration, Export, Parsed, Result};
use std::collections::BTreeSet;
use tree_sitter::{Node, Parser};

/// Parses declarations without descending into executable function bodies.
pub(crate) fn dart(source: &str) -> Result<Parsed> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_dart::LANGUAGE.into())?;
    let tree = parser
        .parse(source, None)
        .ok_or("Dart parser returned no tree")?;
    let mut output = Parsed::default();
    errors(tree.root_node(), &mut output.warnings);

    for node in tree
        .root_node()
        .named_children(&mut tree.root_node().walk())
    {
        match node.kind() {
            "comment" | "library_name" | "library_directive" | "part_of_directive" => {}
            "import_or_export" => export(node, source, &mut output),
            "part_directive" => output
                .warnings
                .push(format!("part-directive:{}", node.start_position().row + 1)),
            _ => declaration(node, source, None, &mut output),
        }
    }

    output.declarations.sort();
    output.declarations.dedup();
    Ok(output)
}

/// Retains each syntax error or inserted missing token as a reviewable gap.
fn errors(node: Node<'_>, warnings: &mut Vec<String>) {
    if node.is_error() || node.is_missing() {
        warnings.push(format!(
            "syntax-{}:{}:{}",
            node.kind(),
            node.start_position().row + 1,
            node.start_position().column + 1
        ));
    }

    for child in node.named_children(&mut node.walk()) {
        errors(child, warnings);
    }
}

/// Resolves export URI and show/hide identifiers from the syntax tree.
fn export(node: Node<'_>, source: &str, output: &mut Parsed) {
    let Some(export) = find(node, "library_export") else {
        return;
    };
    let Some(uri) = find(export, "string_literal") else {
        output.warnings.push("export-without-uri".into());

        return;
    };
    let mut item = Export {
        target: text(uri, source).trim_matches(['\'', '"']).into(),
        show: BTreeSet::new(),
        hide: BTreeSet::new(),
    };

    for child in export.named_children(&mut export.walk()) {
        if child.kind() != "combinator" {
            continue;
        }

        let names = descendants(child, "identifier")
            .into_iter()
            .map(|name| text(name, source).to_owned());

        if text(child, source).starts_with("show") {
            item.show.extend(names);
        } else {
            item.hide.extend(names);
        }
    }

    if text(export, source).contains(" if ") {
        output.warnings.push(format!(
            "conditional-export:{}",
            export.start_position().row + 1
        ));
    }

    for alternative in descendants(export, "string_literal").into_iter().skip(1) {
        let mut branch = item.clone();
        branch.target = text(alternative, source).trim_matches(['\'', '"']).into();
        output.exports.push(branch);
    }

    output.exports.push(item);
}

/// Recognizes namespace declarations and members, including constructors/operators.
fn declaration(node: Node<'_>, source: &str, parent: Option<&str>, output: &mut Parsed) {
    match node.kind() {
        "class_declaration"
        | "mixin_declaration"
        | "enum_declaration"
        | "extension_declaration"
        | "extension_type_declaration" => container(node, source, output),
        "class_member" | "method_declaration" | "method_signature" | "declaration" => {
            children(node, source, parent, output)
        }
        "function_declaration"
        | "getter_declaration"
        | "setter_declaration"
        | "external_function_declaration"
        | "external_getter_declaration"
        | "external_setter_declaration" => function(node, source, parent, output),
        "function_signature"
        | "getter_signature"
        | "setter_signature"
        | "constructor_signature"
        | "constant_constructor_signature"
        | "factory_constructor_signature"
        | "redirecting_factory_constructor_signature" => signature(node, source, parent, output),
        "operator_signature" => operator(node, source, parent, output),
        "top_level_variable_declaration"
        | "external_variable_declaration"
        | "initialized_identifier_list"
        | "static_final_declaration_list" => children(node, source, parent, output),
        "identifier_list" => identifiers(node, source, parent, output),
        "initialized_identifier" | "static_final_declaration" | "enum_constant" | "type_alias" => {
            named(node, source, parent, output)
        }
        "comment"
        | "function_body"
        | "function_expression_body"
        | "annotation"
        | "type"
        | "interfaces"
        | "superclass"
        | "mixins"
        | "type_parameters"
        | "redirection"
        | "initializers" => {}
        other => output.warnings.push(format!(
            "unhandled-{other}:{}",
            node.start_position().row + 1
        )),
    }
}

/// Wrapper syntax contributes declarations, while type annotations do not.
fn children(node: Node<'_>, source: &str, parent: Option<&str>, output: &mut Parsed) {
    for child in node.named_children(&mut node.walk()) {
        if !matches!(child.kind(), "annotation" | "type") {
            declaration(child, source, parent, output);
        }
    }
}

/// Function bodies contain locals that must never enter the public namespace.
fn function(node: Node<'_>, source: &str, parent: Option<&str>, output: &mut Parsed) {
    if let Some(signature) = node
        .child_by_field_name("signature")
        .or_else(|| node.named_child(0))
    {
        declaration(signature, source, parent, output);
    }
}

/// Operators expose callable public behavior even without identifier tokens.
fn operator(node: Node<'_>, source: &str, parent: Option<&str>, output: &mut Parsed) {
    if let Some(name) = node.child_by_field_name("operator") {
        add(
            node,
            &format!("operator{}", text(name, source)),
            parent,
            "operator",
            output,
        );
    } else {
        output.warnings.push(format!(
            "operator-without-name:{}",
            node.start_position().row + 1
        ));
    }
}

/// Abstract fields use identifier lists rather than initialized declarations.
fn identifiers(node: Node<'_>, source: &str, parent: Option<&str>, output: &mut Parsed) {
    for name in node
        .named_children(&mut node.walk())
        .filter(|item| item.kind() == "identifier")
    {
        add(name, text(name, source), parent, "field", output);
    }
}

/// A recognized declaration with no resolvable name remains an explicit gap.
fn named(node: Node<'_>, source: &str, parent: Option<&str>, output: &mut Parsed) {
    let name = node.child_by_field_name("name").or_else(|| {
        node.named_children(&mut node.walk())
            .find(|child| matches!(child.kind(), "identifier" | "type_identifier"))
    });

    if let Some(name) = name {
        add(node, text(name, source), parent, node.kind(), output);
    } else {
        output.warnings.push(format!(
            "unnamed-{}:{}",
            node.kind(),
            node.start_position().row + 1
        ));
    }
}

/// Records public containers and the operations they introduce.
fn container(node: Node<'_>, source: &str, output: &mut Parsed) {
    let name_node = node.child_by_field_name("name");
    let name = name_node
        .map(|value| text(value, source))
        .unwrap_or("<unnamed-extension>");

    if name.starts_with('_') {
        return;
    }

    add(node, name, None, node.kind(), output);

    if let Some(representation) = node.child_by_field_name("representation") {
        add(
            representation,
            "new",
            Some(name),
            "representation-constructor",
            output,
        );

        if let Some(field) = representation.child_by_field_name("name") {
            add(
                field,
                text(field, source),
                Some(name),
                "representation-field",
                output,
            );
        }
    }

    if let Some(body) = node.child_by_field_name("body").or_else(|| {
        node.named_children(&mut node.walk())
            .find(|child| matches!(child.kind(), "class_body" | "enum_body" | "extension_body"))
    }) {
        for member in body.named_children(&mut body.walk()) {
            declaration(member, source, Some(name), output);
        }
    }
}

/// Distinguishes unnamed/named constructors from ordinary methods and accessors.
fn signature(node: Node<'_>, source: &str, parent: Option<&str>, output: &mut Parsed) {
    let Some(name_node) = node.child_by_field_name("name") else {
        output.warnings.push(format!(
            "unnamed-{}:{}",
            node.kind(),
            node.start_position().row + 1
        ));

        return;
    };
    let name = text(name_node, source);
    let constructor = node.kind().contains("constructor");
    let names: Vec<_> = node
        .children_by_field_name("name", &mut node.walk())
        .filter(|name| name.is_named())
        .collect();
    let selector = names.get(1).map(|value| text(*value, source));
    let final_name = if constructor {
        selector.unwrap_or("new")
    } else {
        name
    };

    add(node, final_name, parent, node.kind(), output);
}

/// Ignores private members while preserving the containing namespace identity.
fn add(node: Node<'_>, name: &str, parent: Option<&str>, kind: &str, output: &mut Parsed) {
    if name.starts_with('_') {
        return;
    }

    let symbol = parent.map_or_else(|| name.to_owned(), |owner| format!("{owner}.{name}"));
    let root = parent.unwrap_or(name).to_owned();
    output.declarations.push(Declaration {
        symbol,
        root,
        kind: kind.into(),
        line: node.start_position().row + 1,
    });
}

pub(crate) fn text<'a>(node: Node<'_>, source: &'a str) -> &'a str {
    &source[node.byte_range()]
}

pub(crate) fn find<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    if node.kind() == kind {
        return Some(node);
    }

    node.named_children(&mut node.walk())
        .find_map(|child| find(child, kind))
}

pub(crate) fn descendants<'a>(node: Node<'a>, kind: &str) -> Vec<Node<'a>> {
    let mut found = Vec::new();

    if node.kind() == kind {
        found.push(node);
    }

    for child in node.named_children(&mut node.walk()) {
        found.extend(descendants(child, kind));
    }

    found
}
