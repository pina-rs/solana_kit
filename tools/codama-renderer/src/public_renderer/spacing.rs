//! Logical statement separation is derived from parsed Rust, never literal text.
use std::collections::BTreeSet;

use syn::{Expr, Stmt, spanned::Spanned, visit::Visit};

#[derive(Default)]
struct Spacing {
    before: BTreeSet<usize>,
}

fn control(statement: &Stmt) -> bool {
    matches!(
        statement,
        Stmt::Expr(
            Expr::If(_)
                | Expr::Match(_)
                | Expr::ForLoop(_)
                | Expr::While(_)
                | Expr::Loop(_)
                | Expr::Return(_),
            _
        )
    )
}

impl<'ast> Visit<'ast> for Spacing {
    fn visit_block(&mut self, block: &'ast syn::Block) {
        for pair in block.stmts.windows(2) {
            if control(&pair[0]) || control(&pair[1]) {
                self.before.insert(pair[1].span().start().line);
            }
        }

        syn::visit::visit_block(self, block);
    }
}

pub(super) fn format(syntax: &syn::File) -> Result<String, crate::schema::Error> {
    let canonical = prettyplease::unparse(syntax);
    let parsed = syn::parse_file(&canonical)?;
    let mut spacing = Spacing::default();
    spacing.visit_file(&parsed);
    let mut output = String::with_capacity(canonical.len());

    for (index, line) in canonical.lines().enumerate() {
        if spacing.before.contains(&(index + 1)) && !output.ends_with("\n\n") {
            output.push('\n');
        }

        output.push_str(line);
        output.push('\n');
    }

    if prettyplease::unparse(&syn::parse_file(&output)?) != canonical {
        return Err("Rust statement spacing changed parsed source".into());
    }

    Ok(output)
}
